{
  pkgs ? import (builtins.fetchTarball
    https://github.com/NixOS/nixpkgs/archive/4aa36568d413aca0ea84a1684d2d46f55dbabad7.zip
  ) {},
  lib ? pkgs.lib
}:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "tar-zstd-compress";
  version = "0.1.0";

  buildInputs = [
    pkgs.gnutar
    pkgs.zstd
  ];

  cargoLock.lockFile = ./Cargo.lock;

  src = lib.cleanSource ./.;
}
