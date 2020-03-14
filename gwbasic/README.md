# GW-Basic in Docker

Based on my original blog post [here](https://ngeor.com/2020/02/22/gwbasic-in-docker.html)

There are two images available, [gwbasic](https://hub.docker.com/r/ngeor/gwbasic) and [gwbasic-httpd](https://hub.docker.com/r/ngeor/gwbasic-httpd).

You need to have `GWBASIC.EXE`, it is _not_ baked in the images.

## gwbasic

> Runs a GWBasic program

Usage:

```
docker run --rm \
  -v /folder/with/gwbasic:/basic/bin:ro \
  -v /folder/with/program:/basic/src \
  ngeor/gwbasic PROGRAM.BAS
```

## gwbasic-httpd

> Runs Apache HTTPD, supporting BAS files via CGI.

Usage:

```
docker run --rm \
  --name gwbasic-httpd \
  -v /folder/with/gwbasic:/basic/bin:ro \
  -v /folder/with/program:/basic/src \
  -p 8080:80 \
  ngeor/gwbasic-httpd
```

And then visit http://localhost:8080/cgi-bin/PROGRAM.BAS
