{

  config,

  lib,

  pkgs,

  ...

}:

with

lib;

let


  inherit

    (config.boot)

    kernelPatches;


  inherit

    (config.boot.kernel)

    features

    randstructSeed;


  inherit

    (config.boot.kernelPackages)

    kernel;

in

{

  options =

    {

      boot.kernel.features =

        mkOption
        {
          default = {};
          example = literalExpression "{debug= true;}";
          internal = true;
          description = ''
            Kernel features''
          ;
        };

    };

}
