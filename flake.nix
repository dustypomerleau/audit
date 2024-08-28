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
          craneLib = crane.lib.${system}.overrideToolchain fenix.packages.${system}.minimal.toolchain;
          pkgs = nixpkgs.legacyPackages.${system};
        in
        craneLib.buildPackage {
          src = ./.;
          buildInputs = [ ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
        };
    });
}

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
