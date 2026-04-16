FROM rust:1.70 as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/secops /usr/local/bin/secops

ENTRYPOINT ["secops"]
