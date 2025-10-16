FROM rust:1.90-slim AS base

FROM base AS dev

WORKDIR /app

RUN useradd -m appuser
USER appuser

COPY apps/lethephobia_server ./

RUN cargo install cargo-watch

EXPOSE 8080

CMD ["cargo", "watch", "-w", "./crates", "-w", "./servers/internal_api", "-x", "run -p internal_api"]

FROM base AS builder

WORKDIR /app

COPY apps/lethephobia_server ./

RUN cargo build --release -p internal_api

FROM base AS prod

WORKDIR /app

RUN useradd -m appuser
USER appuser

COPY --from=builder /app/target/release/internal_api ./

EXPOSE 8080

CMD ["./internal_api"]
