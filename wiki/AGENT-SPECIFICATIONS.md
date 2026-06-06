# AGENT-SPECIFICATIONS: Autonomous Workforce

This document specifies the roles, responsibilities, and tooling for the **Dark Gravity** autonomous agents.

---

## Rustant (Product Owner Agent)

The "Captain" of the mission. Governs the **Intelligence Context**.

### Responsibilities
- **Spec-Driven Planning**: Queries R2R GraphRAG directly via `R2rClient` for context pruning, then calls `plan_mission` via MCP to plan the mission.
- **Architectural & Security Review**: Audits code via `security_review` MCP tool.

### Implementation
- **File**: `crates/factory-application/src/agents/rustant.rs`
- **Dependencies**: `McpClient`, `R2rClient`
- **Evaluation**: Strategy Score, Token Efficiency

---

## ZeroClaw (Developer Agent)

The "Muscle" of the system. Operates within the **Execution Context**.

### Responsibilities
- **Code Implementation**: Translates spec-driven tasks into source code via `execute_code` (Aider CLI for SEARCH/REPLACE mutations).
- **Verification**: Runs test suites via `run_tests` inside isolated sandbox (gVisor/Firecracker).
- **Self-Correction**: Iterates on code based on test failure feedback (max 3 retries before Agent-Stuck escalation).

### Implementation
- **File**: `crates/factory-application/src/agents/zeroclaw.rs`
- **Dependencies**: `McpClient`
- **Sandbox Drivers**: `SubprocessDriver` (local), `FirecrackerDriver` (micro-VM via KVM/AF_VSOCK)
- **Evaluation**: Test Pass Rate, Execution Latency

---

## DevOps Agent (Aethelgard Loop)

> **Status: Planned (Not yet implemented)**

The "Immune System". Governs the **Remediation Context**.

### Responsibilities
- **CI/CD Auto-Remediation**: Parses pipeline failures, queries R2R GraphRAG for historical fixes, directs Developer Agent to apply patches.
- **Circuit Breaker**: Maximum 3 consecutive auto-remediation attempts before setting `Agent-Stuck` and escalating to humans.
- **Production Incident Polling**: Polls Sentry API every 15 minutes for new exceptions.
- **Backlog Automation**: Auto-grades severity and creates backlog issues from production errors.

### Evaluation
- **Action Success Rate**: Reliability of external API calls.
- **Correction Frequency**: Number of self-correction cycles needed.

---

## Documentation Agent (Superpowers)

> **Status: Planned (Not yet implemented)**

The "Memory Keeper". Manages the **Infrastructure Context** for documentation.

### Responsibilities
- **AST Delta Sync**: `deepwiki-rs` parses git merge deltas via Tree-sitter, extracts AST deltas, upserts embeddings to pgvector.
- **Wiki Regeneration**: Uses Superpowers skills (`writing-plans`, `subagent-driven-development`, `verification-before-completion`) to update Wiki pages in parallel.
- **OSR Gate**: Verifies Orphan Symbol Rate < 5% before any Wiki commit.
- **R&D Grant Packaging**: Auto-compiles telemetry, hours, and git deltas into Hazitek/SPRI European grant schemas.

### Superpowers Skills Loaded
| Skill | Purpose |
| :--- | :--- |
| `writing-plans` | Decompose documentation into atomic tasks |
| `dispatching-parallel-agents` | Spawn isolated subagents per independent task |
| `subagent-driven-development` | Execute each subagent with focused context |
| `verification-before-completion` | OSR check before Wiki commit |
| `requesting-code-review` | Validate C4 diagram syntax and Markdown style |
| `finishing-a-development-branch` | Commit verified docs and close Epic |

---

## Agent Interface

All agents implement the `Agent` trait in `crates/factory-application/src/lib.rs`:

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> String;
    async fn execute(&self, task_description: &str) -> anyhow::Result<Value>;
}
```

---

## LLM Configuration

All agents route through the **LiteLLM Gateway**:

| Model | Provider | Tool Calling |
| :--- | :--- | :--- |
| `ollama/qwen2.5-coder:7b` | LiteLLM (Ollama) | Yes |
| `ollama/qwen2.5:7b` | LiteLLM (Ollama) | No |
| `gemini-3-pro` | LiteLLM (Gemini) | No |
| `gemma4:31b-cloud` | LiteLLM (Ollama) | No |

---

## Deliverable Agent

The 5th DAG phase (`factory:deliver`) is handled by Hatchet Engine directly. Upon security review approval (`approved: true`), it invokes `create_pull_request` via MCP to create a GitHub PR with mission artifacts.
