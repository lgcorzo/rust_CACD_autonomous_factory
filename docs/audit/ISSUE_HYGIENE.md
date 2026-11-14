# Issue Hygiene Proposal (DG-RVP-018 D3)

**Status:** Proposal for owner review (no issues closed or edited automatically).

## Objective
Reduce micro-task issue noise by grouping task-level issues into one parent issue per user story / epic, while preserving traceability via checklists.

## Open Questions for Owner
1. **Agent configuration directories**: The repository currently contains `.agents/`, `.gemini/`, `.jules/`, `.opencode.json`, and `.mcp.json`. Which single agent configuration directory should be retained?
2. **`mlops-python-package`**: Proposed to be extracted into its own dedicated repository (not moved in this PR).
3. **Historical `gitleaks` findings**: `gitleaks detect --source .` reports 67 historical findings across 1,389 commits. Should a `.gitleaksignore` or history-rewrite policy be applied for legacy commits?
