#!/bin/bash
set -e

helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update

helm install keycloak bitnami/keycloak -f keycloak-values.yaml --namespace banana