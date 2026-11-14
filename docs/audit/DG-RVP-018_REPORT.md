# DG-RVP-018: Evidence Integrity, Security Hygiene and Documentation Accuracy

## Phase Summary

| Phase | Scope | PR | Status |
|---|---|---|---|
| **Phase A** | Evidence integrity (`evidence/baseline.md` quarantine, `verified: false` generator, held-out adversarial floor) | [#762](https://github.com/lgcorzo/rust_CACD_autonomous_factory/pull/762) | Merged to `main` |
| **Phase B** | Security hygiene (compile-time gating of `with_allow_insecure_host_execution`, fail-closed GitHub webhook, hardened `ValidatingAdmissionPolicy`, Calico/30Mi+35Mi docs, `Cargo.lock` RUSTSEC updates) | [#768](https://github.com/lgcorzo/rust_CACD_autonomous_factory/pull/768) | Open (awaiting human review & merge) |
| **Phase C** | Code & CI quality (tail-truncated 10KB logs, `tests/test_wiki_quality.py`, `deny.toml`, `rust-toolchain.toml`, CI `cargo deny` + `gitleaks` gates) | [#769](https://github.com/lgcorzo/rust_CACD_autonomous_factory/pull/769) | Open (awaiting human review & merge) |
| **Phase D** | Repository hygiene & documentation (`README.md` verified-facts cleanup, `docs/audit/ISSUE_HYGIENE.md`, `docs/audit/DG-RVP-018_REPORT.md`) | This PR | Open (awaiting human review & merge) |

## Defect Verification Summary
- **A1–A5**: `CONFIRMED` and remediated in PR #762.
- **B1–B5**: `CONFIRMED` and remediated in PR #768 (`B2` already had constant-time HMAC; missing-secret path was hardened to fail closed in all environments).
- **C1**: `CONFIRMED` in `crates/factory-infrastructure/src/github.rs` and `gitlab.rs`; fixed to tail-truncate to last 10KB.
- **C2**: `ALREADY_FIXED` (`pipeline_remediation.rs` already uses `escalation_marker` / fingerprint deduplication).
- **C3–C5**: `CONFIRMED` and remediated in PR #769.
- **D1**: `ALREADY_FIXED` (root scripts `parse_*.py`, `generate_openwiki.py`, `test_hatchet.py`, `test_ziti.rs` are not present in root).
- **D2–D3**: `CONFIRMED` and remediated in Phase D.
