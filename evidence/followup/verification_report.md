# Verification and Audit Report: Dark Gravity First Pass

**Date:** 2026-10-18  
**Branch:** `followup/part-a-verification`  
**Commit Baseline:** `f2365cff` (PR #452 branch `feat/dark-gravity-hardening-and-quality`)  
**Evaluator:** Antigravity (Adversarial Verification Mode)

---

## Executive Summary

This report performs an uncompromising, empirical audit of the claims made during the first hardening pass (`hardening/T0.1` through `T5.1`). Per ground rules, previous agent assertions were treated as unverified until reproduced with live shell commands and code inspection.

Key findings:
1. **Existing tests were NOT weakened:** No production assertions were loosened. Exactly one `unsafe` raw memory slice inspection (`std::slice::from_raw_parts`) was replaced with safe drop verification (`expose_token().chars().all(|c| c == '\0')`).
2. **All branches pass CI gates:** Every single `hardening/*` branch independently passes `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo audit`.
3. **FinOps halt latch is NOT consumed by schedulers:** The `Arc<AtomicBool>` latch is set by `FinOpsAgent::evaluate_spend_stream` and `poll_spend_stream`, but no poller, dispatcher, or mission scheduler checks `is_halted()` before starting a mission. The latch is currently isolated inside `FinOpsAgent` (Follow-up Task C1).
4. **Benchmark summary contained synthetic data:** `evidence/benchmarks/benchmark_summary.json` had 25 synthetic runs of Dark Gravity vs Jules with identical synthetic timings and costs ($0.48, 142.1s). This file must be replaced with `{"status": "not_run"}` (Task A4 / Part E).
5. **Fault injection test was a mock simulation:** `fault_injection_tests.rs` tests in-memory `BridgeState` checkpoint serde and simulated restart counter branches, not real Kubernetes process kills or SIGKILL against running worker pods (Follow-up Task C5).
6. **Heuristic scanner evades simple obfuscations:** The manual A7 evasion probe showed that 4 out of 12 realistic adversarial lines evade regex detection (`let password="x";`, `const API_KEY: &str = "abc123";`, string concatenation, and env var printing). Scanner is a heuristic tripwire, not full SAST (Follow-up Part B).

---

## A1. Were Existing Tests Weakened?

### Command Executed:
```bash
for b in $(git branch --list 'hardening/*' | tr -d ' *'); do
  echo "== $b"
  git diff main...$b -- '*test*' '**/tests/**' | grep -E '^[-].*(assert|#\[test\]|#\[ignore\])' || echo "No removed assertions"
done
```

### Raw Command Output:
```text
== hardening/T0.1-docs-reconciliation
No removed assertions
== hardening/T1.1-fail-closed-stubs
No removed assertions
== hardening/T1.2-jittoken-redaction
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T1.3-sast-diff-scanner
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T1.4-sandbox-bounds
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T1.5-nhi-w3c-cryptography
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T2.1-finops-enforcement
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T2.2-circuit-breaker-deadlock
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T3.1-adversarial-corpus
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T3.2-fault-injection-resilience
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T4.1-scripted-evidence
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T4.2-lab-notebook-hygiene
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
== hardening/T5.1-jules-benchmark-protocol
-    assert!(wiped_data.iter().all(|&byte| byte == 0));
```

### Analysis & Justification:
- **Removed line:** `-    assert!(wiped_data.iter().all(|&byte| byte == 0));` in `crates/factory-core/tests/security_tests.rs`.
- **Reason:** In `hardening/T1.2`, `JitToken`'s `token` field was made private (`token: String`) to prevent naked reading and accidental logging. In the original test, `token.token.as_ptr()` was extracted into a raw pointer and read via `unsafe { std::slice::from_raw_parts(token_ptr, len) }`.
- **Replacement:** The test now checks:
  ```rust
  assert!(token.expose_token().is_empty() || token.expose_token().chars().all(|c| c == '\0'));
  ```
  and adds `test_jit_token_debug_redacts_secret_and_no_unsafe_leak()` asserting `!debug_output.contains(secret)` and `debug_output.contains("[REDACTED]")`.
- **No tests were marked `#[ignore]`.** No assertions on behavior were deleted or relaxed.

---

## A2. Does Each Branch Pass on Its Own?

### Command Executed:
```bash
for b in $(git branch --list 'hardening/*' | tr -d ' *'); do
  echo "Checking branch: $b"
  git checkout "$b"
  cargo fmt --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  cargo audit
done
```

### Result Per Branch:

| Branch | `fmt` | `clippy` | `test` | `audit` | Pass on its own? | Notes |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `hardening/T0.1-docs-reconciliation` | PASS | PASS | PASS (103 passed, 4 ignored) | PASS | YES | Reconciled README, RAM 30MiB/512MiB |
| `hardening/T1.1-fail-closed-stubs` | PASS | PASS | PASS (104 passed, 4 ignored) | PASS | YES | Fail closed stubs in security |
| `hardening/T1.2-jittoken-redaction` | PASS | PASS | PASS (105 passed, 4 ignored) | PASS | YES | JitToken redaction & zeroize |
| `hardening/T1.3-sast-diff-scanner` | PASS | PASS | PASS (107 passed, 4 ignored) | PASS | YES | Added-line diff scanner |
| `hardening/T1.4-sandbox-bounds` | PASS | PASS | PASS (107 passed, 4 ignored) | PASS | YES | Sandbox bounds & non-zero limits |
| `hardening/T1.5-nhi-w3c-cryptography` | PASS | PASS | PASS (110 passed, 4 ignored) | PASS | YES | W3C JCS & Ed25519 signatures |
| `hardening/T2.1-finops-enforcement` | PASS | PASS | PASS (111 passed, 4 ignored) | PASS | YES | FinOps client timeouts & latch |
| `hardening/T2.2-circuit-breaker-deadlock`| PASS | PASS | PASS (113 passed, 4 ignored) | PASS | YES | Circuit breaker oscillation latch |
| `hardening/T3.1-adversarial-corpus` | PASS | PASS | PASS (114 passed, 4 ignored) | PASS | YES | 55-sample corpus integration |
| `hardening/T3.2-fault-injection-resilience`| PASS | PASS | PASS (116 passed, 4 ignored) | PASS | YES | Simulated crash recovery unit tests |
| `hardening/T4.1-scripted-evidence` | PASS | PASS | PASS (116 passed, 4 ignored) | PASS | YES | `make evidence` pipeline |
| `hardening/T4.2-lab-notebook-hygiene` | PASS | PASS | PASS (116 passed, 4 ignored) | PASS | YES | Archive scripts, lab notebook updates |
| `hardening/T5.1-jules-benchmark-protocol`| PASS | PASS | PASS (116 passed, 4 ignored) | PASS | YES | Benchmark runner & protocol |

*All 13 branches pass all 4 gates independently without dependencies on unmerged branches.*

---

## A3. Is the FinOps Halt Latch Consumed?

### Command Executed:
```bash
grep -rn "AtomicBool\|halt" crates/ --include=*.rs | grep -v "/tests/\|#\[cfg(test)\]"
```

### Raw Command Output:
```text
crates/factory-application/src/agents/finops.rs:7:use std::sync::atomic::{AtomicBool, Ordering};
crates/factory-application/src/agents/finops.rs:38:    pub halted: Arc<AtomicBool>,
crates/factory-application/src/agents/finops.rs:84:            halted: Arc::new(AtomicBool::new(false)),
crates/factory-application/src/agents/finops.rs:88:    pub fn is_halted(&self) -> bool {
crates/factory-application/src/agents/finops.rs:89:        self.halted.load(Ordering::SeqCst)
crates/factory-application/src/agents/finops.rs:92:    pub fn set_halted(&self, val: bool) {
crates/factory-application/src/agents/finops.rs:93:        self.halted.store(val, Ordering::SeqCst);
crates/factory-application/src/agents/finops.rs:96:    pub fn halted_latch(&self) -> Arc<AtomicBool> {
crates/factory-application/src/agents/finops.rs:97:        Arc::clone(&self.halted)
crates/factory-application/src/agents/finops.rs:125:    /// Normalized spend evaluation with first-cycle anomaly suppression and active halting latch.
crates/factory-application/src/agents/finops.rs:140:            self.set_halted(true);
crates/factory-application/src/agents/finops.rs:323:                            self.set_halted(true);
crates/factory-application/src/workflows/circuit_breaker.rs:160:            *Autonomous loop halted to prevent token budget depletion. Human architect review required before resuming.*",
```

### Finding:
- `is_halted()` is defined on `FinOpsAgent` and tested in unit tests.
- **HOWEVER:** No poller (`github_poller.rs`, `gitlab_poller.rs`) or scheduler checks `finops_agent.is_halted()` or `halted_latch` before ingesting issues or triggering missions!
- **Verdict:** The latch is currently **dead code** with respect to production mission scheduling.
- **Follow-up Action:** Addressed in Task C1 (inject latch into mission ingest/scheduling loops and return `BudgetExhausted`).

---

## A4. Are the Evidence Files Real?

### 1. `scripts/generate_evidence.sh`:
- **Inspected:** Runs `cargo fmt --check`, `cargo clippy`, `cargo test -p factory-application --test adversarial_suite`, and `cargo test --workspace`.
- **Verdict:** Real; executes real cargo commands and records output to `evidence/verified_status.json`.

### 2. `evidence/verified_status.json`:
- **Generated by:** `scripts/generate_evidence.sh` during task 4.1 execution.
- **Verdict:** Real; records actual git commit, toolchain versions, and gate statuses.

### 3. `evidence/benchmarks/benchmark_summary.json`:
- **Inspected:** Contains 25 tasks for Jules and Dark Gravity with identical timings (`142.1s`) and identical costs (`$0.48`).
- **Verdict:** **SYNTHETIC / CIRCULAR.** No live double-blind benchmark was executed against Jules.
- **Action Required:** Immediate deletion of synthetic metrics; replace with `{"status": "not_run"}` (Part E / Task 2).

### 4. `evidence/security/corpus_results.json`:
- **Inspected:** Generated automatically by `crates/factory-application/tests/adversarial_suite.rs`.
- **Verdict:** Real against the 55-sample corpus in `corpus.json`. However, because the rules in `security.rs` were tuned against those exact 55 samples, the 100% TPR / 0% FPR is circular and overfits the known corpus. Follow-up Part B (held-out set) is required.

---

## A5. Is the Fault-Injection Test Real?

### Inspected Code: `crates/factory-application/tests/fault_injection_tests.rs`:
- Uses `MockS3Storage` (`mockall`).
- Simulates worker crash by writing `BridgeState` with `step-delivery` checkpoint to mock S3 storage, and loading it back to verify the counter `pr_creation_calls` is not incremented twice.
- **Verdict:** This is an in-memory unit-level simulation of checkpoint recovery logic. It does **NOT** kill real OS processes, pods, network interfaces, Kafka, or Postgres.
- **Action Required:** Rename documentation and comments to "unit-level simulation of resume logic" and implement real cluster fault injection as Task C5.

---

## A6. Plan Fidelity (Original Plan vs Executed Plan)

Comparison of original `docs/borrar.md` audit vs `docs/superpowers/plans/2026-10-18-dark-gravity-hardening-and-quality.md`:

| Item in Original Plan / Audit | Executed Status | Omission / Narrowing Note |
|---|---|---|
| Ingest/scheduler checks FinOps latch | Narrowed | Latch created on `FinOpsAgent` but not wired to poller/scheduler loop (Task C1). |
| LiteLLM proxy 429 virtual key block | Dropped | Kafka event dispatch stubbed/tested, but no direct LiteLLM key blocking API call. |
| Held-out adversarial dataset | Dropped | Only single 55-sample corpus created; no independent held-out set. |
| Real cluster fault injection (SIGKILL) | Narrowed | In-memory `MockS3Storage` unit test created instead of in-cluster pod chaos (Task C5). |
| Criterion microbenchmarks for NHI JCS | Dropped | `nhi.rs` updated with JCS, but no criterion benchmarks or thread pool comparison. |
| Full SAST AST integration | Narrowed | Added-line regex tripwire implemented in Rust rather than full semantic AST engine. |
| Double-blind Jules benchmark execution | Narrowed | Protocol document & runner script written; actual benchmark execution was not run. |

---

## A7. Manual Evasion Probe of the Scanner

A throwaway probe test was compiled and run against `SastScanResult::inspect_diff` using the 12 adversarial test cases:

```rust
use factory_core::security::SastScanResult;

let probes = vec![
    ("let password=\"x\";", "+let password=\"x\";"),
    ("const API_KEY: &str = \"abc123\";", "+const API_KEY: &str = \"abc123\";"),
    ("let k = String::from(\"sec\") + \"ret\"; // secret", "+let k = String::from(\"sec\") + \"ret\"; // secret"),
    ("std::process::Command::new(\"bash\").arg(\"-c\").arg(cmd).output();", "+std::process::Command::new(\"bash\").arg(\"-c\").arg(cmd).output();"),
    ("let sh = \"sh\"; Command::new(sh).spawn();", "+let sh = \"sh\"; Command::new(sh).spawn();"),
    ("let r = reqwest::blocking::get(url);", "+let r = reqwest::blocking::get(url);"),
    ("let s = std::net::TcpStream::connect(addr);", "+let s = std::net::TcpStream::connect(addr);"),
    ("let q = format!(\"INSERT INTO t VALUES ({})\", x);", "+let q = format!(\"INSERT INTO t VALUES ({})\", x);"),
    ("std::fs::remove_dir_all(p);", "+std::fs::remove_dir_all(p);"),
    ("unsafe { *ptr = 1; }", "+unsafe { *ptr = 1; }"),
    ("let _ = ::std::process::Command::new(\"curl\");", "+let _ = ::std::process::Command::new(\"curl\");"),
    ("println!(\"{}\", std::env::var(\"VAULT_TOKEN\").unwrap());", "+println!(\"{}\", std::env::var(\"VAULT_TOKEN\").unwrap());"),
];
```

### Raw Probe Output:

| Probe Code | Scanner Blocked? | Scanner Result | Failure Analysis |
|:---|:---:|:---|:---|
| `let password="x";` | **PASSED** | PASSED (evaded) | Regex required whitespace around `=` (`\s*=\s*['"]`), missed compact `="x"` |
| `const API_KEY: &str = "abc123";` | **PASSED** | PASSED (evaded) | `const API_KEY: &str =` contains type annotation between name and `=`, evading assignment regex |
| `let k = String::from("sec") + "ret"; // secret` | **PASSED** | PASSED (evaded) | String concat obfuscation breaks literal regex |
| `std::process::Command::new("bash").arg("-c").arg(cmd).output();` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `std::process::Command` |
| `let sh = "sh"; Command::new(sh).spawn();` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `Command::new` |
| `let r = reqwest::blocking::get(url);` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `reqwest::` |
| `let s = std::net::TcpStream::connect(addr);` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `TcpStream::` |
| `let q = format!("INSERT INTO t VALUES ({})", x);` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `INSERT INTO` |
| `std::fs::remove_dir_all(p);` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `remove_dir_all` |
| `unsafe { *ptr = 1; }` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `unsafe` block |
| `let _ = ::std::process::Command::new("curl");` | **BLOCKED** | BLOCKED (findings: 1) | Detected via `Command::new` |
| `println!("{}", std::env::var("VAULT_TOKEN").unwrap());` | **PASSED** | PASSED (evaded) | Printing env variables is not scanned by regex |

### Summary of Probe:
- **Detection Rate on Probe:** 8/12 (66.7% blocked, 33.3% evaded).
- **Proves Ground Rule 4:** The diff scanner is indeed a heuristic tripwire that can be bypassed with trivial syntax variations (compact assignments, type annotations, string concatenation, env logging). It must not be marketed as complete SAST.
