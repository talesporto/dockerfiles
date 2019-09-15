FROM ubuntu

RUN DEBIAN_FRONTEND=noninteractive apt update
RUN DEBIAN_FRONTEND=noninteractive apt upgrade -y
RUN DEBIAN_FRONTEND=noninteractive apt install -y roundcube
RUN DEBIAN_FRONTEND=noninteractive apt install -y netcat

EXPOSE 80
RUN ln -s /var/lib/roundcube /var/www/html/roundcube

COPY run.sh /usr/sbin/run.sh
COPY config.inc.php /etc/roundcube/config.inc.php

CMD [ "/usr/sbin/run.sh" ]
