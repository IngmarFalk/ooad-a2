FROM rust:latest

WORKDIR usr/src/a2

COPY . .

# Building the project
RUN cargo build --release

# Running the app
CMD cargo run
