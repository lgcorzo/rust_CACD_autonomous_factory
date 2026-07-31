---
iso_doc_type: "Description"
iso_viewpoint: "ContextView"
type: "architecture"
title: "ISO 42010 Context View: System Boundaries & Interfaces"
description: "Context View defining system boundaries, external actors, and communication protocols."
tags: ["iso42010", "context_view", "architecture", "system_boundaries"]
timestamp: "2026-07-31T16:35:00Z"
---

# ISO 42010 Context View: System Boundaries & External Interfaces

## 1. System Boundary Definition

The `rust_CACD_autonomous_factory` forms an autonomous code synthesis, verification, and execution engine operating within a secure, zero-trust infrastructure perimeter.

```mermaid
graph TB
    subgraph External_Ecosystem["External Systems & Enterprise Infra"]
        JIRA["Jira Cloud / Server API"]
        GITLAB["GitLab SCM"]
        KAFKA["Apache Kafka Event Bus"]
        VAULT["HashiCorp Vault Secrets"]
        S3["AWS S3 / R2 Bucket"]
        AETHALGARD["Aethalgard Remediation Webhook"]
        OPENZITI["OpenZiti Zero-Trust Network"]
    end

    subgraph Factory_Boundary["rust_CACD_autonomous_factory (EoI)"]
        CLI["factory-cli (Binary)"]
        MCP["factory-mcp-server (JSON-RPC)"]
        APP["factory-application (Bridge & Workflows)"]
        CORE["factory-core (Domain & Security)"]
        INFRA["factory-infrastructure (Clients)"]
    end

    subgraph Sandbox_Boundary["Isolation Layer"]
        GVISOR["gVisor K8s Sandbox Pod"]
        NATIVE_SUB["Native Subprocess Sandbox"]
    end

    CLI --> APP
    MCP --> APP
    APP --> CORE
    APP --> INFRA
    INFRA --> JIRA
    INFRA --> GITLAB
    INFRA --> KAFKA
    INFRA --> VAULT
    INFRA --> S3
    INFRA --> AETHALGARD
    INFRA --> OPENZITI
    MCP --> Sandbox_Boundary
    Sandbox_Boundary --> CORE
```

---

## 2. External Actor & Interface Specification

| External System | Protocol / Format | Interface Purpose | Implementation File Citation |
| :--- | :--- | :--- | :--- |
| **Aethalgard Remediation Service** | JSON-RPC 2.0 over HTTP POST | Dispatches automated remediation notifications on mission failures | `crates/factory-infrastructure/src/aethalgard.rs:L32-L54` |
| **Model Context Protocol (MCP) Clients** | JSON-RPC 2.0 over stdio / HTTP | Provides tools for code execution, code surgery, indexing, Jira search, and security reviews | `crates/factory-mcp-server/src/protocol.rs:L1-L120` |
| **OpenZiti Network Overlay** | Ziti C/Rust SDK | Establishes encrypted zero-trust microservice tunnels without public IP exposure | `crates/factory-infrastructure/src/ziti.rs:L1-L50` |
| **Apache Kafka** | Kafka Protocol | Broadcasts mission lifecycle events, audit telemetry, and QA status | `crates/factory-infrastructure/src/kafka.rs:L1-L65` |
| **gVisor Kubernetes Cluster** | Kubernetes K8s API / Custom Pod Spec | Runs un-trusted code executions inside kernel-isolated gVisor containers | `crates/factory-mcp-server/src/sandbox.rs:L121-L175` |
