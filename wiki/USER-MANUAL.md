# 🏭 Dark Gravity — User Manual

Welcome to the **Dark Gravity Autonomous CA/CD Multi-Agent Software Factory**. Dark Gravity is a high-performance, long-term agentic system built in Rust and designed for Continuous Agentic / Continuous Deployment (CA/CD). It operates within a Zero Trust Kubernetes cluster, where multiple AI agents ingest, plan, code, validate, and document software autonomously.

This manual will guide you through the initial setup, operational workflows, agent usage, and the tools necessary to interact with the factory.

---

## 1. System Overview

The factory runs a durable multi-agent orchestrator backed by **Hatchet** workflows. It divides labor among four primary agent profiles:

1. **PO Agent (Product Owner):** Uses the Spec-Kit pipeline to write technical specifications, clarify requirements, and define atomic tasks based on the project constitution.
2. **Developer Agent:** Consumes tasks from the PO Agent, writing and refactoring code using Aider CLI within sandboxed Micro-VMs (Firecracker / gVisor).
3. **DevOps Agent:** Triggers the Aethelgard Auto-Remediation loop, catching test and build failures and feeding them back for a max of 3 retries.
4. **Documentation Agent:** Triggered upon merge, using the Superpowers framework to update Git Wiki and C4 models based on real compiled source analysis.

---

## 2. Prerequisites & Setup

### Environment Requirements
- **Rust Toolchain:** Version 1.75+
- **Python:** Version 3.11+
- **Hatchet Server:** For durable DAG workflow orchestration.
- **Kafka:** (Confluent Cloud or local) for telemetry.
- **Docker/Kubernetes:** For deploying agent environments.

### Component Installation

The factory's intelligence is primarily driven from the `rust_CACD_autonomous_factory` codebase. To get started:

```bash
# Clone the Rust Factory Repo
git clone https://github.com/lgcorzo/rust_CACD_autonomous_factory
cd rust_CACD_autonomous_factory

# Build the workspace
cargo build --release
```

Ensure your `.env` is configured correctly for telemetry:
```env
KAFKA_BROKERS=my-kafka-cluster-bootstrap.confluent.svc.cluster.local:9092
```

---

## 3. Running the Factory Worker

The central nervous system of Dark Gravity is the Rust worker. This worker listens to Hatchet events and processes agent missions across the 6-phase DAG (`Ingestion` → `Plan` → `Code` → `Validation` → `Review` → `Delivery`).

To start the factory worker:
```bash
# Point to your MCP gateway URL
cargo run -p factory-cli -- worker --mcp-url http://localhost:8100
```
*Note: All communications run over mTLS 1.3 using the OpenZiti Dark Mesh overlay for Zero Trust security.*

---

## 4. Workflows & Usage Guide

### 4.1. Spec-Driven Development (UC-1)
The **PO Agent** handles specification and task creation using the **Spec-Kit** pipeline. Before writing any code, the factory generates strict plans.

1. Install Spec-Kit:
   ```bash
   uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
   specify init autonomous_factory
   ```
2. Core Pipeline Commands:
   - `/speckit.constitution`: Applies global constraints (e.g., RAM limits, Zero Trust).
   - `/speckit.specify`: Defines what & why.
   - `/speckit.plan`: Outputs the architectural blueprint (`plan.md`).
   - `/speckit.tasks`: Outputs parallelized developer tasks (`tasks.md`).
   - `/speckit.taskstoissues`: Pushes atomic tasks to GitLab Issues.

### 4.2. Tooling: Graphify & Code Review Graph
Agents need profound context. The factory utilizes **Graphify** to construct a navigable AST-based knowledge graph of your project.

**Installation:**
```bash
uv tool install graphifyy
uv tool install code-review-graph
graphify install
code-review-graph install -y --platform antigravity
```

**Usage Commands:**
- **Refresh the Graph (Fast, No API Calls):**
  ```bash
  graphify update .
  ```
- **Ask the Graph:**
  ```bash
  graphify query "Explain the architecture of the Rustant planner"
  ```
- **Code Review Context:**
  ```bash
  code-review-graph detect-changes
  code-review-graph embed
  ```

*(Ensure `.env` contains the required `OPENAI_API_KEY` or LiteLLM gateway endpoints for embeddings).*

### 4.3. Documentation Generation (UC-4)
Once code is merged, the **Documentation Agent** executes automatically. It uses **Superpowers** to prevent hallucinatory documentation. 

