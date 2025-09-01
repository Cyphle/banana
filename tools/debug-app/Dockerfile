# Ubuntu-based runtime with Node.js
FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# Install Node.js 22.x from NodeSource
RUN apt-get update \
    && apt-get install -y ca-certificates curl gnupg \
    && mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | tee /etc/apt/keyrings/nodesource.gpg >/dev/null \
    && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_22.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list \
    && apt-get update \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -g 10001 appgroup \
    && useradd -m -u 10001 -g appgroup appuser

WORKDIR /app

# Install dependencies first (better layer caching)
COPY package.json .
RUN npm ci --omit=dev || npm install --omit=dev

# Copy app source
COPY server.js ./

ENV NODE_ENV=production \
    HOST=0.0.0.0 \
    PORT=8080

USER appuser
EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -sf http://127.0.0.1:8080/healthz || exit 1

CMD ["node", "server.js"]
