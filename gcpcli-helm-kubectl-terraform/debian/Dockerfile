FROM python:3.8-buster

# TAG PATTERN: aws-version__helm-version__kubectl-version__terraform-version
# gcpcli-helm-kubectl-terraform:300.0.0-0__2.14.3__1.13.10__0.12.20


ARG GCP_VERSION="300.0.0-0"
ARG HELM_VERSION="v2.14.3"
ARG KUBECTL_VERSION="v1.13.10"
ARG TERRAFORM_VERSION="0.12.20"

RUN wget -q https://storage.googleapis.com/kubernetes-release/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl -O /usr/local/bin/kubectl \
  && chmod +x /usr/local/bin/kubectl \
  && wget -q https://storage.googleapis.com/kubernetes-helm/helm-${HELM_VERSION}-linux-amd64.tar.gz -O - | tar -xzO linux-amd64/helm > /usr/local/bin/helm \
  && chmod +x /usr/local/bin/helm \
  && wget -q -O terraform.zip https://releases.hashicorp.com/terraform/${TERRAFORM_VERSION}/terraform_${TERRAFORM_VERSION}_linux_amd64.zip \
  && unzip terraform.zip -d /usr/local/bin \
  && rm -rf terraform.zip \
  && helm init -c

RUN echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] http://packages.cloud.google.com/apt cloud-sdk main" | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key --keyring /usr/share/keyrings/cloud.google.gpg  add - && apt-get update -y && apt-get install google-cloud-sdk=${GCP_VERSION} -y

RUN apt-get install gettext libc6-dev

WORKDIR /config
CMD bash




