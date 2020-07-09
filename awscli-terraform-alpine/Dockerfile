FROM python:alpine
ARG TERRAFORM_VERSION="0.12.18"

RUN apk add --update --no-cache ca-certificates openssl zip git && \
  rm -rf /var/cache/apk/*
RUN pip install awscli awsebcli --upgrade --no-cache-dir

RUN wget -O terraform.zip  "https://releases.hashicorp.com/terraform/${TERRAFORM_VERSION}/terraform_${TERRAFORM_VERSION}_linux_amd64.zip" && \
  unzip terraform.zip -d /bin && \
  rm -rf terraform.zip
