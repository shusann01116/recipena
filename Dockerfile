FROM rust:1@sha256:65734d21f103d104fe0d9e508a424f7f60abd10e489d36de8bd36ae6c80e746d AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0c8eac8ea42a167255d03c3ba6dfad2989c15427ed93d16c53ef9706ea4691df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
