FROM rust:1-bookworm AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/labelize /usr/local/bin/labelize
EXPOSE 8080
ENTRYPOINT ["labelize"]
CMD ["serve", "--host", "0.0.0.0", "--port", "8080"]
