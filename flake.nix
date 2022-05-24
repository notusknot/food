{
  description = "Flake for Rust version of food";

  inputs = {
    naersk.url = "github:nix-community/naersk";
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
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
          checkInputs = [ pkgs.sqlite pkgs.cargo pkgs.rustc];
        };
        defaultPackage = packages.food;

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            sqlite
            wasm-pack
            hyperfine
            cargo-bloat
             
            (rust-bin.nightly.latest.default.override { 
                targets = [ "wasm32-unknown-unknown" ]; 
                extensions = [ "rust-src" "clippy" "rustfmt" "rust-analyzer-preview" ];
            })
          ];
        };

        apps.food= utils.lib.mkApp {
          drv = packages.food;
        };
        defaultApp = apps.food;
      });
}
