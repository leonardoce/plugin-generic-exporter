FROM rust:1.76 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/myapp/target \
    cargo build --release && \
    cp /usr/src/myapp/target/release/plugin-generic-exporter /usr/local/bin

FROM debian:bookworm-slim
COPY --from=builder /usr/local/bin/plugin-generic-exporter /usr/local/bin/plugin-generic-exporter
CMD ["plugin-generic-exporter"]