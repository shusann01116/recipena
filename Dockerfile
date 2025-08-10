FROM rust:1@sha256:c50cd6e20c46b0b36730b5eb27289744e4bb8f32abc90d8c64ca09decf4f55ba AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:00cc20b928afcc8296b72525fa68f39ab332f758c4f2a9e8d90845d3e06f1dc4

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
