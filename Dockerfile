FROM rust:1@sha256:c92b3414e418f5b250f13c5bd393f90192f0b4bb4767e64f29c9e583197b27cb AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:329e54034ce498f9c6b345044e8f530c6691f99e94a92446f68c0adf9baa8464

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
