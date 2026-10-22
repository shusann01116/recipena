FROM rust:1@sha256:931c2c0531a8b143e044f0f3914dfe802f15bb3d1955f0b657e9359327ac2ffc AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:329e54034ce498f9c6b345044e8f530c6691f99e94a92446f68c0adf9baa8464

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
