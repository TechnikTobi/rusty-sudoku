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