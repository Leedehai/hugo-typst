# Copyright 2026 Leedehai. All rights reserved.

TYPST_WASM_BIN_ABSPATH := $(abspath ../wasm/typst.wasm)
TYPST_SOURCE_ALL_PATHS := $(shell find src -type f)
HUGO_TYPST_BIN_ABSPATH := $(abspath ../../../hugo-typst)

all: $(TYPST_WASM_BIN_ABSPATH)

$(TYPST_WASM_BIN_ABSPATH): $(TYPST_SOURCE_ALL_PATHS) Cargo.toml
	cargo build --target wasm32-wasip1 --release
	mv target/wasm32-wasip1/release/typst.wasm $@

E2E_OUT_HTML := e2e/out/index.html
E2E_EXP_HTML := e2e/expected/index.html
define test_e2e
	rm -rf e2e/out
	$(HUGO_TYPST_BIN_ABSPATH) build --source e2e --quiet
	if diff -q $(E2E_OUT_HTML) $(E2E_EXP_HTML) > /dev/null; then \
		echo "End-to-end test passed: $(E2E_OUT_HTML) == $(E2E_EXP_HTML)"; \
	else \
		echo "End-to-end test error: $(E2E_OUT_HTML) != $(E2E_EXP_HTML)"; \
		diff $(E2E_OUT_HTML) $(E2E_EXP_HTML); \
	fi
endef

test-e2e: $(HUGO_TYPST_BIN_ABSPATH)
	@$(test_e2e)

clean:
	cargo clean
	rm -rf e2e/out

.PHONY: clean
