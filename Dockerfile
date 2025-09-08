FROM rust:1@sha256:3329e2de3e9ff2d58da56e95ef99a3180a4e76336a676f3fe2b88f0b0d6bcfbf AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:620d8b11ae800f0dbd7995f89ddc5344ad603269ea98770588b1b07a4a0a6872

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
