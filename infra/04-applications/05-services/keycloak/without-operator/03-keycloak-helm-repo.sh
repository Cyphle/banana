#!/bin/bash
set -e

helm install keycloak bitnami/keycloak -f keycloak-values.yaml --namespace banana
# helm uninstall -n banana keycloak
# helm upgrade keycloak bitnami/keycloak -n banana -f 03-keycloak-values.yaml --atomic --wait --timeout 10m