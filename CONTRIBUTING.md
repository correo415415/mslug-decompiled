# Contributing a match

This document describes the day-to-day workflow for turning one more
unmatched ROM address into a verified, byte-exact entry in the registry.
For the *why* behind the conventions used below (naming, wave numbers,
`.equ` aliases, the `symbols.py` duplicate-key gotcha, etc.) see
[`docs/CONVENTIONS.md`](docs/CONVENTIONS.md).

## 0. Prerequisites

```bash
sudo apt install -y gcc-m68k-linux-gnu binutils-m68k-linux-gnu python3 python3-pip
python3 -m pip install --user -r requirements.txt
```

You also need your own legal copy of the ROM at `rom/201-p1.bin`
(MD5 `b6804bc6be580c80d43d187f6f9d2e7c`). Run `./scripts/setup.sh` once to
produce `build/mslug_prom.bin` (MD5 `816b3f74c76b3373993407615f1850fe`),
which is what every other tool below reads.

## 1. Pick a candidate

Two priority queues are available, depending on what you're optimizing for:

```bash
# Ordered by number of already-matched callers (closes the most call-graph
# edges per function — good for steady, low-risk progress).
python3 tools/scan_unmatched_callees.py --top 30

# Ordered by score = effective_size * (1 + log2(callers)) — good for
# sessions focused on decompiling the largest remaining functions.
python3 tools/rank_candidates.py --top 15
```

`rank_candidates.py` flags `TRUNCA->vecino matcheado` when the naive linear
disassembly runs into an already-matched neighbour — that means the real
function is smaller than it looks and you should trust the gap, not the
raw linear-disassembly length.

## 2. Read and classify the function

```bash
python3 tools/inspect_candidates.py 0xADDR1 0xADDR2 ...
```

Decide whether the function's exact byte sequence is reproducible by GCC
(`-Os` bare-metal 68000) or requires hand-written assembly:

- **Reproducible by GCC** → write a `.c` function in `src/`. Most
  mechanical patterns (`rts` stubs, `jsr abs.l; rts` thunks, scheduler
  wrappers, dispatch stubs) fall in this bucket.
- **Not reproducible by GCC** (register-passing conventions no C ABI would
  emit, CCR-flag returns, shared epilogues between functions, PC-relative
  addressing baked into the opcode) → write a `.s` file in `asm/`.

Every new source file, C or ASM, must start with a header comment
documenting:

- The address range and byte size.
- The conceptual C signature (even for `.s` files — this is what the
  function *would* look like in C, for readability).
- Every caller found via `scan_unmatched_callees.py`, and whether it is a
  direct symbol reference or an indirect one behind a local `.equ` alias.
- Any register-passing convention discovered (`a6` = current entity,
  `a2` = sprite command block, etc. — see `include/mslug.h`).

## 3. Register the function

Add one line to `tools/registry.py`:

```python
("MyNewFunction", 0x0ABCDE, 42, "my_new_function.s"),
```

Put it in a wave-tagged comment block near functions from the same
subsystem/cluster if one exists, or at the end of the file otherwise.

If the function was previously a placeholder in `tools/symbols.py`
(`FUN_xxxxxx`, `Sub_xxxxxx`, `ThunkTarget_xxxxxx`), **promote** it: remove
the `SYMBOLS` entry and leave a one-line comment recording the promotion
(`# 0xADDR promovido a MyNewFunction en registry (Wave NN#k)`), then rename
every real caller to point at the new canonical name — see
[`docs/CONVENTIONS.md § Promoting a placeholder`](docs/CONVENTIONS.md#promoting-a-placeholder)
for the exact steps and the pitfalls to avoid.

If the function references *other* still-unmatched external addresses,
add placeholders for them in `tools/symbols.py` instead.

## 4. Verify

```bash
python3 tools/registry_lint.py   # static checks — must print "0 errores, 0 warnings"
python3 tools/match_batch.py     # full compile + link + byte-compare
```

A clean run of `match_batch.py` increases `MATCHED` by exactly the number
of functions you added, and `BYTES` by exactly their combined size. If the
count is off, or the byte-compare fails, use the visual differ:

```bash
python3 tools/asm-differ/diff.py -mwo MyNewFunction
```

Iterate until the diff is empty. **Never** hand-tune bytes to "look right"
without an actual instruction-level explanation — every mismatch has a
concrete cause (wrong addressing mode, wrong compiler flag, missing
`.equ`, etc.).

## 5. Git workflow

This project follows the standard GenSpark workflow:

1. Work on the `genspark_ai_developer` branch (or a feature branch based
   on it).
2. Commit with a descriptive message referencing the wave (e.g.
   `feat(decomp): Wave RR#1 - MyNewFunction ($0abcde, 42 B)`).
3. `git fetch origin main && git rebase origin/main` before opening/updating
   a PR; resolve conflicts favouring the remote side unless your local
   change is the one under review.
4. Squash local commits into one comprehensive commit
   (`git reset --soft HEAD~N && git commit -m "..."`).
5. Push and open a PR from `genspark_ai_developer` into `main`, including
   the `registry_lint.py` and `match_batch.py` output in the description.

CI (`.github/workflows/ci.yml`) runs `registry_lint.py` and a syntax check
on every push — it cannot run `match_batch.py` because that requires the
copyrighted ROM, which is never available to CI. Treat a green
`match_batch.py` run on your own machine as mandatory before every PR.
