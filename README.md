# hello_wgpu
hello world for gpu programing in rust

folowing https://github.com/sotrh/learn-wgpu 

DISSABLE YOUR BROWSER CACH WHEN DEVLOPING!!!

## build
cargo run
RUST_LOG=info cargo run


wasm-pack build --target web -d website/pkg
http-server website/
http-server website/ -c #for dev when we dont want cach