To manually manage these capabilities or add custom skills:
- Look inside `.agents/skills/`.
- Skills like `updating-c4-models` and `writing-wiki-markdown` enforce formatting.
- The factory uses an **Orphan Symbol Rate (OSR)** check. If the agent writes documentation for a variable/class that doesn't actually exist in the code, the pipeline is automatically halted and regenerated.

---

## 5. Standard Operating Procedures (Human-in-the-Loop)

While the factory is highly autonomous, the **Anthropic Institute** guidelines strictly define human interaction at 4 specific vertices:

1. **Epic Creation:** You define the high-level *WHAT*. The PO Agent defines the *HOW*.
2. **Sprint Approval:** The Tech Lead reviews and approves the issues generated by Spec-Kit.
3. **Exception Override:** If an agent gets stuck past its 3-retry auto-remediation limit, a human steps in to unblock.
4. **Merge Approval:** Senior Developers review final Merge Requests strictly for architectural patterns and business logic validation.

---

## 6. Development & Contributing

When making changes to the factory's Rust core (`rust_CACD_autonomous_factory`), enforce standards via:

```bash
# Check formatting
cargo fmt --all -- --check

# Strict Linting
cargo clippy --workspace -- -D warnings

# Execute test suite
cargo test --workspace
```

---

## 7. Connecting a New Project for Autonomous Development

To create a new project in GitLab, connect it to Antigravity (the factory), and start developing autonomously, follow this step-by-step procedure:

### 7.1. Create the Project in GitLab
1. Log in to your GitLab instance.
2. Click **New project/repository** -> **Create blank project**.
3. Provide a project name, set the appropriate visibility level, and initialize it with a README.

### 7.2. Configure Factory Webhooks and Access
To allow Antigravity to listen to events and commit code to your repository:
1. **API Tokens**: Go to **Settings > Access Tokens**. Create a Project Access Token with `api`, `read_repository`, and `write_repository` scopes. Add this token and your GitLab Project ID to the factory worker's `.env` file (`GITLAB_API_TOKEN` and `GITLAB_PROJECT`).
2. **Webhook Setup**: Go to **Settings > Webhooks**. Add a webhook pointing to the factory's internal `n8n` event router URL. Check the box for **Epic events** to trigger the pipeline.
3. **Labels**: Create a project label named `autonomous-plan`.

### 7.3. Initialize the Agent Environment
Clone your new repository locally and prepare the factory's tools:
```bash
# Initialize Spec-Kit for the PO Agent
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
specify init your_project_name

# Build the initial AST Knowledge Graph
graphify install
graphify update .
code-review-graph embed
```
Push the `.specify` configuration and `.graphifyignore` files to your `main` branch.

### 7.4. Start Autonomous Development
The factory is strictly event-driven. To command Antigravity to write code:
1. In GitLab, navigate to **Plan > Epics**.
2. Create a new Epic detailing the high-level business goal or feature you want built (e.g., "Implement a secure REST API for user management").
3. Assign the label `autonomous-plan` to the Epic.
4. **Execution**: The webhook will fire. The PO Agent will ingest the Epic, decompose it into technical `tasks.md`, convert those tasks into GitLab Issues, and the Developer Agent will automatically check out branches and begin writing code inside a secure sandbox.

### 7.5. Connecting Sentry for Incident Auto-Remediation
To close the feedback loop between production failures and development, the factory integrates a **QA Observer Agent** that connects to your project's Sentry instance.

1. **API Tokens**: Generate a Sentry Auth Token with `event:read` and `project:read` permissions.
2. **Environment Variables**: Add the following Sentry credentials to the factory worker's `.env` file:
   ```env
   SENTRY_URL=https://sentry.io          # Or your self-hosted Sentry instance
   SENTRY_API_TOKEN=your-sentry-token
   SENTRY_PROJECT=your-project-slug
   ```
3. **Execution Flow**: Once configured, the QA Observer Agent polls Sentry every 15 minutes. When a new exception or user bug report is detected:
   - The agent extracts the stack trace and telemetry data.
   - It queries the R2R GraphRAG to map the error to the responsible code module or microservice.
   - It automatically generates a prioritized **GitLab Issue** and labels it `autonomous-plan`, seamlessly triggering the PO Agent to plan a fix and the Developer Agent to implement it.
