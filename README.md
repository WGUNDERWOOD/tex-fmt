# tex-fmt <img src="https://github.com/WGUNDERWOOD/tex-fmt/raw/main/extra/logo.svg" alt="tex-fmt" align="right" width=160 />

[![CI](
https://github.com/wgunderwood/tex-fmt/actions/workflows/ci.yml/badge.svg)](
https://github.com/wgunderwood/tex-fmt/actions/workflows/ci.yml)
[![crates.io](
https://img.shields.io/crates/v/tex-fmt?logo=rust)](
https://crates.io/crates/tex-fmt)
[![Packaging status](
https://repology.org/badge/tiny-repos/tex-fmt.svg)](
https://repology.org/project/tex-fmt/versions)
[![license: MIT](
https://shields.io/badge/license-MIT-blue.svg)](
https://mit-license.org/)

An extremely fast LaTeX formatter written in Rust.
Try it out now in your
[browser](https://wgunderwood.github.io/tex-fmt/)!

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

Install the [stable release](https://crates.io/crates/tex-fmt) with

``` shell
cargo install tex-fmt
```

Install from [GitHub](https://github.com/WGUNDERWOOD/tex-fmt) with

```shell
cargo install --git "https://github.com/wgunderwood/tex-fmt"
```

### Nix

Install from
[nixpkgs](
https://search.nixos.org/packages?channel=unstable&query=tex-fmt)
into a temporary shell with

``` shell
nix-shell -p tex-fmt
```

Build from source using flakes with

``` shell
nix build "github:wgunderwood/tex-fmt"
```

Add to your NixOS installation with

```nix
environment.systemPackages = [
  pkgs.tex-fmt
];
```

It is also included in
[treefmt-nix](https://github.com/numtide/treefmt-nix/tree/main).

### Arch Linux

Install from the
[Arch User Repository](https://aur.archlinux.org/packages/tex-fmt).
For example, using the [yay](https://github.com/Jguer/yay) AUR helper:

``` shell
yay -S tex-fmt
```

### Homebrew

Install using
[Homebrew](https://formulae.brew.sh/formula/tex-fmt) with

```shell
brew install tex-fmt
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

The most commonly used options are given below.
For a full list, see the
[options](
https://github.com/WGUNDERWOOD/tex-fmt?tab=readme-ov-file#options)
section below.

``` shell
tex-fmt file.tex             # format file.tex and overwrite
tex-fmt --check file.tex     # check if file.tex is correctly formatted
tex-fmt --print file.tex     # format file.tex and print to stdout
tex-fmt --nowrap file.tex    # do not wrap long lines
tex-fmt --stdin              # read from stdin and print to stdout
tex-fmt --help               # view help information
```

### Configuration

Options can also be read from a configuration file, which
will be read from the following locations, in order of decreasing priority.

- A named config file passed as `tex-fmt --config <config>`
- A file named `tex-fmt.toml` in the current working directory
- A file named `tex-fmt.toml` in the root directory of the current git repository
- A file named `tex-fmt.toml` in a subdirectory titled `tex-fmt/`
  in the user's configuration directory
    - Linux: `~/.config/tex-fmt/tex-fmt.toml`
    - macOS: `/Users/<user>/Library/Application Support/tex-fmt/tex-fmt.toml`
    - Windows: `C:\Users\<user>\AppData\Roaming\tex-fmt\tex-fmt.toml`

Arguments passed on the command line will always override those
specified in configuration files. An example configuration file
is available at
[tex-fmt.toml](https://github.com/WGUNDERWOOD/tex-fmt/blob/main/tex-fmt.toml).
To ignore all config files, use the `--noconfig` flag.

Note for contributors: this repository's configuration file will be
automatically applied if tex-fmt is run from within the repository.
Use `--noconfig` or `--config <config>` to avoid this.

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

### Shell completion

Shell completion scripts can be generated at run-time using the
`--completion <shell>` flag. See the
[completion](
https://github.com/WGUNDERWOOD/tex-fmt/tree/main/completion)
directory for more details.

### Man page

A man page can be generated at run-time using the
`--man` flag. See the
[man](https://github.com/WGUNDERWOOD/tex-fmt/tree/main/man)
directory for more details.

### Run before every commit

You can format your LaTeX files before every commit using
[pre-commit](http://pre-commit.com) with the following `.pre-commit-config.yaml`
in your repository root:

```yaml
repos:
  - repo: https://github.com/WGUNDERWOOD/tex-fmt
    rev: v0.5.3
    hooks:
      - id: tex-fmt
```

For more on how to use pre-commit check out their
[quick start guide](https://pre-commit.com/#quick-start)!

## Performance

When formatting all of the test cases,
tex-fmt is over a thousand times faster than latexindent.

| **Files** | **Lines** | **Size** | **tex-fmt** | **latexindent** | **latexindent -m** |
| --- | --- | --- | --- | --- | --- |
| 51 | 94k | 3.5M | **0.055s** | 106s [x1927] | 127s [x2309] |

## Contribution

Please feel free to open an issue or submit a pull request,
including as much information as you can. Documentation of internals
can be accessed by cloning this repository and running `cargo doc`.

Alternatively, you can
[Buy Me a Coffee](https://buymeacoffee.com/wgunderwood)!

## Limitations

- Semantic parsing of LaTeX code not conducted
- No linting or correction of syntax errors
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

- [LLF](https://repo.or.cz/llf.git).
Lua script, many configuration options

## Options

The following command-line options are offered by tex-fmt.

| Option         | Alias | Default | Description |
| -------------- | ----- | ------- | --- |
| `--check`      | `-c`  |         | Check formatting, do not modify files |
| `--print`      | `-p`  |         | Print to stdout, do not modify files |
| `--nowrap`     | `-n`  |         | Do not wrap long lines |
| `--wraplen`    | `-l`  | `80`    | Line length for wrapping |
| `--tabsize`    | `-t`  | `2`     | Number of characters to use as tab size |
| `--usetabs`    |       |         | Use tabs instead of spaces for indentation |
| `--stdin`      | `-s`  |         | Process stdin as a single file, output to stdout |
| `--config`     |       |         | Path to config file |
| `--noconfig`   |       |         | Do not read any config file |
| `--lists`      |       |         | Extra list environments to be formatted as `itemize` |
| `--verbose`    | `-v`  |         | Show info messages |
| `--quiet`      | `-q`  |         | Hide warning messages |
| `--trace`      |       |         | Show trace messages |
| `--completion` |       |         | Generate a shell completion script |
| `--man`        |       |         | Generate a man page |
| `--args`       |       |         | View arguments passed to tex-fmt |
| `--help`       | `-h`  |         | Print help |
| `--version`    | `-V`  |         | Print version |
