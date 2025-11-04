FROM rust:1@sha256:1d9f2983afa26e4df0eccdf232d0d89b5029d1345f3af613a2019cc8bd8a672c AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0000f9dc0290f8eaf0ecceafbc35e803649087ea7879570fbc78372df7ac649b

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
