FROM rust:1@sha256:c50cd6e20c46b0b36730b5eb27289744e4bb8f32abc90d8c64ca09decf4f55ba AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:bf0494952368db47e9e38eecc325c33f5ee299b1b1ccc5d9e30bdf1e5e4e3a58

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
