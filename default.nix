{ pkgs ? import <nixpkgs> { } }:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  cargoLock.lockFile = ./Cargo.lock;

  cargoLock.outputHashes = {
    "i3ipc-types-0.15.0" = "sha256-8sQHlWOkeRUu7xyUpDXDjop+y9dRUCpMHnAqynTPob8=";
  };

  src = pkgs.lib.cleanSource ./.;

  meta = with pkgs.lib; {
    description = "Set inactive window opacity on Sway";
    homepage = "https://github.com/skystar-p/sway-inactive-window-transparency";
    license = licenses.mit;
  };
}
