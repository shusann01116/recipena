FROM rust:1@sha256:c1d78d1def6bb485954ef945970ef4d713cd61ed0e9f50b2219b8248d397042d AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:72344f7f909a8bf003c67f55687e6d51a441b49661af8f660aa7b285f00e57df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
