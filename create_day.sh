#!/bin/bash
#
set -euo pipefail
day=$1
day0=$(printf "%02d" "$1")

if [ ! -f "src/bin/day$day0.rs" ]; then
	cp src/bin/template.rs "src/bin/day$day0.rs"
fi
curl -sSf --output "./src/inputs/day$day0.txt" "https://adventofcode.com/2024/day/$day/input" --compressed -H 'User-Agent: Anisse AoC fetcher' -H 'Referer: https://adventofcode.com/2024/day/$day' -H "Cookie: $(cat cookie.txt)"
exec nvim -O "src/bin/day$day0.rs" "src/inputs/day$day0.txt"
