FROM openjdk:11-jre-slim

RUN apt-get update \
  && apt-get install curl python3 python3-pip graphviz -y \
  && mkdir -p /usr/local/bin \
  && curl -v -s -o /usr/local/bin/requirements.txt https://raw.githubusercontent.com/nlohmann/swagger_to_uml/master/requirements.txt \
  && curl -v -s -o /usr/local/bin/swagger_to_uml.py https://raw.githubusercontent.com/nlohmann/swagger_to_uml/master/swagger_to_uml.py \
  && curl -v -s -o /usr/local/bin/plantuml.jar https://freefr.dl.sourceforge.net/project/plantuml/plantuml.jar \
  && pip3 install --upgrade --no-cache-dir -r /usr/local/bin/requirements.txt \
  && rm /usr/local/bin/requirements.txt \
  && rm -rf /var/lib/apt/lists/*

COPY *.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/*.sh
RUN mkdir /data
WORKDIR /data
