# Build Stage
FROM rust:1.75-bullseye as builder

WORKDIR /usr/src/events-finder

# Build dependencies separately to improve docker layer caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/*.rs

COPY . .
RUN cargo install --path .

# Runtime Stage
FROM debian:bullseye

WORKDIR /usr/src/events-finder

COPY --from=builder /usr/local/cargo/bin/events-finder /usr/local/bin/events-finder
COPY ./config.yml /usr/src/events-finder/config.yml

CMD ["events-finder"]

