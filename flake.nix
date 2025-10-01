{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
        packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage {
            name = "clickexla";
            src = ./.;
            buildInputs = [
                pkgs.cairo 
                pkgs.gtk4 
                pkgs.alsa-lib 
                pkgs.xdotool 
                pkgs.xorg.libXi 
                pkgs.gcc 
                pkgs.libinput.dev 
                pkgs.systemdMinimal 
                pkgs.xorg.libX11.dev 
                pkgs.xorg.libXtst
                pkgs.rustc 
                pkgs.cargo
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            cargoHash = "sha256-dOKF2LK1uI3Z+e88N8xokT8t/RYJ6AY+KbAZ2awfFzI=";
            postInstall = ''
                mkdir -p $out/share/applications/
                mkdir -p $out/share/icons/hicolor/scalable/apps/
                install -Dm644 clickexla.desktop $out/share/applications/clickexla.desktop
                install -Dm644 icon.svg $out/share/icons/hicolor/scalable/apps/clickexla.svg
            '';
        };

        devShells."x86_64-linux".default= pkgs.mkShell {
            buildInputs = [
                pkgs.cairo 
                pkgs.gtk4 
                pkgs.alsa-lib 
                pkgs.xdotool 
                pkgs.xorg.libXi 
                pkgs.gcc 
                pkgs.libinput.dev 
                pkgs.systemdMinimal 
                pkgs.xorg.libX11.dev 
                pkgs.xorg.libXtst
                pkgs.rustc 
                pkgs.cargo
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}