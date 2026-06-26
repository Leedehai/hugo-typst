# Copyright 2026 Leedehai. All rights reserved.

TYPST_WASM_BIN_ABSPATH := $(abspath ../wasm/typst.wasm)
TYPST_SOURCE_ALL_PATHS := $(shell find src -type f)

all: $(TYPST_WASM_BIN_ABSPATH)

$(TYPST_WASM_BIN_ABSPATH): $(TYPST_SOURCE_ALL_PATHS) Cargo.toml
	cargo build --target wasm32-wasip1 --release
	mv target/wasm32-wasip1/release/typst.wasm $@

clean:
	cargo clean

.PHONY: clean
