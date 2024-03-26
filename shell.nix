let
  rust_overlay = import (builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/72fa0217f76020ad3aeb2dd9dd72490905b23b6f.tar.gz);
  pkgs = import (builtins.fetchTarball https://github.com/NixOS/nixpkgs/archive/df41961bd4b7e838cb997543ea1297f4cbd7da15.tar.gz) {
    overlays = [
      rust_overlay
    ];
  };
  rustVersion = "1.76.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };
in
pkgs.mkShell {
  packages = with pkgs; [
    rust
  ];
}
