echo building

cargo wasm

echo bindings

wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/bevcraft.wasm
