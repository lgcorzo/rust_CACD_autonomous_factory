#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "    Dark Gravity CA/CD Autonomous Factory Evidence Pipeline"
echo "=========================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

EVIDENCE_DIR="${REPO_ROOT}/evidence"
mkdir -p "${EVIDENCE_DIR}/security"

COMMIT_SHA=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
BRANCH_NAME=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
RUSTC_VER=$(rustc --version 2>/dev/null || echo "unknown")
CARGO_VER=$(cargo --version 2>/dev/null || echo "unknown")
OS_INFO=$(uname -srmo 2>/dev/null || echo "Linux")

echo "Commit:    ${COMMIT_SHA}"
echo "Branch:    ${BRANCH_NAME}"
echo "Timestamp: ${TIMESTAMP}"
echo "Toolchain: ${RUSTC_VER}"
echo "OS:        ${OS_INFO}"
echo "----------------------------------------------------------"

# 1. Format check
echo "[1/4] Checking code formatting..."
cargo fmt --check
FMT_STATUS="PASSED"

# 2. Clippy verification
echo "[2/4] Running strict clippy linting..."
cargo clippy --workspace --all-targets -- -D warnings
CLIPPY_STATUS="PASSED"

# 3. Adversarial corpus evaluation
echo "[3/4] Running adversarial diff corpus and prompt-injection suite..."
cargo test -p factory-application --test adversarial_suite -- --nocapture
CORPUS_STATUS="PASSED"

# 4. Workspace test suite
echo "[4/4] Executing all unit and integration tests across workspace..."
cargo test --workspace
TESTS_STATUS="PASSED"

# Build summary status JSON
STATUS_FILE="${EVIDENCE_DIR}/verified_status.json"

cat <<EOF > "${STATUS_FILE}"
{
  "timestamp": "${TIMESTAMP}",
  "commit_sha": "${COMMIT_SHA}",
  "branch": "${BRANCH_NAME}",
  "environment": {
    "rustc": "${RUSTC_VER}",
    "cargo": "${CARGO_VER}",
    "os": "${OS_INFO}"
  },
  "gates": {
    "fmt": "${FMT_STATUS}",
    "clippy": "${CLIPPY_STATUS}",
    "adversarial_corpus": "${CORPUS_STATUS}",
    "workspace_tests": "${TESTS_STATUS}"
  },
  "verified": true
}
EOF

echo "=========================================================="
echo " Evidence collection successful! Output: ${STATUS_FILE}"
if [ -f "${EVIDENCE_DIR}/security/corpus_results.json" ]; then
    echo " Adversarial Corpus Report: ${EVIDENCE_DIR}/security/corpus_results.json"
fi
echo "=========================================================="
