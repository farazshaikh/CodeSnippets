{pkgs ? import <nixpkgs> { config = {allowUnfree = true;};}}:
pkgs.mkShellNoCC {
    name = "frz-dev-env";
    buildInputs = with pkgs; [
      nix
      emacs
      (vscode.override {
        isInsiders = false;
        # Configure VSCode to run without requiring --no-sandbox
        useVSCodeRipgrep = true;
        commandLineArgs = [];
      })
      cargo
      rustc
      rustfmt
      rust-analyzer
      neovim
      vimPlugins.LazyVim
    ];

    # Add shellHook to verify VSCode installation and modify PATH
    shellHook = ''
      echo "Debug: Starting shellHook"
      # Unset NVM related variables to avoid conflicts
      unset npm_config_prefix
      unset NODE_PATH
      # Unset existing code from PATH and ensure nix-provided vscode is first
      export PATH="${pkgs.vscode}/bin:$(echo $PATH | tr ':' '\n' | grep -v '/usr/bin/code' | tr '\n' ':')"
      echo "Environment Setup:"
      echo "----------------"
      echo "VSCode path: $(which code)"
      echo "Available packages:"
      echo $buildInputs
      echo "----------------"
    '';
}