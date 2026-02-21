# Phase 4 Integration (DA-5 + DA-10)

## Planning

- [x] Analyze DA-5 and DA-10 requirements
- [x] Combine requirements into unified plan
- [ ] User approval on implementation plan <!-- id: 1 -->

## Execution — Infrastructure (GitOps)

- [ ] **KEDA Controller** (DA-10)
  - [ ] Add `kedacore` Helm repo to `sources/helmrepositories.yaml` <!-- id: 2 -->
  - [ ] Create `infrastructure/orchestrators/keda/release.yaml` (Namespace: `orchestrators`) <!-- id: 3 -->
  - [ ] Register in `infrastructure/orchestrators/kustomization.yaml` <!-- id: 4 -->

- [ ] **Kafka Topics** (DA-5)
  - [ ] Create `infrastructure/confluent/kafka-topics.yaml` (`mission-input`, `agent-thought`, `mission-artifact`) <!-- id: 5 -->

## Execution — Agent Configuration (LLMOps Repo)

- [ ] **Secrets** (DA-5)
  - [ ] Create/Copy `kafka-credentials` to `agents` namespace (SealedSecret) <!-- id: 6 -->
  - [ ] Create `hatchet-admin-token` in `agents` namespace (SealedSecret) <!-- id: 7 -->

- [ ] **KEDA Resources** (DA-10)
  - [ ] Create `k8s/base/keda-trigger-auth.yaml` (references `kafka-credentials`) <!-- id: 8 -->
  - [ ] Update `k8s/base/opencode-scaledobject.yaml` (correct triggers) <!-- id: 9 -->

- [ ] **Deployment Update**
  - [ ] Update `opencode-deployment.yaml` to remove `optional: true` for Hatchet token <!-- id: 10 -->

## Verification

- [ ] **Infrastructure**: KEDA pods running in `orchestrators` <!-- id: 11 -->
- [ ] **Kafka**: Topics created in Confluent <!-- id: 12 -->
- [ ] **Scaling**: Publish message to `mission-input` -> OpenCode scales 0->1 <!-- id: 13 -->
- [ ] **Hatchet**: OpenCode worker connects to Hatchet engine <!-- id: 14 -->

## Finalization

- [ ] Update 11.0 Doc & Jira (DA-5, DA-10) <!-- id: 15 -->
