# Technical Designs

## 1. Evidence Trust (`F-EVID-01`)
- **Signed Payload:** Canonical JSON/CBOR with domain separator `DG-EVIDENCE-V1`, binding `requirement_id`, `status`, `transition_history`, `verifier_id`, `evidence_sha256[]`, `source_commit`, `deploy_commit`, `policy_version`, `timestamp`, `nonce`.
- **Verification:** Reject `VERIFIED` if signature is missing, `verifier_id` lacks `Verifier` role in trust store, key is revoked, or nonce is replayed.

## 2. NHI & FinOps (`F-NHI-01`, `F-FINOPS-01`)
- **NHI:** Remove optionality of `trust_store` and `replay_cache` in production constructors; gate optional mode behind `#[cfg(test)]`.
- **FinOps:** Abstract `StopStore` trait; keep POSIX RWX for v1 single-node, add PostgreSQL `finops_stop_state` / `nhi_replay_nonces` schema for multi-replica.

## 3. Adversarial Scanner (`F-SCAN-01`)
- **Hybrid Engine:** Lexical normalization (strip whitespace/type annotations around `=`) + provider prefix/pattern matching + Shannon entropy check on extracted literals.
- **Evaluation:** 40 historical samples (regression) + new dev corpus + separate held-out corpus + >= 300 benign samples (Rust, Python, `.env`, YAML, JSON, CI logs).
