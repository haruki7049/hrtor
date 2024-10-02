{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          pkgs,
          lib,
          system,
          ...
        }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust;
          overlays = [ inputs.rust-overlay.overlays.default ];
          src = lib.cleanSource ./.;
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
            cargoClippyExtraArgs = "--verbose -- --deny warning";
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
          manual = pkgs.stdenv.mkDerivation {
            pname = "hrtor-manual";
            version = "dev";
            src = lib.cleanSource ./manual;

            nativeBuildInputs = [
              pkgs.mdbook
            ];

            buildPhase = ''
              mdbook build
            '';

            installPhase = ''
              mkdir -p $out/share
              cp -r book $out/share
            '';
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };

          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
            programs.taplo.enable = true;
            programs.actionlint.enable = true;
            programs.mdformat.enable = true;

            settings.formatter = {
              mdformat.excludes = [
                "CONTRIBUTING.md"
                "CODE_OF_CONDUCT.md"
                "SUPPORT.md"
              ];
            };
          };

          packages = {
            inherit
              hrtor
              llvm-cov
              llvm-cov-text
              manual
              ;
            default = hrtor;
            doc = cargo-doc;
          };

          checks = {
            inherit
              hrtor
              cargo-clippy
              cargo-doc
              llvm-cov
              llvm-cov-text
              manual
              ;
          };

          devShells.default = pkgs.mkShell {
            packages = [
              # Rust
              rust

              # Nix
              pkgs.nil

              # mdBook
              pkgs.mdbook
            ];

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
