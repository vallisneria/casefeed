FROM rust:1.83
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/court-crawler"]
