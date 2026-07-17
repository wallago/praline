builtins.toJSON {
  permissions = {
    allow = [
      "Bash(nix flake check*)"
      "Bash(nix eval*)"
      "Bash(nixos-rebuild dry-build*)"
      "Bash(statix check*)"
      "Bash(deadnix*)"
      "Bash(just*)"
      "Bash(nix build --dry-run*)"
      "Bash(nix search nixpkgs*)"
      "Bash(curl -s https://search.nixos.org*)"
    ];
  };
}
