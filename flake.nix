{
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      fenixLib = fenix.packages.${system};
      toolchain = with fenixLib;
        combine [
          (stable.withComponents [
            "rustc"
            "cargo"
            "rustfmt"
            "clippy"
            "rust-src"
            "rust-docs"
            "rust-std"
            "rust-analyzer"
          ])
        ];
      naerskLib = (pkgs.callPackage naersk {}).override {
        cargo = toolchain;
        rustc = toolchain;
      };

      loomPackage = {
        release,
        debugFlags ? [],
      }:
        import ./default.nix {
          inherit pkgs;
          src = self;
          naersk = naerskLib;
          inherit release;
          inherit debugFlags;
        };
    in {
      packages = rec {
        default = live;

        live = pkgs.writeShellApplication {
          name = "loom-debug";

          runtimeInputs = [
            pkgs.cargo
          ];

          text = ''
            export SLINT_LIVE_PREVIEW=1
            exec cargo run --features slint/live-preview "$@"
          '';
        };

        debug = loomPackage {
          release = false;
          debugFlags = ["--features" "slint/live-preview"];
        };

        release = loomPackage {release = true;};
      };
    });
}
