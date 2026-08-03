self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.repo-builder;
in
{
  options.programs."{name}" = {
    enable = lib.mkEnableOption "{desc}";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "{name}.packages.\${system}.default";
      description = "{desc}";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];
  };
}
