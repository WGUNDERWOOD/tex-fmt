{
  description = "tex-fmt";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    with flake-utils.lib;
      eachSystem allSystems (
        system: let
          pkgs = nixpkgs.legacyPackages.${system};
        in {
          defaultPackage = pkgs.rustPlatform.buildRustPackage {
            pname = "tex-fmt";
            version = "0.1.0";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
          };
        }
      );
}
