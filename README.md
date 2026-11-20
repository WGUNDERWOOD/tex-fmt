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
Indentation, line wrapping and other formatting for LaTeX source code.
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

### CTAN

A package named
[latex-formatter](https://ctan.org/pkg/latex-formatter)
is on CTAN.

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
tex-fmt --sentence-lines file.tex  # put each prose sentence on its own line
tex-fmt --format-tables file.tex   # format tables (align ampersands)
tex-fmt --stdin                    # read from stdin and print to stdout
tex-fmt --help                     # view help information
```

### Configuration

Options can also be set using a configuration file, which
will be read from the following locations, in order of decreasing priority.

- A named config file passed as `tex-fmt --config <PATH>`
- A file named `tex-fmt.toml` in the current working directory
- A file named `tex-fmt.toml` in the root directory of the current git repository
- A file named `tex-fmt.toml` in a subdirectory titled `tex-fmt/`
  in the user's configuration directory
    - Linux: `~/.config/tex-fmt/tex-fmt.toml`
    - macOS: `/Users/<USER>/Library/Application Support/tex-fmt/tex-fmt.toml`
    - Windows: `C:\Users\<USER>\AppData\Roaming\tex-fmt\tex-fmt.toml`

Arguments passed on the command line will always override those
specified in configuration files. An example configuration file
is available at
[tex-fmt.toml](https://github.com/WGUNDERWOOD/tex-fmt/blob/main/tex-fmt.toml).
To ignore all config files, use the `--noconfig` flag.

Note for contributors: this repository's configuration file will be
automatically applied if tex-fmt is run from within the repository.
Use `--noconfig` or `--config <PATH>` to avoid this.

### One sentence per line

Use `--sentence-lines`, or set `sentence-lines = true` in `tex-fmt.toml`, to
join wrapped prose in `.tex` files or on standard input. Long sentences stay on
one line regardless of the configured wrap width. Other file types use their
normal formatting and wrapping settings.

Blank lines separate paragraphs. Only ASCII `.`, `!`, and `?` mark sentence
ends, followed by whitespace or the end of a run, including after closing
quotes and parentheses. Sentence mode joins wrapped lines with a space and is
therefore not recommended for prose that uses other terminators such as `。`.
English-only abbreviation heuristics keep `Dr. Smith`, `Figs. 2 and 3`, and
`no. 12` together. Initials, dotted acronyms, and degrees such as `Ph.D.` stay
with the following word, including in `B. R. Coles` and `p.m. Tomorrow`. A
sentence ending in a single capital letter, such as `vitamin C.` or `so do I.`,
joins the next sentence because tex-fmt cannot distinguish the final letter
from an initial. A sentence ending in `no.` joins the next sentence when it
begins with a number because tex-fmt reads `no.` as an abbreviation for
“number”. After `etc.` or `al.`, a capitalized next word starts a sentence.
Ordinary sentences can start with lowercase letters or numbers.

Use `sentence-abbreviations` for additional unconditional no-break exceptions.
Entries are case-insensitive and accept either `"Dept"` or `"Dept."`.

```toml
sentence-lines = true
sentence-abbreviations = ["Dept", "Approx."]
sentence-skip-commands = ["mydirective"]
sentence-environments = ["frame", "minipage", "mytheorem"]
```

#### Custom commands and environments

Macros are never expanded. Words outside command arguments identify prose. A
command followed by prose can start a run. A line containing only a command can
join unfinished prose when the following line contains prose in the same
paragraph. This also means a solitary layout command such as `\vspace`,
`\caption`, `\label`, `\includegraphics`, or `\centering` between two parts of
an unfinished sentence can join them. List such commands in
`sentence-skip-commands` to keep them on their own lines. Paragraph, sectioning,
and definition commands always bound runs. Structural commands split onto new
lines only at existing whitespace, so packed commands stay together.

Punctuation inside braces, balanced optional arguments, and maths is protected.
An unmatched `[` after a control word is treated as prose. Lines containing
comments, inline code, explicit line breaks, or multiline groups or maths are
protected from sentence reflow. Verbatim and ignored blocks remain protected.
Environments declared in the file with `\newtheorem`, `\newtheorem*`, or
`\declaretheorem` are recognized as prose. Use `sentence-environments` for
additional environments with prose bodies. Bodies declared with
`\newenvironment` need this setting to opt into reflow. For an item-like macro
whose items have no final punctuation, add its name to `sentence-skip-commands`
to keep those lines separate. A literal TOML entry such as
`sentence-abbreviations = ['\etal']` keeps a period immediately after that
control word with the following prose.

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
    rev: v0.5.7
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

Alternatively, you can become a
[GitHub sponsor](https://github.com/sponsors/WGUNDERWOOD)
or
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

- [Badness](https://badness.dev). 
Rust-based, opinionated, also a language server and linter

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
| `--sentence-lines`     |       |         | Put each prose sentence on its own line |
| `--wraplen <N>`        | `-l`  | `80`    | Line length for wrapping |
| `--tabsize <N>`        | `-t`  | `2`     | Number of characters to use as tab size |
| `--usetabs`            |       |         | Use tabs instead of spaces for indentation |
| `--format-tables`      |       |         | Format tables |
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
| `sentence-lines`         | bool     | `false`                | Put each prose sentence on its own line |
| `sentence-abbreviations` | arr[str] | `[]`, `["Dept."]`      | Additional abbreviations which do not end sentences |
| `sentence-skip-commands` | arr[str] | `[]`, `["mydirective"]` | Additional commands which bound sentence reflow |
| `sentence-environments`  | arr[str] | `[]`, `["mytheorem"]`  | Additional environments with prose bodies |
| `wraplen`        | int      | `80`, `100`            | Line length for wrapping |
| `wrapmin`        | int      | `70`, `90`             | Target minimum length for line wrapping |
| `tabsize`        | int      | `2`, `4`               | Number of characters to use as tab size |
| `tabchar`        | str      | `"space"`, `"tab"`     | Character to use for indentation |
| `format-tables`  | bool     | `false`                | Format tables |
| `stdin`          | bool     | `false`                | Process stdin as a single file, output to stdout |
| `lists`          | arr[str] | `[]`, `["myitemize"]`  | Extra list environments to be formatted as `itemize` |
| `verbatims`      | arr[str] | `[]`, `["myverbatim"]` | Extra verbatim environments |
| `no-indent-envs` | arr[str] | `[]`, `["mydocument"]` | Environments which are not indented |
| `wrap-chars`     | arr[str] | `[]`, `["。"]`         | Characters after which lines may be wrapped |
| `verbosity`      | str      | `"warn"`, `"error"`    | Verbosity level for terminal logging |
