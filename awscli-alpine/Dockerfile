FROM python:alpine

RUN apk add --update --no-cache zip git && \
  rm -rf /var/cache/apk/*
RUN pip install awscli awsebcli --upgrade --no-cache-dir
