FROM rust:1.74-alpine

# Install external dependencies on Alpine Linux.
RUN apk add musl-dev 

# Define address & port for rocket server.
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# Setup working directory & initialize binary Cargo crate.
WORKDIR /var/www/
RUN USER=root cargo new --bin rocket-server
WORKDIR /var/www/rocket-server

# Pre-compile Cargo dependencies.
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release;\
    rm ./src/*.rs;\
    rm ./target/release/deps/rocket_server*

# Compile Cargo crate.
COPY ./src ./src
COPY ./templates ./templates
RUN cargo build --release

# Start the server.
COPY ./entrypoint.sh .
ENTRYPOINT ./entrypoint.sh
