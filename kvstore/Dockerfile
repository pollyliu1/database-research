# official Rust image as the base
FROM rust:latest

# install RockDB dependencies
RUN apt-get update && \
    apt-get install -y librocksdb-dev && \
    apt-get clean

# current working directory
WORKDIR /app

# copy cargo files first to leverage caching
COPY Cargo.toml Cargo.lock ./

# pre-fetch dependencies
RUN cargo build --release --locked

# copy entire project source code
COPY . .

# build project
RUN cargo build --release

# command to run the application
CMD ["./target/release/kvstore_project"]
