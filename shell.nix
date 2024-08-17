{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  buildInputs = let
    python = pkgs.python3.withPackages (ps:
      with ps; [
        matplotlib
        pillow
      ]);
  in [
    pkgs.cacert
    pkgs.cargo-flamegraph
    pkgs.cargo-edit
    pkgs.clippy
    pkgs.diff-so-fancy
    pkgs.hyperfine
    pkgs.poppler_utils
    python
    pkgs.python312Packages.grip
    pkgs.rustfmt
    pkgs.shellcheck
    pkgs.texlive.combined.scheme-full
  ];
}
