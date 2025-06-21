{
  description = "Nix devshells!";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = { nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system}.simple = pkgs.mkShell {
      packages = [ pkgs.rustup ];
    };
  };
}
