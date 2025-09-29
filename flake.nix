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
            cargoHash = "sha256-Q3ftrrX9ecDK/9Q5sGx5CDPw3vP+HUmuaHKHdc8w3Fo=";
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