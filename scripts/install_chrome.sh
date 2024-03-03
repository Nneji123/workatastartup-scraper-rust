#!/bin/bash

# Install Chrome
wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list
apt-get update && apt-get -y install google-chrome-stable

# Download Chromedriver
CHROME_VERSION=$(google-chrome --product-version)
wget "https://storage.googleapis.com/chrome-for-testing-public/$CHROME_VERSION/linux64/chromedriver-linux64.zip"
unzip chromedriver-linux64.zip
mv ./chromedriver-linux64/chromedriver ./chromedriver
rm -rf ./chromedriver-linux64 && rm -rf ./chromedriver-linux64.zip
