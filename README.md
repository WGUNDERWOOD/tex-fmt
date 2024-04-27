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

## Example

Before formatting `example.tex`:

``` tex
\documentclass{article}

\begin{document}

\begin{itemize}
\item Lists with items
over multiple lines
\end{itemize}

\begin{align}
E = m c^2
\end{align}

\end{document}
```

After running `tex-fmt example.tex`

``` tex
\documentclass{article}

\begin{document}

\begin{itemize}
  \item Lists with items
    over multiple lines
\end{itemize}

\begin{align}
  E = m c^2
\end{align}

\end{document}
```

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
- No configuration necessary

It does *not* currently aim to provide the following:

- Semantic parsing of LaTeX code
- Linting or correction of syntax errors
- Customization via configuration files
- Compliance with existing formatting guidelines
- Editor integration
- Spell checking

## Performance

Run `perf.sh` to format all test cases in the `tests/` directory.

| **Files** | **Lines** | **Size** | **tex-fmt** | **latexindent** | **latexindent -m** |
| --- | --- | --- | --- | --- | --- |
| 6 | 30k | 1M | **0.0362s** | 11.6s [x322] | 15.3s [x424] |

## Comparison with existing tools

### [latexindent](https://github.com/cmhughes/latexindent.pl)
[Perl](https://www.perl.org/) script,
many configuration options,
slow on large files.

### [LaTeX\_Tidy](http://bfc.sfsu.edu/cgi-bin/hsu.pl?LaTeX_Tidy)
[Perl](https://www.perl.org/) script,
download links seem to be broken.

### [latex-pretty](https://c.albert-thompson.com/latex-pretty/)
Browser-based, uses
[latexindent](https://github.com/cmhughes/latexindent.pl)
as the backend.

### [latexformat.com](https://latexformat.com/)
Browser-based.

### [texpretty](http://ftp.math.utah.edu/pub/texpretty/)
[C](https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html)
program, which works sometimes and appears to be fast.
However, it fails with certain keywords inside brackets.

### [latex-editor](https://latex-editor.pages.dev/formatter/)
Browser-based.

### [LaTeXFmt](https://github.com/engeljh/vim-latexfmt)
[Vim](https://www.vim.org/)
plugin, does not apply indentation.

### [latex-formatter](https://github.com/nfode/latex-formatter)
[Visual Studio](https://visualstudio.microsoft.com/)
plugin, uses
[latexindent](https://github.com/cmhughes/latexindent.pl)
as the backend.
