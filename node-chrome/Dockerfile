FROM alpeware/chrome-headless-stable:ver-77.0.3865.120

RUN apt-get update \
  && apt-get install curl build-essential -y \
  && curl -sL https://deb.nodesource.com/setup_10.x | bash - \
  && apt-get install -y nodejs \
  && rm -rf /var/lib/apt/lists/* \
  && ln -s /opt/google/chrome/chrome /usr/bin/chrome

ENV CHROME_BIN /usr/bin/chrome
ENV CHROMIUM_BIN /usr/bin/chrome
