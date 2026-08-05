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
- Wave HHH — 28 entries (3,110 B): **miniboss finale and wave transitions**
  closing all 5 gaps in `$083BE2..$084828` (`miniboss_finale_083bxx.s`),
  the direct continuation of Wave GGG's miniboss module. Includes dual
  jingle entries (snd `$166`/`$AA`) converging with an x-gate, a firing
  phase with cross-entry reinstallation of the internal global
  `TaskHandler_083e8c`, a bit-scan machine over table `$2EAA60[+0x21<<4]`
  (2x `StateMachineRun`), the finale sprite chain and explosion (score
  `$1000`, blitters `$2EACB8`/`$2EAE76`), a wave-transition module driving
  `$10E39A`/`$10E39C` with a cross-gap handler reference
  (`TaskHandler_0845b8`), and parent/child sync stages with random
  relaunch via tables `$2E7556`/`$2EB02C`. New island-RTS defsyms
  `SetHandlerRts_08440e`/`_0844be`/`_08450a` (+6) and `Jsr5B6Rts_084834`
  (+12); removed 3 forward defsyms that became real symbols
  (`TaskHandler_084410`/`_0844c0`/`_08450c`) and added 17 new forward
  defsyms (`Sub_0008495E`, `Sub_000860E4`..`Sub_00086538`). Matcher:
  **3,585/3,585 functions, 129,952 B (6.1966% of P ROM)**.
- Wave GGG — 24 entries (2,374 B): **a miniboss state-machine module**
  closing all 6 gaps in `$083262..$083BDA` (`miniboss_module_0832xx.s`).
  Crosses the **6% P ROM coverage** mark. Includes three entry variants
  (snd `$A4`, sprite `$2E6CDC`, x-dependent drift), a combat phase
  firing the row blitter (`$43FAC` + `$2EAC8C`), a three-variant escape
  sequence converging on the shared internal global `TaskCont_08364e`
  (recoil, flash `$F0`, timed snd `$1054`, final transform `$2E987E`),
  blink-protected variants reinstalling `TaskHandler_0839a2`, and a
  child follower that mirrors its parent's x/y and self-destructs
  off-screen. New island-RTS defsyms `Jsr5B6Rts_083b90` (+12 inside the
  14-byte `Jsr5B6ThenJmpScheduler_083b84` island) and
  `JsrAbsRts_083be0`; 4 forward defsyms to future helpers
  (`$85FB0`/`$86050`/`$86076`/`$863BE`).
- Wave FFF — 36 entries (2,456 B): **escape handlers and pc-relative
  helpers of the paratrooper squad** closing all 17 gaps in
  `$08283C..$08325A` (`para_squad_helpers_082cxx.s`), completing the
  squad module started in Wave EEE. Escape continuation returns to the
  module's shared frame tail via a cross-file `bra.w` (local label
  `.L827fe` promoted to global `ParaSquad_FrameTail_0827fe`); child
  handlers cover revenge fire with random jitter, table-driven respawn
  with ground clamp, ballistic jumps using the sine/cosine tables
  (`$2C07AC`/`$2C072C`), parachute descent and a randomized rank picker
  (HP `$800`). The jsr-pc helper block implements the anchor-history
  ring (`+0x7E` mod 16), parent-anchor copies, flag relays, a
  difficulty gate (`$2BE098`/`$2BE11A`) and bounds probes falling into
  the already-matched `SetXN_*`/`ClearXN_*` C islands. The spawner
  block re-acquires targets, mounts turret piece pairs and spawns the
  10-troop loop / 3-wave sequence. All 23 forward defsyms from Wave EEE
  were promoted to real symbols; 6 new `SetHandlerRts_*` defsyms added.
