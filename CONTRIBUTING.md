# Contributing

Bug reports, focused feature proposals, documentation fixes, and pull requests
are welcome. Please open an issue before starting a large change so the design
and compatibility impact can be discussed first.

## Development

Rust 1.88 and Python 3 are required. Run the complete local quality gate before
opening a pull request:

```bash
make verify
```

New behavior should include a focused test. Rendering changes should preserve
deterministic output and update the relevant fixture or reference asset when the
visual change is intentional.

By contributing, you agree that your contribution is licensed under the MIT
license used by this repository.
