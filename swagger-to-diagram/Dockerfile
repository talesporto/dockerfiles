FROM openjdk:11-jre-slim

RUN apt-get update && \
  DEBIAN_FRONTEND="noninteractive" \
  apt-get install curl python3 python3-pip graphviz -y \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /usr/local/bin
RUN curl -s -o /usr/local/bin/requirements.txt https://raw.githubusercontent.com/nlohmann/swagger_to_uml/master/requirements.txt
RUN pip3 install -r /usr/local/bin/requirements.txt && rm /usr/local/bin/requirements.txt
RUN curl -s -o /usr/local/bin/swagger_to_uml.py https://raw.githubusercontent.com/nlohmann/swagger_to_uml/master/swagger_to_uml.py
RUN curl -s -o /usr/local/bin/plantuml.jar https://netix.dl.sourceforge.net/project/plantuml/plantuml.jar
COPY *.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/*.sh

RUN mkdir /data
WORKDIR /data