- Wave EEE — 23 entries (3,862 B): **a squad module with a mobile leader**
  (transport + troops) closing all 23 gaps in `$081816..$082834`
  (`para_squad_module_0818xx.s`). The leader (`TaskHandler_081908`,
  924 B) spawns an escort, two turret pieces and a 6-child chain
  (inheriting `+0x7C` as slot index), walks a sprite table at `$2E541E`
  with `$FFFFFFFF` sentinel, and dies with jingle `$1071` while freezing
  input (`$106ED3`). The smoke piece (`TaskHandler_081db0`) fires
  3-round bursts gated by frame bits (`$106F28 & 7`) with
  difficulty-scaled HP; the mobile piece (`TaskHandler_082052`, 936 B)
  has mirrored left/right entry points and 5-round waves spawning
  children. Foot troops move via nested 2D tables
  (`$2E54A2[row][step]`), drop from the ceiling, and scatter with
  random-angle revenge shots (`$5E9B6 & $F` -> vel/angle triplets).
  The finale (`TaskHandler_08267c`) walks a delta-triplet list moving
  TWO escorts per tick, and `TaskHandler_082720` applies knockback
  inherited from the attacker. Adds 34 defsyms (11 island-internal
  `*Rts_*` incl. `ClrRamWordRts_081caa` + 23 forward refs into the
  pc-relative helper block `$82C7C..$831DA`), converts 5 forward
  defsyms to real symbols. Matcher: 3,497 entries, 122,012 bytes
  (5.8180% of the P ROM), green first run.
- Wave DDD — 29 entries (4,102 B): **the death & escape handlers of the
  "Squad Deploy" module**, closing all 23 gaps in `$080736..$08180E`
  (`squad_death_handlers_0807xx.s`) — together with Wave CCC the whole
  `$07FBD2..$08180E` region is now fully decompiled. A two-level dispatch
  table at `$2E3DC0` (8 pointers to arrays of 8 handlers) picks the death
  animation per weapon/soldier type: tumble-with-bounce
  (`Squad_DeathTumble_080bd6`), skid/skid-brake, parabolic blast arc,
  hop-back, or gib explosion (`Squad_DeathPieces_080f32`). Shard sprites
  (`Squad_ShardSprites_08134c`, 742 B) plus four `Template_0812xx`
  variants drive per-fragment sprite/velocity/gravity. The commander's
  escape uses an embedded 32-word bell-curve data table
  (`SquadCurve_BellTable_08175e`, registered as its own entry). Matching
  oddities: a cross-section `bgt.b` at `$8098E` emitted as raw
  `.dc.w 0x6eec` (GAS cannot emit byte branches with relocs), the
  `$8166E..$817CA` gap split into 4 entries to expose internal target
  `$8179E`, Wave CCC's local `.L80704` promoted to global
  `Squad_CmdrTick_080704`, 20 forward defsyms converted to real symbols
  and 9 new `SetHandlerRts_*` defsyms added. Matcher: 3,474 entries,
  118,150 bytes (5.6338% of the P ROM), green first run.
- Wave CCC — 23 entries (2,714 B): **the "Squad Deploy" enemy module**
  at `$07FBD2..$08072E` (`squad_deploy_module_07fbxx.s`), closing all
  23 gaps between the 26 already-matched C islands
  (`SetTaskHandler_*`/`SetC_*`/`ClearC_*`/`SetTaskW_*`) — the module's
  self-replacing handler chain is now complete. Highlights: an 8-slot
  squad manager (`Squad_Mgr8Slots_0803e8`) that tracks occupancy with
  two bitmasks at `+0x77`/`+0x78` ("ever spawned" / "alive now") and
  does a two-pass `btst` scan (fresh slot first, then any dead slot —
  respawn with casualty memory); two child-init templates (pointer-pair
  table `$2E3EBC` vs. fixed rows `$2BEED0`); a 3-row "hatch" sub-module
  whose row variants fall through into a shared body; arc/sine
  follow movement driven by curve tables `$2E22FA`/`$2E2320`/`$2C072C`;
  and the short `lea $ffff.w,a0` ENTITY_NIL idiom. Adds 27 new defsyms
  (13 island-internal `SetHandlerRts_*` + 14 forward refs into
  neighbouring unmatched code). Matcher: 3,445 entries, 114,048 bytes
  (5.4382% of the P ROM), green first run.
