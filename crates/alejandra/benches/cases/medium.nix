{ config, lib, pkgs, ... }:

let
  inherit (config) options;
  test = {
    a = 1;
    b = 2;
    c = {
      d = 3;
      e = 4;
    };
  };
in
lib.mkIf config.enableSomething {
  pkg = pkgs.foo;
  enable = true;
}
