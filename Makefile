# Copyright 2026 Leedehai.

.PHONY: all typst-wasm hugo-typst clean git-pull-force

all: hugo-typst

typst-wasm:
	@echo "Building Typst WASM.."
	$(MAKE) -C internal/warpc/gentypst typst-wasm

hugo-typst: typst-wasm
	@echo "Building Hugo.."
	CGO_ENABLED=1 go build -o hugo-typst -tags extended

clean:
	cd typstlib && cargo clean
	rm -f internal/warpc/wasm/typst.wasm
	rm -f hugo-typst

# The function below needs it, not the default /bin/sh.
SHELL := /bin/bash
define ask_confirmation
	read -p "Are you sure to run git-pull by force? (y/n): " -n 1 -r ANSWER; \
	echo ""; \
	if [[ ! "$$ANSWER" =~ ^[Yy]$$ ]]; then \
		echo "Operation canceled."; \
		exit 1; \
	fi
endef

git-pull-force:
	@$(ask_confirmation)
	git fetch --all
	git reset --hard origin/hugo-typst
