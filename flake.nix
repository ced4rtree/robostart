{
  description = "Robostart FRC Robot Project Initializer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } ({ ... }: {
      systems = import inputs.systems;

      perSystem = { pkgs, self', ... }: let
        naerskLib = pkgs.callPackage inputs.naersk {};
        basePackage = release: naerskLib.buildPackage {
          src = ./.;
          inherit release;
          buildInputs = [ pkgs.openssl ];
          nativeBuildInputs = [ pkgs.pkg-config ];
        };
      in {
        packages.dev = basePackage false;
        packages.robostart = basePackage true;
        packages.default = self'.packages.robostart;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            openssl
            cargo-deb
          ];

          nativeBuildInputs = [ pkgs.pkg-config ];

          env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      };
    });
}
