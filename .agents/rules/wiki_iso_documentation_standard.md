# Rule: Wiki ISO Documentation Standard

All documentation in `wiki/` must follow the ISO/IEC/IEEE 42010:2022 and ISO/IEC/IEEE 15289:2019 standard defined in `.agents/skills/wiki-iso-documentation-standard/SKILL.md`.

## Core Directives

1. **Directory Structure Enforcement**:
   - High-level architecture: `wiki/architecture/`
   - Security & Governance: `wiki/security/`
   - Operations & Runbooks: `wiki/operations/`
   - Quality & Testing: `wiki/quality/`
   - Codebase Crate Modules (1:1 structural mirror): `wiki/modules/{core,application,infrastructure,mcp_server,cli}/`
   - Navigation: Root `Home.md`, `README.md`, `_Sidebar.md`, `index.md`, `GLOSSARY.md`

2. **Mandatory ISO Frontmatter**:
   Every markdown file must declare standard YAML frontmatter containing:
   - `iso_doc_type`: `Description` | `Specification` | `Plan` | `Policy` | `Procedure` | `Report`
   - `iso_viewpoint`: `ArchitectureDescription` | `ContextView` | `ComponentView` | `SequenceView` | `DeploymentView` | `SecurityView` | `QualityView`
   - `type`: `architecture` | `security` | `operations` | `quality` | `module`
   - `title`: Component title
   - `source_path`: Project-relative path to source code
   - `description`: Concise functional scope
   - `tags`: List of classification tags
   - `last_verified_commit`: Short git SHA

3. **Mermaid.js Visualizations**:
   Every document must include an accurate Mermaid diagram (UML class diagram for modules, sequence diagram for workflows, or C4/flowchart for architecture).

4. **Zero Absolute Paths & Strict Relative Linking**:
   - Never use `/mnt/` or `/home/` paths.
   - All internal links must be relative Markdown links (`[Title](../folder/target.md)`).
   - Ensure 0 broken links upon every change.

5. **Zero Truncation**:
   No placeholders (`TODO`, `TBD`, `... existing code ...`). All code and documentation must be complete and compilable.
