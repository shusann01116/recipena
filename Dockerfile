FROM rust:1@sha256:3329e2de3e9ff2d58da56e95ef99a3180a4e76336a676f3fe2b88f0b0d6bcfbf AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:d4e8c4c18626ce7c09104d3b39d9e5541ced61de7fb398a455cb09ae5a7b3598

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
