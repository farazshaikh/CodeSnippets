#!/usr/bin/env nix develop
#! nix-shell
#! nix-shell -i bash --pure
#! nix-shell -p coreutils cacert curl jq python3Packages.xmljson

curl https://github.com/NixOS/nixpkgs/releases.atom | xml2json | jq .