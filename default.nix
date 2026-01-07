{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "niri-status-layer";
  version = "0.1.0";
  
  src = ./.;
  
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  
  nativeBuildInputs = with pkgs; [
    pkg-config
    wrapGAppsHook4
  ];
  
  buildInputs = with pkgs; [
    gtk4
    gtk4-layer-shell
  ];
}
