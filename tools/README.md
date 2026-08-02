# `tools/` — build and analysis scripts

Everything here is plain Python 3 (see [`requirements.txt`](../requirements.txt)
at the repo root for dependencies) plus one vendored diff tool. Scripts
that need the ROM read `build/mslug_prom.bin`, produced by
`../scripts/setup.sh` — see the root [README](../README.md#building).

Historical one-off batch generators (used to bulk-produce the mechanical
`src/*.c` families in early waves, e.g. `gen_task_setters_w.py`) live in
[`../scripts/legacy/`](../scripts/legacy/), not here — this directory is
only for tools that are still part of the active workflow.

## Core build/verification pipeline

| Script | Needs ROM? | Purpose |
|---|:-:|---|
| [`registry.py`](registry.py) | no | Master `REGISTRY` list: `(name, cpu_addr, size, source_file)` for every matched function. Not a script — imported by everything else. |
| [`symbols.py`](symbols.py) | no | `SYMBOLS` dict: `{cpu_addr: name}` for external placeholders resolved via linker `--defsym`. Also imported, not run directly. See [`docs/CONVENTIONS.md`](../docs/CONVENTIONS.md) for the duplicate-key gotcha before editing this file. |
| [`registry_lint.py`](registry_lint.py) | no | Static audit of `registry.py`/`symbols.py` — overlaps, duplicate addresses/names, odd/bad sizes, symbol clashes. Run this after *every* edit to either file; it's the project's "CI check" and the only one that actually runs in `.github/workflows/ci.yml`. |
| [`match_batch.py`](match_batch.py) | **yes** | The source of truth. Compiles `src/*.c` (GCC bare-metal 68000) and assembles `asm/*.s` (GAS), links every `REGISTRY` entry at its absolute LMA, `objcopy`s each section out, and byte-compares it against the corresponding slice of `build/mslug_prom.bin`. Exit code and printed `MATCHED`/`BYTES` counters are what every wave reports progress against. |

## Candidate selection (what to decompile next)

| Script | Needs ROM? | Purpose |
|---|:-:|---|
| [`scan_unmatched_callees.py`](scan_unmatched_callees.py) | **yes** | Priority queue ordered by **popularity**: how many already-matched functions call each unmatched target. Best for steady, low-risk, high-connectivity progress. `python3 tools/scan_unmatched_callees.py --top 30`. |
| [`rank_candidates.py`](rank_candidates.py) | **yes** | Priority queue ordered by **size**: `score = eff_size * (1 + log2(callers))`, where `eff_size = min(linear_disasm_size, gap_to_next_matched_entry)`. Flags `TRUNCA->vecino matcheado` when linear disassembly overruns into an already-matched neighbour (the real function is smaller than it looks). Best for sessions that specifically want to knock out the largest remaining functions. `python3 tools/rank_candidates.py --top 15`. |
| [`inspect_candidates.py`](inspect_candidates.py) | **yes** | Dumps disassembly + hex bytes for a list of candidate addresses, up to the first unconditional `rts`/`rte`/`jmp`. `python3 tools/inspect_candidates.py 0x028D8E 0x043fac ...`. |
| [`measure_coverage.py`](measure_coverage.py) | **yes** | Entropy + whitelisted-opcode-density heuristic estimating the "real code %" of the ROM, as opposed to `match_batch.py`'s raw bytes-matched-vs-2 MiB metric (most of the ROM is data: palettes, tilemaps, level scripts). Feeds [`docs/COVERAGE.md`](../docs/COVERAGE.md). |
| [`scan_input_mask_thunks.py`](scan_input_mask_thunks.py) | **yes** | One-off, cluster-specific scanner for the `InputMask` dispatcher table at `$05CDFC..$05CFA7`. Kept for reference/reproducibility of that wave, not general-purpose. |

## Vendored

| Path | Purpose |
|---|---|
| [`asm-differ/diff.py`](asm-differ/) | Vendored [`simonlindholm/asm-differ`](https://github.com/simonlindholm/asm-differ) (m68k backend). Visual instruction-level diff between the built and target bytes for one function: `python3 tools/asm-differ/diff.py -mwo SymbolName`. Retains its own upstream license — see the file header. Configuration lives in [`../diff_settings.py`](../diff_settings.py). |
