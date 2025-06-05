{
  pkgs,
  lib,
  fetchFromGitHub,
  makeBinaryWrapper,
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "convenience-utils";
  version = "0.1.0";

  nativeBuildInputs = [
    makeBinaryWrapper
  ];

  buildInputs = [
    pkgs.gnutar
    pkgs.zstd
    pkgs.ffmpeg_7-full
  ];

  src = fetchFromGitHub {
    owner = "Duckuk";
    repo = "convenience-utils";
    rev = "3a141e88d99e7f00708e6e899f1ee396166eb8a6";
    hash = "sha256-/0SKUsb+U1tVUUZ7HK+3GmW11F2mLVUUoSLKexHSnaY=";
  };

  useFetchCargoVendor = true;
  cargoHash = "sha256-UKTX40vRxpMK9UUfBCzU8C1A7iYfoalXhopxyl3w3qI=";

  postFixup = let
    tar = pkgs.gnutar;
    zstd = pkgs.zstd;
    ffmpeg = pkgs.ffmpeg_7-full;
  in ''
    wrapProgram $out/bin/tar-zstd-compress \
      --prefix PATH : ${lib.makeBinPath [tar zstd]}
    wrapProgram $out/bin/vp9-compress \
      --prefix PATH : ${lib.makeBinPath [ffmpeg]}

    # Compress binaries with upx
    # ${pkgs.findutils}/bin/find $out/bin -type f -executable -exec ${pkgs.upx}/bin/upx --best --lzma '{}' '+'
  '';

  meta = with lib; {
    homepage = "https://github.com/Duckuk/convenience-utils";
    description = "Simple wrappers around various programs for my own convenience.";
    license = getLicenseFromSpdxId "LGPL-3.0-or-later";
    platforms = lib.platforms.linux;
  };
}
