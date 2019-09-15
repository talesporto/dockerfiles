# docker-webmail

Dockerized webmail using dovecot and roundcube

# Dovecot

## Structure

You'll need a data volume where the mbox files will be stored.

## Authorization

You'll need a passwd database like this:

```
user:{plain}password:1000:8
```

1000 is the user id, 8 is the group id (8 maps to `mail` which is good enough)

## Permissions

Assign the `mail` group to the mail data volume with write permissions:

```
$ sudo chown -R ngeor:mail mail/
$ sudo chmod g+w mail/
```

# Roundcube

Roundcube will be available at http://localhost:8200/roundcube/
