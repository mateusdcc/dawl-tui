.PHONY: verify example bench

verify:
	python3 -m unittest scripts/test_quality.py
	python3 scripts/quality.py
	cargo check --all-targets
	cargo clippy --all-targets -- -D warnings
	cargo test --all-targets

example:
	cargo run -- render examples/approval.dtui --format text --width 180 --height 52

site:
	python3 -m http.server 8080 --directory site

bench:
	cargo bench --bench pipeline

