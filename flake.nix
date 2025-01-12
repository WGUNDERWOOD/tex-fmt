{
  description = "LaTeX formatter written in Rust";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
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
    )
    // {
      overlays.default = import ./overlay.nix;
    };
}
