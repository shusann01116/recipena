FROM rust:1@sha256:8611aebf88a78765560996643e8782827e2f4029204f680b8125e92ae7dc2fed AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:329e54034ce498f9c6b345044e8f530c6691f99e94a92446f68c0adf9baa8464

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
