# Node.js runtime with basic debugging tools
FROM node:22-bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive

# Install extra tools: curl and ping
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates \
       curl \
       iputils-ping \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -g 10001 appgroup \
    && useradd -m -u 10001 -g appgroup appuser

WORKDIR /app

# Install dependencies (leverage Docker layer caching)
COPY package.json ./
RUN npm ci --omit=dev || npm install --omit=dev

# Copy application source
COPY server.js ./

ENV NODE_ENV=production \
    HOST=0.0.0.0 \
    PORT=8080

USER appuser
EXPOSE 8080

CMD ["node", "server.js"]
