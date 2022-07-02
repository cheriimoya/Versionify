with import <nixpkgs> {};
mkShell {
  nativeBuildInputs = [ rustc cargo gcc ];
  buildInputs = [
    rustfmt
    clippy
    cargo-watch
    cargo-edit
    cargo-outdated
    cargo-audit
    pkgconfig
    openssl
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
