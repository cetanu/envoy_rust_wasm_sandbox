FROM envoyproxy/envoy:v1.21.0
WORKDIR /srv
ADD target/wasm32-wasi/release/rust_envoy_wasm_sandbox.wasm /srv/sandbox.wasm
ADD envoy.yaml /srv/envoy.yaml
CMD envoy -c /srv/envoy.yaml

