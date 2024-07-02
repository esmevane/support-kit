FROM rust:1.78.0-slim AS builder
COPY . .
ENV SQLX_OFFLINE true
RUN apt-get update && apt-get install -y libssl-dev pkg-config protobuf-compiler
RUN cargo build -p service-kit --release --bin service-kit

FROM gcr.io/distroless/cc:latest AS runtime
COPY --from=builder /target/release/service-kit service-kit

ENTRYPOINT ["./service-kit", "--app-name", "service-kit", "server", "full"]
