# This file was generated by nvfetcher, please do not modify it manually.
{ fetchgit, fetchurl, fetchFromGitHub, dockerTools }:
{
  arithmetic = {
    pname = "arithmetic";
    version = "4a81e23e1794844b36c53385d343475d4d7eca49";
    src = fetchFromGitHub {
      owner = "sequencer";
      repo = "arithmetic";
      rev = "4a81e23e1794844b36c53385d343475d4d7eca49";
      fetchSubmodules = false;
      sha256 = "sha256-tQwzECNOXhuKzpwRD+iKSEJYl1/wlhMQTJULJSCdTrY=";
    };
    date = "2024-01-23";
  };
  berkeley-hardfloat = {
    pname = "berkeley-hardfloat";
    version = "b3c8a38c286101973b3bc071f7918392343faba7";
    src = fetchFromGitHub {
      owner = "ucb-bar";
      repo = "berkeley-hardfloat";
      rev = "b3c8a38c286101973b3bc071f7918392343faba7";
      fetchSubmodules = false;
      sha256 = "sha256-3j6K/qFuH8PqJT6zHVTIphq9HWxmSGoIqDo9GV1bxmU=";
    };
    date = "2023-10-25";
  };
  chisel = {
    pname = "chisel";
    version = "ae5434977a0c0ede55a46847bfcbc8dbf2286c35";
    src = fetchFromGitHub {
      owner = "chipsalliance";
      repo = "chisel";
      rev = "ae5434977a0c0ede55a46847bfcbc8dbf2286c35";
      fetchSubmodules = false;
      sha256 = "sha256-POPpNMnbe4IidbqSlrgBzWHRn6eeL6gh+MuT3v6bw2w=";
    };
    date = "2024-07-12";
  };
  chisel-interface = {
    pname = "chisel-interface";
    version = "79703e44fb6010a535e6750249f6be0471fa0046";
    src = fetchFromGitHub {
      owner = "chipsalliance";
      repo = "chisel-interface";
      rev = "79703e44fb6010a535e6750249f6be0471fa0046";
      fetchSubmodules = false;
      sha256 = "sha256-1VTN2OfsauJvf+JM/j0uhvixD0+aCnyGOoO74YrCFPA=";
    };
    date = "2024-06-17";
  };
  riscv-opcodes = {
    pname = "riscv-opcodes";
    version = "9fa26954e79d4403eedcbe1b35395001bbbeb8b1";
    src = fetchFromGitHub {
      owner = "riscv";
      repo = "riscv-opcodes";
      rev = "9fa26954e79d4403eedcbe1b35395001bbbeb8b1";
      fetchSubmodules = false;
      sha256 = "sha256-Gt3v8/VVNhB4IFL7kud8Y7EnSM2/2H4urV1AmBviP9E=";
    };
    date = "2024-04-10";
  };
  rvdecoderdb = {
    pname = "rvdecoderdb";
    version = "d65525e7e18004b0877d8fbe2c435296ab986f44";
    src = fetchFromGitHub {
      owner = "sequencer";
      repo = "rvdecoderdb";
      rev = "d65525e7e18004b0877d8fbe2c435296ab986f44";
      fetchSubmodules = false;
      sha256 = "sha256-MzEoFjyUgarR62ux4ngYNFOgvAoeasdr1EVhaCvuh+Q=";
    };
    date = "2024-01-28";
  };
}
