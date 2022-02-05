#!/bin/bash

set -eu

dir=$(dirname "$0")

curl -sS -X POST localhost:3000/api/compile --data-binary "@$dir/example.rs" | jq
