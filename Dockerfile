FROM rust:1@sha256:6e6d04bd50cd4c433a805c58c13f186a508c5b5417b9b61cae40ec28e0593c51 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:d4e8c4c18626ce7c09104d3b39d9e5541ced61de7fb398a455cb09ae5a7b3598

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
