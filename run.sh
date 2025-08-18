#!/usr/bin/env bash

set -eu -o pipefail

curl -s "https://www.pghschools.org/departments/food-services/school-menus/site-kitchen" | \
cargo run > url.txt
