FROM rust:1@sha256:5474dce321a62be62d86355e776a035587345eb385e049760cfb6b04b6fa9025 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:66d87e170bc2c5e2b8cf853501141c3c55b4e502b8677595c57534df54a68cc5

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
