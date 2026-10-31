# Quickstart & Verification Guide: Google AX CRD Workspace Standard

**Feature**: `008-crd-workspace-standard`
**Branch**: `008-crd-workspace-standard`

This guide specifies end-to-end operational verification scenarios to prove compliance with ADR-008 v3.1 and the feature specification.

---

## 1. Prerequisites

1. Kubernetes cluster with `ValidatingAdmissionPolicy` support (MicroK8s v1.30+).
2. Deployed CRD and admission policies:
   ```bash
   kubectl apply -f specs/008-crd-workspace-standard/contracts/crd-agentworkspace.yaml
   kubectl apply -f specs/008-crd-workspace-standard/contracts/validating-admission-policies.yaml
   ```
3. Factory operator binary compiled:
   ```bash
   cargo build -p factory-cli --bin factory-cli
   ```

---

## 2. Validation Scenario 1: Admission Boundary Enforcement (SC-005)

### Test A: Reject Manifest Violating Memory Ceiling (>30Mi)
```bash
cat <<EOF | kubectl apply -f -
apiVersion: darkgravity.io/v1alpha1
kind: AgentWorkspace
metadata:
  name: test-over-limit
  namespace: agents
spec:
  repositoryUrl: "https://github.com/lgcorzo/lince-rs.git"
  targetBranch: "main"
  constitutionRef: ".specify/memory/constitution.md"
  resourceLimits:
    maxRam: "64Mi"
    maxCpu: "200m"
EOF
```
**Expected Outcome**: API server immediately rejects creation with CEL validation error:
`maxRam must be a quantity in (0, 30Mi]`.

### Test B: Reject Sandbox Job Exceeding 30Mi in `development` Namespace
```bash
cat <<EOF | kubectl apply -f -
apiVersion: batch/v1
kind: Job
metadata:
  name: test-sandbox-job-violation
  namespace: development
spec:
  template:
    spec:
      restartPolicy: Never
      containers:
        - name: worker
          image: busybox
          command: ["sh", "-c", "echo test"]
          resources:
            requests:
              memory: "20Mi"
              cpu: "100m"
            limits:
              memory: "40Mi"
              cpu: "250m"
EOF
```
**Expected Outcome**: API server rejects with `ValidatingAdmissionPolicy`:
`container limits must be <= 30Mi / 250m and requests <= limits (ADR-008 §9.2.1)`.

---

## 3. Validation Scenario 2: Workspace Onboarding & Hatchet Dispatch (SC-004, SC-006)

1. Apply valid workspace:
   ```bash
   cat <<EOF | kubectl apply -f -
   apiVersion: darkgravity.io/v1alpha1
   kind: AgentWorkspace
   metadata:
     name: workspace-lince-rs
     namespace: agents
   spec:
     repositoryUrl: "https://github.com/lgcorzo/lince-rs.git"
     targetBranch: "main"
     constitutionRef: ".specify/memory/constitution.md"
     missionRef: "mission-001"
     resourceLimits:
       maxRam: "30Mi"
       maxCpu: "250m"
   EOF
   ```
2. Start operator reconciler:
   ```bash
   RUST_LOG=info ./target/debug/factory-cli operator --leader-election
   ```
3. Verify status with `kubectl get agws`:
   ```bash
   kubectl get agws -n agents
   ```
   **Expected Output Columns**:
   ```text
   NAME                  PHASE     MISSION       COMMIT     ACCEPTED     SYNCED   RUN                AGE
   workspace-lince-rs    Running   mission-001   7a8f9c2    Dispatched   True     wf-run-1092834     15s
   ```

---

## 4. Validation Scenario 3: Re-execution Prevention & Deferral (SC-006)

1. Re-apply the same `missionRef` after completion:
   **Expected Outcome**: Condition `MissionAccepted` changes to `False` with reason `AlreadyExecuted`.
2. Change `missionRef` while mission is `Running`:
   **Expected Outcome**: Condition `MissionAccepted` set to `False` with reason `MissionActive`. Mission does not launch until the active run reaches a terminal state.

---

## 5. Validation Scenario 4: Finalizer Teardown (SC-002)

1. Delete workspace:
   ```bash
   kubectl delete agws workspace-lince-rs -n agents
   ```
2. Observe logs:
   - Cancel sent to Hatchet workflow `wf-run-1092834`.
   - Dynamic credentials revoked.
   - Sandbox pods in `development` deleted.
   - Finalizer `darkgravity.io/workspace-finalizer` removed.
   - Resource purged with 0 orphaned active jobs.

