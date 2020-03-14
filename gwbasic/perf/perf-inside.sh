#!/bin/bash
set -e
COUNT=$1
while [[ $COUNT -gt 0 ]]; do
  if [[ "$2" == "--quiet" ]]; then
    printf '.'
    /usr/local/bin/basic-launcher-rust /basic/src/HELLO.BAS > /dev/null
  else
    /usr/local/bin/basic-launcher-rust /basic/src/HELLO.BAS
  fi
  COUNT=$((COUNT-1))
done
