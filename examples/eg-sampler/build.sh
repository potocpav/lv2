#!/usr/bin/bash

cargo build
mkdir -p eg-sampler.lv2
cp target/debug/*.so eg-sampler.lv2 -v
cp *.ttl eg-sampler.lv2 -v

sudo cp -rv eg-sampler.lv2 /usr/local/lib/lv2/
