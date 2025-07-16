FROM node:20-alpine AS builder

WORKDIR /app

COPY betting-frontend/package*.json ./
RUN npm ci

COPY betting-frontend/ ./
RUN npm run build

FROM caddy:2.8-alpine

COPY --from=builder /app/build /usr/share/caddy
COPY betting-frontend/Caddyfile /etc/caddy/Caddyfile

EXPOSE 80
CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile"]
