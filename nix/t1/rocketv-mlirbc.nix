{ stdenvNoCC

, espresso
, circt

, elaborator
}:
stdenvNoCC.mkDerivation {
  name = "t1-rocketv-elaborated.mlirbc";

  nativeBuildInputs = [ elaborator espresso circt ];

  buildCommand = ''
    mkdir elaborate
    elaborator rocketemu --target-dir elaborate --rocket-config ${../../rocketv/configs/RocketTile.json}
    firtool elaborate/*.fir \
      --annotation-file elaborate/*.anno.json \
      --emit-bytecode \
      --parse-only \
      -o $out
  '';
}
