FROM rust:latest

COPY . .

RUN cargo build --color=always --bin http-improve
WORKDIR ./target/debug/
CMD ./http-improve