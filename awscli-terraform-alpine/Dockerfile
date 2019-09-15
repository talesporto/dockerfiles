FROM python:alpine

RUN apk add --update --no-cache ca-certificates openssl zip git && \
  rm -rf /var/cache/apk/*
RUN pip install awscli awsebcli --upgrade --no-cache-dir

RUN wget -O terraform.zip  "https://releases.hashicorp.com/terraform/0.11.8/terraform_0.11.8_linux_amd64.zip" && \
  unzip terraform.zip -d /bin && \
  rm -rf terraform.zip
