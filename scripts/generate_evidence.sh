#!/usr/bin/env bash
# Dark Gravity Evidence Pipeline (DG-RVP-014).
#
# Executes each requirement's verification command for real, captures stdout/stderr,
# exit code and timestamps into evidence/runs/<REQ>.log, hashes the log, and writes
# evidence records atomically. It NEVER synthesizes success and NEVER auto-promotes
# requirements to VERIFIED:
#   * exit 0      -> outcome=passed,  state=TESTED
#   * exit != 0   -> outcome=failed,  state=IMPLEMENTED
#   * not runnable in this environment (e.g. live cluster requirements in CI)
#                 -> previously collected live evidence is preserved untouched if present,
#                    otherwise outcome=not_executed.
# VERIFIED is only granted by an independent verifier via a separate, explicit step
# (`scripts/verify_evidence.sh`), never by this generator.
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

GENERATOR_VERSION="dg-evidence-generator/2.0.0"
EVIDENCE_DIR="${REPO_ROOT}/evidence"
RUNS_DIR="${EVIDENCE_DIR}/runs"
mkdir -p "${EVIDENCE_DIR}/records" "${EVIDENCE_DIR}/states" "${RUNS_DIR}"

COMMIT_SHA=$(git rev-parse HEAD)
BRANCH_NAME=$(git rev-parse --abbrev-ref HEAD)
ENV_KIND="${EVIDENCE_ENV_KIND:-$([ -n "${GITHUB_ACTIONS:-}" ] && echo ci || echo local)}"
ENV_ID="${EVIDENCE_ENV_ID:-$([ -n "${GITHUB_ACTIONS:-}" ] && echo "github-actions-${RUNNER_OS:-linux}" || hostname)}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"

echo "Commit: ${COMMIT_SHA}  Env: ${ENV_KIND}/${ENV_ID}  Generator: ${GENERATOR_VERSION}"

# REQ | class | command
SPEC_ITEMS=(
  "FR-003|integration|cargo test -j 1 -p factory-mcp-server --test run_tests_security_tests"
  "FR-004|integration|cargo test -j 1 -p factory-mcp-server --test sandbox_driver_tests"
  "FR-005|unit|cargo test -j 1 -p factory-mcp-server --test run_tests_security_tests"
  "FR-006|unit|cargo test -j 1 -p factory-cli --test evidence_contract_tests"
  "FR-007|unit|cargo test -j 1 -p factory-mcp-server --test launch_sandbox_pod_tests"
  "FR-009|integration|cargo test -j 1 -p factory-infrastructure --test finops_concurrency_test"
  "FR-010|integration|cargo test -j 1 -p factory-application --test workflow_recovery_tests"
  "FR-011|integration|cargo test -j 1 -p factory-mcp-server --test route_boundary_tests"
  "FR-012|integration|cargo test -j 1 -p factory-mcp-server --test webhook_security_tests"
  "FR-018|unit|cargo test -j 1 -p factory-cli --test evidence_contract_tests"
)
# Live requirements: only collected when explicitly authorized against a real cluster.
LIVE_ITEMS=(
  "FR-008|live_runtime|bash scripts/collect_live_gvisor_evidence.sh"
)

FAILED=0
run_item() {
  local req="$1" vclass="$2" cmd="$3"
  local log="${RUNS_DIR}/${req}.log"
  local started finished rc
  started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  { echo "# command: ${cmd}"; echo "# started_at: ${started}"; } > "${log}.tmp"
  bash -c "${cmd}" >> "${log}.tmp" 2>&1
  rc=$?
  finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  { echo "# finished_at: ${finished}"; echo "# exit_code: ${rc}"; } >> "${log}.tmp"
  mv -f "${log}.tmp" "${log}"
  [ "${rc}" -ne 0 ] && FAILED=1
  python3 scripts/write_evidence_record.py \
    --req "${req}" --class "${vclass}" --command "${cmd}" --exit-code "${rc}" \
    --started "${started}" --finished "${finished}" --log "evidence/runs/${req}.log" \
    --commit "${COMMIT_SHA}" --branch "${BRANCH_NAME}" \
    --env-kind "${ENV_KIND}" --env-id "${ENV_ID}" --generator "${GENERATOR_VERSION}"
}

for item in "${SPEC_ITEMS[@]}"; do
  IFS='|' read -r req vclass cmd <<< "${item}"
  echo "[${req}] ${cmd}"
  run_item "${req}" "${vclass}" "${cmd}"
done

if [ "${EVIDENCE_COLLECT_LIVE:-false}" = "true" ]; then
  for item in "${LIVE_ITEMS[@]}"; do
    IFS='|' read -r req vclass cmd <<< "${item}"
    ENV_KIND=cluster ENV_ID="${CLUSTER_ID:?CLUSTER_ID required for live collection}" run_item "${req}" "${vclass}" "${cmd}"
  done
else
  echo "Live runtime collection not authorized (EVIDENCE_COLLECT_LIVE!=true); preserving existing live records unchanged."
fi

echo "Validating evidence records (structure, hashes, outcomes) ..."
cargo run -j 1 -q -p factory-cli -- validate-evidence --evidence-dir "${EVIDENCE_DIR}" || FAILED=1

if [ "${FAILED}" -ne 0 ]; then
  echo "Evidence pipeline FAILED: at least one command failed or validation rejected evidence." >&2
  exit 1
fi
echo "Evidence collected. Requirements are TESTED at most; VERIFIED requires independent verification."
