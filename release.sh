#!/bin/bash

set -e

cargo build --release

strip target/release/build-codebase

mkdir -p dist/logs

cp target/release/build-codebase dist/

rsync -a builders dist

rsync -a public dist

rsync -a templates dist

rsync -a config.json dist

rsync -a data.json dist

rsync -a jq dist

rsync -a keys dist

rsync -a log_config.yml dist

rsync -a make_set dist
