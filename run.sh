#!/usr/bin/env bash

set -eu -o pipefail

curl "https://www.pghschools.org/departments/food-services/school-menus/site-kitchen" | \
xmllint \
  --html \
  --xpath "concat('https://www.pghschools.org', string(//main//a/@href))" \
  --format - | \
head -n 1 > url.txt

git diff --exit-code
