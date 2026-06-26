# Copyright 2026 Leedehai. All rights reserved.
#
# To build the Hugo binary, just 'make'.

TYPST_WASM_BIN_ABSPATH := $(abspath internal/warpc/wasm/typst.wasm)
TYPST_SOURCE_CARGO_DIR := internal/warpc/gentypst

all: hugo-typst

typst-wasm:
	@printf "\033[32;1m> Building Typst WASM..\n\033[0m"
	@$(MAKE) -C $(TYPST_SOURCE_CARGO_DIR) $(TYPST_WASM_BIN_ABSPATH)

hugo-typst: $(TYPST_WASM_BIN_ABSPATH)
	@printf "\033[32;1m> Building Hugo..\n\033[0m"
	CGO_ENABLED=1 go build -buildvcs=false -o $@ -tags extended

test-math: hugo-typst
	go test ./tpl/transform/...

test-all: hugo-typst
	go test ./...

format:
	git diff --name-only | grep '\.rs$$' | xargs -r rustfmt
	git diff --name-only | grep '\.go$$' | xargs -r go fmt

clean:
	@$(MAKE) -C $(TYPST_SOURCE_CARGO_DIR) clean
	go clean -cache
	rm -f $(TYPST_WASM_BIN_ABSPATH)
	rm -f hugo-typst

# The function below needs it, not the default /bin/sh.
SHELL := /bin/bash
define confirm_git_pull_force
	read -p "Are you sure to run git-pull by force? (y/n): " -n 1 -r ANSWER; \
	echo ""; \
	if [[ ! "$$ANSWER" =~ ^[Yy]$$ ]]; then \
		echo "Operation canceled."; \
		exit 1; \
	fi
endef

git-pull-force:
	@$(confirm_git_pull_force)
	git fetch --all
	git reset --hard origin/hugo-typst

.PHONY: all typst-wasm test-math test-all format clean git-pull-force
