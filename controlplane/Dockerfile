FROM rust:1.87 AS base

RUN apt-get update && apt-get install -y protobuf-compiler curl && rm -rf /var/lib/apt/lists/*
RUN rustup component add clippy llvm-tools-preview
RUN cargo install cargo-llvm-cov
RUN curl -sSL https://github.com/bufbuild/buf/releases/latest/download/buf-Linux-$(uname -m) -o /usr/local/bin/buf && chmod +x /usr/local/bin/buf

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
