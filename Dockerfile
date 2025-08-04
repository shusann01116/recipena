FROM rust:1@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:bf0494952368db47e9e38eecc325c33f5ee299b1b1ccc5d9e30bdf1e5e4e3a58

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
