{
  description = "Flake for generating my site";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      overlays = [ (import rust-overlay) ];
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ] (
          system:
          f {
            pkgs = import nixpkgs { inherit system overlays; };
          }
        );
    in
    {
      devShells = forAllSystems (
        { pkgs }:
        {
          default = pkgs.mkShell {
            buildInputs = [ pkgs.rust-bin.stable.latest.default ];
          };
        }
      );
      apps = forAllSystems (
        { pkgs }:
        let
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.stable.latest.default;
            rustc = pkgs.rust-bin.stable.latest.default;
          };
          site-generator = rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
          host = pkgs.writeShellApplication {
            name = "host";
            runtimeInputs = [
              pkgs.simple-http-server
              site-generator
            ];
            text = ''
              ${site-generator}/bin/site-generator
              cd _site
              simple-http-server -i
            '';
          };
        in
        {
          default = {
            type = "app";
            program = "${host}/bin/host";
          };

          build = {
            type = "app";
            program = "${site-generator}/bin/site-generator";
          };
        }
      );
    };
}
