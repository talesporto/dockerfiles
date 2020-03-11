#!/bin/bash
#
# Runs the given Basic file with GWBasic in DOSBox.
# Works standalone and within Docker.
# Requires STDIN input.
# BAS files need to be in LF instead of CRLF.
# Supports shebang lines for BAS files.
# Passes environment variables to GWBasic.
# Input output is redirected to STDIN.TXT and STDOUT.TXT.
#
# See run-dos-box.rb for an improved version.

if [ -z "$1" ]; then
  echo "Please specify which file to run"
  exit 2
fi

if [ -r "$1" -a -f "$1" ]; then
  FILE="$1"
elif [ -r "/app/basic/$1" -a -f "/app/basic/$1" ]; then
  FILE="/app/basic/$1"
else
  echo "File $1 not found"
  exit 1
fi

# switch to app directory
cd /app

# copy program to PROGRAM.BAS, strip shebang
grep -v "^#!/" "$FILE" | perl -pe 's/\n/\r\n/g' > PROGRAM.BAS

# save stdin
cat | perl -pe 's/\n/\r\n/g' >STDIN.TXT <&0

# save environment variables
declare -px | grep = | grep -v PATH | sed -e 's/declare -x/SET/g' | tr -d '"' > ENV.BAT

# run it
TERM=xterm dosbox RUNGW.BAT -exit > /tmp/dosbox.log 2>&1

# print stdout
cat STDOUT.TXT | perl -pe 's/\r\n/\n/g'
