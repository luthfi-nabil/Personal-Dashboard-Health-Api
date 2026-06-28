# ---------- Build stage ----------
FROM rust:1.92.0-trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/health-api /app/health-api

CMD ["/app/health-api"]
