# Builder Stage
FROM ubuntu:latest as Builder
WORKDIR /
RUN apt update && \
    apt install -y git && \
    git clone https://github.com/tarunrawatknoldus/network_sniffer.git

# Rust Build Stage
FROM rust:latest as RustBuilder
COPY --from=Builder /network_sniffer/ /network_sniffer/
WORKDIR /network_sniffer/client/
EXPOSE 8000
CMD ["sudo", "cargo", "run"]
