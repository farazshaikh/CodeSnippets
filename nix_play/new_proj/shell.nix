{ pkgs ? import <nixpkgs> { } }:
let
  message = "Hello world";
in
pkgs.mkShellNoCC {
  packages = with pkgs; [ cowsay ];
  shellHook = ''
    cowsay ${message}'';
}


