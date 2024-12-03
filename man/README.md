# Man page generation for tex-fmt

A man page can be generated at run-time using the
`--man` flag, as follows.

```shell
mkdir -p man/man1
tex-fmt --man > man/man1/tex-fmt.1
MANPATH="$PWD/man" man tex-fmt
```

It is also available for download in
[this directory](
https://github.com/WGUNDERWOOD/tex-fmt/tree/main/man/),
but may not be up-to-date with your tex-fmt installation.
