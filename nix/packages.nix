{pkgs, ...}: {
  default = pkgs.rustPlatform.buildRustPackage rec {
    pname = "nyaapp";
    version = "0.1.1";
    src = ../.;

    patches = [./01-localfont.patch];

    cargoRoot = "src-tauri";
    buildAndTestSubdir = cargoRoot;

    cargoHash = "sha256-v8vcbpLRJVnxYRVkW6Bo9IuZcUtUcAFarS4PUiuaty8=";

    npmDeps = pkgs.fetchNpmDeps {
      name = "${pname}-npm-deps-${version}";
      inherit src;
      hash = "sha256-XdgY+6TSdF4obn6jdbS5p6A2GPJv1+rvMel5LURPppA=";
    };

    nativeBuildInputs = with pkgs; [
      cargo-tauri.hook
      nodejs
      npmHooks.npmConfigHook
      pkg-config
      wrapGAppsHook3
    ];

    buildInputs = with pkgs;
      [openssl]
      ++ lib.optionals stdenv.isLinux [
        glib-networking
        webkitgtk_4_1
      ]
      ++ lib.optionals stdenv.isDarwin (
        with darwin.apple_sdk.frameworks; [
          AppKit
          CoreServices
          Security
          WebKit
        ]
      );

    configurePhase = ''
      runHook preConfigure

      mkdir -p src/app/fonts
      cp "${
        pkgs.google-fonts.override {fonts = ["Inter"];}
      }/share/fonts/truetype/Inter[opsz,wght].ttf" src/app/fonts/Inter.ttf

      runHook postConfigure
    '';
  };
}
