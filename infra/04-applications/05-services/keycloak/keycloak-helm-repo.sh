#!/bin/bash
set -e

helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update

helm install keycloak bitnami/keycloak -f keycloak-values.yaml --namespace banana
# helm uninstall -n banana keycloak
# helm upgrade keycloak bitnami/keycloak -n banana -f keycloak-values.yaml --atomic --wait --timeout 10m