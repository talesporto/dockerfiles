# docker-node-chrome

A Docker image based on the official nodeJS image, including headless chrome
support.

## Details

- Builds on top of [nodeJS official image](https://hub.docker.com/_/node/)
- Inspired by
  [weboaks/node-karma-protractor-chrome](https://hub.docker.com/r/weboaks/node-karma-protractor-chrome/)
- Adds headless Chrome support
- Available on [Docker Hub](https://hub.docker.com/r/ngeor/node-chrome/)

## Tags

The tag format consists of the base image tag (e.g. `8-stretch`) and the Chrome
version that was installed during the build.

- `10-stretch-chrome-72.0.3626.96-1`, `10-stretch`, `10`, `latest`
- `8-stretch-chrome-72.0.3626.96-1`, `8-stretch`, `8`
- `8-stretch-chrome-69.0.3497.92-1`
