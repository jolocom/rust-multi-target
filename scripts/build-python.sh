#!/bin/bash

set -ex
source ./variables.sh

cd $BASE_DIR/ffi/rust/keriox_wrapper/
cargo build --release

cp "./target/release/lib${LIB_NAME}.so" "../../python/libs/lib${LIB_NAME}.so"