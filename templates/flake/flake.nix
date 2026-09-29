{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # {slot:flake.inputs}
  };

  outputs =
    {
      self,
      ...
    }@inputs:
    # ── System-agnostic outputs (modules) live out here ──
    {
      homeModules.default = import ./nix/hm-module.nix self;
    }
    # ── Then merge the per-system outputs onto it ──
    //
      inputs.flake-utils.lib.eachSystem
        [
          "x86_64-linux"
          "aarch64-linux"
        ]
        (
          system:
          let
            overlays = [
              # {slot:flake.overlays}
            ];
            pkgs = import inputs.nixpkgs {
              inherit system overlays;
              config.allowUnfree = true;
            };

            # {slot:flake.params}

            # ── Tooling shared ────────────────────────────────────────
            ciTools = with pkgs; [
              # {slot:flake.pkgs.ci}
            ];
            devTools = with pkgs; [
              # {slot:flake.pkgs.dev}
            ];
          in
          {
            # ── Checks (nix flake check) ─────────────────────────────
            checks.check = self.packages.${system}."{name}-debug";

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

                    title=$(gum style --foreground 111 --bold 'praline')
                    desc=$(gum style --foreground 244 --italic "Helper TUI app to scaffold an idiomatic repo.")
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
                buildInputs = ciTools ++ devTools;
              };

            # ── CI Shell (nix develop .#ci) ──────────────────────────
            # Lean: just the toolchain + checks, no editor/claude/shellHook.
            devShells.ci = pkgs.mkShell {
              buildInputs = ciTools;
            };
          }
        );
}
