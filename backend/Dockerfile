FROM rust:1.81.0-slim-bullseye AS base
WORKDIR /build
RUN apt-get update && apt-get -y install pkg-config libssl-dev
COPY Cargo.toml Cargo.lock ./

FROM base AS backend-builder
RUN --mount=type=bind,source=src,target=src \
    --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --locked --release && \
    cp -r ./target/release/backend /build/backend

FROM debian:bullseye-slim AS backend-runtime
RUN apt-get update && apt-get install -y ca-certificates
WORKDIR /app
COPY --from=backend-builder /build/backend app
ENTRYPOINT "./app"
