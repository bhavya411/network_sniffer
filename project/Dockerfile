# Builder Stage
FROM ubuntu:latest as Builder
WORKDIR /app
RUN apt update && \
    apt install -y git
RUN git clone https://github.com/tarunrawatknoldus/network_sniffer.git

# PostgreSQL Stage
FROM postgres:15.1-alpine
ENV POSTGRES_PASSWORD 1234567
ENV POSTGRES_DB demo
COPY database.sql /docker-entrypoint-initdb.d/
EXPOSE 5432

# Rust Stage
FROM rust:latest
WORKDIR /app
COPY --from=Builder /app/network_sniffer /app/network_sniffer
WORKDIR /app/network_sniffer/client/
EXPOSE 8000
CMD ["sudo", "cargo", "run"]
