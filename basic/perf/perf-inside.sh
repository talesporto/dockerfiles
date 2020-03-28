#!/bin/bash
set -e

COUNT=$1

if [[ "$BLR_BASIC_MODE" == "qbasic" ]]; then
  PROGRAM=/basic/src/HELLOQB.BAS
else
  PROGRAM=/basic/src/HELLO.BAS
fi

if [[ "$2" == "--quiet" ]]; then
  while [[ $COUNT -gt 0 ]]; do
    printf '.'
    /usr/local/bin/basic-launcher-rust $PROGRAM > /dev/null
    COUNT=$((COUNT-1))
  done
else
  while [[ $COUNT -gt 0 ]]; do
    /usr/local/bin/basic-launcher-rust $PROGRAM
    COUNT=$((COUNT-1))
  done
fi
