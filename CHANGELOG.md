# Changelog

High-level, English-language summary of notable milestones. This file
tracks repository/process changes and headline decompilation progress;
for the full function-by-function log (in Spanish) see
[`docs/PROGRESO.md`](docs/PROGRESO.md).

The format loosely follows [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Changed
- Reorganized repository for clarity: moved `PROGRESO.md` to `docs/`,
  added `CONTRIBUTING.md`, `docs/CONVENTIONS.md`, `tools/README.md`,
  `requirements.txt`, and a CI workflow (`.github/workflows/ci.yml`) that
  runs `registry_lint.py` and a syntax check on every push (the full
  byte-exact matcher needs the copyrighted ROM and cannot run in CI).

### Added
- Wave TT — 32 functions (1,634 B) across two files, the largest
  contiguous cluster decompiled so far and **green on the first matcher
  run**: the complete "squadron" subsystem (`$041C1A..$0422E4`), the
  flying-swarm formation logic (8 entities with sinusoidal wave motion).
  Covers formation-target computation (table `$286124` + transform
  `$440D0`), steering with table-based atan2 (`$5E018`) and turn-rate
  limited heading, an unfactored **triplet of sine-bob clones** (sine
  table `$2C072C`, phase +0x80/+0x10/+0x10, scale >>8/>>6/>>7), a state
  dispatcher over jump table `$28633C`, the animation-template selection
  chain with mirrored variants (`+0x7C == $FF`, pre-mirrored ROM assets),
  a leader<->member **shared-state protocol** (poll `$041FF6` +
  cooperative-CAS writeback `$042040` over the byte array at `+0x80` of
  the shared struct `+0xC`), the 8-member spawner (`$28615C` records,
  handler `$40F00`), escorted-pair spawners (**non-factored clone pair
  #11**: `SpawnCore_0420FE` 132 B / `SpawnPlain_042188` 126 B), the
  patterned trio spawner (`$28631C`/`$286310`), the heading→velocity
  sincos helper (cosine = sine table + 64 entries: `$2C07AC`), and two
  triangle-wave shade modulators. **Two new nop-padded `trap #15`
  asserts** (`ASSERT(step != 0)` at `$041D74`, `ASSERT(state < 6)` at
  `$041E56`) — the Nazca dev-build assert macro found in Wave SS also
  guards gameplay code. Promotes 5 `symbols.py` placeholders and names
  6 new pc-rel handlers (`SquadMember_Handler_040F00`,
  `SquadMember_OnStateChange_040F82`, `PairChild_HandlerA/B`,
  `TrioChild_HandlerA/B`), which map out the `$040F00..$041C12` member
  handler cluster as the natural next target.
- Wave SS — 12 registry entries (1,384 B gross / +1,366 B net), second
  size-prioritized wave via `tools/rank_candidates.py`. Absorbs 3 Wave-N
  false positives (`ClearXN_028b7c`, `ClearXN_028c14`, `SetXN_028c1a`,
  project FPs #49–#51) and promotes 8 `symbols.py` placeholders to
  canonical definitions:
  - `Entity_HitboxCollide_028A96` ($028A96, 114 B) +
    `Hitbox_OverlapTestXY_028B14` ($028B14, 268 B) — the core
    entity-vs-entity hitbox collision pair: sweep-caller with hit-flag
    propagation into both entities' `flags69`, and the AABB overlap test
    with facing/flip mirroring. **Major forensic find:** four identical
    nop-padded `trap #15` debug-assertion blocks (`ASSERT(min < max)`) —
    first direct evidence of Nazca's development-build assert macro.
  - `ScriptSlotPairTable_0009B4` ($0009B4, 200 B data-in-.text) +
    `TaskSlots_BootInstall_000A7C` ($000A7C, 270 B) — closes the last
    large gap of the $000xxx boot block: a two-subtable (id, script)
    pair list consumed by `Sub_00002B58`, and the boot task installer
    that seeds all 12 static `$100xxx` TCBs (including installing Wave
    MM's `SchedulerBootstrap_Boot_000E8E` into TCB $1001C0) and links
    the player/partner TCB pairs. Reached via the (TCB, handler) table
    at $178000 — no direct `jsr` callers anywhere in the ROM.
  - `TaskList_ChangeAndRunEight_001CD4` ($001CD4, 96 B) — per-frame
    re-arm batch over the 8 gameplay TCBs; the long-documented "$1CD4
    callee" of Wave MM#3, falling through into `JsrAbsThunk_001d34`.
  - Six fix-layer 16×16 glyph drawers ($099F3A/$099F86/$099FD2/
    $099FF2/$09A03C/$09A086, 370 B) — 2×2 fix-tile blocks written
    straight through the LSPC VRAM port ($3C0000): menu cursors A/B,
    two fixed-cell digit counters, and two glyph-run dispatchers, one
    of which exits early by branching into the *middle* (the final
    `rts`) of the already-matched `JsrAbsThunk_09a0b4`.
  - Matcher after SS: **3,143/3,143 functions, 43,556 B (2.0769 % of
    the P ROM)** — green on the first full run, zero regressions.
- Wave RR — 5 functions (516 B), the first wave selected purely by
  **size** via `tools/rank_candidates.py` instead of caller popularity:
  - `Entity_CheckActiveBoxOverlap_072C98` ($072C98, 144 B) and
    `Entity_CheckBoxOverlapWithSelector_0798AC` ($0798AC, 164 B) — sibling
    rectangular hit/detection-box overlap checks against a facing-mirrored
    box, writing a composite result into `target->+0x8E`.
  - `Camera0_RelinkAndWrapScroll_06896A` ($06896A, 112 B) — re-anchors
    camera[0] to a new tile bank and wraps its scroll counter by one
    screen width (320px), part of the infinite-background-scroll
    mechanism.
  - `Entity_MirrorDeltaByFacing_065D32` ($065D32, 12 B) and
    `EntityGroup_SpawnLinkedFromTemplateList_065C94` ($065C94, 84 B) —
    spawns a group of linked sub-entities (e.g. multi-part vehicles) from
    a 12-byte-stride template list, mirroring the X delta by the parent's
    facing flag.
  Promotes 4 `PcThunkTarget_*` placeholders from `tools/symbols.py` to
  canonical names.
- Wave QQ#1 — `PlayerEntity_InitAuxState_032A02` ($032A02, 158 B), player
  entity aux-state initializer, 2 callers.
- Wave QQ#2 — `Entity_ApplyFadeShade_028108` ($028108, 44 B), shared
  entity fade-shade helper, 9 callers.
- `tools/registry_lint.py` — static structural audit of the registry
  (overlaps, duplicate addresses/names, odd/bad sizes, symbol clashes).
- `tools/rank_candidates.py` — size-ranked candidate queue for
  large-function-first decompilation sessions.
- `tools/measure_coverage.py` — entropy/opcode-density heuristic for real
  code coverage, feeding `docs/COVERAGE.md`.

### Fixed
- `tools/symbols.py` had 8 pre-existing duplicate dictionary keys (dead
  code from the plain-dict-literal `SYMBOLS` table silently letting the
  last occurrence win). 4 of them resolved to the wrong name on first
  pass and were hotfixed after landing on `main` — see
  `docs/CONVENTIONS.md § The duplicate-key gotcha` for the root cause and
  the safe fix procedure now documented there.

---

## Progress snapshot

| Metric | Value |
|---|---:|
| Matched functions | **3 134 / 3 134** registered |
| Matched bytes | **42 190 / 42 190** registered |
| P ROM coverage | **42 190 / 2 097 152 B** (2.01 %) |

Regenerate with `python3 tools/match_batch.py` (requires your own ROM —
see `README.md § Building`).
