# Contributing to `rust_CACD_autonomous_factory`

## Repository Layout
- Scripts go in `scripts/`.
- Tests go in `tests/` (or crate-level `tests/`).
- New root-level files require an allowlist entry in `scripts/dev/root_allowlist.txt`.

## Development & Quality Checks
Before opening a pull request, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Pull Request Policy
- Never push directly to `main`.
- All changes must go through an isolated feature branch and pull request.
- Ensure all CI/CD, CodeQL, and gitleaks checks pass before requesting review.
