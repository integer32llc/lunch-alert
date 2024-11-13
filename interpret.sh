#!/usr/bin/env bash

set -eu -o pipefail

xmllint --html --xpath "//main//a" --format full.html > links.html
