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
- 📟&nbsp; Command line interface
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

### Debian

Install from the [Debian archive](https://www.debian.org/distrib/packages)
(trixie and later):

``` shell
apt install tex-fmt
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

### Neovim

A package for Neovim is provided by
[mason.nvim](https://github.com/williamboman/mason.nvim).

### GitHub Action

The [tex-fmt-action](https://github.com/grayespinoza/tex-fmt-action) can install and run tex-fmt.

## Usage

The most commonly used options are given below.
For a full list, see the
[options](
https://github.com/WGUNDERWOOD/tex-fmt?tab=readme-ov-file#options)
section below.

``` shell
tex-fmt file.tex                   # format file.tex and overwrite
tex-fmt --check file.tex           # check if file.tex is correctly formatted
tex-fmt --print file.tex           # format file.tex and print to stdout
tex-fmt --recursive                # recursively format files in current directory
tex-fmt --recursive dir/           # recursively format files in dir
tex-fmt --fail-on-change file.tex  # format file.tex and return exit-code 1 if overwritten
tex-fmt --nowrap file.tex          # do not wrap long lines
tex-fmt --stdin                    # read from stdin and print to stdout
tex-fmt --help                     # view help information
```

### Configuration

Options can also be read from a configuration file, which
will be read from the following locations, in order of decreasing priority.

- A named config file passed as `tex-fmt --config <PATH>`
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
Use `--noconfig` or `--config <PATH>` to avoid this.

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

### Ignoring files in recursive mode

Recursive searches with `--recursive` or `-r` will
ignore patterns in `.gitignore` and `.ignore` files,
following git conventions.

### Shell completion

Shell completion scripts can be generated at run-time using the
`--completion <SHELL>` flag. See the
[completion](
https://github.com/WGUNDERWOOD/tex-fmt/tree/main/completion)
directory for more details.

### Man page

A man page can be generated at run-time using the
`--man` flag. See the
[man](https://github.com/WGUNDERWOOD/tex-fmt/tree/main/man)
directory for more details.

### Pre-commit hook

tex-fmt can be run before every git commit using
[pre-commit](http://pre-commit.com) with the following
`.pre-commit-config.yaml` in your repository root:

```yaml
repos:
  - repo: https://github.com/WGUNDERWOOD/tex-fmt
    rev: v0.5.6
    hooks:
      - id: tex-fmt
```

To prevent the pre-commit hook from modifying your files, add:

```yaml
      - id: tex-fmt
        args: [--check]
```

## Performance

When formatting all of the test cases,
tex-fmt is over a thousand times faster than latexindent.

| **Files** | **Lines** | **Size** | **tex-fmt** | **latexindent** | **latexindent -m** |
| --- | --- | --- | --- | --- | --- |
| 51 | 94k | 3.5M | **0.055s** | 106s [x1927] | 127s [x2309] |

## Contribution

Please feel free to open an issue or submit a pull request,
including as much information as you can. Documentation of internals
can be accessed by cloning this repository and running `cargo doc`,
or by visiting the [docs.rs](https://docs.rs/tex-fmt/latest/tex_fmt/) page.

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

- [bibtex-tidy](https://github.com/FlamingTempura/bibtex-tidy).
JavaScript program, specifically for BibTeX files

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

### Command line options

The following arguments can be passed on the command line.

| Option                 | Alias | Default | Description |
| ---------------------- | ----- | ------- | --- |
| `--check`              | `-c`  |         | Check formatting, do not modify files |
| `--print`              | `-p`  |         | Print to stdout, do not modify files |
| `--fail-on-change`     | `-f`  |         | Fail if files are modified |
| `--recursive`          | `-r`  |         | Recursively search for files to format |
| `--nowrap`             | `-n`  |         | Do not wrap long lines |
| `--wraplen <N>`        | `-l`  | `80`    | Line length for wrapping |
| `--tabsize <N>`        | `-t`  | `2`     | Number of characters to use as tab size |
| `--usetabs`            |       |         | Use tabs instead of spaces for indentation |
| `--stdin`              | `-s`  |         | Process stdin as a single file, output to stdout |
| `--config <PATH>`      |       |         | Path to config file |
| `--noconfig`           |       |         | Do not read any config file |
| `--verbose`            | `-v`  |         | Show info messages |
| `--quiet`              | `-q`  |         | Hide warning messages |
| `--trace`              |       |         | Show trace messages |
| `--completion <SHELL>` |       |         | Generate a shell completion script |
| `--man`                |       |         | Generate a man page |
| `--args`               |       |         | View arguments passed to tex-fmt |
| `--help`               | `-h`  |         | Print help |
| `--version`            | `-V`  |         | Print version |

### Configuration file options

The following arguments can be provided in `tex-fmt.toml`.
The first example in each row is the default value.

| Option           | Type     | Examples               | Description |
| ---------------- | -------- | ---------------------- | --- |
| `check`          | bool     | `false`                | Check formatting, do not modify files |
| `print`          | bool     | `false`                | Print to stdout, do not modify files |
| `fail-on-change` | bool     | `false`                | Fail if files are modified |
| `wrap`           | bool     | `true`                 | Wrap long lines |
| `wraplen`        | int      | `80`, `100`            | Line length for wrapping |
| `wrapmin`        | int      | `70`, `90`             | Target minimum length for line wrapping |
| `tabsize`        | int      | `2`, `4`               | Number of characters to use as tab size |
| `tabchar`        | str      | `"space"`, `"tab"`     | Character to use for indentation |
| `stdin`          | bool     | `false`                | Process stdin as a single file, output to stdout |
| `lists`          | arr[str] | `[]`, `["myitemize"]`  | Extra list environments to be formatted as `itemize` |
| `verbatims`      | arr[str] | `[]`, `["myverbatim"]` | Extra verbatim environments |
| `no-indent-envs` | arr[str] | `[]`, `["mydocument"]` | Environments which are not indented |
| `wrap-chars`     | arr[str] | `[]`, `["。"]`         | Characters after which lines may be wrapped |
| `verbosity`      | str      | `"warn"`, `"error"`    | Verbosity level for terminal logging |
