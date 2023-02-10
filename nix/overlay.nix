final: prev: {
  devShells.default = final.callPackage ./dev-shell.nix { };

  spiffers = final.callPackage ./spiffers.nix { };
}
