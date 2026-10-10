# M-ROM — Z80 sound driver (`201-m1.bin`)

Decompilation of the Metal Slug 1 sound program that runs on the Neo Geo's
Z80 and drives the YM2610. Separate from the 68000 P-ROM work in `asm/` and
`src/`; it has its own toolchain and build.

| | |
|---|---|
| ROM | `201-m1.bin`, 131,072 B, MD5 `6449db0459bcf7584348792009e8f12c` |
| CPU | Zilog Z80 @ 4 MHz |
| Toolchain | `z80asm` (assembler) + `z80dasm` (disassembler), both Debian packages |
| Build | `python3 mrom/tools/build_mrom.py` → `build/mrom/201-m1_rebuilt.bin` |

## Memory map (Z80 side)

| Z80 address | Contents |
|---|---|
| `0000-7FFF` | fixed region = file `0x00000-0x07FFF` |
| `8000-BFFF` | 16 KiB window, one of file banks 2–7 (bank N = file `N*0x4000`; `0x08000-0x1FFFF`) |
| `C000-DFFF` | 8 KiB window (ports `0x09`) — unused by this driver so far |
| `E000-EFFF`, `F000-F7FF` | 4 / 2 KiB windows (ports `0x0A/0x0B`) |
| `F800-FFFF` | 2 KiB work RAM (`F800` tick, `F801..F803` command queue, `F80B` ch mask, `F810` NMI ring index, `F811` current bank, `FFF0..FFFF` NMI command ring) |

Bank switching is done by *reading* I/O ports `0x08..0x0B` with the bank
number in the upper address byte (`ld b,n ; ld c,0Bh ; in a,(c)`), see
`sub_1f7dh` / `sub_1fa6h` in the fixed region. Pointers stored in the
driver's tables are 24-bit **linear file addresses** (`sub_1fa6h` derives
bank = addr >> 14 and window offset = addr & 0x3FFF); the driver starts in
bank 2, so the window only ever shows file banks 2–7 (file `0x4000-0x7FFF`
is the upper half of the fixed region, not a bank). Port `0x00` reads the
68000→Z80 command latch, `out (0Ch),a` writes the reply, `out (18h)`
disables the NMI and `out (08h)` enables it. Port `0x04-0x07` = YM2610.

## Layout

```
mrom/
├── asm/
│   ├── fixed_0000.s          top-level file per region (just `include`s)
│   ├── bank2_8000.s … bank7_8000.s
│   └── raw/                  z80dasm output, one file per region (do not edit)
├── include/                  shared equates (hardware ports, RAM map)
├── tools/
│   ├── mrom_regions.py       region table (name, file offset, size, Z80 org)
│   ├── gen_mrom_asm.py       regenerates asm/raw/*.s from rom/201-m1.bin
│   └── build_mrom.py         assembles every region and compares with the ROM
└── README.md
```

## Workflow

```bash
scripts/setup_mrom.sh internal/mslug.zip     # → rom/201-m1.bin (git-ignored)
python3 mrom/tools/gen_mrom_asm.py           # raw disassembly (already committed)
python3 mrom/tools/build_mrom.py             # ✔ IDÉNTICA expected
```

Progress is made by carving functions out of `asm/raw/<region>.s` into
`asm/<subsystem>/<name>_<z80addr>.s`, replacing the carved range in the
top-level region file with an `include`, naming labels semantically and
re-running `build_mrom.py` after each step. The rebuilt image must stay
byte-identical at all times.

## Status

- [x] ROM extracted, regions identified, round-trip disassembly → assembly
      byte-identical for all 7 regions (fixed + banks 2–7).
- [ ] Entry points annotated (`0000` reset, `0008` RST 8 = YM write, `0038`
      IRQ → `1e7a`, `0066` NMI → `1ed6`, `1e00` main init/loop).
- [ ] Hardware/RAM equates in `include/`.
- [ ] Separate code from data (sequence tables, instrument data, ADPCM
      tables occupy most of the banks).
- [ ] Semantic names for the driver core (command dispatcher, channel
      update, bank switch, YM2610 writes, ADPCM-A/B playback).
