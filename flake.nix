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
            cargoHash = "sha256-3QjT+T2w1Jgz9UeWKaOUBugB9JArz7aPJkCG0gQZXCU=";
            postInstall = ''
                mkdir -p $out/share/applications/
                mkdir -p $out/share/icons/hicolor/128x128/apps/
                install -Dm644 clickexla.desktop $out/share/applications/clickexla.desktop
                install -Dm644 icon.png $out/share/icons/hicolor/128x128/apps/clickexla.png
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