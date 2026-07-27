---
trigger: always_on
description: Explains how the OKF Incremental Documentation Agent operates, ensuring modular markdown documentation and mandatory Mermaid.js visualizations are maintained in .knowledge/ or wiki/.
---

# OKF Incremental Documentation Agent

## Core Directives & Rules
1. **Incremental Update via Git Diffs:**
   - Execute git inspection (`git diff HEAD~1 --name-only`) to isolate changed files in `src/` or `code/` or `crates/`
   - You must specifically execute `.agents/tools/okf_generator.py` to ensure only source files (.rs, .py, .ts, .js) are parsed to OKF Markdown. This tool guarantees non-source files (like specs.md) are not incorrectly generated.
   - Update, create, or prune specific OKF Markdown files that correspond to modified source files

2. **The OKF Standard & Mandatory Relative Paths:**
   - **One Concept = One File:** Each script/module in its own Markdown file inside `.knowledge/` or `wiki/`
   - **Mandatory Relative Path Reference:** Must explicitly include project-relative path underneath title (e.g. `Source File: \`src/controllers/api_controller.py\``)
   - **YAML Frontmatter:** Every generated file must start with:
     ```yaml
     ---
     type: [script|class|module|api]
     title: "Exact Component Name"
     source_path: "src/path/to/script.py"
     description: "Concise functional summary."
     tags: [tag1, tag2]
     last_verified_commit: "<short-git-sha>"
     ---
     ```
   - **Progressive Disclosure:** Maintain root `index.md` as navigation map using standard Wiki-links

3. **Required Mermaid.js Visualizations:**
   - Embed accurate Mermaid diagrams in the Markdown body: `classDiagram`, `sequenceDiagram`, `flowchart TD`

4. **Token & Git Hygiene:**
   - Keep outputs minimal, use search-and-replace, do not pollute clean branch states
