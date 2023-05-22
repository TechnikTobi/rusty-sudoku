# ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# # Our first FROM statement declares the build environment.
# FROM ${BASE_IMAGE} AS builder

# # Add our source code.
# ADD . ./

# # Fix permissions on source code.
# RUN sudo chown -R rust:rust /home/rust

# # Build our application.
# RUN cargo build --release

# # Now, we need to build our _real_ Docker container, copying in `rust-actix-example`.
# FROM alpine:latest
# RUN apk --no-cache add ca-certificates
# COPY --from=builder \
#     /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-actix-example \
#     /usr/local/bin/
# CMD /usr/local/bin/rust-actix-example




FROM rust:1.68.0

WORKDIR /usr/src/rusty-sudoku
COPY . .

EXPOSE 8080

RUN cargo install --path .

CMD ["rusty-sudoku"]



# FROM rust:1.68.0 as build
# ENV PKG_CONFIG_ALLOW_CROSS=1

# WORKDIR /usr/src/rusty-sudoku
# COPY . .

# RUN cargo install --path .

# FROM gcr.io/distroless/cc-debian10

# COPY --from=build /usr/local/cargo/bin/rusty-sudoku /usr/local/bin/rusty-sudoku

# CMD ["rusty-sudoku"]