FROM python:alpine

# TAG PATTERN: aws-version__helm-version__kubectl-version__terraform-version
# awscli-helm-kubectl-terraform:1.14.44__2.14.3__1.13.10__0.12.20


ARG AWS_VERSION="1.14.44"
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

RUN apk add --update --no-cache zip git gcc musl-dev libffi-dev libressl-dev make \
  && rm -rf /var/cache/apk/* \
  && pip install awscli==$AWS_VERSION awsebcli --upgrade --no-cache-dir \
  && curl -o aws-iam-authenticator https://amazon-eks.s3-us-west-2.amazonaws.com/1.13.8/2019-08-14/bin/linux/amd64/aws-iam-authenticator \
  && chmod +x ./aws-iam-authenticator \
  && cp ./aws-iam-authenticator /usr/bin/aws-iam-authenticator \
  && mkdir -p $HOME/bin && cp ./aws-iam-authenticator $HOME/bin/aws-iam-authenticator && export PATH=$HOME/bin:$PATH \
  && echo 'export PATH=$HOME/bin:$PATH' >> ~/.bashrc

RUN apk add --no-cache gettext libintl

WORKDIR /config
CMD bash




