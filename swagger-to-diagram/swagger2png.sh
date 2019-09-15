#!/bin/bash
python3 $(dirname $0)/swagger_to_uml.py $1 | \
  java -jar $(dirname $0)/plantuml.jar -pipe -Tpng > $2
