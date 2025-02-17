{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell { buildInputs = [ pkgs.vim pkgs.emacs pkgs.nodejs ]; }
