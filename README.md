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

### [latexindent.pl](https://github.com/cmhughes/latexindent.pl)
- Perl script
- TODO

### [LaTeX\_Tidy](
https://drerichsu.github.io/drerichsu-homepage/archive/hsu.pl@LaTeX_Tidy.html)
- Perl script
- Download links seem to be broken

### [latex-pretty](https://c.albert-thompson.com/latex-pretty/)
- Browser-based
- Uses latexindent.pl as the backend

### [latexformat.com](https://latexformat.com/)
- Browser-based

### [texpretty](http://ftp.math.utah.edu/pub/texpretty/)
- C program
- This works sometimes and appears to be fast
- However it fails with certain keywords inside brackets

### [latex-editor](https://latex-editor.pages.dev/formatter/)
- Browser-based only

### [vim-latexfmt](https://github.com/engeljh/vim-latexfmt)
- Vim plugin
- Does not apply indentation

### [latex-formatter](https://github.com/nfode/latex-formatter)
- Visual Studio plugin
- Uses latexindent.pl as the backend
