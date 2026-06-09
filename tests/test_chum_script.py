import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "skills" / "chum" / "scripts" / "chum.py"


def run_chum(*args, input_text=None, cwd=ROOT):
    return subprocess.run(
        [sys.executable, str(SCRIPT), *args],
        input=input_text,
        text=True,
        cwd=str(cwd),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


class ChumScriptTests(unittest.TestCase):
    def test_help(self):
        result = run_chum("--help")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("targets", result.stdout)

    def test_targets_reports_missing_specs(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.py").write_text("def add(a, b):\n    return a + b\n")

            result = run_chum("targets", "--root", temp, "--json")

            self.assertEqual(result.returncode, 0, result.stderr)
            data = json.loads(result.stdout)
            self.assertGreaterEqual(data["summary"]["missing"], 1)
            self.assertIn("src/lib.py.spec.md", [row["specPath"] for row in data["targets"]])

    def test_check_returns_one_for_missing_specs(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.py").write_text("def add():\n    return 1\n")

            result = run_chum("check", "--root", temp, "--json")

            self.assertEqual(result.returncode, 1)
            data = json.loads(result.stdout)
            self.assertTrue(data["failures"])

    def test_normalize_and_check_clean_fixture(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.py").write_text("def add():\n    return 1\n")

            file_md = "# `src/lib.py`\n\n## Purpose\n\nAdds numbers.\n"
            result = run_chum(
                "normalize",
                "--root",
                temp,
                "--target",
                "src/lib.py",
                "--stdin",
                "--write",
                "--json",
                input_text=file_md,
            )
            self.assertEqual(result.returncode, 0, result.stderr)

            dir_md = "# `src/`\n\n## Purpose\n\nSource package.\n"
            result = run_chum(
                "normalize",
                "--root",
                temp,
                "--target",
                "src",
                "--stdin",
                "--write",
                input_text=dir_md,
            )
            self.assertEqual(result.returncode, 0, result.stderr)

            root_md = "# `./`\n\n## Purpose\n\nFixture repository.\n"
            result = run_chum(
                "normalize",
                "--root",
                temp,
                "--target",
                ".",
                "--stdin",
                "--write",
                input_text=root_md,
            )
            self.assertEqual(result.returncode, 0, result.stderr)

            result = run_chum("check", "--root", temp, "--json")
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_validate_one_target(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.py").write_text("def add():\n    return 1\n")
            run_chum(
                "normalize",
                "--root",
                temp,
                "--target",
                "src/lib.py",
                "--stdin",
                "--write",
                input_text="# File\n",
            )

            result = run_chum("validate", "--root", temp, "--target", "src/lib.py", "--json")

            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(json.loads(result.stdout)["failures"], [])

    def test_gitignore_and_chumignore(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / ".gitignore").write_text("ignored-by-git/\n")
            (root / ".chumignore").write_text("ignored-by-chum/\n")
            (root / "ignored-by-git").mkdir()
            (root / "ignored-by-chum").mkdir()
            (root / "ignored-by-git" / "a.py").write_text("x = 1\n")
            (root / "ignored-by-chum" / "b.py").write_text("x = 1\n")

            result = run_chum("check", "--root", temp, "--json")

            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(json.loads(result.stdout)["sourceFiles"], 0)

    def test_stale_invalid_and_legacy_markers_are_reported(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            source = root / "src" / "lib.py"
            source.write_text("VALUE = 1\n")
            result = run_chum(
                "normalize",
                "--root",
                temp,
                "--target",
                "src/lib.py",
                "--stdin",
                "--write",
                input_text="# File\n",
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            (root / "src" / "src.spec.md").write_text(
                "# Bad\n\n<!-- chum:backmatter\nschema: 1\nkind: file\ntarget: src\ntodo: []\nunknowns: []\nverify: []\n-->\n"
            )
            (root / "repo.spec.md").write_text(
                "# Root\n\n<!-- chum:backmatter\nschema: 1\nkind: directory\ntarget: .\ntodo: []\nunknowns: []\nverify: []\n-->\n"
            )
            spec = root / "src" / "lib.py.spec.md"
            spec.write_text(spec.read_text() + "\n<!-- SPEC:TODO -->\n")
            source.write_text("VALUE = 2\n")

            result = run_chum("targets", "--root", temp, "--json")

            self.assertEqual(result.returncode, 0, result.stderr)
            rows = {row["specPath"]: row["reasons"] for row in json.loads(result.stdout)["targets"]}
            self.assertIn("stale", rows["src/lib.py.spec.md"])
            self.assertIn("legacy_marker", rows["src/lib.py.spec.md"])
            self.assertIn("invalid_backmatter", rows["src/src.spec.md"])

    def test_init_is_dry_run_by_default_and_write_is_idempotent(self):
        with tempfile.TemporaryDirectory() as temp:
            result = run_chum("init", "--root", temp, "--json")
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertTrue(json.loads(result.stdout)["dryRun"])
            self.assertFalse((Path(temp) / "chum.config.yaml").exists())

            result = run_chum("init", "--root", temp, "--write", "--json")
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertTrue((Path(temp) / "chum.config.yaml").exists())

            result = run_chum("init", "--root", temp, "--write", "--json")
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(json.loads(result.stdout)["written"], [])

    def test_archive_dry_run_and_write(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "design").mkdir()
            (root / "plan" / "example").mkdir(parents=True)
            (root / "design" / "example.md").write_text(
                "---\nchange: example\n---\n# Example\n\nSee [phase](../plan/example/phase-1.md).\n"
            )
            (root / "plan" / "example" / "phase-1.md").write_text(
                "---\nchange: example\n---\n# Phase 1\n"
            )

            result = run_chum("archive", "--root", temp, "example", "--json")
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertTrue(json.loads(result.stdout)["dryRun"])
            self.assertTrue((root / "design" / "example.md").exists())

            result = run_chum("archive", "--root", temp, "example", "--write", "--json")
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertFalse((root / "design" / "example.md").exists())
            self.assertTrue((root / "archive" / "example" / "design" / "example.md").exists())
            self.assertTrue((root / "archive" / "example" / "plan" / "phase-1.md").exists())
            self.assertTrue((root / "archive" / "example" / "README.md").exists())


if __name__ == "__main__":
    unittest.main()
