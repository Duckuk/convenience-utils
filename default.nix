{ pkgs ? <nixpkgs> }:
{
  convenience-utils = pkgs.callPackage ./nix/package.nix {};
}