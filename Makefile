# Copyright 2026 Leedehai.

.PHONY: all build-typst-wasm build-hugo clean

all: build-hugo

build-typst-wasm:
	@echo "Building Typst WASM.."
	$(MAKE) -C internal/warpc/gentypst/Makefile

build-hugo: build-typst-wasm
	@echo "Building Hugo.."
	CGO_ENABLED=1 go build -o hugo-typst -tags extended

clean:
	cd typstlib && cargo clean
	rm -f tpl/typst/internal
	rm -f hugo-typst
