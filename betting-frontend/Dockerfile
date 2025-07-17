FROM node:20-alpine AS builder

WORKDIR /app

COPY betting-frontend/package*.json ./
RUN npm ci

COPY betting-frontend/ ./
RUN npm run build

# FROM caddy:2.8-alpine
#
# COPY --from=builder /app/build /usr/share/caddy
# COPY caddy/Caddyfile /etc/caddy/Caddyfile
#
# EXPOSE 80
# CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile"]

FROM nginx:alpine

COPY --from=builder /app/build /usr/share/nginx/html
COPY betting-frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
