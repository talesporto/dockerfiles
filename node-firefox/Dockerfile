FROM node:10-jessie

RUN apt-get update \
  && DEBIAN_FRONTEND="noninteractive" \
  && apt-get install -y firefox-esr \
  && rm -rf /var/lib/apt/lists/*
