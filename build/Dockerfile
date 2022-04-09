FROM rust:1.60 as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.toml ./

# this build step will cache your dependencies
RUN cargo build --release & rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release
#
# our final base
FROM debian:buster-slim

WORKDIR /root/

# copy the build artifact from the build stage
COPY --from=build /app/target/release/rust-api-base .

# set the startup command to run your binary
ENTRYPOINT ["./rust-api-base"]