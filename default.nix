{ pkgs ? import <nixpkgs> {}, src ? ./. }:
pkgs.rustPlatform.buildRustPackage {
  name = "versionify";
  inherit src;
  nativeBuildInputs = with pkgs; [
    pkg-config
    openssl
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  cargoLock.lockFile = "${src}/Cargo.lock";
  meta = with pkgs.lib; {
    description = "Playlist versioning to git for Spotify";
    homepage = "https://github.com/cheriimoya/versionify";
    license = licenses.mit;
    maintainers = with maintainers; [ cheriimoya ];
    platforms = platforms.unix;
  };
}