- Wave BBB — 27 entries (916 B): **the player-aiming angle tables and
  the input-layout read backend**, closing the entire
  `$05D316..$05D6AA` gap in `input_aim_tables_05d316.s`. The 19
  `AimAngleTable_*` entries are the real "array tables" consumed by
  `Ent_AimUpdate_045022`/`Ent_AimInit_045412` (Wave XX): entry =
  `table[(state & $F)*2]` = target-angle word ($10000 = full turn,
  `$FFFF` = keep), identified per weapon from matched-code refs
  (pistol/default, rocket fan, HMG pair, and the flamethrower/shotgun
  set of 8+4 per-direction tables). Also: `AimDirRows`/3 `SpawnRows`
  byte tables (targets of the 5-pointer table at `$02A5B8`), the two
  full `InputLayout_ReadField*` routines — including a **genuine SNK
  bug** at `$05D628` where the P1 branch reads `$72(a1)` with the
  stale `a1` before the `lea $100300,a1` (inverted order vs. its 4
  siblings) — and `InputCtx_DemoOverride_05D674`, finally resolving
  the `Sub_00005D674` forward defsym pending since Wave U (replay-mode
  redirect of the input context to the recording buffers). Matcher:
  3,422 entries, 111,334 bytes (5.3088% of the P ROM), green first run.
- Wave AAA — 17 entries (43,736 B): **the Mission VM bytecode
  streams**, the project's largest coverage jump ever (+65% of matched
  bytes in one wave, from 3.18% to 5.27% of the P ROM). Single file
  `mission_streams_0e8524.s` covering the contiguous region
  `$0E8524..$0F2FFC`: all mission-event data of the game — which enemy
  spawns, where and when, across the 6 missions. These are the 12
  targets of the `MissionStreamPtrs_044266` pointer table (Wave XX)
  plus the debug-override stream at `$0F260E`. The dump is not a blob:
  a parser replicating the exact strides of `MissionVM_SkipOp_0446B6`
  walks every record (op `$00` spawn 18 B, `$01` periodic spawner with
  nested child, `$02`/`$03` blocks up to `$0D`/`$0E` markers,
  `$04`/`$0A`/`$0B` waits with 2-3 children, `$05..$09` 4-B pauses),
  emitting per-record comments (enemy template, scroll threshold, XY,
  flags) with indentation reflecting real nesting. All 13 streams parse
  with zero invalid opcodes and every `0C 00 FF FF` terminator lands
  exactly where the next stream begins — the strongest possible
  validation of the format deduced in Wave XX. Four aux word-table
  blocks (waypoint/height lists ending in `$FFFF`) packed after the
  M1/M4/M5/M6 terminators are split at their 78 located reference
  targets, each annotated with the referencing code address. Matcher:
  3,395 entries, 110,418 bytes (5.2651% of the P ROM), all byte-exact,
  green on the first matcher run.
- Wave ZZ — 17 entries (4,216 B): **the "MISSION START" / "MISSION
  COMPLETE" mission banner**, hunted down explicitly as a big-function
  wave: it contains the **four largest routines decompiled so far**
  (`BannerLayout_CompleteFinal_07B598` 968 B, `BannerLayout_StartFinal_
  07AF50` 830 B, `BannerLayout_Complete_07B28E` 778 B, `BannerLayout_
  Start_07ACD0` 640 B). Fills all 9 gaps of the `$07A970..$07BA28`
  cluster in a single file `banner_mission_07a970.s`. The banner
  letters drop in one by one: each layout is a linear spawn sequence
  (one `$4AE` scheduler call per letter with glyph index, row, drop
  order and final X/Y), glyphs index the sprite tables `$2DF684`
  (letters) / `$2DF71C` (digits), and scene `$106ECE==5` selects the
  "Final"-mission variants with the extra mission-number row. Letters
  seek their target (delta<<6 velocity), brake below distance `$30`
  (`$5E23A`), blink + landing sound on arrival, and fly off screen on
  close via an escape angle computed by `$5E018`/`$13C0E`. 5 new
  mid-island defsyms. Matcher: 3,378 functions, 66,682 bytes (3.1796%
  of the P ROM), all byte-exact, green on the first matcher run.
