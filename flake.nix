{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    nixpkgs,
    self,
    ...
  }: let
    inherit (nixpkgs) lib;
    withSystem = f:
      lib.fold lib.recursiveUpdate {}
      (map f ["x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"]);
  in
    withSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        overlays.default = final: _: removeAttrs self.packages.${final.system} ["default"];
        overlay = self.overlays.default;

        formatter.${system} = pkgs.alejandra;

        packages.${system} = {
          nixy = pkgs.callPackage ./package.nix {};
          default = self.packages.${system}.nixy;
        };

        devShells.${system}.default = pkgs.mkShell {
          packages = [
            pkgs.rustfmt
            pkgs.rust-analyzer
          ];
          inputsFrom = [
            self.packages.${system}.default
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath [pkgs.openssl];
        };
      }
    );
}
