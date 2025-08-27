FROM rust:1@sha256:26318aeddc7e7335b55ab32f943ec2d400bcc024649f8dbdee569bfa85f0c11d AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:d4e8c4c18626ce7c09104d3b39d9e5541ced61de7fb398a455cb09ae5a7b3598

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
