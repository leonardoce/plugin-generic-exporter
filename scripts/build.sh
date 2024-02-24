#!/usr/bin/env bash

cd "$(dirname "$0")/.." || exit

# The real build happens inside the docker container
cargo check