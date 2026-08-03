#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SKIP = {"target", ".git", ".worktrees"}
MAX_FILE_LINES = 200
MAX_FUNCTION_LINES = 20
MAX_COMPLEXITY = 14
FORBIDDEN = re.compile(r"\.(?:unwrap|expect)\s*\(|\b(?:panic|todo|unimplemented)!\s*\(")


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


def check(path, production=None):
    lines = path.read_text().splitlines()
    errors = file_errors(path, lines)
    if production is None:
        production = is_production(path)
    if production:
        errors.extend(forbidden_errors(path, lines))
    return errors


def file_errors(path, lines):
    errors = []
    if len(lines) > MAX_FILE_LINES:
        errors.append(f"{path}: {len(lines)} lines (max {MAX_FILE_LINES})")
    for start, end in function_ranges(lines):
        errors.extend(function_errors(path, lines, start, end))
    return errors


def function_errors(path, lines, start, end):
    errors = []
    length = end - start + 1
    if length > MAX_FUNCTION_LINES:
        errors.append(f"{path}:{start}: function has {length} lines (max {MAX_FUNCTION_LINES})")
    score = complexity(lines[start - 1:end])
    if score > MAX_COMPLEXITY:
        errors.append(f"{path}:{start}: cyclomatic complexity {score} (max {MAX_COMPLEXITY})")
    return errors


def complexity(lines):
    source = "\n".join(lines)
    branches = len(re.findall(r"\b(?:if|for|while|match)\b", source))
    booleans = source.count("&&") + source.count("||")
    arms = source.count("=>")
    return 1 + branches + booleans + arms


def forbidden_errors(path, lines):
    return [f"{path}:{index}: forbidden panic/unwrap construct" for index, line in enumerate(lines, 1) if FORBIDDEN.search(line)]


def is_production(path):
    try:
        relative = path.resolve().relative_to(ROOT)
    except ValueError:
        return False
    return bool(relative.parts) and relative.parts[0] == "src"


def main():
    errors = [error for path in rust_files() for error in check(path)]
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print("source policy: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
