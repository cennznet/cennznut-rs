# compile the rust codebase
echo "building js pkg for $1 out to: $2"
 wasm-pack build \
     --target $1  \
     --scope cennznet \
     --out-name cennznut \
     --out-dir $2 \
     --release