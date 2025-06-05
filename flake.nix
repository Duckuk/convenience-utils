{
  description = "Flake for convenience-utils";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    (flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
      in rec {
        packages = rec {
          convenience-utils = (import ./default.nix {inherit pkgs;}).convenience-utils;
          default = convenience-utils;
        };

        apps = {
          tar-zstd-compress = {
            type = "app";
            program = "${self.packages.${system}.convenience-utils}/bin/tar-zstd-compress";
          };

          vp9-compress = {
            type = "app";
            program = "${self.packages.${system}.convenience-utils}/bin/vp9-compress";
          };
        };
      }
    )) // {
      overlays.default = import ./nix/overlay.nix;
    };
}
