{pkgs, ...}: {
  # https://devenv.sh/packages/
  packages = with pkgs; [
    pkg-config
    pipewire
    alsa-lib
    gcc
    llvmPackages.libclang
    slint-lsp
    fontconfig
    wayland
    libxkbcommon
    alejandra
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.nix.enable = true;

  env.LD_LIBRARY_PATH = "${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib";
  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";

  env.SLINT_LIVE_PREVIEW = 1;

  processes = {
    hot = {
      exec = "cargo run --features slint/live-preview";
      watch = {
        paths = [./src];
        extensions = ["rs" "toml"];
        ignore = ["*.log" ".git" ".jj" ".devenv" "target"];
      };
    };
  };
}
