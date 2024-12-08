# v0.5.0

Version v0.5.0 is a major release, including breaking changes and substantial new features.

## Changes to existing CLI options
- The option to disable line wrapping has been changed from `--keep` to `--nowrap`.
- The option to set the number of characters used per indentation level has been changed from `--tab` to `--tabsize`.
- The option to set the maximum line length for wrapping has been changed from `--wrap` to `--wraplen`.
- See below for information on the new `--config`, `--noconfig`, `--man`, `--completion`, and `--args` flags.

## Configuration file support
Configuring tex-fmt can now be achieved using a configuration file as well as CLI arguments. The configuration file can be read from a user-specified path with `--config <config>`, from the current working directory, from the root of the current git repository, or from the user's configuration directory, in order of decreasing priority. Arguments passed on the command line will always override those specified in configuration files. Configuration files can be disabled by passing `--noconfig`.

## Man pages
Man pages can be generated using the `--man` flag. Pre-built man pages are also available for download from the GitHub repository.

## Shell completion
Completion files for popular shells, including bash, fish, zsh, elvish and PowerShell, can be generated using the `--completion <shell>` flag. Pre-built completion scripts are also available for download from the GitHub repository.

## Minor changes
- Arguments passed to tex-fmt can be inspected by passing `--args`
- Fixed bug with `\itemsep` matching the `\item` pattern
- Added last non-indented line number to "Indent did not return to zero" error messages
- Removed LTO optimization to improve compile time with minimal effect on run time
- If duplicate file names are provided, they are now removed before formatting
- Added LLF to the list of existing tools
- Changed order of options in help dialogs

# v0.4.7

- Fix bug with `--stdin` adding newlines at EOF
- Fix logic for ignoring verbatim environments
- Ensure sectioning commands begin on new lines
- Various performance improvements
- Add NEWS.md for release notes
- Ensure all test files successfully compile to PDFs
- Better documentation of options in README.md

# v0.4.6

- Added `--wrap` flag to choose line length for wrapping
- Significant changes to central formatting logic to reduce memory allocations
- Treat comment environments as verbatim
- Improved performance with finding comments in source code

# v0.4.5

- Added `--usetabs` to use tabs instead of spaces for indentation
- Fixed a bug with unicode graphemes and comment handling
- Main function now returns `std::process::ExitCode` for a cleaner exit
- Reduced memory allocation in comment handling logic
- Reduced memory allocation when indenting lines
- Caching of pattern matches reduces number of regex searches

# v0.4.4

- Added `--tab` flag for variable tab size [default: 2]
- Fixed bug with incorrect line numbers being printed
- Fixed bug with quadratic complexity of comment checking
- Added Arch User Repository support
- Added VS Code support
- Improved performance by moving environment checking inside main loop
- Upgraded Cargo dependencies
- Included LTO optimization on the release build

# v0.4.3

- Switch output text coloring to the `colored` crate.
- Add `--stdin` flag to read input from stdin (and output to stdout).

# v0.4.2

- Added `--quiet` flag to suppress warning messages
- Allow `tex-fmt main` for `tex-fmt main.tex`
- Internal documentation
- Improved performance
- Added more Clippy lints

# v0.4.1

- Added binary archives to GitHub release

# v0.4.0

## Breaking change
The logic for line wrapping has been changed. Previously, for lines longer than
80 characters, we would break the line at suitable points into chunks of no
more than 80 characters. Then another round of indenting was applied, and this
would often push the length back over 80 characters. A subsequent round of
wrapping was therefore required, and often led to the creation of very short
lines (#6).

The new approach is to take lines longer than 80 characters and remove the
first segment up to 70 characters, pushing the resulting two lines back onto
the queue. When indenting is then reapplied, the lines typically do not go over
80 characters unless the indentation is very deep. However, some lines may now
be truncated to 70 characters rather than 80.

## Other updates

- Added a `--keep` flag to disable line wrapping (#10)
- Improved the central algorithm to avoid multiple passes and improve run-time
  performance (#7)
- Only write the file to disk if the formatting returns a different string, to
  avoid unnecessary editing of modification times

# v0.3.1

- Updated README
- Added project logo

# v0.3.0

- Added a `--check` flag to check if file is correctly formatted
- Fixed bug with line wrapping giving up early
- Shell scripts verified with shellcheck
- Refactored variable names
- Some performance improvements

# v0.2.2

Bump version number

# v0.2.1

Bump version number

# v0.2.0

Bump version number
