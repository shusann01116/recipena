FROM rust:1@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:aa435f48941dbbd18b4a1f3f71992a3afddc6fb913beb411cd4c0fb174e0bfb8

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
