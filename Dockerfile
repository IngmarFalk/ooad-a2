FROM rust:latest

WORKDIR /usr/src/a2

COPY . .

# Building the project
RUN cargo build --release
RUN rustup component add clippy

# Running the app
CMD cargo run
