RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' cargo build -v --target wasm32-unknown-unknown -Z build-std=std,panic_abort --release
wasm-bindgen target/wasm32-unknown-unknown/release/rsfractal.wasm --out-dir docs --no-modules --no-typescript
# Fix missing typeof check for Window when running inside a Worker
sed -i 's/getObject(arg0) instanceof Window/typeof Window !== "undefined" \&\& getObject(arg0) instanceof Window/' docs/rsfractal.js
wasm-opt -O4 docs/rsfractal_bg.wasm -o docs/rsfractal_bg.wasm