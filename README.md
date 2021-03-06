# Envoy WASM filters

Playground for envoy wasm filters written in Rust.

## Development

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

