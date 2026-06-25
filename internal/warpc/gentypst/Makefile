# Copyright 2026 Leedehai.

.PHONY: typst-wasm clean

typst-wasm:
	cargo build --target wasm32-wasip1 --release
	mv target/wasm32-wasip1/release/typst.wasm ../wasm/typst.wasm

clean:
	rm -rf target
