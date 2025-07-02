FROM rust:1@sha256:5771a3cc2081935c59ac52b92d49c9e164d4fed92c9f6420aa8cc50364aead6e AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:eccec5274132c1be0ce5d2c8e6fe41033e64af5e987ccee9007826e4c012069d

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
