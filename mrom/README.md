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
│   ├── fixed_0000.s          top-level file for the fixed region: org, include
│   │                         "neogeo_z80.inc", RAM equates, one include per piece
│   ├── core/                 vectors, main loop, IRQ/NMI, bank switch, command dispatch
│   ├── channel/              envelope helpers, SSG, FM, ADPCM-A, ADPCM-B, channel-type tables
│   ├── seq/                  track allocation and the sequence opcode interpreter
│   ├── data/                 bank pointer table, lookup pages, note map, sequence pointers, tail
│   ├── bank2_8000.s … bank7_8000.s   (still just `include "raw/…"`)
│   └── raw/                  z80dasm output per region (generated; input of split_region.py)
├── include/
│   └── neogeo_z80.inc        I/O ports, bank windows, YM2610 register numbers
├── map/
│   ├── fixed_0000_blocks.txt code/data blocks for z80dasm -b
│   ├── fixed_0000_syms.txt   semantic names (code labels + RAM) for z80dasm -S
│   └── fixed_0000_split.txt  address ranges -> asm/<subsystem>/<file>.s
├── tools/
│   ├── mrom_regions.py       region table (name, file offset, size, Z80 org)
│   ├── z80trace.py           byte-level code tracer (builds the code/data map)
│   ├── trace_code.py         older tracer over z80dasm text (kept for reference)
│   ├── gen_mrom_asm.py       rom -> asm/raw/*.s applying map/<region>_{blocks,syms}.txt
│   ├── split_region.py       asm/raw/<region>.s -> asm/<subsystem>/*.s per map/<region>_split.txt
│   └── build_mrom.py         assembles every region and compares with the ROM
└── README.md
```

## Workflow

```bash
scripts/setup_mrom.sh internal/mslug.zip       # → rom/201-m1.bin (git-ignored)
python3 mrom/tools/z80trace.py                 # code map (add roots as new jump tables appear)
#   edit mrom/map/fixed_0000_blocks.txt / _syms.txt / _split.txt
python3 mrom/tools/gen_mrom_asm.py             # regenerate asm/raw/*.s with blocks + names
python3 mrom/tools/split_region.py fixed_0000  # re-carve asm/<subsystem>/*.s
python3 mrom/tools/build_mrom.py               # ✔ IDÉNTICA expected
```

The carved files are regenerated from the map files, so naming work goes
into `map/fixed_0000_syms.txt` (one `Name_ADDR: equ 0xADDR` per routine or
RAM variable) rather than into the `.s` files directly.  Hand-written
rewrites (e.g. replacing `defb` tables with structured data) go into the
carved files once a range is considered final; from then on that range is
excluded from the split table.  The rebuilt image must stay byte-identical
at all times.

## RAM map (F800-FFFF)

| Address | Name | Use |
|---|---|---|
| `F800` | `wTick` | main loop counter |
| `F801` | `wTickPending` | timer-A ticks waiting to be processed (max 2) |
| `F805` | `wMasterVolume` | FF = full |
| `F808` | `wFadeFlag` | fade-out in progress |
| `F809` | `wTempoStep` | ticks per sequencer step |
| `F80B` | `wYmPortLatch` | port/reg pair saved by RST 8 |
| `F80F/F810` | `wNmiRingRd/Wr` | indices into the 16-byte NMI ring at `FFF0` |
| `F811` | `wCurBank` | file bank mapped at 8000 |
| `F815-F81C` | `wSoundId`, `wSfxPending`, `wSoundParam`, `wMusicId`, `wCmdCounter`, `wSfxId`, `wCmdType` | command state |
| `F81D` | `wRngState` | PRNG state (`Rng_Next_2300`, mixes in R) |
| `F81F/F86B/F8B7` | `wSSG_ChA/B/C` | 3 SSG channels, 0x4C bytes each |
| `F907/F96E/F9D5/FA3C` | `wFM_Ch1..4` | 4 FM channels, 0x67 bytes each |
| `FAA5 … FB72` | `wPCMA_Ch1..6` | 6 ADPCM-A channels, 0x29 bytes each |
| `FB9B` | `wPCMB_Ch` | ADPCM-B channel |
| `FBF5-FBF8` | `wNoteTmp/Len/Vol/Gate` | scratch for note events |
| `FBFC` | `wMusicTracks` | 14 music tracks × 0x23 bytes |
| `FDF2` | `wSfxTracks` | 8 SFX tracks × 0x23 bytes |
| `FF0A` | `wCurTrack` | track being stepped |
| `FF0D` | `wSeqPtr` | sequence read pointer |
| `FFF0` | `wNmiRing` / stack top | 16-byte command ring; SP starts here and grows down |

## Status

- [x] ROM extracted, regions identified, round-trip disassembly → assembly
      byte-identical for all 7 regions (fixed + banks 2–7).
- [x] Entry points annotated (`0000` reset, `0008` RST 8 = YM write, `0038`
      IRQ → `IRQ_Timer_1E7A`, `0066` NMI → `NMI_Command_1ED6`, `Main_Init_1E00`).
- [x] Hardware equates in `include/neogeo_z80.inc`; RAM map in `map/fixed_0000_syms.txt`.
- [x] Code separated from data in the fixed region (14,288 B code, 74 blocks).
- [x] Fixed region carved into 18 files by subsystem (`core/`, `channel/`, `seq/`, `data/`).
- [x] 126 routines/tables with semantic names (command dispatcher, channel
      update entry points, bank switch, sequencer, 26 sequence opcodes).
- [ ] Name the remaining ~460 internal labels (per-channel update bodies).
- [ ] Document the sequence format and instrument tables; analyse banks 2–7
      (sequence data, FM instrument patches, ADPCM sample tables).
- [ ] Replace raw `defb` tables with structured data where the format is understood.
