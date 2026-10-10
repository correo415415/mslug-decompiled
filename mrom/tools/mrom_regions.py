"""Layout of 201-m1.bin (Metal Slug 1 Z80 sound program, 128 KiB).

Z80 0x0000-0x7FFF is always file 0x00000-0x07FFF (fixed region).
Z80 0x8000-0xBFFF is a 16 KiB window onto file bank N*0x4000, N = 2..7,
selected by reading port 0x0B with N in the upper address byte
(`ld b,N ; ld c,0Bh ; in a,(c)`; see sub_1f7dh / sub_1fa6h).
Pointers in the driver's tables are 24-bit *linear file addresses*
(bank = addr >> 14, window offset = (addr & 0x3FFF) | 0x8000).
"""
EXPECT_MD5 = "6449db0459bcf7584348792009e8f12c"

# (name, file offset, size, Z80 origin)
REGIONS = [
    ("fixed_0000", 0x00000, 0x8000, 0x0000),
] + [
    (f"bank{i}_8000", i * 0x4000, 0x4000, 0x8000)
    for i in range(2, 8)
]
