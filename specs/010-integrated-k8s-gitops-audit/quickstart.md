# Quickstart & Verification Guide

**Feature**: `010-integrated-k8s-gitops-audit`

---

## 1. Local Workspace Verification Commands

### Test Execution Outcome Validation
```bash
# Verify run_tests rejects missing executables and does not simulate success
cargo test -p factory-mcp-server --test run_tests_security_tests

# Verify SubprocessDriver fails closed in production mode
cargo test -p factory-mcp-server --test sandbox_driver_tests
```

### FinOps Admission Gate Validation
```bash
# Verify FinOps hard-stop halts mission admission
cargo test -p factory-application --test finops_admission_tests
```

### Complete Quality Gates
```bash
# 1. Format check
cargo fmt --check

# 2. Clippy lint
cargo clippy --workspace --all-targets -- -D warnings

# 3. Full test suite
cargo test --workspace
```

---

## 2. GitOps & Kubernetes Manifest Validation

```bash
# Verify Kustomize builds cleanly for all agent manifests
cd /mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo
kubectl kustomize infrastructure/agents/
```

