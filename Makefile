.PHONY: all test check fmt fmt-check adversarial evidence clean

all: fmt-check check test evidence

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

check:
	cargo clippy --workspace --all-targets -- -D warnings

adversarial:
	cargo test -p factory-application --test adversarial_suite -- --nocapture

test:
	cargo test --workspace

evidence:
	bash scripts/generate_evidence.sh

clean:
	cargo clean
