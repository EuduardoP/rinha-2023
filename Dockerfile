FROM rust:latest AS build

WORKDIR /app
COPY . .
RUN cargo build --release

# ---------- Runtime ----------
FROM debian:bookworm-slim

# Instalar libpq (necessário para diesel ou tokio-postgres com PQ)
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/rinha-2023 /app/rinha-2023

CMD ["/app/rinha-2023"]
