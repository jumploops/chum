use crate::config::{OpenAiAuthMode, OpenAiSwimConfig};
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

const GUIDANCE: &str = "Run codex login, run codex login --device-auth, set CODEX_API_KEY for codex exec, or set OPENAI_API_KEY for direct API fallback.";
const DEFAULT_DIRECT_MODEL: &str = "gpt-4.1-mini";

pub trait EnvLookup {
    fn var(&self, name: &str) -> Option<String>;
}

pub struct SystemEnv;

impl EnvLookup for SystemEnv {
    fn var(&self, name: &str) -> Option<String> {
        env::var(name).ok().and_then(non_empty)
    }
}

pub trait CodexStatusProbe {
    fn find_binary(&self, configured: &str) -> Option<PathBuf>;
    fn login_status(&self, binary: &Path) -> CodexLoginStatus;
}

pub struct SystemCodexStatusProbe;

impl CodexStatusProbe for SystemCodexStatusProbe {
    fn find_binary(&self, configured: &str) -> Option<PathBuf> {
        find_binary(configured)
    }

    fn login_status(&self, binary: &Path) -> CodexLoginStatus {
        match Command::new(binary).args(["login", "status"]).output() {
            Ok(output) => {
                let summary = command_summary(&output.stdout, &output.stderr)
                    .unwrap_or_else(|| "codex login status produced no output".into());
                CodexLoginStatus {
                    success: output.status.success(),
                    summary,
                }
            }
            Err(error) => CodexLoginStatus {
                success: false,
                summary: format!("failed to run codex login status: {error}"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodexLoginStatus {
    pub success: bool,
    pub summary: String,
}

#[derive(Clone)]
pub struct CodexExecConfig {
    pub binary: PathBuf,
    pub model: Option<String>,
    pub reasoning_effort: Option<String>,
    pub strict_chatgpt: bool,
}

#[derive(Clone)]
pub struct OpenAiApiKeyConfig {
    pub api_key: String,
    pub api_key_env: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiAuthStatus {
    pub provider: String,
    pub requested_auth_mode: String,
    pub resolved_auth_mode: String,
    pub codex_binary: Option<String>,
    pub codex_status: Option<String>,
    pub codex_api_key_present: bool,
    pub direct_api_key_present: bool,
    pub direct_api_key_env: Option<String>,
    pub guidance: Option<String>,
}

pub enum OpenAiAuthResolution {
    CodexExec {
        config: CodexExecConfig,
        status: OpenAiAuthStatus,
    },
    DirectApiKey {
        config: OpenAiApiKeyConfig,
        status: OpenAiAuthStatus,
    },
    Missing {
        status: OpenAiAuthStatus,
    },
}

impl OpenAiAuthResolution {
    #[cfg(test)]
    pub fn status(&self) -> &OpenAiAuthStatus {
        match self {
            Self::CodexExec { status, .. }
            | Self::DirectApiKey { status, .. }
            | Self::Missing { status } => status,
        }
    }

    pub fn into_status(self) -> OpenAiAuthStatus {
        match self {
            Self::CodexExec { status, .. }
            | Self::DirectApiKey { status, .. }
            | Self::Missing { status } => status,
        }
    }
}

#[derive(Debug, Clone)]
struct OpenAiAuthSettings {
    requested_auth: OpenAiAuthMode,
    codex_binary: String,
    codex_model: Option<String>,
    codex_reasoning_effort: Option<String>,
    direct_model: String,
    strict_chatgpt: bool,
}

pub fn resolve_openai_auth(
    config: &OpenAiSwimConfig,
    env: &dyn EnvLookup,
    probe: &dyn CodexStatusProbe,
) -> Result<OpenAiAuthResolution> {
    let settings = OpenAiAuthSettings::from_config(config, env)?;
    let codex_api_key_present = env.var("CODEX_API_KEY").is_some();
    let direct_api_key = detect_direct_api_key(env).map(|mut key| {
        key.model = settings.direct_model.clone();
        key
    });
    match settings.requested_auth {
        OpenAiAuthMode::ApiKey => {
            return Ok(match direct_api_key {
                Some(api_key) => direct_resolution(settings, api_key, None, None, false),
                None => missing_resolution(settings, None, None, codex_api_key_present, None),
            });
        }
        OpenAiAuthMode::Auto | OpenAiAuthMode::Codex => {}
    }

    let codex_binary = probe.find_binary(&settings.codex_binary);
    let codex_status = codex_binary
        .as_deref()
        .map(|binary| probe.login_status(binary));
    let codex_ready = codex_binary.is_some()
        && (codex_api_key_present || codex_status.as_ref().is_some_and(|status| status.success));

    if codex_ready {
        let binary = codex_binary.expect("checked codex binary presence");
        return Ok(codex_resolution(
            settings,
            binary,
            codex_status,
            codex_api_key_present,
            direct_api_key.as_ref(),
        ));
    }

    if settings.requested_auth == OpenAiAuthMode::Auto {
        if let Some(api_key) = direct_api_key {
            return Ok(direct_resolution(
                settings,
                api_key,
                codex_binary,
                codex_status,
                codex_api_key_present,
            ));
        }
    }

    Ok(missing_resolution(
        settings,
        codex_binary,
        codex_status,
        codex_api_key_present,
        direct_api_key.as_ref(),
    ))
}

impl OpenAiAuthSettings {
    fn from_config(config: &OpenAiSwimConfig, env: &dyn EnvLookup) -> Result<Self> {
        let requested_auth = match env.var("CHUM_OPENAI_AUTH") {
            Some(value) => OpenAiAuthMode::parse(&value).map_err(anyhow::Error::msg)?,
            None => config.auth,
        };
        let codex_binary = env
            .var("CHUM_CODEX_BINARY")
            .unwrap_or_else(|| config.codex_binary.clone());
        let codex_model = env.var("CHUM_CODEX_MODEL").or_else(|| config.model.clone());
        let codex_reasoning_effort = env
            .var("CHUM_CODEX_REASONING_EFFORT")
            .or_else(|| config.reasoning_effort.clone());
        let direct_model = env
            .var("CHUM_OPENAI_MODEL")
            .or_else(|| config.model.clone())
            .unwrap_or_else(|| DEFAULT_DIRECT_MODEL.into());
        let strict_chatgpt = env
            .var("CHUM_CODEX_STRICT_CHATGPT")
            .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"));

        Ok(Self {
            requested_auth,
            codex_binary,
            codex_model,
            codex_reasoning_effort,
            direct_model,
            strict_chatgpt,
        })
    }
}

fn codex_resolution(
    settings: OpenAiAuthSettings,
    binary: PathBuf,
    codex_status: Option<CodexLoginStatus>,
    codex_api_key_present: bool,
    direct_api_key: Option<&OpenAiApiKeyConfig>,
) -> OpenAiAuthResolution {
    let status = base_status(
        &settings,
        "codex",
        Some(binary.display().to_string()),
        codex_status,
        codex_api_key_present,
        direct_api_key,
        None,
    );
    OpenAiAuthResolution::CodexExec {
        config: CodexExecConfig {
            binary,
            model: settings.codex_model,
            reasoning_effort: settings.codex_reasoning_effort,
            strict_chatgpt: settings.strict_chatgpt,
        },
        status,
    }
}

fn direct_resolution(
    settings: OpenAiAuthSettings,
    api_key: OpenAiApiKeyConfig,
    codex_binary: Option<PathBuf>,
    codex_status: Option<CodexLoginStatus>,
    codex_api_key_present: bool,
) -> OpenAiAuthResolution {
    let status = base_status(
        &settings,
        "apiKey",
        codex_binary.map(|path| path.display().to_string()),
        codex_status,
        codex_api_key_present,
        Some(&api_key),
        None,
    );
    OpenAiAuthResolution::DirectApiKey {
        config: api_key,
        status,
    }
}

fn missing_resolution(
    settings: OpenAiAuthSettings,
    codex_binary: Option<PathBuf>,
    codex_status: Option<CodexLoginStatus>,
    codex_api_key_present: bool,
    direct_api_key: Option<&OpenAiApiKeyConfig>,
) -> OpenAiAuthResolution {
    OpenAiAuthResolution::Missing {
        status: base_status(
            &settings,
            "missing",
            codex_binary.map(|path| path.display().to_string()),
            codex_status,
            codex_api_key_present,
            direct_api_key,
            Some(GUIDANCE.into()),
        ),
    }
}

fn base_status(
    settings: &OpenAiAuthSettings,
    resolved_auth_mode: &str,
    codex_binary: Option<String>,
    codex_status: Option<CodexLoginStatus>,
    codex_api_key_present: bool,
    direct_api_key: Option<&OpenAiApiKeyConfig>,
    guidance: Option<String>,
) -> OpenAiAuthStatus {
    OpenAiAuthStatus {
        provider: "openai".into(),
        requested_auth_mode: settings.requested_auth.to_string(),
        resolved_auth_mode: resolved_auth_mode.into(),
        codex_binary,
        codex_status: codex_status.map(|status| status.summary),
        codex_api_key_present,
        direct_api_key_present: direct_api_key.is_some(),
        direct_api_key_env: direct_api_key.map(|key| key.api_key_env.clone()),
        guidance,
    }
}

pub fn detect_direct_api_key(env: &dyn EnvLookup) -> Option<OpenAiApiKeyConfig> {
    for name in [
        "CHUM_OPENAI_API_KEY",
        "CODEX_OPENAI_API_KEY",
        "OPENAI_API_KEY",
    ] {
        if let Some(api_key) = env.var(name) {
            return Some(OpenAiApiKeyConfig {
                api_key,
                api_key_env: name.into(),
                model: env
                    .var("CHUM_OPENAI_MODEL")
                    .unwrap_or_else(|| DEFAULT_DIRECT_MODEL.into()),
            });
        }
    }
    None
}

fn find_binary(configured: &str) -> Option<PathBuf> {
    let configured_path = PathBuf::from(configured);
    if configured_path.is_absolute() || configured_path.components().count() > 1 {
        return configured_path.is_file().then_some(configured_path);
    }

    env::var_os("PATH").and_then(|path| {
        env::split_paths(&path)
            .map(|dir| dir.join(configured))
            .find(|candidate| candidate.is_file())
    })
}

fn command_summary(stdout: &[u8], stderr: &[u8]) -> Option<String> {
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(stdout));
    if !stderr.is_empty() {
        combined.push('\n');
        combined.push_str(&String::from_utf8_lossy(stderr));
    }
    combined
        .lines()
        .rev()
        .find_map(|line| non_empty(line.to_string()))
}

fn non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub fn missing_auth_error(status: &OpenAiAuthStatus) -> anyhow::Error {
    if status.requested_auth_mode == "apiKey" {
        return anyhow!(
            "Direct OpenAI API key not found. Set CHUM_OPENAI_API_KEY or OPENAI_API_KEY."
        );
    }
    anyhow!(
        "OpenAI auth not available for requested mode `{}`. {}",
        status.requested_auth_mode,
        status.guidance.as_deref().unwrap_or(GUIDANCE)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct FakeEnv(BTreeMap<String, String>);

    impl FakeEnv {
        fn with(mut self, key: &str, value: &str) -> Self {
            self.0.insert(key.into(), value.into());
            self
        }
    }

    impl EnvLookup for FakeEnv {
        fn var(&self, name: &str) -> Option<String> {
            self.0.get(name).cloned()
        }
    }

    struct FakeProbe {
        binary: Option<PathBuf>,
        login_success: bool,
        probes: std::cell::Cell<usize>,
    }

    impl FakeProbe {
        fn logged_in() -> Self {
            Self {
                binary: Some(PathBuf::from("/tmp/codex")),
                login_success: true,
                probes: std::cell::Cell::new(0),
            }
        }

        fn missing() -> Self {
            Self {
                binary: None,
                login_success: false,
                probes: std::cell::Cell::new(0),
            }
        }
    }

    impl CodexStatusProbe for FakeProbe {
        fn find_binary(&self, _configured: &str) -> Option<PathBuf> {
            self.probes.set(self.probes.get() + 1);
            self.binary.clone()
        }

        fn login_status(&self, _binary: &Path) -> CodexLoginStatus {
            CodexLoginStatus {
                success: self.login_success,
                summary: if self.login_success {
                    "Logged in using ChatGPT".into()
                } else {
                    "Not logged in".into()
                },
            }
        }
    }

    #[test]
    fn auto_prefers_logged_in_codex() {
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::Auto,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default().with("OPENAI_API_KEY", "sk-test"),
            &FakeProbe::logged_in(),
        )
        .unwrap();

        assert_eq!(resolution.status().resolved_auth_mode, "codex");
    }

    #[test]
    fn auto_uses_codex_api_key_when_login_fails() {
        let probe = FakeProbe {
            binary: Some(PathBuf::from("/tmp/codex")),
            login_success: false,
            probes: std::cell::Cell::new(0),
        };
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::Auto,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default().with("CODEX_API_KEY", "sk-test"),
            &probe,
        )
        .unwrap();

        assert_eq!(resolution.status().resolved_auth_mode, "codex");
    }

    #[test]
    fn auto_falls_back_to_direct_api_key() {
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::Auto,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default().with("CHUM_OPENAI_API_KEY", "sk-test"),
            &FakeProbe::missing(),
        )
        .unwrap();

        assert_eq!(resolution.status().resolved_auth_mode, "apiKey");
        assert_eq!(
            resolution.status().direct_api_key_env.as_deref(),
            Some("CHUM_OPENAI_API_KEY")
        );
    }

    #[test]
    fn forced_codex_does_not_fall_back() {
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::Codex,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default().with("OPENAI_API_KEY", "sk-test"),
            &FakeProbe::missing(),
        )
        .unwrap();

        assert_eq!(resolution.status().resolved_auth_mode, "missing");
    }

    #[test]
    fn forced_api_key_does_not_probe_codex() {
        let probe = FakeProbe::logged_in();
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::ApiKey,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default().with("OPENAI_API_KEY", "sk-test"),
            &probe,
        )
        .unwrap();

        assert_eq!(resolution.status().resolved_auth_mode, "apiKey");
        assert_eq!(probe.probes.get(), 0);
    }

    #[test]
    fn env_auth_override_is_parsed() {
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::Codex,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default()
                .with("CHUM_OPENAI_AUTH", "api-key")
                .with("OPENAI_API_KEY", "sk-test"),
            &FakeProbe::logged_in(),
        )
        .unwrap();

        assert_eq!(resolution.status().requested_auth_mode, "apiKey");
        assert_eq!(resolution.status().resolved_auth_mode, "apiKey");
    }

