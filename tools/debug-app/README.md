Debug App (Node.js + Fastify)

This is a tiny HTTP server meant to be deployed in-cluster and accessed via kubectl port-forward to help you reach the internal network.

What it does
- Starts a Fastify server and listens on 0.0.0.0:8080 by default.
- Endpoints:
  - GET /          -> returns "debug-app (Node.js + Fastify) is running"
  - GET /healthz   -> returns 200 OK

Local run
- Prerequisites: Node.js >= 20
- Commands:
  - cd debug-app
  - npm install
  - npm start
  - Visit http://127.0.0.1:8080

Docker build
- cd debug-app
- docker build -t debug-app:latest .

Kubernetes deploy (example)
- Edit the image in debug-app/k8s/deployment.yaml if you push to a registry.
- kubectl apply -f debug-app/k8s/deployment.yaml -n banana

Port-forward to access from your machine
- Forward to the Service (preferred):
  kubectl -n banana port-forward svc/debug-app 9999:80
  Then open: http://localhost:9999/

- Or forward directly to the Deployment/Pod:
  kubectl -n banana port-forward deploy/debug-app 9999:8080
  Then open: http://localhost:9999/

Configuration
- HOST and PORT env vars control the bind address (default HOST=0.0.0.0, PORT=8080).
