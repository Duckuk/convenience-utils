{
  pkgs,
  lib,
  fetchFromGitHub,
  makeBinaryWrapper,
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "convenience-utils";
  version = "1.0.0";

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
    rev = "21dfb71cc4b07627b6e266dcc05a8a103f41fe77";
    hash = "sha256-YUeyQf1c83L/WPPlX6EsrTgvjg8S8l8+FuBziTA7Ud0=";
  };

  useFetchCargoVendor = true;
  cargoHash = "sha256-a2Bx3t8W3JDjaA0fJIxYzCB8k/1DXD4b43uQfu8WmQQ=";

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
