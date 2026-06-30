# Hugo-Typst

Bring [Typst](https://typst.app)'s math prowess to [Hugo](https://gohugo.io/).

> [!NOTE]
> The repository is kept in a way that makes it easy for me to create a PR to
> the [Hugo repostory](https://github.com/gohugoio/hugo) at some point.
> 
> That said, it also serves as a fork of Hugo. The resulting executable from the
> build process, `hugo-typst`, can be a drop-in replacement of the official
> `hugo` executable, and can work with existing Hugo themes and CI/CD workflows
> of Hugo-built websites. This repository will sync with the Hugo codebase from
> time to time.

[Typst](https://typst.app) is a new markup-based typesetting system that is
designed to be as powerful as LaTeX while being much easier to learn and use.

Hugo already supports math in LaTeX. With the help of
[Wazero](https://wazero.io/), a WebAssembly (WASM) runtime for Go program, Hugo
can run the WASM module of [QuickJS](https://bellard.org/quickjs/), an embedded
JavaScript engine written in C. Further, Hugo uses that engine to run
[KaTeX](https://katex.org/), a LaTeX reimplementation in JavaScript for web.
As an optimization, [Javy](https://github.com/bytecodealliance/javy) is
responsible for compiling JavaScript to QuickJS bytecode so that QuickJS does
not have to parse the text of JavaScript code at Hugo runtime.

To support Typst too, here we use Wazero to run the WASM binary of Typst, which
is written in Rust. Typst supports many output formats, including for web. This
project uses Typst to compile math into
[MathML](https://developer.mozilla.org/en-US/docs/Web/MathML).

## Current state

- [x] Add Typst WASM, RPC integration
- [x] :tada: (MVP) Compile to MathML, happy path
- [x] MathML diagnostics
- [ ] MathML rendering with CSS stylesheets
- [ ] (maybe) Compile to SVG?

## Integration using RPC

Hugo integrates with WASM modules through the `warpc.go` ("WASM RPC") layer.

With Go's goroutine feature, Hugo maintains a pool of WASM workers (including
that for Typst) and communicates with them with in-memory JSON-based RPC. Each
worker deserializes requests received from `stdin`, does its work, and writes
responses to `stdout`.

Hugo uses this goroutine+RPC architecture instead of making direct function
calls into WASM modules, in order to bypass the complexity of passing
input/output as pointers (and managing their memory), and to prevent crashes
from leaking into Hugo's main process.

## Build

```bash
# Ensure you have Go and Rust toolchains.
$ go version
go version go1.26.4
$ cargo version
cargo 1.96.0 (30a34c682 2026-05-25)

# Ensure your Rust toolchain supports cross-compilation to wasm32-wasip1.
$ rustup target add wasm32-wasip1

# Ensure you have Make.
$ make --version
GNU Make 3.81

# Build it.
$ make

# Find the resulting binary at the project root. Use it just like a normal
# Hugo binary.
$ ./hugo-typst version
```

## Test

```bash
# Run math-related tests.
make test-math

# Run all tests in Hugo.
make test-all
```

## Usage

Step 1. As with LaTeX, configure the passthrough, so that Hugo knows
it needs to send the content between the delimiters to the math renderer
before sending the file to the Markdown processor.
```yaml
# hugo.yaml (or hugo.toml/hugo.json depending on your choice)
markup:
  goldmark:
    extensions:
      passthrough:
        delimiters:
          block:
          - - $$
            - $$
          inline:
          - - '\('
            - '\)'
        enable: true
```

Step 2. As with LaTeX, configure the
[passthrough template](https://gohugo.io/render-hooks/passthrough/#example), so
that Hugo knows to pass the math content to Typst.
```html
# layouts/_markup/render-passthrough.html
TODO
```

Step 3. In Markdown files, write your math equation in Typst.
```markdown
TODO
```

Step 4. In base template, conditionally include the CSS within the head element:
```html
TODO
```

## Reference: Typst CLI usage

MathML output is supported since Typst 0.15 behind feature flag
`--features html`.

```bash
$ typst compile -f html --features html - -
```

This command will bring up `stdin`. Type `$ E = m c^2 $`, and then Ctrl+D to
end the input stream. Then, Typst will print to `stdout` with something like
this (with omission and formatting):

```text
<!DOCTYPE html><html lang="en">
<head>...
<style> ... </style>
</head>
<body>
<math display="block">
  <mi>𝐸</mi><mo>=</mo><mi>𝑚</mi><msup><mi>𝑐</mi><mn>2</mn></msup></math>
</body>
</html>
```

## License

MIT License, see [LICENSE.txt](./LICENSE.txt).

For license of Hugo and dependencies thereof, see the `License` section in
[README.hugo.md](./README.hugo.md).