- Wave YY — 41 entries (2,576 B): **5-direction aiming turret,
  Boss2/Miniboss2 state machines, vehicle deploy/launch and Enemy46**.
  Fills the first 6 gaps after the megablock (`$04580C..$046258`). Two
  files: `turret_boss2_04580c.s` — ground-snap firing turret with five
  per-direction initializers dispatched through the 5-pointer data
  jump-table `Turret_InitTable_045CD6` (angle tables `$2895x`), target
  tracking with angle smoothing, a common tail that spawns `Boss2Shot`
  on the `$100800` pool via the `$4AE` scheduler, plus a Boss2/Miniboss2
  block mirroring the Wave XX boss pattern (explode / fall / flag-swap /
  table-dispatched death; miniboss attach / ride / random-impulse hop
  kill). `vehicle_deploy_045f2c.s` — vehicle deployment: an animation
  data table (two 80-byte scripts, `0x03`/`0x04` headers, `FFFF FFFF`
  terminator), angle-computed launch via `$13C0E`, ballistic flight
  branching into two crash handlers, two depth comparators returning
  flags through `ori.b #$11,ccr; rts` islets, and the Enemy46 state
  machine whose tail references two future-gap routines (`Fn_00046260`,
  `Fn_000463C2`) resolved by defsym. 7 new symbols (5 mid-island + 2
  forward). Matcher: 3,361 functions, 62,466 bytes (2.9786% of the
  P ROM), all byte-exact, green on the first matcher run.
- Wave XX — 66 entries (5,356 B), a new single-wave record: **the
  mission event VM, the enemy spawner and the player aiming core**.
  Fills all 25 remaining gaps between the matched C islands of
  `$04422A..$045806`, welding the contiguous megablock
  `$040EF2..$045806` (~18.7 KB with no holes). Three files:
  `mission_event_vm_04422a.s` — per-scene mission bytecode streams
  (14-slot pointer table at `$044266`) with 13 opcodes gated on player
  position / live-enemy count, a parallel skip iterator, and the
  periodic spawner task; notable find: SNK left development asserts
  (`nop;nop;cmpi;nop;trap #15` opcode-range guards) compiled into the
  retail ROM. `mission_spawn_boss_0448a6.s` — `Spawn_FromStream`
  (materializes enemies from 18-byte stream records against the
  template table at `$E8000`) plus the boss state machine
  (intro/engage/phase-fire/descend, projectile with random spread) and
  the miniboss mount logic. `ent_aim_input_044f8a.s` — the player
  aiming core: `Ent_AimUpdate_045022` (1,008 B) resolves the target
  angle per weapon through the angle tables at `$5D326..$5D546`
  (including a diagonal-transition matrix for the flamethrower) and
  integrates it with friction easing; per-weapon input masks, fire
  cadence gate and ground probing. 7 new mid-island defsyms + 19 new
  externals. Matcher: 3,320 functions, 59,890 bytes (2.8558% of the
  P ROM), all byte-exact, green on the first run.
