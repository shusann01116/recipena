FROM rust:1@sha256:f58923369ba295ae1f60bc49d03f2c955a5c93a0b7d49acfb2b2a65bebaf350d AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:66d87e170bc2c5e2b8cf853501141c3c55b4e502b8677595c57534df54a68cc5

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
