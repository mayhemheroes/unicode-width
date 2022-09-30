FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /unicode-width
WORKDIR /unicode-width/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /unicode-width/fuzz/target/x86_64-unknown-linux-gnu/release/unicodewidth-fuzz /