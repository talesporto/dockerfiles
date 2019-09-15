FROM ubuntu:xenial

ARG CHROME_VERSION="71.0.3578.98-0ubuntu0.16.04.1"

RUN apt-get update
RUN apt-get install chromium-browser=${CHROME_VERSION} -y
RUN apt-get install curl -y
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -
RUN apt-get install -y nodejs
RUN rm -rf /var/lib/apt/lists/*
ENV CHROME_BIN /usr/bin/chromium
