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

        shellHook = ''
          # https://github.com/tauri-apps/tauri/issues/12361
          export GDK_BACKEND=x11 # seems like a bug

          export GDK_SCALE=2 # optionally for hidpi monitors
          export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/"
        '';
      };
    });
}
