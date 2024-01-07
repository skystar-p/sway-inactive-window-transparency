{
  description = "sway-inactive-window-transparency flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    let systems = [ "x86_64-linux" "aarch64-linux" ];
    in
    flake-utils.lib.eachSystem systems (
      system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        defaultPackage = (import ./. { inherit pkgs; });
      }
    );
}

