#!/bin/bash
set -e

echo "Generating Secret templates for agents namespace..."

# Ensure we are in the correct directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR/../k8s/base"

# Kafka credentials pattern (dummy for now if not using SASL, or replace with real)
kubectl create secret generic kafka-credentials \
  --from-literal=username="agent-user" \
  --from-literal=password="supersecret" \
  -n agents \
  --dry-run=client -o yaml > kafka-credentials-secret.yaml

# Hatchet admin token from .env or override
HATCHET_TOKEN="${HATCHET_CLIENT_TOKEN:-dummy-token}"
kubectl create secret generic hatchet-admin-token \
  --from-literal=token="$HATCHET_TOKEN" \
  -n agents \
  --dry-run=client -o yaml > hatchet-admin-token-secret.yaml

echo "Secret templates generated."
echo "If using SealedSecrets, please run kubeseal on these files:"
echo "kubeseal --format=yaml < kafka-credentials-secret.yaml > kafka-credentials-sealed-secret.yaml"
echo "kubeseal --format=yaml < hatchet-admin-token-secret.yaml > hatchet-admin-token-sealed-secret.yaml"
