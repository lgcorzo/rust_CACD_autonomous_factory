#!/usr/bin/env bash
# Protected live gVisor evidence collector (DG-RVP-014 §4.2).
# Requires explicit operator authorization (EVIDENCE_COLLECT_LIVE=true) and kubeconfig
# for the target cluster. Never run in ordinary CI. Writes raw kubectl JSON only;
# it never writes expected values.
set -euo pipefail
: "${EVIDENCE_COLLECT_LIVE:?must be explicitly set to true by an operator}"
[ "${EVIDENCE_COLLECT_LIVE}" = "true" ] || { echo "not authorized"; exit 2; }
NS="${SANDBOX_NAMESPACE:-factory-sandboxes}"
OUT="evidence/security/live-runtime-observations.json"
TMP="$(mktemp)"
{
  echo '{'
  echo "\"collected_at\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
  echo "\"runtimeclass\": $(kubectl get runtimeclass gvisor -o json),"
  echo "\"namespace\": $(kubectl get ns "${NS}" -o json),"
  echo "\"networkpolicies\": $(kubectl get networkpolicy -n "${NS}" -o json),"
  echo "\"sandbox_pods\": $(kubectl get pods -n "${NS}" -l app.kubernetes.io/component=sandbox-workload -o json),"
  echo "\"nodes\": $(kubectl get nodes -o json | jq '{items: [.items[] | {name: .metadata.name, uid: .metadata.uid, containerRuntimeVersion: .status.nodeInfo.containerRuntimeVersion, kubeletVersion: .status.nodeInfo.kubeletVersion}]}')"
  echo '}'
} > "${TMP}"
jq . "${TMP}" > "${OUT}"
rm -f "${TMP}"
# Fail unless at least one sandbox pod is observed with runtimeClassName=gvisor and exit 0.
jq -e '[.sandbox_pods.items[] | select(.spec.runtimeClassName=="gvisor") | .status.containerStatuses[0].state.terminated.exitCode == 0] | any' "${OUT}" >/dev/null
echo "Live observations written to ${OUT}"

