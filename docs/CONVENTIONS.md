# Project conventions

Formal reference for the naming, registry, and promotion conventions used
throughout this decompilation. These evolved organically across ~40 waves
of work (see [`PROGRESO.md`](PROGRESO.md)); this document is the
distilled, current-state version — read it before touching
`tools/registry.py` or `tools/symbols.py`.

## Naming

| Prefix / pattern | Meaning |
|---|---|
| `PascalCase_ADDR` (e.g. `EntitySetSpriteMap`, `PlayerEntity_InitAuxState_032A02`) | A function whose behaviour is understood well enough to give it a semantic name. The trailing `_ADDR` (lowercase hex, no `$`/`0x`) is included whenever the name alone could be ambiguous or when multiple functions share a family name. |
| `FUN_ADDR` | Placeholder for an external reference whose behaviour has **not** been analysed yet. Lives only in `tools/symbols.py`, never in `tools/registry.py`. |
| `Sub_ADDR` | Placeholder for a small helper seen only as a callee (usually a `jsr`/`bsr` target) that hasn't been decompiled yet. Same rules as `FUN_ADDR`. |
| `ThunkTarget_ADDR` | Placeholder specifically for the target of a `jsr abs.l`/`jmp abs.l` thunk family (`JsrAbsThunk_*`, `JmpAbsThunk_*`). |
| `PcThunkTarget_ADDR` | Same as above but for the PC-relative thunk family (`JsrPcThunk_*`). |

All of the placeholder prefixes above are **temporary by convention**: as
soon as a function gets a real name and a `REGISTRY` entry, the
placeholder must be *promoted* (§ below), not left dangling.

## The registry (`tools/registry.py`)

```python
REGISTRY = [
    ("SymbolName", 0x0ABCDE, 42, "source_file.s"),   # (name, cpu_addr, size, source_file)
    ...
]
```

Rules:

- Every entry must pass `tools/match_batch.py` in isolation (comment out
  everything else and it should still link and match — in practice this is
  checked incrementally, one wave at a time).
- `size` is the exact byte count of the function, including any trailing
  padding/alignment bytes that belong to it structurally (e.g. a shared
  epilogue absorbed from a neighbour).
- `source_file` is relative to `src/` (for `.c`) or `asm/` (for `.s`) —
  `match_batch.py` infers the frontend from the extension.
- Group related entries under a `# ---- Wave XX: ...` comment block. This
  is what makes `docs/PROGRESO.md`'s wave table possible to reconstruct
  and keeps `git blame`/`git log -p` readable.
- Run `python3 tools/registry_lint.py` after every edit. It is the
  project's static "CI check" and catches, without needing the ROM:
  - **E1 OVERLAP** — two entries overlap in address space.
  - **E2 DUP-ADDR** / **E3 DUP-NAME** — duplicate address or symbol name.
  - **E4 ODD-ADDR** — odd (non-word-aligned) start address (68000 code is
    always word-aligned; an odd address means a boundary mistake).
  - **E5 BAD-SIZE** — non-positive or implausible size.
  - **E6 SYM-CLASH** — a `REGISTRY` name collides with a `SYMBOLS`
    placeholder at a *different* address (the linker would silently pick
    one).
  - **W1 SYM-DUP-SRC** / **W2 FILE-MISS** — warnings for symbols defined in
    multiple places in the source text, or a `source_file` that doesn't
    exist on disk.

## The external symbol table (`tools/symbols.py`)

```python
SYMBOLS = {
    0x00ADDR: "SomeName",   # {cpu_addr: name}
    ...
}
```

`build_defsyms()` in `tools/match_batch.py` turns every entry into a
`-Wl,--defsym=Name=0xADDR` linker flag, which is how `.s`/`.c` files call
into addresses that don't have a `REGISTRY` entry (and therefore no real
body) yet.

### ⚠️ The duplicate-key gotcha

`SYMBOLS` is a plain Python dict literal. If the same `0xADDR` key appears
more than once in the source text, **the textually last occurrence
silently wins** — Python does not warn, and neither did this project's
tooling until `registry_lint.py`'s W1 check was added. A "fix" that
guesses which of two duplicate names is "more correct" by grepping call
sites is exactly how the Wave QQ-adjacent regression (fixed in PR #2)
happened. The only safe rule:

> **Always keep the textually last occurrence of a duplicate key, and
> delete only the earlier (already-shadowed, inert) one(s).** Verify with
> `python3 -c "from tools.symbols import SYMBOLS; print(hex(0xADDR), SYMBOLS.get(0xADDR))"`
> *before* editing, not after.

### Promoting a placeholder

When a `FUN_xxxxxx` / `Sub_xxxxxx` / `ThunkTarget_xxxxxx` placeholder gets
decompiled and earns a real `REGISTRY` entry:

1. Find every real caller with `tools/scan_unmatched_callees.py` (or an
   ad-hoc `grep -rn PLACEHOLDER_NAME asm/ src/`).
2. **Check for the `.equ` local-alias pattern first.** Many `.s` callers
   don't reference the global symbol at every call site — they define one
   local alias near the top/bottom of the file and use it everywhere:

   ```asm
   .equ    .Lposthook, FUN_00028108
   ...
   jsr     .Lposthook(pc)
   jsr     .Lposthook(pc)
   ```

   In that case you only need to edit the single `.equ` line, not every
   `jsr`/`jmp`.
3. For callers that reference the placeholder name directly (no `.equ`
   indirection — including inline `__asm__` in `.c` files, which *is* a
   real symbolic reference resolved by the assembler, not a raw opcode
   byte), rename every occurrence.
4. Remove the entry from `SYMBOLS`, leaving a comment:
   `# 0xADDR promovido a NewName en registry (Wave NN#k).`
5. Add the `REGISTRY` entry.
6. Run `registry_lint.py` then `match_batch.py`. The match count should go
   up by exactly one function and the byte count by exactly its size —
   nothing else should change, since promotion is a pure rename.

## Wave numbering

Waves are lettered `A, B, C, ... Z, AA, BB, ...` in chronological order,
with `#N` suffixes for sub-batches within an active wave (e.g. `Wave
QQ#1`, `Wave QQ#2`). A wave groups functions that were investigated and
verified together, usually because they share a subsystem, a caller
cluster, or a common structural pattern. See the `## Waves` table in
[`PROGRESO.md`](PROGRESO.md) for the full history.

## Source file naming

`.s`/`.c` file names are `snake_case`, derived from the primary function's
semantic name and/or its address, e.g. `entity_apply_fade_shade_028108.s`
for `Entity_ApplyFadeShade_028108`. A single file may hold several
related functions (e.g. an absorbed cluster) — name it after the cluster,
not just the first function.
