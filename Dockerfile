FROM rust:1@sha256:20d4b66c77751ca03a3311139a8d7928001b4d5dd9530a95b069c603f20587b1 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:72344f7f909a8bf003c67f55687e6d51a441b49661af8f660aa7b285f00e57df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
