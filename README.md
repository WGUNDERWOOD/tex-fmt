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
LaTeX source code files with the following properties.

- Handling of common file types: `.tex`, `.bib`, `.cls`, `.sty`
- Very good run-time performance
- Basic configuration options
- Sensible defaults

It does *not* currently aim to provide the following:

- Semantic parsing of LaTeX code
- Linting or correction of syntax errors
- Compliance with any existing formatting guidelines
- Editor integration
- Spell checking
