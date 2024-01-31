#include <math.h>
#include <stdint.h>
#include <stdio.h>

const int vl = 128;

const int max_bits = 4096;
const int word_bits = 16;
const int way = vl / word_bits / 2; // 4
const int s = 65; // ceil((max_bits / word_bits + 1) / way)

// 4096 / 16 = 64
// 64 + 4 comes from s * way = 260
const uint32_t a[256 + 4] = {0xFFFF, 0xFFFF};
const uint32_t b[256 + 4] = {0xFFFF, 0xFFFF};
// for mmm_mem.pl, we need to ensure abr1 = 0
uint32_t abr1[256 + 4] = {0};
const uint32_t p[256 + 4] = {0xa285, 0xfbcb, 0x2b62, 0xc04c, 0x08e6, 0x5b0b, 0x44c7, 0x7403, 0x1291, 0x1a5f, 0x3100, 0x34d4, 0x7006, 0xa589, 0x5ee5, 0xa841, 0xc2ef, 0x5c18, 0x70c9, 0x3b54, 0x2d96, 0xb89c, 0xaff1, 0x466a, 0x9efa, 0xb6dd, 0x7749, 0x4bbc, 0x38b3, 0x93fc, 0xa629, 0x3f00, 0x88b9, 0xcbb2, 0xa694, 0x1a52, 0xc96b, 0xbe5d, 0x2fe8, 0xadfd, 0xa394, 0x57ed, 0x4b79, 0x9b5f, 0xd37e, 0xa94f, 0x5559, 0xea7d, 0xd1a9, 0x753b, 0xe079, 0x81de, 0xfe08, 0x738a, 0x9438, 0x845c, 0x3358, 0x7ab4, 0xd10c, 0xfe88, 0xfc76, 0x81cb, 0xfd86, 0x0eb3, 0x7887, 0x64e1, 0x7a8f, 0x5dd8, 0xb6df, 0x97cd, 0x01af, 0x0b77, 0xb51b, 0xb900, 0xd6ba, 0xd89b, 0xe71c, 0x8eba, 0xc1e7, 0x91b1, 0xb0db, 0x0c73, 0xe5e2, 0x1df4, 0x87c7, 0xd94a, 0x57bb, 0xcb1f, 0x85ae, 0xbd7e, 0xb359, 0x753c, 0x41e3, 0xb515, 0x9896, 0xdddd, 0x8ee9, 0x90b1, 0xdd10, 0x0d41, 0x5496, 0x7aef, 0xfaf3, 0x5ff1, 0x249c, 0x9549, 0xd7cf, 0xab30, 0x5a92, 0x6532, 0x9f61, 0x471d, 0x212f, 0x7ad1, 0x7034, 0x4fe8, 0x7a6b, 0x793d, 0x09d8, 0x1dcd, 0xb60b, 0xe2fe, 0x01e0, 0xd2db, 0xea7f, 0xf5d0, 0x7759, 0x2242, 0xe701, 0x1a09, 0xf53a, 0x710f, 0x5224, 0x56f4, 0xf439, 0xb1e3, 0xffdd, 0xac3b, 0x78ee, 0x4ade, 0x20a5, 0x962d, 0x6fa4, 0x1f4c, 0x13e6, 0x4476, 0x9d8f, 0x2ea2, 0x2769, 0xd6d6, 0x88a1, 0xf82d, 0x2540, 0xb340, 0x03de, 0x7565, 0x7418, 0xd207, 0x55e1, 0x0c8d, 0x3547, 0xf16b, 0xf073, 0x0c77, 0xe3db, 0xc58b, 0x7073, 0xb415, 0x7f44, 0xa294, 0xdae5, 0x69e4, 0x76cf, 0x169e, 0xa3fc, 0x9412, 0x0795, 0x80b3, 0x3b23, 0xede4, 0xa2e5, 0x7a61, 0xbe6f, 0x0b36, 0x033a, 0x1a06, 0x4b08, 0x3232, 0x1a5b, 0xb722, 0x4ee3, 0x12ab, 0xe4d9, 0xe94c, 0x9e3c, 0xcf07, 0x16c0, 0x3dee, 0xb38c, 0x067c, 0x0c58, 0x7868, 0x3705, 0x915b, 0x5084, 0x6db6, 0x0a94, 0x3c3a, 0x0f02, 0x123c, 0xfcc9, 0xa81f, 0x9a09, 0xa3e5, 0xbd77, 0x324b, 0x4b30, 0x501f, 0xf4c3, 0xd3f8, 0x659c, 0x3cc0, 0xf7d4, 0x2fc2, 0xe326, 0x276d, 0xca2f, 0xbb0d, 0x04a4, 0x814f, 0x8db7, 0xf4e7, 0x467f, 0x0686, 0x42ff, 0xb319, 0xaf84, 0xf027, 0x0ff5, 0x5558, 0x5565, 0xe9fd, 0x4573, 0x873d, 0x3491, 0xa800, 0xbc9d, 0x484f, 0x34bd, 0x34bf, 0x6d55, 0xb44e, 0x5eb5, 0xc68c, 0x50a7, 0xc996};
const uint32_t mu = 0xf9b3;

// 32 = 2 * word_bits
// mu is of 16 bits
// R is 2 ** (max_bits + word_bits)
void mmm(uint32_t* r, const uint32_t* a, const uint32_t* b, const uint32_t* p, const uint32_t mu);

int test() {
  mmm(abr1, a, b, p, mu);
  // for(int i = 0; i != 260; ++i) {
  //   printf("%04lX ", abr1[i]);
  // }
  return 0;
}
