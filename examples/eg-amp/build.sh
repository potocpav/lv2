#!/usr/bin/bash

cargo build
mkdir -p eg-amp.lv2
cp target/debug/*.so eg-amp.lv2 -v
cp *.ttl eg-amp.lv2 -v

sudo cp -rv eg-amp.lv2 /usr/local/lib/lv2/
