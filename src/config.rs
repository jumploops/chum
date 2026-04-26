use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub version: u32,
    pub active_dirs: Vec<String>,
    pub archive_dir: String,
    pub live_doc_glob: String,
    pub source: SourceConfig,
    pub specs: SpecConfig,
    pub markers: MarkerConfig,
    pub swim: SwimConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceConfig {
    pub respect_gitignore: bool,
    pub ignore_files: Vec<String>,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecConfig {
    pub placement: String,
    pub file_pattern: String,
    pub directory_pattern: String,
    pub root_spec: String,
    pub backmatter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerConfig {
    pub todo: Vec<String>,
    pub unknown: Vec<String>,
    pub verify: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwimConfig {
    pub provider: String,
    pub max_passes: usize,
    pub concurrency: usize,
    pub require_empty_todo_unknown_and_verify: bool,
    pub allow_external_verify: bool,
    pub openai: OpenAiSwimConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiSwimConfig {
    pub auth: OpenAiAuthMode,
    pub codex_binary: String,
    pub model: Option<String>,
    pub reasoning_effort: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenAiAuthMode {
    Auto,
    Codex,
    ApiKey,
}

impl OpenAiAuthMode {
    pub fn parse(value: &str) -> std::result::Result<Self, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "codex" => Ok(Self::Codex),
            "apikey" | "api-key" | "api_key" => Ok(Self::ApiKey),
            other => Err(format!(
                "invalid OpenAI auth mode `{other}`; expected auto, codex, or apiKey"
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Codex => "codex",
            Self::ApiKey => "apiKey",
        }
    }
}

impl fmt::Display for OpenAiAuthMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Serialize for OpenAiAuthMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OpenAiAuthMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialConfig {
    version: Option<u32>,
    active_dirs: Option<Vec<String>>,
    archive_dir: Option<String>,
    live_doc_glob: Option<String>,
    source: Option<PartialSourceConfig>,
    specs: Option<PartialSpecConfig>,
    markers: Option<PartialMarkerConfig>,
    swim: Option<PartialSwimConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialSourceConfig {
    respect_gitignore: Option<bool>,
    ignore_files: Option<Vec<String>>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialSpecConfig {
    placement: Option<String>,
    file_pattern: Option<String>,
    directory_pattern: Option<String>,
    root_spec: Option<String>,
    backmatter: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialMarkerConfig {
    todo: Option<Vec<String>>,
    unknown: Option<Vec<String>>,
    verify: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialSwimConfig {
    provider: Option<String>,
    max_passes: Option<usize>,
    concurrency: Option<usize>,
    require_empty_todo_unknown_and_verify: Option<bool>,
    allow_external_verify: Option<bool>,
    openai: Option<PartialOpenAiSwimConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialOpenAiSwimConfig {
    auth: Option<OpenAiAuthMode>,
    codex_binary: Option<String>,
    model: Option<Option<String>>,
    reasoning_effort: Option<Option<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            active_dirs: vec![
                "design".into(),
                "plan".into(),
                "debug".into(),
                "review".into(),
            ],
            archive_dir: "archive".into(),
            live_doc_glob: "**/*.spec.md".into(),
            source: SourceConfig {
                respect_gitignore: true,
                ignore_files: vec![".gitignore".into(), ".chumignore".into()],
                include: vec![
                    "**/*.{c,cc,cpp,cxx,h,hpp,cs,css,go,html,java,js,jsx,kt,kts,m,mm,php,py,rb,rs,scss,sh,swift,ts,tsx,vue}".into(),
                ],
                exclude: vec![
                    ".git/**".into(),
                    ".hg/**".into(),
                    ".svn/**".into(),
                    "node_modules/**".into(),
                    "vendor/**".into(),
                    "dist/**".into(),
                    "build/**".into(),
                    "target/**".into(),
                    "coverage/**".into(),
                    "archive/**".into(),
                    "**/{test,tests,__tests__,spec,specs,fixture,fixtures,script,scripts,migration,migrations}/**".into(),
                    "**/*.{test,spec}.{js,jsx,ts,tsx,py,rb,go,rs,swift,java,kt,kts,cs,php}".into(),
                    "**/*config.{js,jsx,ts,tsx,cjs,mjs,json,yaml,yml,toml}".into(),
                    "**/*.min.*".into(),
                    "**/*.generated.*".into(),
                ],
            },
            specs: SpecConfig {
                placement: "inline".into(),
                file_pattern: "{path}.spec.md".into(),
                directory_pattern: "{dir}/{basename}.spec.md".into(),
                root_spec: "repo.spec.md".into(),
                backmatter: "required".into(),
            },
            markers: MarkerConfig {
                todo: vec!["SPEC:TODO".into()],
                unknown: vec!["SPEC:UNKNOWN".into()],
                verify: vec!["SPEC:VERIFY".into()],
            },
            swim: SwimConfig {
                provider: "openai".into(),
                max_passes: 5,
                concurrency: 4,
                require_empty_todo_unknown_and_verify: true,
                allow_external_verify: false,
                openai: OpenAiSwimConfig {
                    auth: OpenAiAuthMode::Auto,
                    codex_binary: "codex".into(),
                    model: None,
                    reasoning_effort: None,
                },
            },
        }
    }
}

impl Config {
    pub fn load(root: &Path) -> Result<Self> {
        let mut config = Self::default();
        let path = root.join("chum.config.yaml");
        if path.exists() {
            let raw = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            let partial: PartialConfig = serde_yaml::from_str(&raw)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            config.apply(partial);
        }
        Ok(config)
    }

    pub fn default_yaml() -> Result<String> {
        Ok(serde_yaml::to_string(&Self::default())?)
    }

    fn apply(&mut self, partial: PartialConfig) {
        if let Some(version) = partial.version {
            self.version = version;
        }
        if let Some(active_dirs) = partial.active_dirs {
            self.active_dirs = active_dirs;
        }
        if let Some(archive_dir) = partial.archive_dir {
            self.archive_dir = archive_dir;
        }
        if let Some(live_doc_glob) = partial.live_doc_glob {
            self.live_doc_glob = live_doc_glob;
        }
        if let Some(source) = partial.source {
            if let Some(value) = source.respect_gitignore {
                self.source.respect_gitignore = value;
            }
            if let Some(value) = source.ignore_files {
                self.source.ignore_files = value;
            }
            if let Some(value) = source.include {
                self.source.include = value;
            }
            if let Some(value) = source.exclude {
                self.source.exclude = value;
            }
        }
        if let Some(specs) = partial.specs {
            if let Some(value) = specs.placement {
                self.specs.placement = value;
            }
            if let Some(value) = specs.file_pattern {
                self.specs.file_pattern = value;
            }
            if let Some(value) = specs.directory_pattern {
                self.specs.directory_pattern = value;
            }
            if let Some(value) = specs.root_spec {
                self.specs.root_spec = value;
            }
            if let Some(value) = specs.backmatter {
                self.specs.backmatter = value;
            }
        }
        if let Some(markers) = partial.markers {
            if let Some(value) = markers.todo {
                self.markers.todo = value;
            }
            if let Some(value) = markers.unknown {
                self.markers.unknown = value;
            }
            if let Some(value) = markers.verify {
                self.markers.verify = value;
            }
        }
        if let Some(swim) = partial.swim {
            if let Some(value) = swim.provider {
                self.swim.provider = value;
            }
            if let Some(value) = swim.max_passes {
                self.swim.max_passes = value;
            }
            if let Some(value) = swim.concurrency {
                self.swim.concurrency = value;
            }
            if let Some(value) = swim.require_empty_todo_unknown_and_verify {
                self.swim.require_empty_todo_unknown_and_verify = value;
            }
            if let Some(value) = swim.allow_external_verify {
                self.swim.allow_external_verify = value;
            }
            if let Some(openai) = swim.openai {
                if let Some(value) = openai.auth {
                    self.swim.openai.auth = value;
                }
                if let Some(value) = openai.codex_binary {
                    self.swim.openai.codex_binary = value;
                }
                if let Some(value) = openai.model {
                    self.swim.openai.model = value;
                }
                if let Some(value) = openai.reasoning_effort {
                    self.swim.openai.reasoning_effort = value;
                }
            }
        }
    }
}

pub fn normalize_root(path: &Path) -> Result<Utf8PathBuf> {
    let root = if path.exists() {
        path.canonicalize()
            .with_context(|| format!("failed to resolve {}", path.display()))?
    } else {
        path.to_path_buf()
    };
    Utf8PathBuf::from_path_buf(root)
        .map_err(|path| anyhow::anyhow!("path is not valid UTF-8: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn partial_openai_swim_config_merges_with_defaults() {
        let temp = tempdir().unwrap();
        fs::write(
            temp.path().join("chum.config.yaml"),
            "swim:\n  openai:\n    auth: codex\n",
        )
        .unwrap();

        let config = Config::load(temp.path()).unwrap();

        assert_eq!(config.swim.openai.auth, OpenAiAuthMode::Codex);
        assert_eq!(config.swim.openai.codex_binary, "codex");
        assert_eq!(config.swim.max_passes, 5);
    }

    #[test]
    fn auth_mode_accepts_api_key_spellings() {
        assert_eq!(
            OpenAiAuthMode::parse("apiKey").unwrap(),
            OpenAiAuthMode::ApiKey
        );
        assert_eq!(
            OpenAiAuthMode::parse("api-key").unwrap(),
            OpenAiAuthMode::ApiKey
        );
        assert!(OpenAiAuthMode::parse("browser").is_err());
    }
}
