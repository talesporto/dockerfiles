#!/bin/sh
if [ -z ${PASSWORD} ]; then
  PASSWORD=$(< /dev/urandom tr -dc A-Za-z0-9 | head -c${1:-8};echo;)
  echo "Generated password for user 'files': ${PASSWORD}"
fi
# set ftp user password
echo "files:${PASSWORD}" |/usr/sbin/chpasswd
chown files:files /home/files/ -R

echo "Creating the certificate"
openssl req -x509 -nodes -days 3650 -newkey rsa:4096 \
  -keyout /etc/vsftpd/vsftpd.pem -out /etc/vsftpd/vsftpd.pem \
  -batch || { echo "Failed to create the vsftpd certificate"; exit 1; }

if [ -n "${IMPLICIT_SSL}" ]; then
  # overwrite configuration
  sed -i -e "s/implicit_ssl=YES/implicit_ssl=${IMPLICIT_SSL}/g" /etc/vsftpd/vsftpd.conf
fi

if [ -z $1 ]; then
  /usr/sbin/vsftpd /etc/vsftpd/vsftpd.conf
else
  $@
fi
