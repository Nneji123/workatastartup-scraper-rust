FROM ubuntu:latest

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    wget \
    unzip \
    dpkg \
    gnupg

# Install Chrome
ADD ./github/scripts/install_chrome_and_chromedriver.sh /app

RUN chmod u+x install_chrome_and_chromedriver.sh && \
    ./install_chrome_and_chromedriver.sh

# TODO: Add extra dependencies, if needed
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

ADD . /app

# Commands for docker run
CMD make clean && \
    make ready
