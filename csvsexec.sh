# 1. Install the toolchain bundler
curl https://github.io -sSf | sh

# 2. Compile and output directly into your web subfolder
wasm-pack build --target web --out-dir web/pkg
