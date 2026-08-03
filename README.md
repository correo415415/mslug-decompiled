# Metal Slug 1 — Matching Decompilation

[![CI](https://github.com/correo415415/mslug-decompiled/actions/workflows/ci.yml/badge.svg)](https://github.com/correo415415/mslug-decompiled/actions/workflows/ci.yml)

A work-in-progress matching decompilation of **Metal Slug: Super Vehicle-001**
(Nazca / SNK, 1996) for the SNK Neo Geo MVS/AES arcade platform.

The goal is a byte-exact reconstruction of the game's program ROM (`201-p1.bin`)
from a mix of hand-written C and annotated 68000 assembly, following the same
methodology as [n64decomp/sm64][sm64] and [zeldaret/oot][oot].

**No copyrighted content is redistributed by this repository.** You must own a
legal copy of Metal Slug to build the ROM; the build system reads your ROM at
runtime and never bundles it.

[sm64]: https://github.com/n64decomp/sm64
[oot]: https://github.com/zeldaret/oot

---

## Current status

| Metric | Value |
|---|---:|
| Matched functions | **3 361 / 3 361** registered |
| Matched bytes | **62 466 / 62 466** registered |
| P ROM coverage | **62 466 / 2 097 152 B** (2.98 %) |
| Processed P ROM MD5 (target) | `816b3f74c76b3373993407615f1850fe` |

Matched functions are guaranteed to reassemble to bytes that are bitwise
identical to the original ROM. See [`CHANGELOG.md`](CHANGELOG.md) for a
headline summary of recent waves and [`docs/PROGRESO.md`](docs/PROGRESO.md)
for the full function-by-function log (in Spanish). Coverage will continue to
grow as the callgraph is walked outward from the already-matched entry points
and as large, self-contained functions are tackled directly (see
`tools/rank_candidates.py`).

---

## Repository layout

```
mslug/
├── .github/workflows/      CI: registry_lint.py + syntax check (no ROM needed)
├── src/                    Decompiled C sources (matched by GCC bare-metal 68000)
├── asm/                    Hand-written 68000 assembly sources (matched by GAS)
│   └── non_matchings/      Raw ROM dumps by address for functions not yet analysed
├── include/                Public headers (mslug.h, mslug_regs.h)
├── linker/                 Linker scripts (neogeo.ld, neogeo_matched.ld)
├── tools/                  Active tooling used by the build — see tools/README.md
│   ├── match_batch.py      Compile + link + byte-compare every registered function
│   ├── registry.py         Master registry: (symbol, cpu_addr, size, source)
│   ├── symbols.py          External symbol table (linker --defsym)
│   ├── registry_lint.py    Static structural audit of registry.py/symbols.py
│   ├── scan_unmatched_callees.py   Priority queue ordered by caller count
│   ├── rank_candidates.py  Priority queue ordered by function size
│   ├── measure_coverage.py Real-code-% heuristic feeding docs/COVERAGE.md
│   └── asm-differ/         Vendored simonlindholm/asm-differ (m68k backend)
├── scripts/                One-shot helper scripts
│   ├── setup.sh            Process baserom into build/mslug_prom.bin + verify
│   └── legacy/             Historical batch generators (Waves A–R)
├── docs/                   Design notes and reversing logs
│   ├── CONVENTIONS.md      Naming/registry/promotion conventions — read before editing
│   ├── COVERAGE.md         Real coverage analysis
│   └── PROGRESO.md         Detailed function-by-function progress log (Spanish)
├── build/                  Build artefacts (git-ignored except for reports)
├── diff_settings.py        Configuration for asm-differ
├── requirements.txt        Python dependencies (capstone, colorama, ...)
├── Makefile
├── CONTRIBUTING.md         Workflow for matching a new function
├── CHANGELOG.md            Headline summary of notable changes
└── README.md
```

---

## How it works

Metal Slug 1 is not a typical C-compiled title. Reverse-engineering the ROM
reveals structural evidence that most of the game logic was written directly
in Motorola 68000 assembly:

- Functions share epilogues (conditional branches falling through into the
  first byte of the next function).
- Values are returned via the CCR flags, not in `d0`.
- Parameter passing uses fixed register conventions (`a6` for the current
  entity, `a2` for sprite command blocks) that no C ABI would emit.
- Functions abut with no alignment padding.

Because a matching decompilation requires bit-exact output, the project uses
a **dual-frontend** model:

- **`src/*.c`** for functions whose GCC output happens to match the ROM
  byte-for-byte. This covers the ~2 900 sub-routines that are structurally
  simple: return stubs, tail-call thunks, CCR helpers, task-handler
  trampolines, dispatch tables, etc.
- **`asm/*.s`** for functions whose exact instruction sequence is not
  reproducible from C. Each `.s` file documents the conceptual C signature,
  the parameter conventions, and any structural evidence found while
  reversing the function.

Both frontends feed the same linker script, which places every registered
function at its absolute CPU address. The matcher then extracts each
function with `objcopy` and byte-compares it against the corresponding
slice of the processed P ROM.

---

## Toolchain

| Component | Version | Purpose |
|---|---|---|
| `m68k-linux-gnu-gcc` | 13.3.0 (Debian/Ubuntu) | Bare-metal C compiler for 68000 |
| `m68k-linux-gnu-as` | binutils 2.42 | GAS assembler for `.s` files |
| `m68k-linux-gnu-ld` | binutils 2.42 | Linker with custom script |
| `m68k-linux-gnu-objcopy` | binutils 2.42 | Section extraction for matching |
| `python3` | ≥ 3.10 | Build orchestration and matcher |
| `capstone` (Python) | ≥ 5.0 | Disassembly for the callee scanner |
| [asm-differ][differ] | vendored | Visual diff between built and target |

[differ]: https://github.com/simonlindholm/asm-differ

Compile flags used by the matcher (equivalent to `ngdevkit`'s
`m68k-neogeo-elf-gcc`, without the newlib runtime):

```
-mcpu=68000 -nostdlib -nostartfiles -ffreestanding -fno-builtin
-fno-exceptions -fomit-frame-pointer -fno-strict-aliasing
-fno-toplevel-reorder -fno-common -Os
-ffunction-sections -fdata-sections
```

---

## Building

### Prerequisites

```bash
sudo apt install -y gcc-m68k-linux-gnu binutils-m68k-linux-gnu \
                    python3 python3-pip
python3 -m pip install --user -r requirements.txt
```

### Bring your own ROM

Place the original program ROM at the project root:

```
rom/201-p1.bin      MD5 b6804bc6be580c80d43d187f6f9d2e7c   (arcade set "mslug")
```

### Process and verify

```bash
./scripts/setup.sh
```

This byte-swaps the ROM and swaps the two 1 MiB banks to produce
`build/mslug_prom.bin`, then verifies its MD5 against the expected
`816b3f74c76b3373993407615f1850fe`.

### Run the matcher

```bash
make match
```

or directly:

```bash
python3 tools/match_batch.py
```

A successful run prints:

```
====================================================================
  MATCHED : 3134/3134 funciones
  BYTES   : 42,190/42,190 (registrados)
  ROM     : 42,190/2,097,152  (2.0118%)
====================================================================
```

Also run the static registry audit (fast, no ROM needed, and what CI runs
on every push):

```bash
python3 tools/registry_lint.py
```

---

## Contributing a match

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full step-by-step
workflow (candidate selection, classification, registration, promotion of
placeholders, verification, and the git/PR process), and
[`docs/CONVENTIONS.md`](docs/CONVENTIONS.md) for the naming and registry
conventions behind it. Quick-reference tool inventory: see
[`tools/README.md`](tools/README.md).

---

## Non-goals

- This is **not** a source port. The output is a ROM image, not a native PC
  build.
- This is **not** a rewrite. Reimplementations that "look like" the original
  are not accepted; every matched function must produce the exact bytes of
  the original ROM.
- This is **not** an emulator. Run the resulting ROM in
  [MAME](https://www.mamedev.org/) or [FBNeo](https://github.com/finalburnneo/FBNeo).

---

## Legal

- Metal Slug and all associated trademarks and copyrights are the property of
  SNK Corporation. This project is not affiliated with, endorsed by, or
  connected to SNK in any way.
- No ROM data, sprite data, sound data, or any other copyrighted asset from
  the game is committed to this repository. The build system operates on a
  ROM that the user supplies.
- The source code, tools and documentation written for this project are
  released under the terms of the [MIT license](LICENSE), with the sole
  exception of the vendored `tools/asm-differ/` which retains its original
  license (see the file header).

---

## Acknowledgements

- The [NeoGeo Development Wiki](https://wiki.neogeodev.org/) for hardware
  documentation.
- Damien Ciabrini's [ngdevkit](https://github.com/dciabrin/ngdevkit) for
  establishing the modern GCC toolchain for the Neo Geo.
- Simon Lindholm's [asm-differ](https://github.com/simonlindholm/asm-differ)
  for the m68k diff backend.
- The N64 decomp community for pioneering the matching-decompilation
  methodology this project imitates.
