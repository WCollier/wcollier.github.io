{
  description = "Flake for generating my site";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cv = {
      url = "github:wcollier/cv";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, cv }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [ ( import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        site-generator = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        cv-package = cv.packages.${system}.default;
        wcollier = pkgs.stdenv.mkDerivation {
          name = "wcollier.github.io";
          src = ./.;
          buildInputs = [ site-generator cv-package ];
          buildPhase = ''
            mkdir out
            ${site-generator}/bin/site-generator
          '';
          installPhase = ''
            mkdir -p $out/_site
            cp -r _site $out/
            cp ${cv-package}/cv.pdf $out/_site/static/
          '';
        };
        host = pkgs.writeShellApplication {
          name = "host";
          runtimeInputs = [ pkgs.simple-http-server wcollier ];
          text = ''
            cd ${wcollier}/_site
            simple-http-server -i
          '';
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [ rust-bin.stable.latest.default ];
        };

        apps.default = {
          type = "app";
          program = "${host}/bin/host";
        };

        packages.default = wcollier;
      }
    );
}
