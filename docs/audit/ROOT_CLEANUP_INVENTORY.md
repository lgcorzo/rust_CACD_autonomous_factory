# Root Cleanup Inventory

**Profile:** **A. Original code repo** (`rust_CACD_autonomous_factory` is an original Rust + Python codebase).

> **Note:** Loose scripts and test files (`parse_*.py`, `generate_openwiki.py`, `test_hatchet.py`, `test_ziti.rs`) were already moved out of the repository root in an earlier cleanup (`scripts/archive/legacy_parsers/`, `scripts/archive/`, `scripts/generate_openwiki.py`).

## Root Entries Inventory

| Path | What it is | Who uses it (file & line) | Proposed action | Reason |
|---|---|---|---|---|
| `AGENTS.md` | Agent instructions & HITL policy | Coding agents at repo root | `KEEP` | Standard root agent rules file |
| `Cargo.toml` | Rust workspace manifest | `cargo`, CI workflows (`.github/workflows/`) | `KEEP` | Required at repo root for Cargo workspace |
| `Cargo.lock` | Rust dependency lockfile | `cargo`, `Dockerfile` | `KEEP` | Required at repo root for reproducible builds |
| `Dockerfile` | Container image build definition | CI/CD & local Docker builds | `KEEP` | Standard root build file |
| `.dockerignore` | Docker build context exclusions | `docker build` | `KEEP` | Standard root Docker config |
| `.gitignore` | Git ignore rules | `git` | `KEEP` | Standard root Git config |
| `.graphifyignore` | Graphify AST exclusions | `graphify` | `KEEP` | Tool reads it from repo root |
| `LICENSE` | Repository license | GitHub / legal | `KEEP` | Standard root project file |
| `Makefile` | Developer task runner | Developers | `KEEP` | Standard root build/task file |
| `README.md` | Project documentation | GitHub / developers | `KEEP` | Standard root project file |
| `SECURITY.md` | Security policy | GitHub / security | `KEEP` | Standard root project file |
| `deny.toml` | `cargo-deny` license/advisory config | `cargo deny` in CI | `KEEP` | Standard root Cargo-deny config |
| `rust-toolchain.toml` | Rust toolchain pin | `rustup` / `cargo` | `KEEP` | Standard root toolchain pin |
| `.mcp.example.json` | Example MCP server config | Developers / agents | `KEEP` | Template for `.mcp.json` |
| `.opencode.json` | OpenCode agent config | OpenCode CLI | `KEEP` | Tool reads it from repo root |
| `.agents/` | Agent skills, rules, and tools | Coding agents | `KEEP` | Tool reads it from repo root; document in `docs/agents/README.md` |
| `.gemini/` | Gemini agent config | Gemini / Antigravity | `KEEP` | Tool reads it from repo root; document in `docs/agents/README.md` |
| `.jules/` | Jules agent config | Jules | `KEEP` | Tool reads it from repo root; document in `docs/agents/README.md` |
| `.cargo/` | Cargo config | `cargo` | `KEEP` | Standard Cargo directory |
| `.github/` | GitHub Actions workflows & templates | GitHub | `KEEP` | Required at repo root |
| `benches/` | Rust benchmarks | `cargo bench` | `KEEP` | Standard Rust workspace directory |
| `config/` | Runtime/service configuration | Application & deploy | `KEEP` | Project config directory |
| `crates/` | Rust workspace crates | `Cargo.toml` | `KEEP` | Core Rust source |
| `deploy/` | Kubernetes / GitOps manifests | Cluster deployment | `KEEP` | Out of scope to move |
| `docs/` | Project documentation & audits | Developers / wiki | `KEEP` | Standard docs directory |
| `evaluate/` | Evaluation outputs (`evaluate/reports/summary.md`) | Evaluation scripts | `OWNER DECISION` | Contains `evaluate/reports/summary.md` (tracked); if generated output, propose `.gitignore` or removal |
| `evidence/` | Verification evidence records | `factory-cli` / CI | `KEEP` | Tracked evidence directory |
| `mlops-python-package/` | Embedded Python MLOps package | Python/MLOps workflows | `KEEP` | Keep for now; moving to its own repo requires extracting git history (`git subtree split`), publishing/pinning as a package dependency, and updating CI/Dockerfile references |
| `scripts/` | Utility, wiki, and archived scripts | CI / developers | `KEEP` | Target directory for scripts |
| `specs/` | Spec Kit feature specifications | Spec Kit / developers | `KEEP` | Active feature specs |
| `tests/` | Integration & contract tests | `cargo test` / `pytest` | `KEEP` | Standard test directory |
| `wiki/` | Generated OpenWiki documentation | OpenWiki / developers | `KEEP` | Project wiki |

## Security, Large Files, and Untracked Root Items

- **Credential-like files:**
  - `.env` (present in repository root, **untracked / gitignored** — rotate if it contains live secrets, and never commit).
- **Tracked files over 1 MB:**
  - `docs/assets/audio/full_narration.wav` (14M)
  - `docs/assets/screencast_dark_gravity_walkthrough.mp4` (27M)
- **Untracked / gitignored local tool & build directories in root:**
  - `.code-review-graph/` (`graph.db` 30M), `graphify-out/` (`graph.json` 4.6M+), `.pytest_cache/`, `.specify/`, `.superpowers/`, `.venv/`, `.vscode/`, `target/`, `.mcp.json`, `evidence/runs/baseline-20261114T152614Z/`.
- **Proposed deletions:** None (no unused tracked files in the root; `evaluate/reports/summary.md` is marked `OWNER DECISION`).
