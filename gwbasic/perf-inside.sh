#!/bin/bash
set -e
COUNT=$1
while [[ $COUNT -gt 0 ]]; do
  ruby run-dos-box.rb HELLO.BAS
  COUNT=$((COUNT-1))
done
