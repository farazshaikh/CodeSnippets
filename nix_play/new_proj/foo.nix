{ pkgs ? import <nixpkgs> { } }:

# Accessing a package from nixpkgs

let
  f = pkgs.stdenv.mkDerivation {

  },
  myPackage = pkgs.stdenv.mkDerivation
    {

      name = "my-package";

      buildInputs = [ pkgs.gcc ];

      # ... rest of the package definition

    };

in myPackage
