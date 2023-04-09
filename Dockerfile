FROM rust:latest

RUN apt-get update && apt-get install -y iproute2 iptables ethtool iputils-ping net-tools

WORKDIR /project

COPY . /project
