#!/bin/bash -eu

cd "${SRC}/fenirer"

cargo build --release -p fenirer-fuzz

for target in fenirer_fuzzer decode_fuzzer chunked_fuzzer; do
  cp "target/release/${target}" "${OUT}/"
done
