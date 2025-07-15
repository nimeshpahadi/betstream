FROM node:20-alpine AS builder

WORKDIR /app

# Install only declared dependencies
COPY betting-frontend/package*.json ./
RUN npm ci

# Copy the React source
COPY betting-frontend/ ./

# Build React app (creates /app/build)
RUN npm run build

# ---------- Runtime Stage ----------
FROM nginx:alpine

# Copy compiled React files from /app/build → Nginx public folder
COPY --from=builder /app/build /usr/share/nginx/html

# Custom Nginx config (optional, if it exists)
COPY betting-frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
