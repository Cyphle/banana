#!/bin/bash

kubectl apply -f https://raw.githubusercontent.com/keycloak/keycloak-k8s-resources/26.3.3/kubernetes/keycloaks.k8s.keycloak.org-v1.yml
kubectl apply -f https://raw.githubusercontent.com/keycloak/keycloak-k8s-resources/26.3.3/kubernetes/keycloakrealmimports.k8s.keycloak.org-v1.yml

kubectl apply -n banana -f https://raw.githubusercontent.com/keycloak/keycloak-k8s-resources/26.3.3/kubernetes/kubernetes.yml
