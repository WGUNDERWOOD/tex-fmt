{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  buildInputs = with pkgs; [
    rustfmt
    clippy
    cargo-flamegraph
    cacert
    hyperfine
    texlive.combined.scheme-full
    diff-so-fancy
    poppler_utils
    shellcheck
  ];
}
