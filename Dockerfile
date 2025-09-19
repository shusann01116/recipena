FROM rust:1@sha256:eabb786e74b520e7ea45baca03ea20c3e8c6dc037c392d457badf05d8e5818b5 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:620d8b11ae800f0dbd7995f89ddc5344ad603269ea98770588b1b07a4a0a6872

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
