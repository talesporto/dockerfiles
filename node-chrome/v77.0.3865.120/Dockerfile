FROM alpeware/chrome-headless-stable:ver-77.0.3865.120

RUN apt-get update
RUN apt-get install curl build-essential -y
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -
RUN apt-get install -y nodejs
RUN rm -rf /var/lib/apt/lists/*
RUN ln -s /opt/google/chrome/chrome /usr/bin/chrome
ENV CHROME_BIN /usr/bin/chrome
ENV CHROMIUM_BIN /usr/bin/chrome
