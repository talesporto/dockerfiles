# vsftpd

FTPS images based on vsftpd.

The `vsftpd` image offers implicit FTP over TLS on port 990.

The username of the user that can access the FTP server is `files`.

## Environment variables

- `PASSWORD`: Sets the password of the `files` user.
- `IMPLICIT_SSL`: Controls whether the SSL mode is implicit or not. By default,
  this setting is set to `YES`. You can switch it off with `NO`.

## Usage examples

Usage in docker-compose:

```yaml
ftps:
  image: ngeor/vsftpd
  environment:
    PASSWORD: secret
  volumes:
    - ./ftp-srv:/home/files
  ports:
    - "990:990"
    - "10090-10100:10090-10100"
```

Usage in bitbucket-pipelines:

```yaml
definitions:
  services:
    ftps:
      image: ngeor/vsftpd
      variables:
        PASSWORD: secret
```
