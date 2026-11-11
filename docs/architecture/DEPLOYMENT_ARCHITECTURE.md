> **Superseded 2026-11-11:** This historical architecture document describes a target topology, not a live deployment inventory. `deploy/` defines no `Namespace` objects, and the repository does not establish active CNI enforcement or live workload placement. See [the documentation fact register](../audit/DOC_FACT_REGISTER.md); deployed namespace inventory and enforcement are **TO CONFIRM**.

# Deployment Architecture & Plane Boundaries

**Specification**: `010-integrated-k8s-gitops-audit`  
**Date**: `2026-11-02`

---

## 1. Multi-Plane Architecture Overview

Dark Gravity separates responsibilities into four distinct planes:
1. **Control Plane**: Workflow dispatchers, Hatchet scheduler, FinOps monitors, and the MCP server.
2. **Security & Identity Plane**: HashiCorp Vault, Keycloak, and Ed25519 NHI credential issuers.
3. **Execution Plane (Isolated Sandboxes)**: Ephemeral Kubernetes Jobs configured for gVisor (`runsc`). Their deployed namespace is **TO CONFIRM**.
4. **GitOps & Reconciliation Plane**: FluxCD controllers synchronizing manifests from `gitops_internal_lgcorzo`.

---

## 2. Mermaid Plane Architecture Diagram

```mermaid
flowchart TD
    subgraph GitOpsPlane ["GitOps Plane (FluxCD)"]
        GitRepo["Git Repository: gitops_internal_lgcorzo"]
        FluxKustomize["Flux Kustomization Controller"]
        GitRepo -->|Reconcile| FluxKustomize
    end

    subgraph ControlPlane ["Control Plane (namespace TO CONFIRM)"]
        MCPServer["Factory MCP Server"]
        Hatchet["Hatchet Workflow Engine"]
        FinOps["FinOps Budget Monitor"]
        Scheduler["Mission Admission Gate"]
        
        Scheduler -->|Query spend status| FinOps
        Hatchet -->|Dispatch tasks| MCPServer
    end

    subgraph SecurityPlane ["Security Plane (namespace TO CONFIRM)"]
        Vault["HashiCorp Vault (ACL Policies)"]
        Keycloak["Keycloak OIDC"]
        SealedSecrets["Bitnami SealedSecrets"]
    end

    subgraph IsolatedExecutionPlane ["Execution Plane (namespace TO CONFIRM)"]
        SandboxJob["Sandbox Job (python/rust)"]
        gVisor["gVisor Sandbox Boundary (runsc)"]
        NetPolicy["Default-Deny NetworkPolicy"]
        
        SandboxJob --- gVisor
        SandboxJob --- NetPolicy
    end

    FluxKustomize -->|Applies Hardened Manifests| IsolatedExecutionPlane
    FluxKustomize -->|Applies Config & RBAC| ControlPlane
    
    MCPServer -->|Launch Sandbox Job via K8s API| SandboxJob
    MCPServer -->|Request Repository JIT Token| Vault
```

---

## 3. Network Isolation Policy Architecture

```mermaid
flowchart LR
    SandboxWorkload["Sandbox Pod (namespace TO CONFIRM)"]
    
    subgraph DeniedEndpoints ["Targets of the configured deny-all egress policy"]
        KubeAPI["Kubernetes API Server (443)"]
        Metadata["Metadata Service (169.254.169.254)"]
        VaultAdmin["Vault Cluster Admin (8200)"]
        InternalDBs["PostgreSQL / MinIO / Kafka"]
    end

    subgraph AllowedEndpoints ["Approved Whitelist Only"]
        GitHost["Authorized Git Remote / Package Mirror"]
    end

    SandboxWorkload -.-x|DENIED| KubeAPI
    SandboxWorkload -.-x|DENIED| Metadata
    SandboxWorkload -.-x|DENIED| VaultAdmin
    SandboxWorkload -.-x|DENIED| InternalDBs
    SandboxWorkload -->|RESTRICTED HTTPS| GitHost
```
