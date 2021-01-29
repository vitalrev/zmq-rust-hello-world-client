# 1: Build the exe
FROM rust as builder
WORKDIR app

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get install -y libtool libzmq3-dev

COPY . .
# This works with the dummy project generated by `cargo new app --bin`
RUN cargo build --release --bin app

RUN ls -la /app/target/release

FROM rust as runtime

RUN apt-get update && \
    apt-get install -y libzmq3-dev

WORKDIR app
COPY --from=builder /app/target/release/app /usr/local/bin

RUN ls -la /usr/local/bin

WORKDIR /usr/local/bin

ENTRYPOINT ["./app"]
