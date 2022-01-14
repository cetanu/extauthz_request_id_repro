FROM rust:1.54
RUN rustup component add rustfmt

WORKDIR /project

# Build dependencies
RUN mkdir src
RUN echo "fn main() {}" >> src/main.rs
COPY Cargo.toml Cargo.toml
RUN cargo build
RUN rm -f src/main.rs

# Build protobufs + binary
COPY build.rs build.rs
COPY proto proto
COPY src src
RUN cargo build

# Extract binary
RUN cp target/debug/server /project/server

CMD ["/project/server", "--statsd_address", "statsd:8125"]
