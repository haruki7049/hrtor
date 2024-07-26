{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, treefmt-nix, rust-overlay, crane }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ (import rust-overlay) ]; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
        src = ./.;
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
        };
        hrtor = craneLib.buildPackage {
          inherit src cargoArtifacts;
          strictDeps = true;

          doCheck = true;
        };
        cargo-clippy = craneLib.cargoClippy {
          inherit src cargoArtifacts;
          cargoClippyExtraArgs = "--verbose -- --deny warnings";
        };
        cargo-doc = craneLib.cargoDoc {
          inherit src cargoArtifacts;
        };
        llvm-cov-text = craneLib.cargoLlvmCov {
          inherit cargoArtifacts src;
          cargoExtraArgs = "--locked";
          cargoLlvmCovCommand = "test";
          cargoLlvmCovExtraArgs = "--text --output-dir $out";
        };
        llvm-cov = craneLib.cargoLlvmCov {
          inherit cargoArtifacts src;
          cargoExtraArgs = "--locked";
          cargoLlvmCovCommand = "test";
          cargoLlvmCovExtraArgs = "--html --output-dir $out";
        };
      in
      {
        formatter = treefmtEval.config.build.wrapper;

        packages.default = hrtor;
        packages.doc = cargo-doc;
        packages.llvm-cov = llvm-cov;
        packages.llvm-cov-text = llvm-cov-text;

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        checks = {
          inherit hrtor cargo-clippy cargo-doc llvm-cov llvm-cov-text;
          formatting = treefmtEval.config.build.check self;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rust
          ];

          shellHook = ''
            export PS1="\n[nix-shell:\w]$ "
          '';
        };
      }
    );
}
