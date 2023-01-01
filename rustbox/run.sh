#!/usr/bin/env bash
set -ex

# Go to rustbox root dir
cd "$(dirname $0)"

# Just do `cargo run` and pass all arguments to it
cargo run "$@"