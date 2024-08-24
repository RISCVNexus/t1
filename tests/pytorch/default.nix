{ lib
, linkerScript
, buddy-mlir
, makeBuilder
, findAndBuild
, getTestRequiredFeatures
, t1main
, callPackage
}:

let
  builder = makeBuilder { casePrefix = "pytorch"; };
  build = { caseName, sourcePath }:
    callPackage (sourcePath + "/build.nix") {
      buildBuddyE2ETest = { optPhase, ... }@overrides: builder
        ({
          inherit caseName;

          passthru.featuresRequired = getTestRequiredFeatures sourcePath;

          nativeBuildInputs = [ buddy-mlir.pyenv buddy-mlir ];

          src = sourcePath;

          configurePhase = ''
            declare -A optArtifacts translateArtifacts llcArtifacts
          '';

          translatePhase = ''
            if [[ -z "$optArtifacts" ]]; then
              echo "optPhase doesn't produce optArtifacts, abort" >&2
              exit 1
            fi

            for mlir in ''${optArtifacts[@]}; do
              echo "Translating $mlir"
              buddy-translate --buddy-to-llvmir "$mlir" -o "$mlir.ll"

              translateArtifacts+=("$mlir.ll")
            done
          '';

          llcPhase = ''
            if [[ -z "$translateArtifacts" ]]; then
              echo "translatePhase doesn't produce translateArtifacts, abort" >&2
              exit 1
            fi

            for llvmir in ''${translateArtifacts[@]}; do
              echo "Compiling $llvmir"
              buddy-llc "$llvmir" \
                -mtriple=riscv32 \
                -target-abi=ilp32f \
                -mattr=+m,+f,+zve32f \
                -riscv-v-vector-bits-min=128 \
                --filetype=obj \
                -o "$llvmir.o"

              llcArtifacts+=("$llvmir.o")
            done
          '';

          linkPhase = ''
            if [[ -z "$llcArtifacts" ]]; then
              echo "llcPhase doesn't produce any llcArtifacts" >&2
              exit 1
            fi

            echo "Building final binary"
            mkdir -p _include
            cp ${./memref.hpp} _include/memref.hpp

            $CXX -nostdlib -I _include -c ${caseName}.cc -o host.o
            $CC -T${linkerScript} \
              host.o ''${llcArtifacts[@]} ${t1main} \
              -o $pname.elf
          '';

          buildPhase = ''
            runHook preBuild

            runPhase optPhase
            runPhase translatePhase
            runPhase llcPhase
            runPhase linkPhase

            runHook postBuild
          '';
        } // overrides);
    };
in
findAndBuild ./. build
