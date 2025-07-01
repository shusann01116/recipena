FROM rust:1@sha256:749d5f12aa5f38ebf81012a0385b8e6adcb7b6e8f494961d559e8a7264803d4f AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:eccec5274132c1be0ce5d2c8e6fe41033e64af5e987ccee9007826e4c012069d

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
