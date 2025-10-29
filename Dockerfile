FROM rust:1@sha256:e227f20ec42af3ea9a3c9c1dd1b2012aa15f12279b5e9d5fb890ca1c2bb5726c AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0000f9dc0290f8eaf0ecceafbc35e803649087ea7879570fbc78372df7ac649b

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
