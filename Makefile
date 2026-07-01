# Copyright 2026 Leedehai. All rights reserved.
#
# To build the Hugo binary, just 'make'.

HUGOTYPST_ROOT_ABSPATH := $(abspath .)
TYPST_WASM_BIN_ABSPATH := $(abspath internal/warpc/wasm/typst.wasm)
TYPST_SOURCE_CARGO_DIR := internal/warpc/gentypst
# The functions below need it, not the default /bin/sh.
SHELL := /bin/bash

all: hugo-typst

typst-wasm:
	@printf "\033[32;1m> Building Typst WASM..\n\033[0m"
	@$(MAKE) -C $(TYPST_SOURCE_CARGO_DIR) $(TYPST_WASM_BIN_ABSPATH)

hugo-typst: typst-wasm
	@printf "\033[32;1m> Building Hugo..\n\033[0m"
	CGO_ENABLED=1 go build -buildvcs=false -o $@ -tags extended

test-e2e: hugo-typst
	@printf "\033[32;1m> Testing e2e..\n\033[0m"
	@$(MAKE) -C $(TYPST_SOURCE_CARGO_DIR) test-e2e

test-math: hugo-typst
	@printf "\033[32;1m> Testing math..\n\033[0m"
	go test ./tpl/transform/...
	@$(MAKE) -C . test-e2e

test-all: hugo-typst
	@printf "\033[32;1m> Testing all..\n\033[0m"
	go test ./...
	@$(MAKE) -C . test-e2e

format:
	git diff --name-only | grep '\.rs$$' | xargs -r rustfmt
	git diff --name-only | grep '\.go$$' | xargs -r gofmt -w

clean:
	@$(MAKE) -C $(TYPST_SOURCE_CARGO_DIR) clean
	go clean -cache
	rm -f $(TYPST_WASM_BIN_ABSPATH)
	rm -f hugo-typst

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

.PHONY: all typst-wasm test-math test-e2e test-all format clean git-pull-force
