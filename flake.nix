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
          installPhase = ''
            runHook preInstall
            install -Dm755 target/release/clickexla $out/bin/clickexla
            install -Dm644 settings.json $out/share/clickexla/settings.json
            install -Dm644 icon.png $out/share/icons/hicolor/48x48/apps/clickexla.png
            install -Dm644 clickexla.desktop $out/share/applications/clickexla.desktop
            runHook postInstall
          '';
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

