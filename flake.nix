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
      packages = import ./nix/packages.nix {inherit pkgs;};

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          nodejs
          rustfmt

          just
          sqlite
        ];

        nativeBuildInputs = with pkgs; [
          # Rust toolchain
          rustc
          cargo
          pkg-config
          wrapGAppsHook4
          wrapGAppsHook3
        ];

        buildInputs = with pkgs; [
          openssl
          glib-networking

          webkitgtk_4_1
        ];

        shellHook = with pkgs; ''
          export XDG_DATA_DIRS=${gsettings-desktop-schemas}/share/gsettings-schemas/${gsettings-desktop-schemas.name}:${gtk3}/share/gsettings-schemas/${gtk3.name}:$XDG_DATA_DIRS;
          export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
        '';
      };
    });
}
