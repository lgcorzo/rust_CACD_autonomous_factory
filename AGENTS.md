<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
at specs/005-pipeline-error-remediation/plan.md
<!-- SPECKIT END -->

## Branch & Pull Request Policy: Human-in-the-Loop (HITL)

- Agents are empowered to create feature branches, commit code, push upstream, create Pull Requests (PRs) / Merge Requests (MRs), and resolve CI/CD failures.
- **STRICT HUMAN-IN-THE-LOOP REQUIREMENT**: Under no circumstances may an agent automatically merge a PR or MR into `main` or default branches.
- The merge process **MUST be a manual human action**.
- Upon ensuring all CI/CD pipelines, linters, and CodeQL checks pass with green status, the agent must present the PR/MR link and test evidence to the user and await manual human review and merge.

