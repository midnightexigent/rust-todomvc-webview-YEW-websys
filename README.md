# Webview using the YEW framework with Web sys

This is an example that showcases a usage of the [webview](https://github.com/Boscop/web-view) rust bindings library with the [yew](https://yew.rs/docs/en/) framework.

This reproduces the exact same example provided [here](https://github.com/yewstack/yew/tree/v0.17/examples/todomvc), but instead of serving the output of `wasm-pack`, it embeds it into an executable which is the webview

This is inspired from [this example](https://github.com/Extrawurst/rust-webview-todomvc-yew) but uses [warp](https://github.com/seanmonstar/warp) + [tokio](https://github.com/tokio-rs/tokio) instead of [hyper](https://github.com/hyperium/hyper) for the server and the [web-sys](https://docs.rs/web-sys/0.3.47/web_sys/) instead of [stdweb](https://docs.rs/stdweb/0.4.20/stdweb/) as it seems to be recommended by the [yew](https://yew.rs/docs/en/getting-started/choose-web-library) documentation

## Quickstart

just run `make run`

## Manually run

- First, you'll need to have [wasm-pack](https://github.com/rustwasm/wasm-pack) installed `cargo install wasm-pack`
- Then, generate all of the **web** stuff in the [static directory](static): `cd todomvc && wasm-pack build --target web --out-name wasm --out-dir ../static`
- Now run the webview that embeds the output of the previous step: `cargo run --release`
- To only build `cargo build --release`

![image](https://imgur.com/2KaqTL8l.png)
