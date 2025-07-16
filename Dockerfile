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

RUN apk add --no-cache tzdata

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/betting-api ./manualbettingserver

EXPOSE 3001

CMD ["./manualbettingserver"]
