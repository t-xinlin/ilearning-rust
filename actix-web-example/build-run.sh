#!/bin/bash
set +e
cur_path=`pwd`
cargo build --release --features foo
cp -f target/release/actix-web-example ./
cp -rf src/conf ./
cp -rf src/test.txt ./
f_path=${cur_path}/test.txt
echo $f_path
export IGNORE_CASE=true&&./actix-web-example abc $f_path
