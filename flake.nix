{
  description = "Robostart FRC Robot Project Initializer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { nixpkgs, naersk, ... }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages."${system}";
    naerskLib = pkgs.callPackage naersk {};
  in {
    packages."${system}".default = naerskLib.buildPackage {
      src = ./.;
      buildInputs = [ pkgs.openssl ];
      nativeBuildInputs = [ pkgs.pkg-config ];
    };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustup
        openssl
      ];

      nativeBuildInputs = [ pkgs.pkg-config ];

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
