from pathlib import Path
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[2]
PROJECT_LICENSE = "GPL-3.0-or-later"


class LicenseMetadataTests(unittest.TestCase):
    def read(self, path):
        return (ROOT / path).read_text()

    def test_wall_package_inherits_project_license(self):
        manifest = tomllib.loads(self.read("Cargo.toml"))
        self.assertEqual(manifest["workspace"]["package"]["license"], PROJECT_LICENSE)
        self.assertEqual(manifest["package"]["license"], {"workspace": True})

    def test_project_ships_its_license_and_dependency_texts(self):
        self.assertTrue((ROOT / "LICENSE").is_file())
        texts = sorted(path.name for path in (ROOT / "LICENSES").glob("*.txt"))
        self.assertEqual(
            texts,
            ["Apache-2.0.txt", "CC-BY-4.0.txt", "MIT.txt", "OFL-1.1.txt", "Zlib.txt"],
        )


if __name__ == "__main__":
    unittest.main()
