FROM python:3.8-alpine

# TAG PATTERN: aws-version__helm-version__kubectl-version__terraform-version
# gcpcli-helm-kubectl-terraform:300.0.0__2.14.3__1.13.10__0.12.20


ARG GCP_VERSION="300.0.0"
ARG HELM_VERSION="v2.14.3"
ARG KUBECTL_VERSION="v1.13.10"
ARG TERRAFORM_VERSION="0.12.20"

RUN apk add --no-cache ca-certificates bash curl git \
  && wget -q https://storage.googleapis.com/kubernetes-release/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl -O /usr/local/bin/kubectl \
  && chmod +x /usr/local/bin/kubectl \
  && wget -q https://storage.googleapis.com/kubernetes-helm/helm-${HELM_VERSION}-linux-amd64.tar.gz -O - | tar -xzO linux-amd64/helm > /usr/local/bin/helm \
  && chmod +x /usr/local/bin/helm \
  && wget -q -O terraform.zip https://releases.hashicorp.com/terraform/${TERRAFORM_VERSION}/terraform_${TERRAFORM_VERSION}_linux_amd64.zip \
  && unzip terraform.zip -d /usr/local/bin \
  && rm -rf terraform.zip \
  && helm init -c

RUN apk add --update --no-cache zip \
  && wget -O google-cloud-sdk.tar.gz https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-${GCP_VERSION}-linux-x86_64.tar.gz \
  && tar zxvf google-cloud-sdk.tar.gz \
  && google-cloud-sdk/install.sh --quiet --usage-reporting false --command-completion true --path-update true

RUN apk add --no-cache gettext libintl

WORKDIR /config
CMD bash




