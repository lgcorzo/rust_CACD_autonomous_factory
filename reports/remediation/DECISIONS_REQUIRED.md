# Decisions Required (Remaining)

All major architectural decisions (Rondas 1–3) are **APPROVED**. Only operational confirmations remain during execution:
1. Confirmation of which of the 67 historical Gitleaks findings (if any) are still active and require rotation (`PR-6`).
2. Selection of the target Kubernetes cluster for `PR-10` (`RUNTIME_VERIFIED` of `gVisor/runsc` and real mission).
3. Choice of single agent configuration directory to retain in `rust_CACD_autonomous_factory` (`.agents`, `.gemini`, `.jules`, `.opencode.json`, `.mcp.json`).
