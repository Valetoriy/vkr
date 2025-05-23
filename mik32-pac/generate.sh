#!/usr/bin/env bash

rm -rf src
svd2rust --target riscv -i mik32.svd
form -i lib.rs -o src
rm lib.rs build.rs device.x
cargo fmt
