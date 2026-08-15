{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    claude-code = {
      url = "github:sadjow/claude-code-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
      claude-code,
      ...
    }:
    # ── System-agnostic outputs (modules) live out here ──
    {
      # nixosModules.default = import ./nix/module.nix self;
      # homeModules.default = import ./nix/hm-module.nix self;
    }
    # ── Then merge the per-system outputs onto it ──
    //
      flake-utils.lib.eachSystem
        [
          "x86_64-linux"
          "aarch64-linux"
        ]
        (
          system:
          let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
              inherit system overlays;
              config.allowUnfree = true;
            };

            # ── Toolchain ─────────────────────────────────────────────
            rust = pkgs.rust-bin.nightly.latest.default;

            naersk' = pkgs.callPackage naersk {
              cargo = rust;
              rustc = rust;
            };

            # ── Build helper ──────────────────────────────────────────
            buildApp =
              { release }:
              let
                name = "praline";
                desc = "Helper TUI app to scaffold an idiomatic repo.";
              in
              pkgs.callPackage ./nix/package.nix {
                inherit
                  naersk'
                  release
                  name
                  desc
                  ;
                src = ./.;
              };

            # ── Claude Settings ─────────────────────────────────────
            claude = claude-code.packages.${system}.default;
            claudeLocalSettings = builtins.toJSON (
              import ./nix/claude_plugins.nix
              // {
                permissions.allow =
                  import ./nix/claude_nix_permissions.nix ++ import ./nix/claude_rust_permissions.nix;
              }
            );
            claudeSettingsFile = pkgs.writeText "settings.local.json" claudeLocalSettings;

            # ── Tooling shared by the dev shell and CI ───────────────
            ciTools = with pkgs; [
              rust

              # rust tooling
              cargo-nextest
              cargo-deny
              cargo-audit
              cargo-machete
              cargo-edit
              cargo-msrv

              # repo tooling
              typos
              committed
              git-cliff
              act
              gh
              lychee
              editorconfig-checker
              taplo

              # nix tooling
              nixfmt
              statix
              deadnix

              # deps
              sqlite
            ];
          in
          {
            # ── Packages ──────────────────────────────────────────────
            packages = rec {
              praline = buildApp { release = true; };
              praline-debug = buildApp { release = false; };
              default = praline;
            };

            # ── Checks (nix flake check) ─────────────────────────────
            checks.check = self.packages.${system}.praline-debug;

            # ── Dev Shell (nix develop) ──────────────────────────────
            devShells.default = pkgs.mkShell {
              buildInputs =
                ciTools
                ++ (with pkgs; [
                  rust-analyzer
                  just
                  claude
                  nodejs
                ]);
              shellHook = ''
                mkdir -p .claude
                install -m 644 ${claudeSettingsFile} .claude/settings.local.json
              '';
            };

            # ── CI Shell (nix develop .#ci) ──────────────────────────
            # Lean: just the toolchain + checks, no editor/claude/shellHook.
            devShells.ci = pkgs.mkShell {
              buildInputs = ciTools;
            };
          }
        );
}
