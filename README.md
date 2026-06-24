# Hugo-Typst

Bring [Typst](https://typst.app)'s math prowess to [Hugo](https://gohugo.io/).

[Typst](https://typst.app) is a new markup-based typesetting system that is
designed to be as powerful as LaTeX while being much easier to learn and use.

Hugo already supports math in LaTeX. To do so, Hugo embeds
[Wazero](https://wazero.io/), a WASM runtime for Go program, so that it can run
the WASM binary of [QuickJS](https://bellard.org/quickjs/), an embedded
JavaScript engine written in C, and uses that engine to run
[KaTeX](https://katex.org/), a LaTeX reimplementation in JavaScript for web.

To support Typst too, here we use Wazero to run the WASM binary of Typst, which
is written in Rust. Typst supports many output formats, including for web. This
project uses Typst to compile math into MathML.

## How to gain the Typst math

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

## Build

```bash
# Clone this repository.

# Ensure you have Go and Rust toolchains.
$ go version
go version go1.26.4
$ cargo version
cargo 1.96.0 (30a34c682 2026-05-25)

# Ensure you have Make.
$ make --version
GNU Make 3.81

# Build it.
make

# Find the resulting binary at the project root. Use it
# just like a normal Hugo binary.
./hugo-typst version
```

## License

MIT License, see [LICENSE.txt](./LICENSE.txt).

For license of Hugo and dependencies thereof, see the `License` section in
[README.hugo.md](./README.hugo.md).
