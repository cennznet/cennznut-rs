#!/bin/bash
set -ex
echo "building js pkg for $1 out to: $2"
wasm-pack build \
    --target $1 \
    --scope cennznet \
    --out-name cennznut \
    --out-dir $2 \
    --release

# Remove wasm-pack generated files
# They are unintentionally excluding required files when `npm pack` is run
cd $2
rm package.json README.md .gitignore LICENSE
