{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  buildInputs = let
    python = pkgs.python3.withPackages (ps:
      with ps; [
        grip
        matplotlib
        pillow
      ]);
  in [
    pkgs.alejandra
    pkgs.bacon
    pkgs.cacert
    pkgs.cargo-edit
    pkgs.cargo-flamegraph
    pkgs.cargo-shear
    pkgs.clippy
    pkgs.diff-so-fancy
    pkgs.gh
    pkgs.hyperfine
    pkgs.lld
    pkgs.poppler_utils
    pkgs.ripgrep
    pkgs.rustfmt
    pkgs.shellcheck
    pkgs.texlive.combined.scheme-full
    pkgs.wasm-bindgen-cli
    python
  ];
}
