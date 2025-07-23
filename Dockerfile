FROM rust:1@sha256:d8fb475b1e114a71c7c841b9e5024de21af70c732acb019edff64207eec4a6a8 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:eccec5274132c1be0ce5d2c8e6fe41033e64af5e987ccee9007826e4c012069d

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
