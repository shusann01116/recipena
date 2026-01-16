FROM rust:1@sha256:bed2d7f8140d73c26f16c298c91ae8487a09f40d3840c0d8d139537e1b51e148 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:66d87e170bc2c5e2b8cf853501141c3c55b4e502b8677595c57534df54a68cc5

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
