# Envoy WASM filters

Playground for envoy wasm filters written in Rust.

## Development

- [proxy-wasm-rust](https://github.com/proxy-wasm/proxy-wasm-rust-sdk)

### Toolchain

Install rustup
``` bash
curl https://sh.rustup.rs -sSf | sh
```

Get wasm32 target
``` bash
rustup target add wasm32-unknown-unknown
```

### Local workflow

Build wasm artifact
``` bash
cargo build --target wasm32-unknown-unknown --release
```

Sart local envoy proxy
``` bash
docker-compose up -d --build
```

Hit envoy endpoint
``` bash
curl -i http://localhost:18000
```

