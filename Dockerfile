FROM rust:1@sha256:6ff07edce8775d0f64be7aba9197229407301bddf2054d62c27b541a6238a181 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:329e54034ce498f9c6b345044e8f530c6691f99e94a92446f68c0adf9baa8464

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
