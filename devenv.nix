{pkgs, ...}: {
  # https://devenv.sh/packages/
  packages = with pkgs; [
    pkg-config
    pipewire
    alsa-lib
    gcc
    llvmPackages.libclang
    alejandra
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.nix.enable = true;

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";
}
