# hello_wgpu
hello world for gpu programing in rust

folowing https://github.com/sotrh/learn-wgpu 

DISSABLE YOUR BROWSER CACH WHEN DEVLOPING!!!

## build
cargo run
RUST_LOG=info cargo run


wasm-pack build --target web -d website/pkg
http-server website/
http-server website/ -c-1 #for dev when we dont want cach

## coding style 

I am purposfully making a bit of a mess with the use:: statments so that I know what functions depend on them 
when I am less of a rust noob I would probably go back to using it properly like I do in python and c++