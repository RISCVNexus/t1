{ stdenv
, runCommand
, lib
, cmake
, coreutils
, python3
, git
, fetchFromGitHub
, ninja
}:

let
  llvm = stdenv.mkDerivation rec {
    pname = "llvm-project";
    version = "e31d27e46048ccc3294d6b215dc778b3390e7834";
    requiredSystemFeatures = [ "big-parallel" ];
    nativeBuildInputs = [ cmake ninja python3 ];
    src = fetchFromGitHub {
      owner = "llvm";
      repo = pname;
      rev = version;
      hash = "sha256-CM3+amf2SpOiUBzdnO7sryTwmGcC0NVabNNvuatcCDQ=";
    };
    cmakeDir = "../llvm";
    cmakeFlags = [
      "-DLLVM_ENABLE_BINDINGS=OFF"
      "-DLLVM_ENABLE_OCAMLDOC=OFF"
      "-DLLVM_BUILD_EXAMPLES=OFF"
      "-DLLVM_ENABLE_PROJECTS=mlir;clang"
      "-DLLVM_TARGETS_TO_BUILD=host;RISCV"
      "-DLLVM_INSTALL_UTILS=ON"
    ];
    checkTarget = "check-mlir check-clang";
    postInstall = ''
      cp include/llvm/Config/config.h $out/include/llvm/Config
    '';
  };
  mlir_dir = runCommand "mlir_dir" { } ''
    mkdir -p $out
    ln -s ${llvm.src}/* $out
    cp -r ${llvm} $out/build
  '';

in
stdenv.mkDerivation rec {
  pname = "buddy-mlir";
  version = "a357ed2665686651ce3d85be598a29cb0530f877";
  src = fetchFromGitHub {
    owner = "buddy-compiler";
    repo = pname;
    rev = version;
    hash = "sha256-bX6eI23Qa1XdahsJbgX+orVhONRwa9n0BUO5B7ZYoXk=";
  };

  requiredSystemFeatures = [ "big-parallel" ];

  nativeBuildInputs = [ cmake ninja ];

  passthru = { inherit llvm; };

  cmakeFlags = [
    "-DMLIR_DIR=${mlir_dir}/build/lib/cmake/mlir"
    "-DLLVM_DIR=${mlir_dir}/build/lib/cmake/llvm"
  ];
}

