FROM maven:3.6-jdk-11-slim

ARG HELM_VERSION="v2.8.0"
ARG KUBECTL_VERSION="v1.11.3"
ARG TERRAFORM_VERSION="0.11.8"
ARG ANT_VERSION="1.10.5"
ARG GRADLE_VERSION="5.0"

RUN curl -s https://storage.googleapis.com/kubernetes-release/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl -o /usr/local/bin/kubectl \
  && chmod +x /usr/local/bin/kubectl \
  && curl -s https://storage.googleapis.com/kubernetes-helm/helm-${HELM_VERSION}-linux-amd64.tar.gz | tar -xzO linux-amd64/helm > /usr/local/bin/helm \
  && chmod +x /usr/local/bin/helm \
  && curl -s -o terraform.zip https://releases.hashicorp.com/terraform/${TERRAFORM_VERSION}/terraform_${TERRAFORM_VERSION}_linux_amd64.zip \
  && unzip terraform.zip -d /usr/local/bin \
  && rm -rf terraform.zip \
  && helm init -c \
  && curl -s -o apache-ant-${ANT_VERSION}-bin.tar.gz http://www.us.apache.org/dist//ant/binaries/apache-ant-${ANT_VERSION}-bin.tar.gz \
  && tar -xzf apache-ant-${ANT_VERSION}-bin.tar.gz \
  && mv apache-ant-${ANT_VERSION} /opt/ant \
  && rm apache-ant-${ANT_VERSION}-bin.tar.gz \
  &&  update-alternatives --install "/usr/bin/ant" "ant" "/opt/ant/bin/ant" 1 \
  && update-alternatives --set "ant" "/opt/ant/bin/ant" \
  && curl -s -o gradle.zip "https://downloads.gradle.org/distributions/gradle-${GRADLE_VERSION}-bin.zip" \
  && unzip gradle.zip -d /opt \
  && rm -rf gradle.zip \
  && update-alternatives --install "/usr/bin/gradle" "gradle" "/opt/gradle-${GRADLE_VERSION}/bin/gradle" 1 \
  && update-alternatives --set "gradle" "/opt/gradle-${GRADLE_VERSION}/bin/gradle"

WORKDIR /config
CMD bash
