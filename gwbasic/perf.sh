#!/bin/bash
docker build -t gwbasic-httpd -f Dockerfile.httpd .
docker run --rm -d --name gwbasic-httpd -p 8080:80 -v $PWD/rest:/basic gwbasic-httpd
START=`date +%s%3N`
for i in {1..100} ; do curl --data "hello $i" -H "Content-Type: text/plain" http://localhost:8080/api/todo ; done
STOP=`date +%s%3N`
DIFF=$((STOP-START))
echo $DIFF
docker stop gwbasic-httpd
