{
    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    inputs.flake-utils.url = "github:numtide/flake-utils";
    outputs = {nixpkgs,flake-utils,...}: 
         flake-utils.lib.eachDefaultSystem (system: with import nixpkgs { inherit system;}; {
            packages.default = rustPlatform.buildRustPackage rec {
                pname = "papertrail-downloader";
                version = "git";
                cargoLock.lockFile = ./Cargo.lock;
                src = pkgs.lib.cleanSource ./.;
                PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
                nativeBuildInputs = [pkg-config];
            };
        });
    
}
