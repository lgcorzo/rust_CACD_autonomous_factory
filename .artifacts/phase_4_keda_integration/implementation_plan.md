# Implementation Plan - Phase 4 Integration (DA-5 + DA-10)

## Goal

Complete **Phase 4** of the Autonomous Agent Factory by deploying **KEDA** for autoscaling and fully integrating **OpenCode workers** with Hatchet and Kafka.

## User Review Required

> [!IMPORTANT]
> **KEDA Location**: Deploying KEDA to `orchestrators` namespace as requested (standard is `keda` ns, but `orchestrators` works).
> **Secrets**: We need to duplicate `kafka-credentials` and `hatchet-admin-token` into the `agents` namespace so OpenCode and KEDA can access them.

## Proposed Changes

### Repository: `gitops_internal_lgcorzo`

#### [MODIFY] [infrastructure/sources/helmrepositories.yaml](file:///mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo/infrastructure/sources/helmrepositories.yaml)

- Add `kedacore` repo (`https://kedacore.github.io/charts`).

#### [NEW] [infrastructure/orchestrators/keda/release.yaml](file:///mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo/infrastructure/orchestrators/keda/release.yaml)

- `HelmRelease` for KEDA (v2.12.1), namespace `orchestrators`.

#### [MODIFY] [infrastructure/orchestrators/kustomization.yaml](file:///mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo/infrastructure/orchestrators/kustomization.yaml)

- Include `keda`.

#### [NEW] [infrastructure/confluent/kafka-topics.yaml](file:///mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo/infrastructure/confluent/kafka-topics.yaml)

- Define `KafkaTopic` resources: `mission-input`, `agent-thought`, `mission-artifact`.

### Repository: `llmops-python-package` branch feature/KEDA

#### [NEW] [k8s/base/secrets.yaml](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/k8s/base/secrets.yaml)

- **Placeholder/Instruction**: We need SealedSecrets for `kafka-credentials` and `hatchet-admin-token` in `agents` namespace.
- I will create a shell script `scripts/generate_secrets.sh` to help generate these from existing secrets.

#### [NEW] [k8s/base/keda-trigger-auth.yaml](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/k8s/base/keda-trigger-auth.yaml)

- `TriggerAuthentication` referencing `kafka-credentials`.

#### [MODIFY] [k8s/base/opencode-scaledobject.yaml](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/k8s/base/opencode-scaledobject.yaml)

- Add `authenticationRef` pointing to `keda-trigger-auth`.

#### [MODIFY] [k8s/base/opencode-deployment.yaml](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/k8s/base/opencode-deployment.yaml)

- Revert `optional: true` for Hatchet token (enforce strict startup).

## Verification Plan

### Automated

- `flux reconcile kustomization infrastructure`
- `kubectl get pods -n orchestrators -l app=keda-operator`

### Manual

1.  **Secret Check**: Confirm `kafka-credentials` and `hatchet-admin-token` exist in `agents`.
2.  **Scaling Test**:
    - `kubectl scale deployment opencode-agent --replicas=0 -n agents`
    - Send message to `mission-input` (using `kcat` or Hatchet UI).
    - Watch: `kubectl get hpa -n agents -w` -> replicas should go 0 -> 1 -> N.

## Jira Task Management (DA-5 / DA-10)

- **Initial Update**: Use the `jira_management` skill to append this implementation plan to the description of the relevant Jira ticket (e.g., DA-5 or DA-10).
- **Progress Updates**: As features are developed and tested, add comments to the Jira ticket outlining the advances, completed tasks, and context (files changed, infrastructure configured), following the `jira_management` skill guidelines.
