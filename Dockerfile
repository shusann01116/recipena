FROM rust:1@sha256:ded05442c17728d9fb8ed5ecb0779617e197119aa9b7893e70ad77c4edc070c2 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:00cc20b928afcc8296b72525fa68f39ab332f758c4f2a9e8d90845d3e06f1dc4

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
