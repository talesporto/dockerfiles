# GW-Basic and QBasic in Docker

Based on my original blog post [here](https://ngeor.com/2020/02/22/gwbasic-in-docker.html)

There are two images available, [basic](https://hub.docker.com/r/ngeor/basic) and [basic-httpd](https://hub.docker.com/r/ngeor/basic-httpd).

You need to have `GWBASIC.EXE` or `QBASIC.EXE`, it is _not_ baked in the images.

## basic

> Runs a GWBasic or QBasic program

Usage:

```
docker run --rm \
  -v /folder/with/basic:/basic/bin:ro \
  -v /folder/with/program:/basic/src \
  ngeor/basic PROGRAM.BAS
```

## basic-httpd

> Runs Apache HTTPD, supporting BAS files via CGI.

Usage:

```
docker run --rm \
  --name basic-httpd \
  -v /folder/with/basic:/basic/bin:ro \
  -v /folder/with/program:/basic/src \
  -p 8080:80 \
  ngeor/basic-httpd
```

And then visit http://localhost:8080/cgi-bin/PROGRAM.BAS
