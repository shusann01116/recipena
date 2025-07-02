FROM rust:1@sha256:526343cb056f31e3eac263be19b5095eba0abbcdcea00352cd8756ccd7ba2375 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:eccec5274132c1be0ce5d2c8e6fe41033e64af5e987ccee9007826e4c012069d

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
