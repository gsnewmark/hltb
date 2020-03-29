let
  pkgs = import <nixpkgs> {};
in
with pkgs;
mkShell {
  buildInputs = [
    openssl
    pkgconfig
    rustup
  ];
}
