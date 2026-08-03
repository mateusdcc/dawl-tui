#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SKIP = {"target", ".git", ".worktrees"}


def rust_files():
    for path in ROOT.rglob("*.rs"):
        relative = path.relative_to(ROOT)
        if not any(part in SKIP for part in relative.parts):
            yield path


def function_ranges(lines):
    start = None
    depth = 0
    for index, line in enumerate(lines, 1):
        if start is None and re.search(r"\bfn\s+\w+", line):
            start = index
            depth = line.count("{") - line.count("}")
            if depth <= 0 and ";" in line:
                start = None
        elif start is not None:
            depth += line.count("{") - line.count("}")
            if depth <= 0:
                yield start, index
                start = None


def check(path):
    lines = path.read_text().splitlines()
    errors = []
    if len(lines) > 200:
        errors.append(f"{path}: {len(lines)} lines (max 200)")
    for start, end in function_ranges(lines):
        if end - start + 1 > 20:
            errors.append(f"{path}:{start}: function has {end-start+1} lines (max 20)")
    return errors


def main():
    errors = [error for path in rust_files() for error in check(path)]
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print("source policy: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
