// Copyright 2026 Leedehai. All rights reserved.

package warpc

import (
	_ "embed"
)

//go:embed wasm/typst.wasm
var typstWasm []byte

type TypstInput struct {
	Expression string       `json:"expression"`
	Options    TypstOptions `json:"options"`
}

// TypstOptions defines the options for the Typst rendering.
type TypstOptions struct {
	// html, mathml (default), htmlAndMathml
	Output string `json:"output"`

	// If true, display math in display mode, false in inline mode.
	DisplayMode bool `json:"displayMode"`
}

type TypstOutput struct {
	Output string `json:"output"`
}
