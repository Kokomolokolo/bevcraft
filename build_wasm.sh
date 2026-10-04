echo building

cargo build --release --target wasm32-unknown-unknown

echo bindings

wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/bevcraft.wasm
