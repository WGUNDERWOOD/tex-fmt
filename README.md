# tex-fmt <img src="https://github.com/WGUNDERWOOD/tex-fmt/raw/main/extra/logo.svg" alt="tex-fmt" align="right" width=160 />

[![CI](
https://github.com/wgunderwood/tex-fmt/actions/workflows/ci.yml/badge.svg)](
https://github.com/wgunderwood/tex-fmt/actions/workflows/ci.yml)
[![crates.io](
https://img.shields.io/crates/v/tex-fmt?logo=rust)](
https://crates.io/crates/tex-fmt)
[![license: MIT](
https://shields.io/badge/license-MIT-blue.svg)](
https://mit-license.org/)

An extremely fast LaTeX formatter written in Rust.

<table width="100%">
<tr>
<td>
<b>Input</b>
</td>
<td>
<b>Output</b>
</td>
</tr>
<tr>
<td>

``` tex
\documentclass{article}

\begin{document}

\begin{itemize}
\item Lists with items
over multiple lines
\end{itemize}

\begin{equation}
E = m c^2
\end{equation}

\end{document}
```
</td>
<td>

``` tex
\documentclass{article}

\begin{document}

\begin{itemize}
  \item Lists with items
    over multiple lines
\end{itemize}

\begin{equation}
  E = m c^2
\end{equation}

\end{document}
```
</td>
</tr>
</table>

- ⚡&nbsp; Extremely fast run-time performance
- 🔧&nbsp; Minimal configuration required
- 📟&nbsp; Command-line interface
- 📜&nbsp; Handles LaTeX file types `.tex`, `.bib`, `.cls`, and `.sty`
- 🦀&nbsp; Written entirely in safe Rust

## Installation

### Cargo

``` shell
# install stable release
cargo install tex-fmt

# install from github
cargo install --git "https://github.com/wgunderwood/tex-fmt"
```

### Nix

``` shell
nix build "github:wgunderwood/tex-fmt"
```

### Binary download

Binaries for various platforms are available on the GitHub
[releases](https://github.com/WGUNDERWOOD/tex-fmt/releases) page.

## Usage
``` shell
tex-fmt file.tex             # format file.tex and overwrite
tex-fmt --check file.tex     # check if file.tex is correctly formatted
tex-fmt --print file.tex     # format file.tex and print to STDOUT
tex-fmt --keep file.tex      # keep lines, do not wrap
tex-fmt --help               # view help information
```

### Disabling the formatter

Ending a source line with `% tex-fmt: skip` disables formatting for that line.
To disable the formatter for a block, use `% tex-fmt: off` and `% tex-fmt: on`.

``` tex
\documentclass{article}

\begin{document}

    This line is skipped % tex-fmt: skip

% tex-fmt: off
  These lines are also
    not formatted or wrapped
% tex-fmt: on

\end{document}
```

Verbatim environments including `verbatim`, `Verbatim`, `lstlisting`
and `minted` are automatically skipped.

## Performance

When formatting all of the test cases,
tex-fmt is over a thousand times faster than latexindent.

| **Files** | **Lines** | **Size** | **tex-fmt** | **latexindent** | **latexindent -m** |
| --- | --- | --- | --- | --- | --- |
| 49 | 94k | 3.5M | **0.096s** | 97s [x1001] | 125s [x1288] |

## Contribution

Please feel free to open an issue or submit a pull request,
including as much information as you can. Documentation of internals
can be accessed by cloning this repository and running `cargo doc`.

## Limitations

- Semantic parsing of LaTeX code not conducted
- No linting or correction of syntax errors
- Customization via configuration files not supported
- Compliance with existing formatting guidelines not guaranteed
- Editor integration not currently provided
- No spelling or grammar checking

## Existing tools

- [latexindent](https://github.com/cmhughes/latexindent.pl).
Perl script, many configuration options, slow on large files

- [LaTeXTidy](http://bfc.sfsu.edu/cgi-bin/hsu.pl?LaTeX_Tidy).
Perl script, download links seem to be broken

- [latex-pretty](https://c.albert-thompson.com/latex-pretty/).
Browser-based, uses latexindent as the backend

- [latexformat.com](https://latexformat.com/).
Browser-based

- [texpretty](http://ftp.math.utah.edu/pub/texpretty/).
C program which works sometimes and appears to be fast

- [latex-editor](https://latex-editor.pages.dev/formatter/).
Browser-based

- [LaTeXFmt](https://github.com/engeljh/vim-latexfmt).
Vim plugin, does not apply indentation

- [latex-formatter](https://github.com/nfode/latex-formatter).
Visual Studio plugin, uses latexindent as the backend
