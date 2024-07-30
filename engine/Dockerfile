FROM rust:1.80-slim
WORKDIR /engine
COPY . .
RUN apt-get update && apt-get install -y sqlite3 && rm -rf /var/lib/apt/lists/*
RUN cargo build -r
CMD cargo run -r
