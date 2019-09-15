FROM ubuntu

RUN DEBIAN_FRONTEND=noninteractive apt update
RUN DEBIAN_FRONTEND=noninteractive apt upgrade -y
RUN DEBIAN_FRONTEND=noninteractive apt install -y dovecot-imapd

COPY *.conf /etc/dovecot/conf.d/
COPY *.ext /etc/dovecot/conf.d/

VOLUME [ "/var/mail/user/mail" ]
VOLUME [ "/etc/dovecot/users" ]

EXPOSE 143
EXPOSE 993

RUN mkdir -p /var/mail/user/indexes
RUN chmod 777 /var/mail/user/indexes
RUN mkdir -p /var/mail/user/control
RUN chmod 777 /var/mail/user/control

CMD [ "dovecot", "-F" ]
