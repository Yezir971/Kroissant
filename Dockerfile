# syntax=docker/dockerfile:1.7

FROM rust:1.92-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system app \
    && useradd --system --gid app --home /app --shell /usr/sbin/nologin app

WORKDIR /app

COPY --from=builder /app/target/release/kroissant /usr/local/bin/kroissant
COPY static ./static
COPY data ./data

RUN mkdir -p data \
    && chown -R app:app /app

USER app

ENV HOST=0.0.0.0
ENV PORT=3000
ENV DATABASE_URL=sqlite://data/kroissant.sqlite
ENV JWT_SECRET=dev-secret-change-me-kroissant
ENV RUST_LOG=info

EXPOSE 3000

CMD ["kroissant"]
