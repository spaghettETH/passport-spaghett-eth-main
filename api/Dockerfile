FROM rust:latest

# Setting up work directory
WORKDIR /app

# Copying the source code
COPY ./api/src /app/src
COPY ./api/Cargo.toml /app/Cargo.toml
COPY ./api/Rocket.toml /app/Rocket.toml

# Building the source code
RUN cargo build --release

# Exposing the binary
CMD ["/app/target/release/passport-api-rs"]