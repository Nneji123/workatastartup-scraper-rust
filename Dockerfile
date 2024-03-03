FROM ubuntu:latest

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

# TODO: Add extra dependencies, if needed
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

# Commands for docker run
CMD make clean && \
    make ready
