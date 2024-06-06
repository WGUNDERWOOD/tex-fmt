{
  description = "A LaTeX formatter written in Rust";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        packages = {
          default = pkgs.callPackage ./default.nix {inherit pkgs;};
        };
        devShells = {
          default = pkgs.callPackage ./shell.nix {inherit pkgs;};
        };
      }
    );
}
