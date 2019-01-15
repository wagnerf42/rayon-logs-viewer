with import <nixpkgs> {}; {
  env = stdenv.mkDerivation {
    name = "rayon-logs-viewer";
    buildInputs = [ xorg.libX11 hwloc ];
    LD_LIBRARY_PATH = "${stdenv.lib.makeLibraryPath [ xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi ]}:/run/opengl-driver/lib";
  };
}
