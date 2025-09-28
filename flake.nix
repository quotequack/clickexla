{
  description = "a silly keyboard sound program :D";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { inherit system; };
        rust = pkgs.rustPlatform;

      in {
        packages.default = rust.buildRustPackage {
          pname = "clickexla";
          version = "0.1.0";
          
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [ pkgs.libinput.dev pkgs.pkg-config ];
          buildInputs = [ pkgs.cairo pkgs.gtk4 pkgs.alsa-lib pkgs.xdotool pkgs.xorg.libXi pkgs.gcc pkgs.libinput.dev pkgs.systemdMinimal pkgs.xorg.libX11.dev pkgs.xorg.libXtst ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.xdotool
            pkgs.gcc
            pkgs.cairo
            pkgs.gtk4
            pkgs.libinput.dev
            pkgs.systemdMinimal
            pkgs.xorg.libXtst
            pkgs.xorg.libX11.dev
            pkgs.pkg-config
            pkgs.cargo
            pkgs.alsa-lib
            pkgs.rustc
            pkgs.rust-analyzer
            pkgs.xorg.libXi
          ];
        };
      }
    );
}

