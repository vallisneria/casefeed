FROM rust:1.85-alpine AS builder
WORKDIR /app
COPY . .
RUN apk add --no-cache clang lld musl-dev git perl openssl-libs-static pkgconf openssl-dev make
RUN cd court-crawler && cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/court-crawler .
CMD ["./court-crawler"]
