import os
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
VERIFY_ROOT = Path(os.environ.get("SKWD_VERIFY_ROOT", ROOT.parent / "skwd-verify"))


class PythonTestCoverageTests(unittest.TestCase):
    def test_every_python_test_uses_the_wall_suite_directory(self):
        unexpected = [
            path.relative_to(ROOT).as_posix()
            for path in (ROOT / "scripts").rglob("test_*.py")
            if path.parent != ROOT / "scripts/tests"
        ]
        self.assertEqual(unexpected, [])

    def test_shared_driver_discovers_the_callers_suite_directory(self):
        runner = (VERIFY_ROOT / "scripts/python_suite.py").read_text()
        self.assertIn('"python-general": "scripts/tests"', runner)
        self.assertIn("Path.cwd()", runner)
        self.assertIn('pattern="test_*.py"', runner)
        self.assertIn('default="python-general"', runner)


if __name__ == "__main__":
    unittest.main()
