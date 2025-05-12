{
  description = "A hrtor's flake";

  nixConfig = {
    extra-substituters = [
      "https://hrtor.cachix.org"
    ];
    extra-trusted-public-keys = [
      "hrtor.cachix.org-1:fXGmUinLwE/TpyGhEyTGYEzk6L5cyJ0soYB385+k9lg="
    ];
  };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

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
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };

          treefmt = {
            projectRootFile = "flake.nix";

            # Nix
            programs.nixfmt.enable = true;

            # Rust
            programs.rustfmt.enable = true;

            # TOML
            programs.taplo.enable = true;

            # GitHub Actions
            programs.actionlint.enable = true;

            # Markdown
            programs.mdformat.enable = true;

            # ShellScript
            programs.shellcheck.enable = true;
            programs.shfmt.enable = true;

            settings.formatter = {
              mdformat.excludes = [
                "CODE_OF_CONDUCT.md"
              ];
            };
          };

          packages = {
            inherit hrtor;
            default = hrtor;
            doc = cargo-doc;
          };

          checks = {
            inherit
              hrtor
              cargo-clippy
              cargo-doc
              ;
          };

          devShells.default = pkgs.mkShell {
            packages = [
              # Rust
              rust

              # Nix
              pkgs.nil
            ];

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
