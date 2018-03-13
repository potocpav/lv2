#!/usr/bin/bash

touch build.rs
cargo build
cp target/debug/librust_amp.so rust-amp.lv2/ -v

sudo cp -rv rust-amp.lv2 /usr/local/lib/lv2/
