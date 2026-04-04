# CI Pipeline Design: Rust Autonomous Factory

This document specifies the automated checks performed on the `rust_CACD_autonomous_factory` repository for every Pull Request (PR) and push to the `main` branch.

## Objectives

- **Code Quality**: Ensure all code follows standard Rust formatting and linting rules.
- **System Stability**: Verify that changes do not break existing business logic or agent functions.
- **Security**: Prevent regressions in security-critical tools like `SecurityReviewTool`.

## Workflow Configuration

The primary CI job is defined in `.github/workflows/check.yml` and consists of the following steps:

### 1. Style Check (`rustfmt`)

Forces the codebase to remain consistent by checking for formatting violations.

- **Command**: `cargo fmt --all -- --check`
- **Standard**: [Official Rust Style Guide](https://github.com/rust-lang/rust-mode)

### 2. Static Analysis (`clippy`)

Lints the code for cognitive complexity, performance anti-patterns, and logical errors.

- **Command**: `cargo clippy --workspace -- -D warnings`
- **Standard**: Default "warnings as errors" to maintain high source quality.

### 3. Automated Testing (`cargo test`)

Runs the full suite of unit, integration, and security tests.

- **Command**: `cargo test --workspace`
- **Execution Environment**: `ubuntu-latest`

## Continuous Deployment (Stubbed)

The `publish.yml` workflow is currently stubbed to prevent accidental deployment of unverified environments. Future expansions will include:

- Binary artifact generation.
- Docker image building and pushing to ECR/GHCR.

---

> [!NOTE]
> All PRs must have a passing CI status before they can be merged. No exceptions are made for "experimental" agent code.
