#!/bin/sh

echo "Waiting for db..."

while ! nc -z db 3306; do
  sleep 1 # wait a sec
done

mysql --user=roundcube --password=roundcube --host=db roundcube < /usr/share/roundcube/SQL/mysql.initial.sql

exec /usr/sbin/apache2ctl -D FOREGROUND -k start
