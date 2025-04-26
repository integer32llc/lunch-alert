#!/usr/bin/env bash

set -eu -o pipefail

xmllint --html --xpath "concat('https://www.pghschools.org', string(//main//a/@href))" --format full.html | head -n 1 > url.txt
