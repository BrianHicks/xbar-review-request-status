{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nmattia/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        darwinInputs = if pkgs.stdenv.isDarwin then
          [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ]
        else
          [ ];
      in rec {
        # `nix build`
        packages.xbar-review-request-status = naersk-lib.buildPackage {
          root = ./.;
          buildInputs = [
            pkgs.libiconv
            pkgs.openssl
            pkgs.pkg-config
            pkgs.rustPackages.clippy
          ] ++ darwinInputs;

          doCheck = true;
          checkPhase = ''
            cargo test
            cargo clippy -- --deny warnings
          '';
        };
        defaultPackage = packages.xbar-review-request-status;
        overlay = final: prev: {
          xbar-review-request-status = packages.xbar-review-request-status;
        };

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs;
            [
              cargo
              cargo-edit
              cargo-insta
              rustPackages.clippy
              rustc
              rustfmt

              # for some reason this seems to be required, especially on macOS
              libiconv
            ] ++ darwinInputs;
        };
      });
}

