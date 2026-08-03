import importlib.util
import tempfile
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

    def test_production_unwrap_is_rejected(self):
        errors = self.check_source("fn run() { value.unwrap(); }")
        self.assertTrue(any("forbidden" in error for error in errors))

    def test_high_cyclomatic_complexity_is_rejected(self):
        body = "\n".join(f"    if c{i} {{ x += 1; }}" for i in range(14))
        errors = self.check_source(f"fn run() {{\n    let mut x = 0;\n{body}\n}}")
        self.assertTrue(any("complexity" in error for error in errors))

    def check_source(self, source):
        with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False) as handle:
            handle.write(source)
            path = Path(handle.name)
        try:
            return QUALITY.check(path, production=True)
        finally:
            path.unlink(missing_ok=True)


if __name__ == "__main__":
    unittest.main()