- Wave WW — 7 functions (1,624 B): **the scroll VM**.
  `SceneScriptVM_Frame_0437DA` (1,288 B, one of the largest single
  functions matched so far) is the per-frame bytecode interpreter for
  scene scripts (program counter in `$10815C`): 23 opcodes dispatched
  through a PC-relative `bra.w` jump table where opcode `$16`
  *overflows the table* straight into its inline handler — a new
  pattern. Opcode `$02` yields the frame: saturated scroll integration
  via `Scroll_ClampToRange_043E3A` (limit-crossing detection by sign
  XOR), diagonal segments slave the Y velocity to the X delta, and the
  progress high-water mark updates per axis mode before tail-calling
  `CameraApplyAll4_043D86`. Also: the scroll edge-arrival CCR test,
  two helpers of the 4 KB collision map at `$106F6C+$7C` (12-bit cell
  index, 8x8 cells in 2-column blocks), a pure-C GCC-derived pair of
  camera smoothing velocity presets, and 3 size bumps for trailing
  `rts` already emitted but registered short. The contiguous megablock
  now spans `$040EF2..$04422A` (~13.1 KB). Matcher: 3,254 functions,
  54,534 bytes (2.6004% of the P ROM), all byte-exact.
- Wave VV — 41 functions (4,482 B) across three files, a new single-wave
  record that completes the contiguous megablock `$040EF2..$0434C2`
  (~9.6 KB): melee guard family (dual A/B handler, 5 states),
  Charger enemy (init dispatcher woven through 4 consecutive
  SetTaskHandler C islands — new pattern —, 5-hit attack with deferred
  continuations stored in `+0x78` as 32-bit immediates, mirrored
  windups), Skirmisher, and the 16-heading animation set (1,332 B of
  data transcribed and script-verified against the ROM, first sighting
  of the `$1600` HOLD terminator). Adds 3 new mid-island `rts` symbols,
  documents 3 dead `bra.w` template remnants plus a dead `moveq`, and
  the wave's only `bsr.b` (which forces a section merge). Matcher:
  3,247 functions / 52,910 B (2.5229% of P ROM), green on first run
  for all three parts.
- Wave UU — 31 functions (3,246 B) across two files, a new single-wave
  record and **green on the first matcher run**: the complete member and
  child state machines of the squadron subsystem (`$040F00..$041C12`),
  closing the entire gap between the death handler
  `Jsr5B6ThenJmpScheduler_040ef2` (C) and Wave TT — `$040EF2..$0422E4`
  (5.1 KiB contiguous) is now fully decompiled.
  Part 1 (`asm/squad_member_states_040fxx.s`, 13 fn, 1,268 B): the state
  machine of the 8 squadron members spawned by `Squad_SpawnEight` —
  aim-tracking with table atan2 and stepped heading turns, timed cyclic
  idle animation driven by measured template pairs, the full
  leader<->member shared-state protocol in action (poll / write-back /
  arrival tag with the `+0x86` anti-bounce latch), hit recoil with
  i-frames, and clone pair #12 (`PoseFromHeading`/`B`).
  Part 2 (`asm/squad_children_handlers_0414xx.s`, 18 fn, 1,970 B): the
  derived children — escorted pair (with the wave's largest function,
  `PairChild_HandlerB` at 308 B, leader-mirrored template selection),
  orbital trio with target tracker (timeout + period tables), zigzag and
  drop falls (RNG jitter, 6-byte record tables), and six death/despawn
  sequences.
  Architectural findings: a forensic **dead store** at `$041A96`
  (`movea.l #-1,a0` immediately overwritten by `lea $28610A.l,a0` —
  hand-edited code template), **11 uses of the branch-to-mid-island-rts
  idiom** (proving the `SetTaskHandler` islands were generated together
  with the states), the project's first direct conditional branch into
  an already-matched C function (`bcs.w` to `$40EF2`), and a
  grandparent double-dereference confirming a 3-level entity hierarchy.
  Promotes the 6 handler symbols named in Wave TT to real code and adds
  11 mid-island rts symbols. Matcher: 3,206 functions, 48,428 B
  (2.3092% of the 2 MiB P ROM).
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
