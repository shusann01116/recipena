FROM rust:1@sha256:0e18a916cc41ed3a99356ded9b3c8b0f91e15249d4507a18ec9af4edb7a631e5 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0000f9dc0290f8eaf0ecceafbc35e803649087ea7879570fbc78372df7ac649b

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
