build:
	cargo build --release --target 'wasm32-wasi'
	docker-compose build

run: build
	docker-compose up envoy

test:
	http GET :80/headers
