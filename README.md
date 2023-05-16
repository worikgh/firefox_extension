# Call Rust Code from a Firefox Extension

Aiming to build an extension that changes the colour of all links.

## Status

Have wasm being called from an ordinary web page.  Implemented an interface for the `pub fn add(a: i32, b: i32) -> i32 ` example

## Build

In the `wasm_link_color` directory:

```
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --no-typescript --out-dir .. target/wasm32-unknown-unknown/release/wasm_link_color.wasm
```

## Serve

In the root directory run `python3 -m http.server 8000`

## Run

Navigate to http://localhost:8000/index.html 
