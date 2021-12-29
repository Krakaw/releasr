FROM rust:1.57 as builder

RUN USER=root cargo new --bin releasr
WORKDIR ./releasr
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
ADD src ./src

RUN rm ./target/release/deps/releasr*
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /releasr/target/release/releasr /usr/local/bin/releasr
EXPOSE 8080
CMD ["releasr"]
