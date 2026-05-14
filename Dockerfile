FROM rust:1-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations
COPY config ./config
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 appuser
WORKDIR /app
COPY --from=builder /app/target/release/task-auth-api /app/task-auth-api
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/config /app/config
ENV RUST_LOG=info
EXPOSE 3000
HEALTHCHECK CMD curl --fail http://127.0.0.1:3000/health/ready || exit 1
USER appuser
ENTRYPOINT ["/app/task-auth-api"]
