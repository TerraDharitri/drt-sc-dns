#!/bin/sh

# drtpy --verbose contract build dns

CONTRACT_PATH="dns"

CONTRACT_NAME="numbat-wasm-sc-dns"

rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}.wasm
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}.abi.json
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}.imports.json


cd ${CONTRACT_PATH}/meta
cargo run build