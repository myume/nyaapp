{
  description = "Development environment for the nyaapp";

  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          nodejs
          cargo-tauri

          just
        ];

        nativeBuildInputs = with pkgs; [
          # Rust toolchain
          rustc
          cargo
          pkg-config
          wrapGAppsHook4
        ];

        buildInputs = with pkgs; [
          openssl
          glib-networking

          webkitgtk_4_1
        ];
      };
    });
}
