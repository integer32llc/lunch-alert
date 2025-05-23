#!/usr/bin/env bash

set -eu -o pipefail

curl -s "https://www.pghschools.org/departments/food-services/school-menus/site-kitchen" | \
xmllint \
  --html \
  --xpath "concat('https://www.pghschools.org', string(//main//a/@href))" \
  --format - | \
head -n 1 > url.txt
