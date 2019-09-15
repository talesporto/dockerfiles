FROM maven:3-jdk-8-slim

RUN apt-get update && \
  apt-get install -y netcat python3-pip && \
  rm -rf /var/lib/apt/lists/*

# install awscli
RUN pip3 install awscli --upgrade --no-cache-dir
