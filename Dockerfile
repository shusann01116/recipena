FROM rust:1@sha256:02a86ffc3024f74ce38fe975b8d748c5c121a70f28d832ed04803bb76dc0e259 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:620d8b11ae800f0dbd7995f89ddc5344ad603269ea98770588b1b07a4a0a6872

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
