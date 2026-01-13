FROM rust:1@sha256:1417b7faaabd8547ecd3c43f98fc5bd7f06295a40935bdbb81240210f7127f76 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:66d87e170bc2c5e2b8cf853501141c3c55b4e502b8677595c57534df54a68cc5

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
