#!/usr/bin/env bash
# Records raw cluster state with timestamps and a SHA-256 manifest.
# Read-only. Run by a human against the intended cluster; never from CI.
set -euo pipefail

if [[ "${CONFIRM_CAPTURE:-}" != "yes" ]]; then
  echo "Refusing to run: set CONFIRM_CAPTURE=yes after checking 'kubectl config current-context'." >&2
  exit 2
fi

OUT="${1:-evidence/runs/baseline-$(date -u +%Y%m%dT%H%M%SZ)}"
mkdir -p "$OUT"
CTX="$(kubectl config current-context)"
{
  echo "context: $CTX"
  echo "captured_at_utc: $(date -u +%FT%TZ)"
  echo "kubectl: $(kubectl version --client -o json | tr -d '\n')"
  echo "git_commit: $(git rev-parse HEAD)"
} > "$OUT/capture-meta.txt"

RESOURCES=(
  "pods -A" "deployments -A" "statefulsets -A" "services -A" "ingress -A"
  "networkpolicies -A" "runtimeclasses" "crds"
  "validatingadmissionpolicies" "validatingadmissionpolicybindings"
  "nodes" "namespaces"
)
for r in "${RESOURCES[@]}"; do
  name="$(echo "$r" | tr ' ' '_' | tr -d '-')"
  # shellcheck disable=SC2086
  kubectl get $r -o json > "$OUT/${name}.json"
  date -u +%FT%TZ > "$OUT/${name}.captured_at"
done

( cd "$OUT" && sha256sum ./*.json ./*.txt ./*.captured_at > MANIFEST.sha256 )
echo "Wrote $OUT (verify with: cd $OUT && sha256sum -c MANIFEST.sha256)"
