import subprocess
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
FORBIDDEN_PREFIXES = (
    "packaging/debian/out",
    "packaging/fedora/out",
    "test-results",
)
IGNORED_EXAMPLES = (
    "artifacts/package.tar.xz",
    "dist/package.rpm",
    "packaging/debian/out-build/package.deb",
    "packaging/fedora/out-local/package.rpm",
    "test-results-vm-arch/unit.json",
    "coverage.lcov",
    "release-sbom.cdx.json",
    "clippy.json",
    "semgrep-inventory.json",
    "cargo-geiger.txt",
)
EXTERNAL_VERIFY_FILES = (
    "scripts/bench-engines.py",
    "scripts/check-unsafe-code.sh",
    "scripts/ci_provenance.py",
    "scripts/ci_report.py",
    "scripts/e2e-hw.py",
    "scripts/e2e-interact.py",
    "scripts/e2e-library.py",
    "scripts/e2e-perf.py",
    "scripts/perf-combos.py",
    "scripts/python_suite.py",
    "scripts/tests/test_acceptance_paths.py",
    "scripts/tests/test_ci_report.py",
    "scripts/wl_wire.py",
)
RETIRED_LOCAL_PATHS = (
    ".scannerwork/report-task.txt",
    ".split/source-backup",
    "Mods",
    "World",
    "perf-sweep-latest.json",
    "scripts/e2e-baselines/selector.png",
    "sonar-project.properties",
    "the",
)


class RepositoryArtifactPolicyTests(unittest.TestCase):
    def test_generated_outputs_are_absent_from_the_tracked_worktree(self):
        tracked = subprocess.run(
            ["git", "ls-files", "-z"], cwd=ROOT, check=True, capture_output=True
        ).stdout.decode().split("\0")
        forbidden = [
            path
            for path in tracked
            if path
            and path.startswith(FORBIDDEN_PREFIXES)
            and (ROOT / path).exists()
        ]
        self.assertEqual(forbidden, [])

    def test_local_output_families_are_ignored(self):
        process = subprocess.run(
            ["git", "check-ignore", "--no-index", "--stdin"],
            cwd=ROOT,
            input="\n".join(IGNORED_EXAMPLES) + "\n",
            text=True,
            capture_output=True,
        )
        ignored = set(process.stdout.splitlines())
        self.assertEqual(ignored, set(IGNORED_EXAMPLES))

    def test_external_system_tests_are_not_reintroduced(self):
        present = [path for path in EXTERNAL_VERIFY_FILES if (ROOT / path).exists()]
        self.assertEqual(present, [])

    def test_retired_local_artifacts_are_neither_present_nor_ignored(self):
        present = [path for path in RETIRED_LOCAL_PATHS if (ROOT / path).exists()]
        self.assertEqual(present, [])
        process = subprocess.run(
            ["git", "check-ignore", "--no-index", "--stdin"],
            cwd=ROOT,
            input="\n".join(RETIRED_LOCAL_PATHS) + "\n",
            text=True,
            capture_output=True,
        )
        self.assertEqual(process.stdout.splitlines(), [])


if __name__ == "__main__":
    unittest.main()
