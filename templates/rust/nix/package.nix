# {if:flake.package}
{
  pkgs,
  lib,
  naersk',
  src,
  release ? true,
}:
naersk'.buildPackage {
  name = "{name}";
  inherit src release;

  buildInputs = with pkgs; [
    pkg-config
    udev
  ];
  meta = {
    description = "{desc}";
    license = with lib.licenses; [
      asl20
      mit
    ];
  };
}
# {endif:flake.package}
