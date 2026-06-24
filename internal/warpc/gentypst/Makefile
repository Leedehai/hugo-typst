# Copyright 2026 Leedehai.

.PHONY: build-typst-wasm clean

build-typst-wasm:
	cargo build --target wasm32-wasip1 --release
	mv target/wasm32-wasip1/release/typstlib.wasm ../wasm/typst.wasm

clean:
	rm -rf target
