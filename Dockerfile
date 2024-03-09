FROM rust:1.76.0-slim-buster

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    wget \
    unzip \
    dpkg \
    gnupg \
    make

# Install Chrome
COPY ./scripts/install_chrome.sh /app

RUN chmod u+x install_chrome.sh && \
    ./install_chrome.sh

COPY . /app

RUN rustup component add rustfmt && \
    rustup component add clippy 

# Commands for docker run, later change cargo build to make ready(to run tests before building image)
RUN make clean && \
    cargo build 
