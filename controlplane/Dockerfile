FROM rust:1.85 AS base

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

FROM base AS development
WORKDIR /app/controlplane
CMD ["cargo", "run", "--bin", "server"]

FROM base AS build
WORKDIR /app
RUN cargo build --release --manifest-path controlplane/Cargo.toml

FROM debian:bookworm-slim AS release
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/controlplane/target/release/server /usr/local/bin/server
EXPOSE 50052
CMD ["server"]
