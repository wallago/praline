self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.praline;
  settingsFormat = pkgs.formats.toml { };
  configFile = settingsFormat.generate "config.toml" cfg.settings;
in
{
  options.programs.praline = {
    enable = lib.mkEnableOption "Helper TUI app to scaffold an idiomatic repo.";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      description = "Helper TUI app to scaffold an idiomatic repo.";
    };

    settings = lib.mkOption {
      type = settingsFormat.type;
      default = { };
      example = {
        keybindings = {
          scroll_down = "e";
          scroll_up = "i";
          scroll_right = "o";
          scroll_left = "n";
          generate = "g";
          quit = "q";
        };
      };
      description = ''
        Configuration written to tool's config.toml.
        Every key is optional; omitted keys keep the built-in defaults.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."praline/config.toml" = lib.mkIf (cfg.settings != { }) {
      source = configFile;
    };
  };
}
