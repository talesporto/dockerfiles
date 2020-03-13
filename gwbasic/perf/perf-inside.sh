#!/bin/bash
set -e
COUNT=$1
while [[ $COUNT -gt 0 ]]; do
  if [[ "$2" == "--quiet" ]]; then
    printf '.'
    ruby /usr/local/bin/run-dos-box.rb /basic/src/HELLO.BAS > /dev/null
  else
    ruby /usr/local/bin/run-dos-box.rb /basic/src/HELLO.BAS
  fi
  COUNT=$((COUNT-1))
done
