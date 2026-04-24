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
      buildInputs = [
        pkgs.samply
        pkgs.wayland
        pkgs.vulkan-loader
        pkgs.vulkan-tools
        pkgs.wayland
        pkgs.wayland-protocols
        pkgs.libXcursor
        pkgs.libXrandr
        pkgs.libXi
        pkgs.vulkan-loader
        pkgs.libxkbcommon
        rust
        pkgs.SDL2
        pkgs.rust-analyzer
        pkgs.clippy
      ];

      shellHook = ''
        export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}
        exec zsh -c "nvim"
      '';
    };
  };
}

