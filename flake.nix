{
  description = "Hyprland monitor config ";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, ... }: 
  let
    system = "x86_64-linux";
    overlays = [ 
      (import rust-overlay)
    ];
    pkgs = import nixpkgs { inherit system overlays; };

    rust = pkgs.rust-bin.stable.latest.default;

  in {
    devShells.${system}.default = pkgs.mkShell rec {
      nativeBuildInputs = [
        pkgs.pkg-config
        pkgs.gcc13
      ];
      buildInputs = with pkgs; [
        libXcursor
        libXrandr
        libXi
        libxkbcommon
        rust
        rust-analyzer
        clippy

        #for raylib
        glfw
        wayland
        libxkbcommon
        wayland-protocols
        extra-cmake-modules
        libclang
      ];

      shellHook = ''
        export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}
        exec zsh
      '';
    };
  };
}

