{
  pkgs,
  src,
  naersk,
  release ? false,
  debugFlags ? [],
}:
naersk.buildPackage {
  name = "loom";
  inherit src;
  nativeBuildInputs = with pkgs; [
    pkg-config
    pipewire
    alsa-lib
    gcc
    llvmPackages.libclang
    fontconfig
    autoPatchelfHook
    # wayland
    # libxkbcommon
  ];
  buildInputs = with pkgs; [
    fontconfig
  ];

  doCheck = false;

  cargoBuildFlags = (
    if release
    then ["--release"]
    else debugFlags
  );

  LD_LIBRARY_PATH = "${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib";
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";
}
