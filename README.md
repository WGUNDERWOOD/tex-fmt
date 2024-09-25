# tex-fmt <img src="https://github.com/WGUNDERWOOD/tex-fmt/raw/main/extra/logo.svg" alt="tex-fmt" align="right" width=160 />

[![CI](
https://github.com/wgunderwood/tex-fmt/actions/workflows/ci.yml/badge.svg)](
https://github.com/wgunderwood/tex-fmt/actions/workflows/ci.yml)
[![crates.io](
https://img.shields.io/crates/v/tex-fmt?logo=rust)](
https://crates.io/crates/tex-fmt)
[![AUR Version](
https://img.shields.io/aur/version/tex-fmt?logo=archlinux&label=AUR)](
https://aur.archlinux.org/packages/tex-fmt)
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

If you would like tex-fmt to appear in
[nixpkgs](https://github.com/NixOS/nixpkgs),
please add a 👍 reaction to the
[pull request](https://github.com/NixOS/nixpkgs/pull/341818).
Alternatively, to build from source, use

``` shell
nix build "github:wgunderwood/tex-fmt"
```

### Arch Linux

Install from the
[Arch User Repository](https://aur.archlinux.org/packages/tex-fmt).
For example, using the [yay](https://github.com/Jguer/yay) AUR helper:

``` shell
yay -S tex-fmt
```

### Binary download

Binaries for various platforms are available on the GitHub
[releases](https://github.com/WGUNDERWOOD/tex-fmt/releases) page.

### Visual Studio Code
Integration with VS Code is provided by the
[LaTeX Workshop](https://github.com/James-Yu/LaTeX-Workshop)
extension. You will need to first install tex-fmt
through one of the above methods.

## Usage
``` shell
tex-fmt file.tex             # format file.tex and overwrite
tex-fmt --check file.tex     # check if file.tex is correctly formatted
tex-fmt --print file.tex     # format file.tex and print to STDOUT
tex-fmt --keep file.tex      # do not wrap long lines
tex-fmt --stdin              # read from STDIN and print to STDOUT
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
| 49 | 94k | 3.5M | **0.085s** | 99s [x1169] | 130s [x1541] |

## Contribution

Please feel free to open an issue or submit a pull request,
including as much information as you can. Documentation of internals
can be accessed by cloning this repository and running `cargo doc`.

Alternatively, you can
[Buy Me a Coffee](https://buymeacoffee.com/wgunderwood)!

## Limitations

- Semantic parsing of LaTeX code not conducted
- No linting or correction of syntax errors
- Customization via configuration files not supported
- Compliance with existing formatting guidelines not guaranteed
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
