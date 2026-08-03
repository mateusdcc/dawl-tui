import importlib.util
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("quality.py")
SPEC = importlib.util.spec_from_file_location("quality", MODULE_PATH)
QUALITY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(QUALITY)


class QualityPolicyTest(unittest.TestCase):
    def test_worktree_sources_are_scanned(self):
        paths = {path.relative_to(QUALITY.ROOT).as_posix() for path in QUALITY.rust_files()}
        self.assertIn("src/model/mod.rs", paths)


if __name__ == "__main__":
    unittest.main()
