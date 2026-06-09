#!/usr/bin/env python3
# /// script
# requires-python = ">=3.9"
# dependencies = []
# ///
"""Deterministic processor for the chum repository documentation skill."""

from __future__ import annotations

import argparse
import dataclasses
import datetime as _dt
import fnmatch
import hashlib
import json
import os
import re
import shutil
import sys
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Sequence, Tuple


OPEN = "<!-- chum:backmatter"
CLOSE = "-->"
SOURCE_EXTENSIONS = {
    ".c",
    ".cc",
    ".cpp",
    ".cxx",
    ".h",
    ".hpp",
    ".cs",
    ".css",
    ".go",
    ".html",
    ".java",
    ".js",
    ".jsx",
    ".kt",
    ".kts",
    ".m",
    ".mm",
    ".php",
    ".py",
    ".rb",
    ".rs",
    ".scss",
    ".sh",
    ".swift",
    ".ts",
    ".tsx",
    ".vue",
}
TEXT_EXTENSIONS = {".md", ".markdown", ".txt", ".rst", ".adoc"}
DEFAULT_EXCLUDE = [
    ".git/**",
    ".hg/**",
    ".svn/**",
    "node_modules/**",
    "vendor/**",
    "dist/**",
    "build/**",
    "target/**",
    "coverage/**",
    "archive/**",
    "**/{test,tests,__tests__,spec,specs,fixture,fixtures,script,scripts,migration,migrations}/**",
    "**/*.{test,spec}.{js,jsx,ts,tsx,py,rb,go,rs,swift,java,kt,kts,cs,php}",
    "**/*config.{js,jsx,ts,tsx,cjs,mjs,json,yaml,yml,toml}",
    "**/*.min.*",
    "**/*.generated.*",
]
MARKERS = {
    "todo": "SPEC:TODO",
    "unknowns": "SPEC:UNKNOWN",
    "verify": "SPEC:VERIFY",
}
SKILL_ROOT = Path(__file__).resolve().parents[1]
AGENTS_SNIPPET = SKILL_ROOT / "references" / "agents-snippet.md"


class ChumError(Exception):
    def __init__(self, message: str, code: int = 3):
        super().__init__(message)
        self.code = code


@dataclasses.dataclass
class Config:
    version: int = 1
    active_dirs: List[str] = dataclasses.field(
        default_factory=lambda: ["design", "plan", "debug", "review"]
    )
    archive_dir: str = "archive"
    live_doc_glob: str = "**/*.spec.md"
    source_respect_gitignore: bool = True
    source_ignore_files: List[str] = dataclasses.field(
        default_factory=lambda: [".gitignore", ".chumignore"]
    )
    source_include: List[str] = dataclasses.field(
        default_factory=lambda: ["**/*" + ext for ext in sorted(SOURCE_EXTENSIONS)]
    )
    source_exclude: List[str] = dataclasses.field(default_factory=lambda: list(DEFAULT_EXCLUDE))
    root_spec: str = "repo.spec.md"


@dataclasses.dataclass
class SourceFile:
    rel_path: str
    abs_path: Path
    spec_path: Path


@dataclasses.dataclass
class SourceDir:
    rel_path: str
    abs_path: Path
    spec_path: Path


@dataclasses.dataclass
class Discovery:
    root: Path
    source_files: List[SourceFile]
    source_dirs: List[SourceDir]
    live_specs: List[str]
    active_docs: List[str]
    archive_docs: List[str]
    ignored_count: int
    warnings: List[str]


@dataclasses.dataclass
class ParsedBackmatter:
    backmatter: Dict[str, Any]
    start: int
    end: int
    line: int


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    if not hasattr(args, "handler"):
        parser.print_help()
        return 0
    try:
        result_code = args.handler(args)
        return int(result_code or 0)
    except ChumError as error:
        print(f"error: {error}", file=sys.stderr)
        return error.code
    except BrokenPipeError:
        return 1
    except Exception as error:  # pragma: no cover - defensive top-level guard
        print(f"error: {error}", file=sys.stderr)
        return 3


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="chum.py",
        description="Deterministic processor for chum live specs and workflow docs.",
    )
    sub = parser.add_subparsers(dest="command")

    targets = sub.add_parser("targets", help="List specs that need agent attention.")
    add_root(targets)
    targets.add_argument("--json", action="store_true", help="Print JSON output.")
    targets.add_argument("--kind", choices=["file", "directory", "root", "all"], default="all")
    targets.add_argument(
        "--reason",
        choices=["missing", "stale", "invalid", "incomplete", "all"],
        default="all",
    )
    targets.add_argument("--limit", type=int, default=None, help="Maximum targets to print.")
    targets.add_argument("--offset", type=int, default=0, help="Targets to skip before printing.")
    targets.add_argument("--output", help="Write JSON output to this path.")
    targets.set_defaults(handler=cmd_targets)

    check = sub.add_parser("check", help="Validate live specs for a repository.")
    add_root(check)
    check.add_argument("--json", action="store_true", help="Print JSON output.")
    check.add_argument("--allow-stale", action="store_true", help="Do not fail stale hashes.")
    check.add_argument(
        "--allow-external-verify",
        action="store_true",
        help="Allow verify lists and SPEC:VERIFY markers.",
    )
    check.add_argument("--include", action="append", default=[], help="Explicit include glob.")
    check.add_argument("--include-archive", action="store_true", help="Include archive docs.")
    check.set_defaults(handler=cmd_check)

    normalize = sub.add_parser("normalize", help="Normalize a spec and update chum backmatter.")
    add_root(normalize)
    normalize.add_argument("--target", required=True, help="Source file or directory target.")
    normalize.add_argument("--spec", help="Override spec path.")
    normalize.add_argument("--input", help="Read Markdown from a file.")
    normalize.add_argument("--stdin", action="store_true", help="Read Markdown from stdin.")
    normalize.add_argument("--write", action="store_true", help="Write the normalized spec.")
    normalize.add_argument("--json", action="store_true", help="Print write metadata as JSON.")
    normalize.set_defaults(handler=cmd_normalize)

    validate = sub.add_parser("validate", help="Validate one target/spec pair.")
    add_root(validate)
    validate.add_argument("--target", required=True, help="Source file or directory target.")
    validate.add_argument("--json", action="store_true", help="Print JSON output.")
    validate.add_argument("--allow-stale", action="store_true")
    validate.add_argument("--allow-external-verify", action="store_true")
    validate.set_defaults(handler=cmd_validate)

    init = sub.add_parser("init", help="Initialize workflow directories and config.")
    add_root(init)
    init.add_argument("--write", action="store_true", help="Write planned changes.")
    init.add_argument("--dry-run", action="store_true", help="Preview changes without writing.")
    init.add_argument("--json", action="store_true", help="Print JSON output.")
    init.add_argument(
        "--with-agents-template",
        action="store_true",
        help="Create or update AGENTS.md with a chum workflow snippet.",
    )
    init.set_defaults(handler=cmd_init)

    archive = sub.add_parser("archive", help="Archive completed change docs.")
    add_root(archive)
    archive.add_argument("change_id", help="Change identifier to archive.")
    archive.add_argument("--title", help="Archive title.")
    archive.add_argument("--include", action="append", default=[], help="Explicit include glob.")
    archive.add_argument("--exclude", action="append", default=[], help="Exclude glob.")
    archive.add_argument("--source-ref", help="Source branch/ref metadata.")
    archive.add_argument("--pr", help="Pull request metadata.")
    archive.add_argument("--write", action="store_true", help="Move files and write manifest.")
    archive.add_argument("--dry-run", action="store_true", help="Preview without writing.")
    archive.add_argument("--json", action="store_true", help="Print JSON output.")
    archive.add_argument("--force", action="store_true", help="Reserved escape hatch.")
    archive.set_defaults(handler=cmd_archive)

    return parser


