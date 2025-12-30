FROM rust:1@sha256:c57bc0d341a0915841d9f9a420af28d027996d8705af5c591dbd069dba2d10b4 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0c8eac8ea42a167255d03c3ba6dfad2989c15427ed93d16c53ef9706ea4691df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
