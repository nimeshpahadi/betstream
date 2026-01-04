# ---------- Build Stage ----------
FROM rustlang/rust:nightly-alpine3.19 AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig build-base tzdata

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to satisfy Cargo for fetch step
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Fetch dependencies (cached if Cargo.toml and Cargo.lock unchanged)
RUN cargo fetch

# Now copy the real source code
COPY src ./src

# Copy migrations directory (required by sqlx migrate macro)
COPY migrations ./migrations

# Build the actual release binary
RUN cargo build --release

# ---------- Runtime Stage ----------
FROM alpine:3.19

RUN apk add --no-cache bash curl tzdata

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/betting-api ./manualbettingserver

# Copy seed script
COPY ./data/seed.sh /data/seed.sh
RUN chmod +x /data/seed.sh

EXPOSE 3001

#CMD ["./manualbettingserver"]

# CMD: start backend in background, wait until ready, run seed, keep backend running
CMD bash -c '\
    ./manualbettingserver & \
    BACKEND_PID=$!; \
    echo "⏳ Waiting for backend to start..."; \
    until curl -s http://localhost:3001/api/v1/accounts >/dev/null 2>&1; do sleep 1; done; \
    echo "✅ Backend ready. Running seeder..."; \
    /data/seed.sh; \
    wait $BACKEND_PID \
'
