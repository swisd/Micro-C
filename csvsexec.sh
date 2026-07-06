curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"





# 1. Install the toolchain bundler
#curl https://github.io -sSf | sh

#export PATH="$HOME/.cargo/bin:$PATH"

# 2. Compile and output directly into your web subfolder
#wasm-pack build --target web --out-dir web/pkg


rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release --target-dir ./web/

# 1. Install the standalone binder tool
cargo install wasm-bindgen-cli

# 2. Extract Javascript bindings from the binary into your web asset bundle
~/.cargo/bin/wasm-bindgen ./web/wasm32-unknown-unknown/release/micro_c.wasm --target web --out-dir ./web/pkg

cd web
git init
git add .
git commit -m "Deploy compiler client layer"
git remote add origin https://github.com
git checkout -b gh-pages
git push -f origin gh-pages
