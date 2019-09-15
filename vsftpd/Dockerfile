# sources of inspiration:
# https://github.com/onjin/docker-alpine-vsftpd/blob/master/docker-entrypoint.sh
# https://github.com/hwoarang/docker-vsftpd/blob/master/vsftpd.conf.template
#
# about SSL reuse (not used)
# http://eng.wealthfront.com/2016/06/10/connecting-to-an-ftps-server-with-ssl-session-reuse-in-java-7-and-8/

FROM alpine

RUN apk add --no-cache vsftpd openssl

RUN adduser -h /home/files -s /bin/false -D files

COPY vsftpd.conf /etc/vsftpd/vsftpd.conf

ADD docker-entrypoint.sh /
VOLUME /home/files

EXPOSE 990 10090-10100

CMD /docker-entrypoint.sh
