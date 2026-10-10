"""Layout of 201-m1.bin (Metal Slug 1 Z80 sound program, 128 KiB)."""
EXPECT_MD5 = "6449db0459bcf7584348792009e8f12c"

# (name, file offset, size, Z80 origin)
REGIONS = [
    ("fixed_0000", 0x00000, 0x8000, 0x0000),   # always mapped at 0x0000-0x7FFF
] + [
    (f"bank{i}_8000", i * 0x4000, 0x4000, 0x8000)   # 16 KiB window at 0x8000-0xBFFF
    for i in range(1, 8)
]
