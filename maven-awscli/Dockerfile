FROM maven:3.6-jdk-11-slim

RUN apt-get update \
  && apt-get install -y netcat python3-pip \
  && pip3 install awscli --upgrade --no-cache-dir \
  && rm -rf /var/lib/apt/lists/*
