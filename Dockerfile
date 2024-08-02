FROM rust:latest

WORKDIR /app

COPY . .

ENTRYPOINT ["cargo", "run"]