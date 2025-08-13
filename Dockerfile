FROM rust:1@sha256:5fa1490d5cd16725196511190baad604ddebedcd6e52f1036de46a1a75c85bce AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:00cc20b928afcc8296b72525fa68f39ab332f758c4f2a9e8d90845d3e06f1dc4

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
