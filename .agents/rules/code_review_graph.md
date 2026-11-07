---
trigger: always_on
description: "Mandatory rules for using code-review-graph MCP tools and CLI during code analysis, pre-modification impact assessment, and post-modification verification"
---

# code-review-graph Agent Rules

This project has a persistent code-review-graph knowledge graph indexed at the repository root.

## Mandatory Directives

### 1. Pre-Modification Impact Assessment (MANDATORY BEFORE EDITING)
Before altering any existing function, struct, trait, or module:
- **Trace Blast Radius**: Call `get_impact_radius_tool(target="<SymbolName>")` or run `code-review-graph impact --target "<SymbolName>"`. Do NOT edit blindly without inspecting upstream dependents.
- **Trace Callers & Dependents**: Call `query_graph_tool(pattern="callers_of", target="<SymbolName>")`.
- **Trace Affected Flows**: Call `get_affected_flows_tool(file_paths=["<path_to_file>"])` to check which end-to-end execution paths cross the target code.
- **Check Existing Test Coverage**: Call `query_graph_tool(pattern="tests_for", target="<SymbolName>")` to identify existing tests that must be updated or run.

### 2. Post-Modification Risk Analysis (MANDATORY AFTER EDITING)
Immediately after modifying code files in a session:
- **Analyze Git Diff Impact**: Call `detect_changes_tool` to assess risk scores, affected functions, and churn.
- **Update the Graph**: Run `code-review-graph update` via CLI to ensure the SQLite knowledge graph and AST indices stay synchronized with working tree changes.

### 3. Complementary Usage with Graphify
- Use **`graphify`** for high-level conceptual questions, community navigation in `wiki/`, and God-node architecture overviews (`graphify query`, `graphify explain`).
- Use **`code-review-graph`** for deep structural AST impact analysis (`get_impact_radius_tool`), call-graph verification (`query_graph_tool`), execution flows (`get_affected_flows_tool`), and change risk detection (`detect_changes_tool`).
