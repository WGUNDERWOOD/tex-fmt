# Shell completion for tex-fmt

Shell completion scripts can be generated at run-time using the
`--completion <shell>` flag, as detailed below. Completion scripts
generated at compile-time are also available for download in
[this directory](
https://github.com/WGUNDERWOOD/tex-fmt/tree/main/completion/),
but they may not be up-to-date with your tex-fmt installation.

For **bash**:

```shell
dir="$XDG_CONFIG_HOME/bash_completion"
mkdir -p "$dir"
tex-fmt --completion bash > "$dir/tex-fmt.bash"
```

For **fish**:

```shell
dir="$XDG_CONFIG_HOME/fish/completions"
mkdir -p "$dir"
tex-fmt --completion fish > "$dir/tex-fmt.fish"
```

For **zsh**:

```shell
dir="$HOME/.zsh-complete"
mkdir -p "$dir"
tex-fmt --completion zsh > "$dir/_tex-fmt"
```

For **elvish**:

```shell
dir="$HOME/.elvish/lib"
mkdir -p "$dir"
tex-fmt --completion elvish > "$dir/tex-fmt.elv"
use tex-fmt
```

For **PowerShell**, create the completions:

```shell
tex-fmt --completion powershell > _tex-fmt.ps1
```

Then add `. _tex-fmt.ps1` to your PowerShell profile.
If the `_tex-fmt.ps1` file is not on your `PATH`, do
`. /path/to/_tex-fmt.ps1` instead.
