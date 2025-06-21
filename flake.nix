{
  description = "Vic Eye cataract audit";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      crane,
      fenix,
      flake-utils,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default =
        let
          craneLib =
            (crane.mkLib nixpkgs.legacyPackages.${system}).overrideToolchain
              fenix.packages.${system}.minimal.toolchain;

          pkgs = nixpkgs.legacyPackages.${system};
        in
        craneLib.buildPackage {
          src = ./.;
          buildInputs = [ ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
        };
    });
}

# todo: start with the dockerfile in leptos docs:
# https://book.leptos.dev/deployment/ssr.html
# then use nix example below, from:
# https://dev.to/johnreillymurray/rust-environment-and-docker-build-with-nix-flakes-19c1
# to nixify it

# outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
#     flake-utils.lib.eachDefaultSystem (system:
#       let
#         overlays = [ (import rust-overlay) ];
#         pkgs = import nixpkgs { inherit system overlays; };
#         rustVersion = pkgs.rust-bin.stable.latest.default;
#
#         rustPlatform = pkgs.makeRustPlatform {
#           cargo = rustVersion;
#           rustc = rustVersion;
#         };
#
#         myRustBuild = rustPlatform.buildRustPackage {
#           pname =
#             "rust_nix_blog"; # make this what ever your cargo.toml package.name is
#           version = "0.1.0";
#           src = ./.; # the folder with the cargo.toml
#
#           cargoLock.lockFile = ./Cargo.lock;
#         };
#
#         dockerImage = pkgs.dockerTools.buildImage {
#           name = "rust-nix-blog";
#           config = { Cmd = [ "${myRustBuild}/bin/rust_nix_blog" ]; };
#         };
#
#       in {
#         packages = {
#           rustPackage = myRustBuild;
#           docker = dockerImage;
#         };
#         defaultPackage = dockerImage;
#         devShell = pkgs.mkShell {
#           buildInputs =
#             [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
#         };
#       });

# see: https://crane.dev/local_development.html

# {
#   inputs = {
#     nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
#
#     crane = {
#       url = "github:ipetkov/crane";
#       inputs.nixpkgs.follows = "nixpkgs";
#     };
#
#     flake-utils.url = "github:numtide/flake-utils";
#   };
#
#   outputs =
#     {
#       self,
#       nixpkgs,
#       crane,
#       flake-utils,
#       ...
#     }:
#     flake-utils.lib.eachDefaultSystem (
#       system:
#       let
#         pkgs = nixpkgs.legacyPackages.${system};
#         craneLib = crane.mkLib pkgs;
#
#         my-crate = craneLib.buildPackage {
#           src = craneLib.cleanCargoSource ./.;
#
#           buildInputs =
#             [
#               # Add additional build inputs here
#             ]
#             ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
#               # Additional darwin specific inputs can be set here
#               pkgs.libiconv
#             ];
#
#           # Additional environment variables can be set directly
#           # MY_CUSTOM_VAR = "some value";
#         };
#       in
#       {
#         packages.default = my-crate;
#
#         devShells.default = craneLib.devShell {
#           # Additional dev-shell environment variables can be set directly
#           MY_CUSTOM_DEV_URL = "http://localhost:3000";
#
#           # Automatically inherit any build inputs from `my-crate`
#           inputsFrom = [ my-crate ];
#
#           # Extra inputs (only used for interactive development)
#           # can be added here; cargo and rustc are provided by default.
#           packages = [
#             pkgs.cargo-audit
#             pkgs.cargo-watch
#           ];
#         };
#       }
#     );
# }
