{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # {if:rust}
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    # {endif:rust}
    # {if:claude}
    claude-code = {
      url = "github:sadjow/claude-code-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # {endif:claude}
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      # {if:rust}
      rust-overlay,
      naersk,
      # {endif:rust}
      # {if:claude}
      claude-code,
      # {endif:claude}
      ...
    }:
    # {if:rust}
    # ── System-agnostic outputs (modules) live out here ──
    {
      nixosModules.default = import ./nix/module.nix self;
      homeModules.default = import ./nix/hm-module.nix self;
    }
    # ── Then merge the per-system outputs onto it ──
    //
    # {endif:rust}
      flake-utils.lib.eachSystem
        [
          "x86_64-linux"
          "aarch64-linux"
        ]
        (
          system:
          let
            # {if:rust}
            overlays = [ (import rust-overlay) ];
            # {endif:rust}
            pkgs = import nixpkgs {
              inherit 
                system 
                # {if:rust}
                overlays
                # {endif:rust}
              ;
              config.allowUnfree = true;
            };

            # {if:rust}
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
                name = "{name}";
                desc = "{desc}";
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
            # {endif:rust}

            # {if:claude}
            # ── Claude Settings ─────────────────────────────────────
            claude = claude-code.packages.${system}.default;
            # {endif:claude}

            # ── Tooling shared by the dev shell and CI ───────────────
            ciTools = with pkgs; [
              # {if:rust}
              rust
              # rust tooling
              cargo-nextest
              cargo-edit
              # {endif:rust}

              # {if:audit}
              cargo-audit
              # {endif:audit}
              # {if:machete}
              cargo-machete
              # {endif:machete}
              # {if:deny}
              cargo-deny
              # {endif:deny}
              # {if:typos}
              typos
              # {endif:typos}
              # {if:committed}
              committed
              # {endif:committed}
              # {if:cliff}
              git-cliff
              # {endif:cliff}
              # {if:taplo}
              taplo
              # {endif:taplo}
              # {if:editorconfig}
              editorconfig-checker
              # {endif:editorconfig}

              # nix tooling
              nixfmt
              statix
              deadnix

              # crate deps
            ];
          in
          {
            # {if:rust}
            # ── Packages ──────────────────────────────────────────────
            packages = rec {
              {name} = buildApp { release = true; };
              {name}-debug = buildApp { release = false; };
              default = {name};
            };

            # ── Checks (nix flake check) ─────────────────────────────
            checks.check = self.packages.${system}.{name}-debug;
            # {endif:rust}

            # ── Dev Shell (nix develop) ──────────────────────────────
            devShells.default =
              let
                banner = pkgs.writeShellApplication {
                  name = "project-banner";
                  runtimeInputs = with pkgs; [
                    gum
                    jq
                    git
                    coreutils
                    findutils
                  ];
                  text = ''
                    export CLICOLOR_FORCE=1
                    cd "$(git rev-parse --show-toplevel)"

                    created=$(git log --reverse --format=%as | sed -n 1p)
                    updated=$(git log -1 --format=%cr)
                    commits=$(git rev-list --count HEAD)
                    nixpkgs=$(date -d @"$(jq -r .nodes.nixpkgs.locked.lastModified flake.lock)" +%F)

                    title=$(gum style --foreground 111 --bold '{name}')
                    desc=$(gum style --foreground 244 --italic "{desc}")
                    keys=$(gum style --foreground 80 --align right --padding "0 2 0 0" \
                      created updated commits nixpkgs)
                    vals=$(gum style --foreground 255 \
                      "$created" "$updated" "$commits" "$nixpkgs")

                    header=$(gum join --vertical --align center "$title" "" "$desc")

                    body=$(gum join --vertical --align center \
                      "$header" "" "$(gum join --horizontal "$keys" "$vals")")

                    gum style --border double --border-foreground 111 \
                      --margin "1 2" --padding "1 4" "$body"
                  '';
                };
              in
              pkgs.mkShell {
                PROJECT_BANNER = pkgs.lib.getExe banner;
                buildInputs =
                  ciTools
                  ++ (with pkgs; [
                    # {if:rust}
                    rust-analyzer
                    # {endif:rust}
                    # {if:justfile}
                    just
                    # {endif:justfile}
                    # {if:claude}
                    claude
                    # {endif:claude}
                    nodejs
                  ]);
              };

            # ── CI Shell (nix develop .#ci) ──────────────────────────
            # Lean: just the toolchain + checks, no editor/claude/shellHook.
            devShells.ci = pkgs.mkShell {
              buildInputs = ciTools;
            };
          }
        );
}