def add_root(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--root", default=".", help="Repository root path.")


def cmd_targets(args: argparse.Namespace) -> int:
    root = normalize_root(args.root)
    config = load_config(root)
    discovery = discover(root, config)
    target_rows = build_targets(discovery, config, allow_stale=False, allow_external_verify=False)
    target_rows = filter_targets(target_rows, args.kind, args.reason)
    total = len(target_rows)
    rows = target_rows[args.offset :]
    if args.limit is not None:
        rows = rows[: args.limit]
    report = {
        "root": str(root),
        "summary": target_summary(discovery, target_rows),
        "offset": args.offset,
        "limit": args.limit,
        "totalTargets": total,
        "targets": rows,
        "warnings": discovery.warnings,
    }
    emit_report(report, json_output=args.json, output_path=args.output)
    return 0


def cmd_check(args: argparse.Namespace) -> int:
    report = check_report(
        normalize_root(args.root),
        allow_stale=args.allow_stale,
        allow_external_verify=args.allow_external_verify,
        explicit_include=args.include,
        include_archive=args.include_archive,
    )
    if args.json:
        print_json(report)
    elif report["failures"]:
        print(
            "chum check failed: "
            f"{len(report['failures'])} issue(s), "
            f"{report['sourceFiles']} source files, "
            f"{report['sourceDirs']} source directories",
            file=sys.stderr,
        )
        for failure in report["failures"]:
            print(f"- {failure['path']}: {failure['message']}", file=sys.stderr)
    else:
        print(
            f"chum check passed: {report['sourceFiles']} source files, "
            f"{report['sourceDirs']} source directories"
        )
    return 1 if report["failures"] else 0


def cmd_normalize(args: argparse.Namespace) -> int:
    root = normalize_root(args.root)
    config = load_config(root)
    markdown = read_markdown_input(args)
    normalized, spec_path, kind, target = normalize_markdown(
        root=root,
        config=config,
        target_arg=args.target,
        markdown=markdown,
        spec_override=args.spec,
    )
    if args.write:
        spec_path.parent.mkdir(parents=True, exist_ok=True)
        spec_path.write_text(normalized, encoding="utf-8")
        report = {
            "root": str(root),
            "target": target,
            "kind": kind,
            "specPath": rel_path(spec_path, root),
            "written": True,
        }
        if args.json:
            print_json(report)
        else:
            print(f"wrote {rel_path(spec_path, root)}")
    else:
        if args.json:
            print_json(
                {
                    "root": str(root),
                    "target": target,
                    "kind": kind,
                    "specPath": rel_path(spec_path, root),
                    "written": False,
                    "markdown": normalized,
                }
            )
        else:
            sys.stdout.write(normalized)
            if not normalized.endswith("\n"):
                sys.stdout.write("\n")
    return 0


def cmd_validate(args: argparse.Namespace) -> int:
    root = normalize_root(args.root)
    config = load_config(root)
    discovery = discover(root, config)
    target_rel = normalize_target_arg(args.target)
    failures: List[Dict[str, str]] = []
    matched = False

    for source in discovery.source_files:
        if source.rel_path == target_rel:
            matched = True
            failures.extend(validate_source_file(source, args.allow_stale, args.allow_external_verify))
            break
    for directory in discovery.source_dirs:
        if directory.rel_path == target_rel or (target_rel == "." and directory.rel_path == ""):
            matched = True
            failures.extend(validate_source_dir(directory, args.allow_external_verify))
            break

    if not matched:
        raise ChumError(f"target is not a discovered source file or directory: {args.target}", 1)

    report = {
        "root": str(root),
        "target": "." if target_rel == "" else target_rel,
        "failures": failures,
        "warnings": [],
    }
    if args.json:
        print_json(report)
    elif failures:
        for failure in failures:
            print(f"- {failure['path']}: {failure['message']}", file=sys.stderr)
    else:
        print(f"target clean: {report['target']}")
    return 1 if failures else 0


def cmd_init(args: argparse.Namespace) -> int:
    root = normalize_root(args.root)
    config = load_config(root)
    write = bool(args.write and not args.dry_run)
    planned: List[str] = []
    written: List[str] = []

    for dirname in [*config.active_dirs, config.archive_dir]:
        path = root / dirname
        if not path.exists():
            planned.append(f"create directory {dirname}")
            if write:
                path.mkdir(parents=True, exist_ok=True)
                written.append(dirname)

    archive_readme = root / config.archive_dir / "README.md"
    if not archive_readme.exists():
        planned.append(f"create {rel_path(archive_readme, root)}")
        if write:
            archive_readme.parent.mkdir(parents=True, exist_ok=True)
            archive_readme.write_text(
                "# Archive\n\nHistorical change artifacts live here. Treat live `*.spec.md` files as current truth.\n",
                encoding="utf-8",
            )
            written.append(rel_path(archive_readme, root))

    config_path = root / "chum.config.yaml"
    if not config_path.exists():
        planned.append("create chum.config.yaml")
        if write:
            config_path.write_text(default_config_yaml(), encoding="utf-8")
            written.append("chum.config.yaml")

    if args.with_agents_template:
        agent_doc = root / "AGENTS.md"
        planned.append("update AGENTS.md")
        if write:
            append_agent_snippet(agent_doc)
            written.append("AGENTS.md")

    report = {"root": str(root), "dryRun": not write, "planned": planned, "written": written}
    if args.json:
        print_json(report)
    elif not write:
        print("chum init dry run")
        for item in planned:
            print(f"- {item}")
    else:
        print(f"chum init wrote {len(written)} item(s)")
        for item in written:
            print(f"- {item}")
    return 0


def cmd_archive(args: argparse.Namespace) -> int:
    root = normalize_root(args.root)
    config = load_config(root)
    check = check_report(root, allow_stale=False, allow_external_verify=False)
    warnings: List[str] = []
    check_status = "failed" if check["failures"] else "passed"
    if check["failures"]:
        warnings.append(f"chum check failed before archive with {len(check['failures'])} issue(s)")

    moves = plan_archive_moves(root, config, args)
    if not moves:
        raise ChumError(f"no active Markdown docs matched change `{args.change_id}`", 1)

    write = bool(args.write and not args.dry_run)
    if write:
        final_warnings = list(warnings)
        move_map = {root / m["from"]: root / m["to"] for m in moves}
        for move in moves:
            src = root / move["from"]
            dest = root / move["to"]
            if dest.exists():
                raise ChumError(f"archive destination already exists: {rel_path(dest, root)}", 3)
            content = src.read_text(encoding="utf-8")
            rewritten, link_warnings = rewrite_markdown_links(content, src, dest, move_map, root)
            final_warnings.extend(link_warnings)
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_text(rewritten, encoding="utf-8")
        for move in moves:
            (root / move["from"]).unlink()
        write_archive_manifest(root, config, args, moves, final_warnings, check_status)
        warnings = final_warnings

    report = {
        "changeId": args.change_id,
        "root": str(root),
        "dryRun": not write,
        "checkStatus": check_status,
        "moved": moves,
        "warnings": warnings,
    }
    if args.json:
        print_json(report)
    elif not write:
        print(f"chum archive dry run: {len(moves)} file(s)")
        for move in moves:
            print(f"- {move['from']} -> {move['to']}")
        for warning in warnings:
            print(f"warning: {warning}", file=sys.stderr)
    else:
        print(f"chum archive moved {len(moves)} file(s)")
        for move in moves:
            print(f"- {move['from']} -> {move['to']}")
        for warning in warnings:
            print(f"warning: {warning}", file=sys.stderr)
    return 0


def normalize_root(path: str) -> Path:
    root = Path(path).expanduser()
    if root.exists():
        root = root.resolve()
    else:
        root = root.absolute()
    return root


def load_config(root: Path) -> Config:
    config = Config()
    path = root / "chum.config.yaml"
    if not path.exists():
        return config
    try:
        raw = parse_simple_yaml(path.read_text(encoding="utf-8"))
    except Exception as error:
        raise ChumError(f"failed to parse chum.config.yaml: {error}", 2)
    if not isinstance(raw, dict):
        return config
    config.version = int(raw.get("version", config.version))
    config.active_dirs = list(raw.get("activeDirs", config.active_dirs))
    config.archive_dir = str(raw.get("archiveDir", config.archive_dir))
    config.live_doc_glob = str(raw.get("liveDocGlob", config.live_doc_glob))
    source = raw.get("source", {})
    if isinstance(source, dict):
        config.source_respect_gitignore = bool(
            source.get("respectGitignore", config.source_respect_gitignore)
        )
        config.source_ignore_files = list(source.get("ignoreFiles", config.source_ignore_files))
        config.source_include = list(source.get("include", config.source_include))
        config.source_exclude = list(source.get("exclude", config.source_exclude))
    specs = raw.get("specs", {})
    if isinstance(specs, dict):
        config.root_spec = str(specs.get("rootSpec", config.root_spec))
    return config


def discover(
    root: Path,
    config: Config,
    explicit_include: Optional[List[str]] = None,
    include_archive: bool = False,
) -> Discovery:
    explicit_include = explicit_include or []
    ignore_patterns = load_ignore_patterns(root, config)
    source_files: List[SourceFile] = []
    source_dirs: set[str] = set()
    live_specs: List[str] = []
    active_docs: List[str] = []
    archive_docs: List[str] = []
    ignored_count = 0
    warnings: List[str] = []

    for current, dirs, files in os.walk(root):
        current_path = Path(current)
        rel_dir = "" if current_path == root else rel_path(current_path, root)
        kept_dirs = []
        for dirname in sorted(dirs):
            rel = join_rel(rel_dir, dirname)
            if is_ignored(rel + "/", config.source_exclude, ignore_patterns):
                ignored_count += 1
            else:
                kept_dirs.append(dirname)
        dirs[:] = kept_dirs

        for filename in sorted(files):
            abs_path = current_path / filename
            rel = rel_path(abs_path, root)
            if is_ignored(rel, [], ignore_patterns):
                ignored_count += 1
                continue
            if is_archive_doc(rel, config):
                archive_docs.append(rel)
                if not include_archive:
                    ignored_count += 1
                    continue
            if is_live_spec(rel):
                live_specs.append(rel)
                continue
            if is_active_doc(rel, config):
                active_docs.append(rel)
                continue
            explicitly_included = bool(explicit_include) and match_any(rel, explicit_include)
            if explicit_include and not explicitly_included:
                ignored_count += 1
                continue
            if not explicitly_included and not is_source_include(rel, config.source_include):
                ignored_count += 1
                continue
            if not explicitly_included and match_any(rel, config.source_exclude):
                ignored_count += 1
                continue
            if Path(rel).suffix in TEXT_EXTENSIONS:
                ignored_count += 1
                continue
            source_files.append(
                SourceFile(rel_path=rel, abs_path=abs_path, spec_path=file_spec_path(root, rel))
            )
            for parent in source_parent_dirs(rel):
                source_dirs.add(parent)

    dir_rows = [
        SourceDir(
            rel_path=reldir,
            abs_path=root if reldir == "" else root / reldir,
            spec_path=dir_spec_path(root, reldir, config),
        )
        for reldir in sorted(source_dirs)
    ]
    return Discovery(
        root=root,
        source_files=sorted(source_files, key=lambda item: item.rel_path),
        source_dirs=dir_rows,
        live_specs=sorted(live_specs),
        active_docs=sorted(active_docs),
        archive_docs=sorted(archive_docs),
        ignored_count=ignored_count,
        warnings=warnings,
    )


def load_ignore_patterns(root: Path, config: Config) -> List[str]:
    patterns: List[str] = []
    for name in config.source_ignore_files:
        if name == ".gitignore" and not config.source_respect_gitignore:
            continue
        path = root / name
        if not path.exists():
            continue
        for raw in path.read_text(encoding="utf-8").splitlines():
            line = raw.strip()
            if not line or line.startswith("#") or line.startswith("!"):
                continue
            if line.endswith("/"):
                line = line + "**"
            patterns.append(line)
    return patterns


def is_source_include(path: str, patterns: Sequence[str]) -> bool:
    suffix = Path(path).suffix
    if suffix in SOURCE_EXTENSIONS:
        return True
    return match_any(path, patterns)


def is_ignored(path: str, exclude_patterns: Sequence[str], ignore_patterns: Sequence[str]) -> bool:
    return match_any(path, exclude_patterns) or match_any(path, ignore_patterns)


def match_any(path: str, patterns: Sequence[str]) -> bool:
    normalized = path.strip("/")
    for pattern in patterns:
        for expanded in brace_expand(pattern):
            pat = expanded.strip("/")
            if not pat:
                continue
            candidates = [pat]
            if pat.startswith("**/"):
                candidates.append(pat[3:])
            for candidate in candidates:
                if candidate.endswith("/**"):
                    prefix = candidate[:-3].rstrip("/")
                    if normalized == prefix or normalized.startswith(prefix + "/"):
                        return True
                if "/" not in candidate and (
                    normalized == candidate or normalized.startswith(candidate + "/")
                ):
                    return True
                if fnmatch.fnmatch(normalized, candidate):
                    return True
    return False


def brace_expand(pattern: str) -> List[str]:
    match = re.search(r"\{([^{}]+)\}", pattern)
    if not match:
        return [pattern]
    before, after = pattern[: match.start()], pattern[match.end() :]
    results: List[str] = []
    for part in match.group(1).split(","):
        results.extend(brace_expand(before + part + after))
    return results


def is_live_spec(path: str) -> bool:
    return path.endswith(".spec.md")


def is_archive_doc(path: str, config: Config) -> bool:
    return path.startswith(config.archive_dir.rstrip("/") + "/") and is_markdown(path)


def is_active_doc(path: str, config: Config) -> bool:
    first = path.split("/", 1)[0]
    return first in config.active_dirs and is_markdown(path)


def is_markdown(path: str) -> bool:
    return Path(path).suffix in {".md", ".markdown"}


def file_spec_path(root: Path, rel_source: str) -> Path:
    return root / f"{rel_source}.spec.md"


def dir_spec_path(root: Path, rel_dir: str, config: Config) -> Path:
    if rel_dir in {"", "."}:
        return root / config.root_spec
    basename = Path(rel_dir).name or "repo"
    return root / rel_dir / f"{basename}.spec.md"


def source_parent_dirs(rel_source: str) -> List[str]:
    parents: List[str] = []
    parent = Path(rel_source).parent
    while str(parent) not in {"", "."}:
        parents.append(parent.as_posix())
        parent = parent.parent
    parents.append("")
    return parents


def build_targets(
    discovery: Discovery,
    config: Config,
    allow_stale: bool,
    allow_external_verify: bool,
) -> List[Dict[str, Any]]:
    rows: List[Dict[str, Any]] = []
    for source in discovery.source_files:
        reasons, counts = source_file_reasons(source, allow_stale, allow_external_verify)
        if reasons:
            rows.append(
                target_row(
                    discovery.root,
                    "file",
                    source.rel_path,
                    source.spec_path,
                    reasons,
                    counts,
                    source,
                )
            )
    for directory in discovery.source_dirs:
        reasons, counts = source_dir_reasons(directory, allow_external_verify)
        if reasons:
            rows.append(
                target_row(
                    discovery.root,
                    "root" if directory.rel_path == "" else "directory",
                    "." if directory.rel_path == "" else directory.rel_path,
                    directory.spec_path,
                    reasons,
                    counts,
                    None,
                    children=child_specs(discovery.root, directory.rel_path, discovery, config),
                )
            )
    return sorted(rows, key=lambda row: (row["kind"], row["target"]))


def target_row(
    root: Path,
    kind: str,
    target: str,
    spec_path: Path,
    reasons: List[str],
    counts: Dict[str, int],
    source: Optional[SourceFile],
    children: Optional[List[str]] = None,
) -> Dict[str, Any]:
    row: Dict[str, Any] = {
        "kind": kind,
        "target": target,
        "sourcePath": source.rel_path if source else None,
        "specPath": rel_path(spec_path, root),
        "reasons": reasons,
        "todo": counts.get("todo", 0),
        "unknowns": counts.get("unknowns", 0),
        "verify": counts.get("verify", 0),
        "children": children or [],
    }
    if source:
        row["sourceHash"] = sha256_file(source.abs_path)
        row["sourceUpdatedAt"] = modified_time(source.abs_path)
    return row

def target_summary(discovery: Discovery, targets: Sequence[Dict[str, Any]]) -> Dict[str, int]:
    return {
        "sourceFiles": len(discovery.source_files),
        "sourceDirs": len(discovery.source_dirs),
        "ignoredCount": discovery.ignored_count,
        "targets": len(targets),
        "missing": count_reason(targets, "missing"),
        "stale": count_reason(targets, "stale"),
        "invalid": count_reason(targets, "invalid_backmatter"),
        "incomplete": sum(
            1
            for target in targets
            if any(reason in target["reasons"] for reason in ["todo", "unknowns", "verify", "legacy_marker"])
        ),
    }


def count_reason(targets: Sequence[Dict[str, Any]], reason: str) -> int:
    return sum(1 for target in targets if reason in target["reasons"])


def filter_targets(rows: List[Dict[str, Any]], kind: str, reason: str) -> List[Dict[str, Any]]:
    if kind != "all":
        rows = [row for row in rows if row["kind"] == kind]
    if reason != "all":
        if reason == "invalid":
            rows = [row for row in rows if "invalid_backmatter" in row["reasons"]]
        elif reason == "incomplete":
            rows = [
                row
                for row in rows
                if any(r in row["reasons"] for r in ["todo", "unknowns", "verify", "legacy_marker"])
            ]
        else:
            rows = [row for row in rows if reason in row["reasons"]]
    return rows


def source_file_reasons(
    source: SourceFile,
    allow_stale: bool,
    allow_external_verify: bool,
) -> Tuple[List[str], Dict[str, int]]:
    if not source.spec_path.exists():
        return ["missing"], {"todo": 0, "unknowns": 0, "verify": 0}
    reasons: List[str] = []
    counts = {"todo": 0, "unknowns": 0, "verify": 0}
    try:
        parsed = parse_backmatter_file(source.spec_path)
        bm = parsed.backmatter
        if bm.get("kind") != "file" or bm.get("target") != source.rel_path:
            reasons.append("invalid_backmatter")
        if not allow_stale and bm.get("source_hash") != sha256_file(source.abs_path):
            reasons.append("stale")
        append_open_item_reasons(reasons, counts, bm, allow_external_verify)
    except ChumError:
        reasons.append("invalid_backmatter")
    marker_reasons = legacy_marker_reasons(source.spec_path, allow_external_verify)
    reasons.extend(marker_reasons)
    return unique(reasons), counts


def source_dir_reasons(
    directory: SourceDir,
    allow_external_verify: bool,
) -> Tuple[List[str], Dict[str, int]]:
    if not directory.spec_path.exists():
        return ["missing"], {"todo": 0, "unknowns": 0, "verify": 0}
    reasons: List[str] = []
    counts = {"todo": 0, "unknowns": 0, "verify": 0}
    expected_target = "." if directory.rel_path == "" else directory.rel_path
    try:
        parsed = parse_backmatter_file(directory.spec_path)
        bm = parsed.backmatter
        if bm.get("kind") != "directory" or bm.get("target") != expected_target:
            reasons.append("invalid_backmatter")
        append_open_item_reasons(reasons, counts, bm, allow_external_verify)
    except ChumError:
        reasons.append("invalid_backmatter")
    reasons.extend(legacy_marker_reasons(directory.spec_path, allow_external_verify))
    return unique(reasons), counts


def append_open_item_reasons(
    reasons: List[str],
    counts: Dict[str, int],
    bm: Dict[str, Any],
    allow_external_verify: bool,
) -> None:
    for key in ["todo", "unknowns", "verify"]:
        items = as_list(bm.get(key, []))
        counts[key] = len(items)
        if items and not (key == "verify" and allow_external_verify):
            reasons.append(key)


def legacy_marker_reasons(path: Path, allow_external_verify: bool) -> List[str]:
    try:
        content = path.read_text(encoding="utf-8")
    except OSError:
        return ["invalid_backmatter"]
    reasons = []
    if MARKERS["todo"] in content or MARKERS["unknowns"] in content:
        reasons.append("legacy_marker")
    if MARKERS["verify"] in content and not allow_external_verify:
        reasons.append("legacy_marker")
    return reasons


def validate_source_file(
    source: SourceFile,
    allow_stale: bool,
    allow_external_verify: bool,
) -> List[Dict[str, str]]:
    reasons, _counts = source_file_reasons(source, allow_stale, allow_external_verify)
    return reasons_to_failures(source.spec_path, reasons, source.rel_path)


def validate_source_dir(directory: SourceDir, allow_external_verify: bool) -> List[Dict[str, str]]:
    reasons, _counts = source_dir_reasons(directory, allow_external_verify)
    display = "." if directory.rel_path == "" else directory.rel_path
    return reasons_to_failures(directory.spec_path, reasons, display)


def reasons_to_failures(path: Path, reasons: Sequence[str], target: str) -> List[Dict[str, str]]:
    failures = []
    for reason in reasons:
        message = {
            "missing": f"missing spec for `{target}`",
            "invalid_backmatter": "invalid or mismatched chum backmatter",
            "stale": "source_hash is stale or missing",
            "todo": "todo must be empty",
            "unknowns": "unknowns must be empty",
            "verify": "verify must be empty",
            "legacy_marker": "contains legacy SPEC marker",
        }.get(reason, reason)
        failures.append({"path": str(path), "message": message})
    return failures


def check_report(
    root: Path,
    allow_stale: bool,
    allow_external_verify: bool,
    explicit_include: Optional[List[str]] = None,
    include_archive: bool = False,
) -> Dict[str, Any]:
    config = load_config(root)
    discovery = discover(root, config, explicit_include, include_archive)
    failures: List[Dict[str, str]] = []
    for source in discovery.source_files:
        failures.extend(validate_source_file(source, allow_stale, allow_external_verify))
    for directory in discovery.source_dirs:
        failures.extend(validate_source_dir(directory, allow_external_verify))
    return {
        "root": str(root),
        "sourceFiles": len(discovery.source_files),
        "sourceDirs": len(discovery.source_dirs),
        "ignoredCount": discovery.ignored_count,
        "failures": failures,
        "warnings": discovery.warnings,
    }


def normalize_markdown(
    root: Path,
    config: Config,
    target_arg: str,
    markdown: str,
    spec_override: Optional[str] = None,
) -> Tuple[str, Path, str, str]:
    target_rel = normalize_target_arg(target_arg)
    target_abs = root if target_rel in {"", "."} else root / target_rel
    if target_rel in {"", "."} or target_abs.is_dir():
        kind = "directory"
        target = "." if target_rel in {"", "."} else target_rel
        spec_path = Path(spec_override) if spec_override else dir_spec_path(root, "" if target == "." else target, config)
        bm = directory_backmatter(root, "" if target == "." else target, config, "chum skill")
    else:
        kind = "file"
        target = target_rel
        spec_path = Path(spec_override) if spec_override else file_spec_path(root, target_rel)
        if not target_abs.exists():
            raise ChumError(f"target source file does not exist: {target_arg}", 1)
        bm = file_backmatter(target_rel, target_abs, "chum skill")
    if spec_override and not spec_path.is_absolute():
        spec_path = root / spec_path
    try:
        parsed = parse_backmatter(markdown)
        for key in ["todo", "unknowns", "verify"]:
            bm[key] = as_list(parsed.backmatter.get(key, []))
    except ChumError:
        pass
    return replace_or_append_backmatter(markdown, bm), spec_path, kind, target


def file_backmatter(target: str, source_path: Path, generated_by: str) -> Dict[str, Any]:
    return {
        "schema": 1,
        "kind": "file",
        "target": target,
        "source_hash": sha256_file(source_path),
        "source_updated_at": modified_time(source_path),
        "spec_updated_at": now(),
        "generated_by": generated_by,
        "todo": [],
        "unknowns": [],
        "verify": [],
    }


def directory_backmatter(root: Path, rel_dir: str, config: Config, generated_by: str) -> Dict[str, Any]:
    discovery = discover(root, config)
    return {
        "schema": 1,
        "kind": "directory",
        "target": "." if rel_dir in {"", "."} else rel_dir,
        "spec_updated_at": now(),
        "generated_by": generated_by,
        "children": child_specs(root, rel_dir, discovery, config),
        "todo": [],
        "unknowns": [],
        "verify": [],
    }


def child_specs(root: Path, rel_dir: str, discovery: Discovery, config: Config) -> List[str]:
    children: List[str] = []
    rel_dir_norm = "" if rel_dir in {"", "."} else rel_dir
    for source in discovery.source_files:
        if parent_rel(source.rel_path) == rel_dir_norm:
            children.append(rel_path(source.spec_path, root))
    for directory in discovery.source_dirs:
        if directory.rel_path == rel_dir_norm:
            continue
        if parent_rel(directory.rel_path) == rel_dir_norm:
            children.append(rel_path(directory.spec_path, root))
    return sorted(children)


def parent_rel(path: str) -> str:
    parent = Path(path).parent.as_posix()
    return "" if parent == "." else parent


def read_markdown_input(args: argparse.Namespace) -> str:
    if args.stdin:
        return sys.stdin.read()
    if args.input:
        return Path(args.input).read_text(encoding="utf-8")
    raise ChumError("normalize requires --stdin or --input", 2)


def parse_backmatter_file(path: Path) -> ParsedBackmatter:
    try:
        content = path.read_text(encoding="utf-8")
    except OSError as error:
        raise ChumError(f"failed to read {path}: {error}", 3)
    try:
        return parse_backmatter(content)
    except ChumError as error:
        raise ChumError(f"failed to parse backmatter in {path}: {error}", error.code)


def parse_backmatter(content: str) -> ParsedBackmatter:
    start = content.find(OPEN)
    if start == -1:
        raise ChumError("missing chum:backmatter block", 1)
    yaml_start = start + len(OPEN)
    relative_end = content[yaml_start:].find(CLOSE)
    if relative_end == -1:
        raise ChumError("unterminated chum:backmatter block", 1)
    end = yaml_start + relative_end + len(CLOSE)
    if content[end:].find(OPEN) != -1:
        raise ChumError("multiple chum:backmatter blocks found", 1)
    body = content[yaml_start : yaml_start + relative_end].strip()
    line = content[:start].count("\n") + 1
    try:
        parsed = parse_simple_yaml(body)
    except Exception as error:
        raise ChumError(f"invalid YAML near line {line}: {error}", 1)
    if not isinstance(parsed, dict):
        raise ChumError(f"invalid YAML near line {line}: expected mapping", 1)
    return ParsedBackmatter(parsed, start, end, line)


def replace_or_append_backmatter(content: str, backmatter: Dict[str, Any]) -> str:
    block = render_backmatter(backmatter)
    try:
        parsed = parse_backmatter(content)
        before = content[: parsed.start].rstrip()
        after = content[parsed.end :].lstrip("\n")
        return f"{before}\n\n{block}{after}"
    except ChumError:
        base = content.rstrip()
        if base:
            return f"{base}\n\n{block}"
        return block


def render_backmatter(backmatter: Dict[str, Any]) -> str:
    return f"{OPEN}\n{dump_simple_yaml(backmatter)}\n-->"


def parse_simple_yaml(text: str) -> Any:
    lines = [
        line.rstrip()
        for line in text.splitlines()
        if line.strip() and not line.strip().startswith("#") and line.strip() != "---"
    ]
    value, index = parse_yaml_mapping(lines, 0, 0)
    if index < len(lines):
        raise ValueError(f"unexpected line: {lines[index]}")
    return value


def parse_yaml_mapping(lines: List[str], index: int, indent: int) -> Tuple[Dict[str, Any], int]:
    result: Dict[str, Any] = {}
    while index < len(lines):
        line = lines[index]
        current_indent = len(line) - len(line.lstrip(" "))
        if current_indent < indent:
            break
        if current_indent > indent:
            raise ValueError(f"unexpected indentation: {line}")
        stripped = line.strip()
        if stripped.startswith("- "):
            break
        if ":" not in stripped:
            raise ValueError(f"expected key: value: {line}")
        key, raw_value = stripped.split(":", 1)
        raw_value = raw_value.strip()
        index += 1
        if raw_value:
            result[key] = parse_scalar(raw_value)
        else:
            if index >= len(lines):
                result[key] = []
            else:
                next_indent = len(lines[index]) - len(lines[index].lstrip(" "))
                next_stripped = lines[index].strip()
                if next_stripped.startswith("- "):
                    result[key], index = parse_yaml_list(lines, index, next_indent)
                elif next_indent <= indent:
                    result[key] = []
                else:
                    result[key], index = parse_yaml_mapping(lines, index, next_indent)
    return result, index


def parse_yaml_list(lines: List[str], index: int, indent: int) -> Tuple[List[Any], int]:
    result: List[Any] = []
    while index < len(lines):
        line = lines[index]
        current_indent = len(line) - len(line.lstrip(" "))
        if current_indent < indent:
            break
        if current_indent != indent or not line.strip().startswith("- "):
            break
        raw = line.strip()[2:].strip()
        result.append(parse_scalar(raw))
        index += 1
    return result, index


def parse_scalar(value: str) -> Any:
    if value == "[]":
        return []
    if value in {"null", "None", "~"}:
        return None
    if value in {"true", "True"}:
        return True
    if value in {"false", "False"}:
        return False
    if value.startswith(("'", '"')) and value.endswith(("'", '"')):
        return value[1:-1]
    try:
        return int(value)
    except ValueError:
        return value


def dump_simple_yaml(data: Dict[str, Any]) -> str:
    order = [
        "schema",
        "kind",
        "target",
        "source_hash",
        "source_updated_at",
        "spec_updated_at",
        "generated_by",
        "children",
        "todo",
        "unknowns",
        "verify",
    ]
    lines: List[str] = []
    for key in order + sorted(k for k in data if k not in order):
        if key not in data or data[key] is None:
            continue
        value = data[key]
        if isinstance(value, list):
            if value:
                lines.append(f"{key}:")
                for item in value:
                    lines.append(f"- {quote_yaml_scalar(str(item))}")
            else:
                lines.append(f"{key}: []")
        else:
            lines.append(f"{key}: {quote_yaml_scalar(str(value))}")
    return "\n".join(lines)


def quote_yaml_scalar(value: str) -> str:
    if value == "":
        return '""'
    if value.startswith(("[", "{", "-", "#")) or "\n" in value:
        return json.dumps(value)
    return value


def default_config_yaml() -> str:
    return """version: 1
activeDirs:
- design
- plan
- debug
- review
archiveDir: archive
liveDocGlob: "**/*.spec.md"
source:
  respectGitignore: true
  ignoreFiles:
  - .gitignore
  - .chumignore
  include:
  - "**/*.{c,cc,cpp,cxx,h,hpp,cs,css,go,html,java,js,jsx,kt,kts,m,mm,php,py,rb,rs,scss,sh,swift,ts,tsx,vue}"
  exclude:
  - ".git/**"
  - ".hg/**"
  - ".svn/**"
  - "node_modules/**"
  - "vendor/**"
  - "dist/**"
  - "build/**"
  - "target/**"
  - "coverage/**"
  - "archive/**"
  - "**/{test,tests,__tests__,spec,specs,fixture,fixtures,script,scripts,migration,migrations}/**"
  - "**/*.{test,spec}.{js,jsx,ts,tsx,py,rb,go,rs,swift,java,kt,kts,cs,php}"
  - "**/*config.{js,jsx,ts,tsx,cjs,mjs,json,yaml,yml,toml}"
  - "**/*.min.*"
  - "**/*.generated.*"
specs:
  placement: inline
  filePattern: "{path}.spec.md"
  directoryPattern: "{dir}/{basename}.spec.md"
  rootSpec: repo.spec.md
  backmatter: required
"""


def plan_archive_moves(root: Path, config: Config, args: argparse.Namespace) -> List[Dict[str, str]]:
    discovery = discover(root, config)
    frontmatter_matches: List[str] = []
    folder_matches: List[str] = []
    filename_matches: List[str] = []

    for rel in discovery.active_docs:
        if is_live_spec(rel) or match_any(rel, args.exclude):
            continue
        content = (root / rel).read_text(encoding="utf-8")
        if frontmatter_change_id(content) == args.change_id:
            frontmatter_matches.append(rel)
        if folder_match(rel, config, args.change_id):
            folder_matches.append(rel)
        if filename_match(rel, args.change_id):
            filename_matches.append(rel)

    if frontmatter_matches:
        selected = set(frontmatter_matches)
    elif folder_matches:
        if filename_matches:
            raise ChumError(
                f"ambiguous archive matches for `{args.change_id}`; use --include to select exact docs",
                1,
            )
        selected = set(folder_matches)
    else:
        selected = set(filename_matches)

    for include in args.include:
        for rel in discovery.active_docs:
            if match_any(rel, [include]) and not is_live_spec(rel) and not match_any(rel, args.exclude):
                selected.add(rel)

    moves = []
    for from_rel in sorted(selected):
        if is_live_spec(from_rel):
            continue
        moves.append({"from": from_rel, "to": archive_target(config, args.change_id, from_rel)})
    return moves


def frontmatter_change_id(content: str) -> Optional[str]:
    if not content.startswith("---"):
        return None
    end = content.find("\n---", 3)
    if end == -1:
        return None
    try:
        data = parse_simple_yaml(content[3:end])
    except Exception:
        return None
    if isinstance(data, dict) and data.get("change") is not None:
        return str(data["change"])
    return None


def folder_match(path: str, config: Config, change_id: str) -> bool:
    parts = path.split("/")
    return len(parts) > 2 and parts[0] in config.active_dirs and parts[1] == change_id


def filename_match(path: str, change_id: str) -> bool:
    return Path(path).stem == change_id


def archive_target(config: Config, change_id: str, from_rel: str) -> str:
    parts = from_rel.split("/")
    if len(parts) > 2 and parts[0] in config.active_dirs and parts[1] == change_id:
        rest = "/".join(parts[2:])
        return f"{config.archive_dir}/{change_id}/{parts[0]}/{rest}"
    return f"{config.archive_dir}/{change_id}/{from_rel}"


def rewrite_markdown_links(
    content: str,
    old_abs: Path,
    new_abs: Path,
    move_map: Dict[Path, Path],
    root: Path,
) -> Tuple[str, List[str]]:
    warnings: List[str] = []

    def replace(match: re.Match[str]) -> str:
        label, url = match.group(1), match.group(2)
        if re.match(r"^[a-zA-Z][a-zA-Z0-9+.-]*:", url) or url.startswith("#"):
            return match.group(0)
        path_part, suffix = split_link_suffix(url)
        linked_old = (old_abs.parent / path_part).resolve()
        if linked_old in move_map:
            linked_new = move_map[linked_old]
            new_url = os.path.relpath(linked_new, start=new_abs.parent).replace(os.sep, "/") + suffix
            return f"[{label}]({new_url})"
        if linked_old.exists() and linked_old.is_file() and not linked_old.suffix.lower() in {".md", ".markdown"}:
            warnings.append(
                f"linked local asset not archived: {rel_path(linked_old, root)} from {rel_path(old_abs, root)}"
            )
            new_url = os.path.relpath(linked_old, start=new_abs.parent).replace(os.sep, "/") + suffix
            return f"[{label}]({new_url})"
        return match.group(0)

    return re.sub(r"\[([^\]]+)\]\(([^)]+)\)", replace, content), warnings


def split_link_suffix(url: str) -> Tuple[str, str]:
    for sep in ["#", "?"]:
        if sep in url:
            path, rest = url.split(sep, 1)
            return path, sep + rest
    return url, ""


def write_archive_manifest(
    root: Path,
    config: Config,
    args: argparse.Namespace,
    moves: List[Dict[str, str]],
    warnings: List[str],
    check_status: str,
) -> None:
    path = root / config.archive_dir / args.change_id / "README.md"
    path.parent.mkdir(parents=True, exist_ok=True)
    title = args.title or title_from_id(args.change_id)
    frontmatter: Dict[str, Any] = {
        "id": args.change_id,
        "archived_at": now(),
        "check_status": check_status,
        "archived_paths": [move["from"] for move in moves],
        "related_live_docs": [],
        "warnings": warnings,
    }
    if args.source_ref:
        frontmatter["source_ref"] = args.source_ref
    if args.pr:
        frontmatter["pr"] = args.pr
    content = f"---\n{dump_simple_yaml(frontmatter)}\n---\n\n# {title}\n\nHistorical change artifacts for this completed change.\n"
    path.write_text(content, encoding="utf-8")


def append_agent_snippet(path: Path) -> None:
    snippet = load_agent_snippet()
    heading = first_level_two_heading(snippet)
    content = path.read_text(encoding="utf-8") if path.exists() else ""
    if heading not in content:
        path.write_text(content.rstrip() + "\n\n" + snippet.rstrip() + "\n", encoding="utf-8")


def load_agent_snippet() -> str:
    try:
        snippet = AGENTS_SNIPPET.read_text(encoding="utf-8")
    except OSError as error:
        raise ChumError(f"failed to read AGENTS snippet: {error}", 3)
    first_level_two_heading(snippet)
    return snippet


def first_level_two_heading(markdown: str) -> str:
    for line in markdown.splitlines():
        if line.startswith("## ") and not line.startswith("### "):
            return line.strip()
    raise ChumError("AGENTS snippet must start with a level-2 Markdown heading", 3)


def emit_report(report: Dict[str, Any], json_output: bool, output_path: Optional[str]) -> None:
    if output_path:
        Path(output_path).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        return
    if json_output:
        print_json(report)
    else:
        summary = report.get("summary", {})
        print(
            "chum targets: "
            f"{summary.get('targets', 0)} target(s), "
            f"{summary.get('sourceFiles', 0)} source files, "
            f"{summary.get('sourceDirs', 0)} source directories"
        )
        for row in report.get("targets", []):
            print(f"- {row['kind']} {row['target']}: {', '.join(row['reasons'])}")


def print_json(value: Any) -> None:
    print(json.dumps(make_json_safe(value), indent=2, sort_keys=False))


def make_json_safe(value: Any) -> Any:
    if isinstance(value, Path):
        return str(value)
    if dataclasses.is_dataclass(value):
        return dataclasses.asdict(value)
    if isinstance(value, dict):
        return {k: make_json_safe(v) for k, v in value.items()}
    if isinstance(value, list):
        return [make_json_safe(v) for v in value]
    return value


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return "sha256:" + digest.hexdigest()


def modified_time(path: Path) -> Optional[str]:
    try:
        ts = path.stat().st_mtime
    except OSError:
        return None
    return _dt.datetime.fromtimestamp(ts, tz=_dt.timezone.utc).isoformat().replace("+00:00", "Z")


def now() -> str:
    return _dt.datetime.now(tz=_dt.timezone.utc).isoformat().replace("+00:00", "Z")


def rel_path(path: Path, root: Path) -> str:
    try:
        return path.resolve().relative_to(root.resolve()).as_posix()
    except Exception:
        return path.as_posix()


def join_rel(left: str, right: str) -> str:
    return right if not left else f"{left}/{right}"


def normalize_target_arg(value: str) -> str:
    value = value.replace("\\", "/").strip("/")
    return "" if value in {"", "."} else value


def as_list(value: Any) -> List[str]:
    if value is None:
        return []
    if isinstance(value, list):
        return [str(item) for item in value]
    return [str(value)]


def unique(items: Iterable[str]) -> List[str]:
    seen = set()
    result = []
    for item in items:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result


def title_from_id(value: str) -> str:
    return " ".join(part[:1].upper() + part[1:] for part in re.split(r"[-_]+", value) if part)


if __name__ == "__main__":
    sys.exit(main())
