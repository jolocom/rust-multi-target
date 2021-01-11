#!/bin/bash

set -ex
source ./scripts/variables.sh

cd $BASE_DIR/flapigen/rust/keriox_wrapper/

cargo build --release

cp "./target/release/lib${LIB_NAME}.so" "../../python/libs/lib${LIB_NAME}.so"
python3 ../../python/test.py