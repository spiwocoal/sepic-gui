{
  description = "eframe devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    esp-dev.url = "github:mirrexagon/nixpkgs-esp-dev";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    esp-dev,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay) esp-dev.outputs.overlays.default];
      pkgs = import nixpkgs {
        inherit system overlays;
        config.permittedInsecurePackages = [
          "python3.13-ecdsa-0.19.1"
        ];
      };
    in
      with pkgs; {
        devShells.default = mkShell rec {
          buildInputs = [
            # Rust
            (rust-bin.stable.latest.default.override {
              targets = ["x86_64-pc-windows-msvc" "aarch64-apple-darwin"];
            })
            rust-analyzer
            cargo-xwin

            # misc. libraries
            openssl
            pkg-config

            # GUI libs
            libxkbcommon
            libGL
            fontconfig

            # wayland libraries
            wayland

            # x11 libraries
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11

            systemdLibs

            gdb

            esp-idf-full

            # Python
            python3
            minicom
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      });
}
