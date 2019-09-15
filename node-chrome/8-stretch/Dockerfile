FROM node:8-stretch

ARG CHROME_VERSION="70.0.3538.110-1~deb9u1"

RUN apt-get update && \
  DEBIAN_FRONTEND="noninteractive" \
  apt-get install -y --no-install-recommends \
  chromium=${CHROME_VERSION} \
  && rm -rf /var/lib/apt/lists/*

USER node

ENV CHROME_BIN /usr/bin/chromium
