{
  description = "Flake for Rust version of food";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };

    utils = {
      url = "github:numtide/flake-utils";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        naersk-lib = naersk.lib."${system}";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      rec {
        packages.food = naersk-lib.buildPackage {
          pname = "food";
          root = ./.;
          doCheck = true;
          checkInputs = [ pkgs.cargo pkgs.rustc ];
        };
        defaultPackage = packages.food;

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            hyperfine
            wasm-pack
             
            (rust-bin.stable.latest.default.override { 
                targets = [ "wasm32-unknown-unknown" ]; 
                extensions = [ "rust-src" "clippy" "rustfmt" ];
            })
          ];
        };

        apps.food= utils.lib.mkApp {
          drv = packages.food;
        };
        defaultApp = apps.food;
      });
}