    #[test]
    fn direct_api_key_precedence_prefers_chum_env() {
        let key = detect_direct_api_key(
            &FakeEnv::default()
                .with("OPENAI_API_KEY", "sk-openai")
                .with("CODEX_OPENAI_API_KEY", "sk-codex")
                .with("CHUM_OPENAI_API_KEY", "sk-chum"),
        )
        .unwrap();

        assert_eq!(key.api_key_env, "CHUM_OPENAI_API_KEY");
        assert_eq!(key.api_key, "sk-chum");
    }

    #[test]
    fn direct_api_key_precedence_preserves_legacy_order() {
        let key = detect_direct_api_key(
            &FakeEnv::default()
                .with("OPENAI_API_KEY", "sk-openai")
                .with("CODEX_OPENAI_API_KEY", "sk-codex"),
        )
        .unwrap();

        assert_eq!(key.api_key_env, "CODEX_OPENAI_API_KEY");
        assert_eq!(key.api_key, "sk-codex");

        let key =
            detect_direct_api_key(&FakeEnv::default().with("OPENAI_API_KEY", "sk-openai")).unwrap();
        assert_eq!(key.api_key_env, "OPENAI_API_KEY");
    }

    #[test]
    fn forced_api_key_missing_error_mentions_direct_mode() {
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::ApiKey,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default(),
            &FakeProbe::logged_in(),
        )
        .unwrap();
        let error = missing_auth_error(resolution.status()).to_string();

        assert!(error.contains("Direct OpenAI API key not found"));
    }

    #[test]
    fn missing_auth_status_does_not_include_secret_values() {
        let resolution = resolve_openai_auth(
            &OpenAiSwimConfig {
                auth: OpenAiAuthMode::Codex,
                codex_binary: "codex".into(),
                model: None,
                reasoning_effort: None,
            },
            &FakeEnv::default().with("OPENAI_API_KEY", "sk-secret"),
            &FakeProbe::missing(),
        )
        .unwrap();

        let serialized = serde_json::to_string(resolution.status()).unwrap();
        assert!(serialized.contains("OPENAI_API_KEY"));
        assert!(!serialized.contains("sk-secret"));
    }
}
