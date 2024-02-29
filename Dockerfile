FROM ubuntu:latest

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    wget \
    unzip \
    dpkg

RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list
RUN apt-get update && apt-get -y install google-chrome-stable


# TODO: Add extra dependencies, if needed
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Commands for docker run
CMD make clean && \
    make ready
