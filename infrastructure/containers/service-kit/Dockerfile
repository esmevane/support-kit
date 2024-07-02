FROM rust:1.78.0-slim AS builder
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build -p service-kit --release --bin service-kit

# FROM arm64v8/rust:1.71.1-slim AS chef
# RUN cargo install cargo-chef

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json

FROM gcr.io/distroless/cc:latest AS runtime
COPY --from=builder /target/release/service-kit service-kit

ENTRYPOINT ["./service-kit"]

# FROM rust:1 AS chef 
# # We only pay the installation cost once, 
# # it will be cached from the second build onwards
# RUN cargo install cargo-chef 
# WORKDIR app

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare  --recipe-path recipe.json

# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# # Build application
# COPY . .
# RUN cargo build --release --bin app

# # We do not need the Rust toolchain to run the binary!
# FROM debian:bookworm-slim AS runtime
# WORKDIR app
# COPY --from=builder /app/target/release/app /usr/local/bin
# ENTRYPOINT ["/usr/local/bin/app"]

# FROM --platform=linux/arm64 lukemathwalker/cargo-chef:latest-rust-1.71.1 as chef
# WORKDIR /app
# RUN apt update && apt install lld clang -y

# FROM --platform=linux/arm64 chef as planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM --platform=linux/arm64 chef as builder
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json
# COPY . .
# ENV SQLX_OFFLINE true
# RUN cargo build -p emblem-web --release --bin emblem-web

# FROM --platform=linux/arm64 gcr.io/distroless/cc:latest
# COPY --from=builder /app/target/release/emblem-web emblem-web

# ENTRYPOINT ["./emblem-web"]

