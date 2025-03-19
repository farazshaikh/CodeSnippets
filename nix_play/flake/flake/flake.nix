{
  description = "Development environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }: {
    devShells.x86_64-linux = {
      default = let
        pkgs = import nixpkgs {
          system = "x86_64-linux";
          config = { allowUnfree = true; };
        };
      in pkgs.mkShellNoCC {
        name = "frz-dev-env";
        buildInputs = with pkgs; [
          nix
          emacs
          vscode
          cargo
          rustc
          rustfmt
          rust-analyzer
          neovim
          vimPlugins.LazyVim
        ];
      };
    };
  };
}
