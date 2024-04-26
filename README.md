# tex-fmt

[![CI](
https://github.com/wgunderwood/tex-fmt/actions/workflows/flake.yml/badge.svg)](
https://github.com/wgunderwood/tex-fmt/actions/workflows/flake.yml)
[![license: MIT](
https://shields.io/badge/license-MIT-blue.svg)](
https://mit-license.org/)

A [LaTeX](https://www.latex-project.org/)
formatter written in
[Rust](https://www.rust-lang.org/)

## Installation

### Nix

```
nix build "github:wgunderwood/tex-fmt"
```

### Cargo

```
cargo install --git "https://github.com/wgunderwood/tex-fmt"
```
## Aims

This project aims to provide a command-line tool for formatting
LaTeX source code files with the following properties:

- Handling of the common LaTeX file types `.tex`, `.bib`, `.cls`, and `.sty`
- Very good run-time performance
- Basic configuration options
- Sensible defaults

It does *not* currently aim to provide the following:

- Semantic parsing of LaTeX code
- Linting or correction of syntax errors
- Compliance with any existing formatting guidelines
- Editor integration
- Spell checking

## Performance

## Comparison with existing tools

### latexindent.pl
- Perl script
- Available on [GitHub](https://github.com/cmhughes/latexindent.pl)
- TODO

### LaTeX\_Tidy
- Perl script
- Available on Eric Hsu's [website](
https://drerichsu.github.io/drerichsu-homepage/archive/hsu.pl@LaTeX_Tidy.html)
- Link seems broken both here and on the older
[website](http://bfc.sfsu.edu/cgi-bin/hsu.pl?LaTeX_Tidy)

### latex-pretty
- Uses latexindent.pl as the backend
- Available on Albert Thompson's [website](
https://c.albert-thompson.com/latex-pretty/)
- Browser-based only

### latexformat.com
- Also made by Albert Thompson
- Available [online](https://latexformat.com/)
- Browser-based only

### texpretty
- Made by Nelson Beebe
- C program
- Available [online](http://ftp.math.utah.edu/pub/texpretty/)
- This works sometimes and appears to be fast
- However it often fails with certain keywords inside brackets

### latex-editor
- Available [online](https://latex-editor.pages.dev/formatter/)
- Browser-based only

### vim-latexfmt
- Available on [GitHub](https://github.com/engeljh/vim-latexfmt)
- Does not apply indentation
- Vim plugin only

### latex-formatter
- Available on [GitHub](https://github.com/nfode/latex-formatter)
- Visual Studio plugin only
- Uses latexindent.pl as the backend
