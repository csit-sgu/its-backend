FROM rust:buster

ENV CARGO_HOME=/rust
WORKDIR /app
COPY . .
RUN apt-get install libssl-dev
RUN --mount=type=cache,target=/app/target \
    cargo build --release && cp ./target/release/web_service /bin
ENTRYPOINT [ "/bin/web_service" ]

