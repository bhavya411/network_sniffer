FROM ubuntu:latest as Builder
WORKDIR /app
RUN apt update && \
    apt install -y git
RUN git clone https://github.com/tarunrawatknoldus/network_sniffer.git

# Build Stage
FROM rust:latest
COPY --from=Builder /app/network_sniffer /app/network_sniffer
WORKDIR /app/network_sniffer/actix
EXPOSE 2000
CMD ["sudo", "cargo", "run"]
