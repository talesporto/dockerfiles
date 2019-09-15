# dockerfiles

A home for my various custom Docker images

## \*-helm-kubectl-terraform

Docker images that include helm, kubectl, and terraform.

- Inspired by [dtzar/helm-kubectl](https://github.com/dtzar/helm-kubectl)
- Includes `terraform`

### Tags

The tag scheme is a bit complex because of the multiple packages.

`helm-version__kubectl-version__terraform-version`

e.g.

`2.8.2__0.11.3__0.11.0`

### helm-kubectl-terraform

[![Docker Pulls](https://img.shields.io/docker/pulls/ngeor/helm-kubectl-terraform.svg)](https://hub.docker.com/r/ngeor/helm-kubectl-terraform/)

Based on alpine.

### helm-kubectl-terraform

| Name                                           | helm    | kubectl | terraform |
| ---------------------------------------------- | ------- | ------- | --------- |
| helm-kubectl-terraform:2.12.3**1.12.4**0.11.11 | v2.12.3 | v1.12.4 | 0.11.11   |

### jdk-helm-kubectl-terraform

[![Docker Pulls](https://img.shields.io/docker/pulls/ngeor/jdk-helm-kubectl-terraform.svg)](https://hub.docker.com/r/ngeor/jdk-helm-kubectl-terraform/)

Based on `maven:3.6-jdk-11-slim`. Includes ant and gradle.

| Name                                               | helm    | kubectl | terraform |
| -------------------------------------------------- | ------- | ------- | --------- |
| jdk-helm-kubectl-terraform:2.12.3**1.12.4**0.11.11 | v2.12.3 | v1.12.4 | 0.11.11   |

### python-helm-kubectl-terraform

[![Docker Pulls](https://img.shields.io/docker/pulls/ngeor/python-helm-kubectl-terraform.svg)](https://hub.docker.com/r/ngeor/python-helm-kubectl-terraform/)

Based on python:alpine

### python-helm-kubectl-terraform

| Name                                                  | helm    | kubectl | terraform |
| ----------------------------------------------------- | ------- | ------- | --------- |
| python-helm-kubectl-terraform:2.12.3**1.12.4**0.11.11 | v2.12.3 | v1.12.4 | 0.11.11   |

### ruby-helm-kubectl-terraform

[![Docker Pulls](https://img.shields.io/docker/pulls/ngeor/ruby-helm-kubectl-terraform.svg)](https://hub.docker.com/r/ngeor/ruby-helm-kubectl-terraform/)

Based on ruby:2.5.3-alpine

### ruby-helm-kubectl-terraform

| Name                                                | helm    | kubectl | terraform |
| --------------------------------------------------- | ------- | ------- | --------- |
| ruby-helm-kubectl-terraform:2.12.3**1.12.4**0.11.11 | v2.12.3 | v1.12.4 | 0.11.11   |
