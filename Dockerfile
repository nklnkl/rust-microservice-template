FROM rust:slim-buster

# Install libpq-dev for PostgreSQL support
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Move the binary to /usr/local/bin
RUN cp target/release/your_project_name /usr/local/bin/your_project_name

# Run the binary directly
CMD ["/usr/local/bin/your_project_name"]
