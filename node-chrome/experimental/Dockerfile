# This image contains Chromium 75.0.3762.0
FROM alpeware/chrome-headless-trunk:rev-649600

RUN apt-get update
RUN apt-get install curl -y
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -
RUN apt-get install -y nodejs
RUN rm -rf /var/lib/apt/lists/*
RUN ln -s /chrome-linux/chrome /usr/bin/chrome
ENV CHROME_BIN /usr/bin/chrome
