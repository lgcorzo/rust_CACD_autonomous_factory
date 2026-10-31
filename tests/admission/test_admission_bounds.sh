#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "Running Admission Boundary Hard Resource Ceiling Test Suite"
echo "Target Namespace: development"
echo "Ceilings: RAM <= 30Mi, CPU <= 250m"
echo "=========================================================="

# Ensure development namespace exists
kubectl create namespace development --dry-run=client -o yaml | kubectl apply -f -

# Test 1: Valid container within limits (Should PASS)
echo -n "Test 1: Compliant pod within limits (20Mi, 100m)... "
cat <<EOF | kubectl apply --dry-run=server -f - >/dev/null 2>&1
apiVersion: v1
kind: Pod
metadata:
  name: test-compliant-pod
  namespace: development
spec:
  restartPolicy: Never
  containers:
    - name: worker
      image: busybox
      command: ["sh", "-c", "echo ok"]
      resources:
        requests:
          memory: "10Mi"
          cpu: "50m"
        limits:
          memory: "20Mi"
          cpu: "100m"
EOF
echo "PASSED (Admitted)"

# Test 2: Reject Pod Exceeding Memory Limit (>30Mi) (Should FAIL)
echo -n "Test 2: Pod exceeding memory ceiling (40Mi)... "
if cat <<EOF | kubectl apply --dry-run=server -f - >/dev/null 2>&1
apiVersion: v1
kind: Pod
metadata:
  name: test-over-mem-pod
  namespace: development
spec:
  restartPolicy: Never
  containers:
    - name: worker
      image: busybox
      resources:
        requests:
          memory: "20Mi"
          cpu: "50m"
        limits:
          memory: "40Mi"
          cpu: "100m"
EOF
then
    echo "FAILED (Unexpected admission)"
    exit 1
else
    echo "PASSED (Rejected by admission)"
fi

# Test 3: Reject Pod Exceeding CPU Limit (>250m) (Should FAIL)
echo -n "Test 3: Pod exceeding CPU ceiling (500m)... "
if cat <<EOF | kubectl apply --dry-run=server -f - >/dev/null 2>&1
apiVersion: v1
kind: Pod
metadata:
  name: test-over-cpu-pod
  namespace: development
spec:
  restartPolicy: Never
  containers:
    - name: worker
      image: busybox
      resources:
        requests:
          memory: "20Mi"
          cpu: "100m"
        limits:
          memory: "30Mi"
          cpu: "500m"
EOF
then
    echo "FAILED (Unexpected admission)"
    exit 1
else
    echo "PASSED (Rejected by admission)"
fi

# Test 4: Reject Pod with Missing Limits (Should FAIL)
echo -n "Test 4: Pod with missing resource limits... "
if cat <<EOF | kubectl apply --dry-run=server -f - >/dev/null 2>&1
apiVersion: v1
kind: Pod
metadata:
  name: test-missing-limits-pod
  namespace: development
spec:
  restartPolicy: Never
  containers:
    - name: worker
      image: busybox
      resources:
        requests:
          memory: "20Mi"
          cpu: "100m"
EOF
then
    echo "FAILED (Unexpected admission)"
    exit 1
else
    echo "PASSED (Rejected by admission)"
fi

echo "=========================================================="
echo "All admission ceiling tests passed successfully! ✅"
echo "=========================================================="
