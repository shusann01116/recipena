FROM rust:1@sha256:48851a839d6a67370c9dbe0e709bedc138e3e404b161c5233aedcf2b717366e4 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0c8eac8ea42a167255d03c3ba6dfad2989c15427ed93d16c53ef9706ea4691df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
