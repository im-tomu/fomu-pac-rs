#!/usr/bin/env bash

svd2rust -i Fomu.svd --target riscv
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
