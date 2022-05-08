{
    description = "Rust project flake";

    inputs.flake-utils.url = "github:numtide/flake-utils";

    outputs = { self, nixpkgs, flake-utils }:
        flake-utils.lib.eachDefaultSystem 
        (system: let pkgs = nixpkgs.legacyPackages.${system}; in {
            devShell = pkgs.mkShell {
                buildInputs = with pkgs; [
                    cargo rustc rust-analyzer rustfmt hyperfine cargo-bloat sqlite
                ];
            };
        }
    );
}
