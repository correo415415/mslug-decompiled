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
