# Build stage
FROM rust:slim-buster as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-watch
RUN cargo build --release

# Production stage
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/your_project_name /usr/local/bin/your_project_name

# Create a non-root user
RUN useradd -m myuser
USER myuser

CMD ["/usr/local/bin/your_project_name"]
