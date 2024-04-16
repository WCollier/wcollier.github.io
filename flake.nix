{
  description = "Flake for generating my site";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [ ( import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        site-generator = rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        host = pkgs.writeShellApplication {
          name = "host";
          runtimeInputs = [ pkgs.simple-http-server site-generator ];
          text = ''
            ${site-generator}/bin/site-generator
            cd _site
            simple-http-server -i
          '';
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [ rust-bin.stable.latest.default ];
        };

        apps = {
          default = {
            type = "app";
            program = "${host}/bin/host";
          };

          build = {
            type = "app";
            program = "${site-generator}/bin/site-generator";
          };
        };
      }
    );
}
