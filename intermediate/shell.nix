{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [ gcc rustc rustup cargo ];
  RUST_BACKTRACE = 1;
}