FROM debian:bullseye as final

WORKDIR /opt/app
COPY ./target/release/web_service . 

# CMD ["./web_service"]
