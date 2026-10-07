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
- Wave UUUU — 180 entries (15,488 B, 1 data range): `$066000..$06A000`
  (`barrel_paratrooper_shield_tank_066xxx.s`): floating barrel / sea mine
  (tail of `Barrel_Tmpl8D`), paratrooper spawner + paratrooper (anim script
  table `$66CD8`), shield soldier (tmpl 108/109, child `Shield_*` absorbs
  frontal hits, converts to soldier when lost), scene-5 gate and airship
  (parts, lights, hatch, camera hook on `$106F6C`), enemy tank (driver bails
  to soldier, turret, homing missile, `trap #15` sprite-index asserts).
  Templates `$E8000[53..55,94,95,108,109]`. 180/180 byte-exact.
- Wave TTTT — 210 entries (15,242 B, no data ranges): `$062000..$066000`
  (`sniper_camper_mortar_062xxx.s`): LateProp tail (TakeHit/HPCheck/Spawn
  helpers), turret-car gunner, sniper (aim via Atan2, flees as soldier),
  entrenched camper (converts to soldier via `$4A0D4`), scene-5 tent debris,
  mortar (crew, shells, `jmp $6A7D6`), fixed cannon, hostage/POW (rescue
  counter `$10E276..$10E27B`, music `$105D`, 7 template entries), patrol
  soldier (sprite table `$2C6510`), scene-3 prop, barrel (tmpl 141). Templates
  `$E8000[59..63,76..85,141]`. 210/210 byte-exact.
- Wave SSSS — 221 entries (15,144 B, 10 data ranges): `$05E000..$062000`
  (`late_props_turrets_05exxx.s`): late-runtime shared helpers (`Atan2_Angle256`,
  `Target_AcquireNearestPlayer`, `Players_AliveMask`, `Player_GetEntity`,
  `Parent_Copy*`, `Rng_Seed`/`Rng_Mask`, `Hit_ClassifyAttack`, hit-sound
  tables, `Fix_DrawMessageRow` + 30 message tile rows), residual collision
  debug task (`DebugColl_*`, DIP bit1), tower soldier / hut occupant / hut
  door (children of `Prop_TowerBase`/`Prop_Hut`), generic breakables + shards,
  signs, homing marker targets, item props, obstacles, crates, aiming turret
  (`AimTurret_*`) and late props — 42 `$E8000` Mission-VM templates.
  `gen_asm_region.py`: normalize `exg.l` → `exg` (GAS rejects the suffix).
- Wave RRRR — 126 entries (12,874 B, 5 data ranges): `$03DA98..$040EF2`
  (`results_pow_squadleader_03daxx.s`): mission-results screen (`Results_*`:
  per-player columns Score→Bonus→Total→Winner, rescued-POW roster with
  random name/portrait pick, prize sprite, blinking banners, fix-layer
  drawing), POW prisoner (`Pow_*`: tied/freed/idle/walk/jump/crouch/fall/
  rescued chain, rescue credit per player, anim tables) and the flying
  squad leader (`SquadLeader_*`: formation orders +$84, swoop/dive/circle,
  death with 6 explosions, respawn). Matcher 5925/5925, 17.18 %.
- Wave QQQQ — 97 entries (7,396 B, 7 data ranges): `$04FA50..$051914`
  (`allen_oneil_04fa50.s`): boss Allen O'Neil (`Allen_*`: target acquisition,
  physics with step-probe mover, decision checks, jumps, knife, machine gun
  with `Allen_Bullet`, `Allen_Grenade` + explosion, 5-segment HP bar, death
  chain, touch sensors), 19 piece spawners for the barrier/gatehouse/fortress
  props of Wave PPPP, tower/bunker/nest blit helpers, memory-card glue
  (`MemCard_*`), player helpers and nibble pack/unpack. Matcher 5799/5799, 16.56 %.
- Wave PPPP — 40 entries (5,296 B): `$04E580..$04FA50`
  (`props_fortress_04e5xx.s`): fortress-mission props — barrier, gatehouse/
  gate and fortress chains (`Active → Damaged → Wreck`, blockers, roof, door,
  turret mount, side) spawning each other via `Coord_ScreenToLocal $44022`
  and `MissionWatch_Spawn $4429E` on scroll thresholds; generic roof, 3-phase
  fix blink, trigger sensor, crate, bouncing debris (`Hop/Roll`), signs A/B,
  boat (+ blast/debris) and `FixTile_Set11C2`. Matcher 5702/5702, 16.21 %.
- Wave OOOO — 64 entries (6,384 B): `$04CBD4..$04E580`
  (`turret_car_props_04cbxx.s`): enemy turret vehicle (`TurretCar_*`: 32-step
  turret angle with `Idle/Track/Recoil`, `Body`, `Driver*`, guided `Cannon`
  with a 16-entry angle history firing `Shell`s, angle/offset helpers) and
  the third batch of destructible mission props (`Prop_Static/Lamp/Hut/
  Tower/TowerFlag/Bunker/Bridge/Nest/Shed/Barrier` with their damaged/wreck
  phases, MissionWatch `$4429E`, blits `$5022A`). Matcher 5662/5662, 15.96 %.
- Wave NNNN — 53 entries (3,814 B, incl. a 24 B pointer table):
  `$04BB9A..$04CBD4` (`gun_platform_04bbxx.s`): enemy gun emplacement
  (templates `$E8214..$E8220`): 4-variant base `GunPlatform_Spawn`, burst
  cycle `Rearm/Aim/FireA/FireB`, children `Hatch*`, `Shield*`, human
  `RiderA/B_*` (die through HumanDeath), `GunPlatform_Gun`, destruction
  states and flying parts; plus the scroll-driven spawn-stream reader
  `SpawnStream_ReadNext/Dispatch` (`$1081B2`). Matcher 5598/5598, 15.65 %.
- Wave MMMM — 41 entries (4,746 B): `$053F96..$055258`
  (`props_mission_053fxx.s`): second batch of destructible mission props
  sharing the Wave GGGG template (`Prop_Building`, `Prop_Column`,
  `Prop_CompoundWall` + compound parent `Prop_Compound_*`, `Prop_NeonSign`
  with fix-layer updates `NeonSign_Fix*`/`NeonSign_Tiles*`, `Prop_Stall`,
  `Prop_Fragile`, `Prop_Breakable`, `Prop_HitStages`, `Prop_Small`,
  `Prop_ScaledHP`, 5-variant `Prop_MultiStage`, `FixBlink2_PhaseA/B`,
  `Prop_Blocker`). Referenced from the mission spawn records at
  `$096FAE..$097166`. Closes `$0527BA..$055258`. Matcher 5545/5545, 15.47 %.
- Wave LLLL — 25 entries (2,880 B, of which 560 B are the human-death
  state pointer tables): `$049430..$049FC4` (`pow_hang_0494xx.s`): the POW
  hanging from a rope (`PowHang_SpawnVariants`, `Swing`/`Struggle` with
  sin/cos swing physics and angle-driven animation, rope child
  `RopeIdle/RopeStruggle/RopeCut/RopeBroken` driven by the parent/child
  +$78/+$79 protocol, `Freed`), the falling POW template `$E81B8`
  (`PowFall_Spawn` + `PowFall_Shadow`), animation-script callbacks, slot/hit
  checks, `HumanDeath_StateTbls_049d8a`/`StateTblPtrs_049faa`. Closes the
  whole `$0478FC..$04BB8E` block. Matcher 5504/5504, 15.25 %.
- Wave KKKK — 42 entries (6,990 B, of which 3,876 B are sprite/pointer
  tables): `$049FF2..$04BB8E` (`human_death_049fxx.s`): the shared death
  module of human entities (soldiers, POWs) — `HumanDeath_Dispatch_049ff2`
  (selects the death state from the 4 tables at `$49FAA[kind]` indexed by
  damage type +$58), `HumanDeath_TumbleBack/TumbleFwd`, `Collapse`,
  `Knockdown`, `InitBurst`/`BurstLand`, `Launched` (8.8 zoom towards the
  camera), `Burning`/`BurnedDown` + `FlameChild`, corpses, blood splashes,
  smoke pair, physics helpers (`PhysicsAir/Ground/Fall`, `DampVelocity`)
  and the sprite tables `HumanDeath_SpriteTbls_04ac56`.
  `gen_asm_region.py`: promoted global labels now also get a local `.L`
  alias used from inside their own entry (GAS rejected backward `bra.b`
  to a global symbol > 128 B away).
  Matcher: 5,479/5,479, 316,848 B (15.11 %); real code coverage 54.0 %.
- Wave JJJJ — 45 entries (2,242 B): `$048A44..$049430`
  (`pow_helpers_048axx.s`): the pc-relative helpers of the POW prisoner —
  common state tails (`Pow_FreeStateTail_048a44`, `Pow_TiedStateTail_048a90`),
  the thrown reward item (`PowItem_Toss_048ba0` → `PowItem_Settle_048b34`),
  hit/direction fx (`PowFx_HitBurst_048ca4`, `PowFx_DirSprite_048b56`), the
  rope child of the tied prisoner (`PowRope_Spawn/Idle/Struggle/BrokenA/B/
  HitCheck`), free-variant init/physics (`Pow_FreeInit_048ea6`,
  `Pow_TiedSwingStep_048f54`, `Pow_ScrollAndProbe_048fb0`) and the
  distance-based decision helpers (`Pow_TargetInReach/InBox/AngleInMask`,
  `Pow_ShouldRunAway/ShouldWait/ShouldTurn`, `Pow_CanBeRescued`,
  `Pow_AtScreenEdge`, `Pow_HitReceivedCheck`). The POW module
  `$0478FC..$049430` is now complete.
  Matcher: 5,437/5,437, 309,858 B (14.78 %); real code coverage 52.6 %.
- Wave IIII — 42 entries (4,416 B): `$0478FC..$048A3C`
  (`pow_prisoner_0478xx.s`): the POW prisoner entity — spawn variant table
  (`Pow_SpawnVariantTbl_0478fc`, `Pow_SpawnInit_04797c`), free-roaming
  states (`Pow_Idle/WalkToward/RunRight/RunLeft/RunAway/Wait/WalkFree/
  Stop/Turn/Hurt/GetUp`), the rescue sequence (`Pow_RescueStart`,
  `Pow_RescueSalute`, `Pow_RescueGiveItem`, `Pow_RescueThanks`,
  `Pow_RescueLeave`), the already-free variants (`Pow_Free*`, two spawn
  variants `$0483D2/$0483E2`) and the tied-up prisoner
  (`Pow_SpawnTiedVariant_048898`, `Pow_TiedIdle/Struggle/Freed`).
  Matcher: 5,392/5,392, 307,616 B (14.67 %); real code coverage 52.2 %.
- Wave HHHH — 20 entries (1,316 B): `$0539F0..$053F96`
  (`props_helpers_0539xx.s`): the pc-relative helpers of the destructible
  props — the flame trap (`Prop_TrapFlame_053a42` + base/hit fx, victim
  pointer in +$50 = P1/P2), player burning (`Prop_BurnFollowVictim_053c64`,
  `Prop_BurnSmokePuff_053cf2`), `Prop_PlayBreakMusicByPhase_053e0c`,
  debris script runners (`Prop_RunDebrisScriptByPhase/ByPrio`,
  `Prop_GateDebrisA..D`), `Prop_PickRandomItemPtr_053e9c`,
  sprite sync helpers and `Prop_IndestructibleChild_0539f0`. The props
  module `$0527BA..$053F96` is now complete.
  Matcher: 5,350/5,350, 303,200 B (14.46 %); real code coverage 51.3 %.
- Wave GGGG — 27 entries (4,648 B): `$0527BA..$0539E2`
  (`props_destructible_0527xx.s`): the destructible scenery props spawned
  from the mission spawn lists (`$096Cxx`) — `Prop_Sign/Wall/Large/
  Explosive/Tower/Gate` with their `*Stage2`/`*Wreck` states,
  `Prop_HouseVariants_052e20` (7 entries, duck pair spawn) and
  `Prop_HutVariants_0530cc` (10 entries), `Prop_Breakable2Stage_0527ba`,
  `Prop_Indestructible_053964`, debris (`PropDebris_Chunk/Flying*`), item
  drops (`PropDrop_Item_053768`) and the fix-layer blinker
  `FixBlink_PhaseA/B`. All share the init/scroll/anim/hit/HP/offscreen
  skeleton documented in the header.
  `tools/gen_asm_region.py`: `move.l #imm8,dN` ($203C, not optimised by
  SNK) is now emitted as raw `.dc.w` — GAS turned it into `moveq` even
  with the `:l` suffix.
  Matcher: 5,330/5,330, 301,884 B (14.40 %); real code coverage 51.0 %.
- Wave FFFF — 38 entries (4,214 B): `$056ACC..$057D04`
  (`soldier_helpers_056axx.s`, 3 data blocks: popcount table `$56ED2`, two
  84-byte melee attack tables `$57440/$57494`): the rebel soldier helper
  cluster — per-frame physics `Soldier_PhysicsStep_056acc`, the decision
  routine `Soldier_Think_056b92` (target pick via
  `Soldier_FindNearestPlayer_056b38`, platform-edge probes, RNG thresholds
  +$80..+$8A), grab anchors (`Soldier_PickGrabAnchor_056e4a`,
  `Soldier_GrabLatch_057b06`, `Soldier_GrabSlideToAnchor_057bb4`,
  `Soldier_GrabFollowPlayer_057cc0`, `Soldier_GrabStruggleProgress_056f10`),
  spawn (`Soldier_InitCommon_0570a8`, the 24-stub variant table
  `Soldier_SpawnVariants_057226` targeted by `JmpAbsThunk_06313c`,
  `Soldier_SpawnDispatch_05752c`, `Soldier_Leap_057880`), the main
  `Soldier_Walk_Loop_057582`, `Soldier_TestMeleeRange_0574e8`,
  `Soldier_TestSurrender_056fa0`. 5 interior labels promoted in
  `soldier_states_057dxx.s`. The whole rebel soldier module
  `$056ACC..$059342` is now complete.
  Matcher: 5,303/5,303, 297,236 B (14.17 %); real code coverage 50.1 %.
- Wave EEEE — 54 entries (5,694 B): `$057D04..$059342`
  (`soldier_states_057dxx.s`, 1 data block: 4-pointer taunt animation table
  `$58DF8`): the rebel infantry soldier state machine — player grab
  (`Soldier_GrabPlayer/GrabStruggle*/GrabBreak*/GrabThrown*`, anchored via
  `PlayerSlot_ClaimAnchor $8F85C` on +$7A), locomotion (`Soldier_RunToward/
  RunByTable/Run_Loop/Step*/Brake`), `Soldier_Idle/IdleFidget` with RNG
  thresholds in +$80..+$8A, hurt/land, `Soldier_Flee*/Retreat*/Stand/Jump`,
  `Soldier_Surrender*`, two grenade throws (`Soldier_ThrowGrenadeA/B*`,
  `ThrowGrenadeAim`), `Soldier_Taunt*`, and the spawn variants
  (`Soldier_SpawnVariantTbl_058f1e`, `Soldier_Spawn*`) referenced from
  `MeleeGuard_DeathToExtern_0427CA`. Leaf `Entity_CmpField10WithLink8_059332`.
  Matcher: 5,265/5,265, 293,022 B (13.97 %); real code coverage 49.3 %.
- Wave DDDD — 9 entries (8,084 B): `$05AA96..$05CA2A`
  (`sprite_queue_render_05aaxx.s`): the sprite-queue backend — the 8-way
  enqueue jump table `SpriteDispatchJT_05AA96` (flip H/V, ADD/SUB queue),
  the water-reflection hook `Sprite_DispatchSplashHook_05b1b2`, the
  end-of-frame heapsort + SCB1 writer `SpriteQueue_SortAndRenderSCB1_05b232`
  / `SpriteQueue_RenderRange_05b370`, the Duff-unrolled VRAM column writers
  `SCB1_WriteTileColumn_05b52e` / `SCB1_WriteTileColumnTerm_05bf76`
  (4x32 `jmp (pc,dN)` tables), the SCB2/3/4 pass
  `SpriteQueue_RenderSCB234_05b400` and `Vblank_FlushSpriteQueue_05c9d6`.
  `tools/gen_asm_region.py`: `pcrel_target` now resolves the base of
  `jmp X(pc,dN.w)` so indexed jump tables get their local label.
  Matcher: 5,211/5,211, 287,328 B (13.70 %); real code coverage 48.2 %.
- Wave CCCC — 70 entries (11,812 B): `$02AE3E..$02DD20`
  (`slug_states_02aexx.s`, 7 data blocks: two 5-pointer drop-variant
  tables `$2AE90/$2AEE4`, music tables `$2B8CE/$2BA34/$2BB9A`, air-steer
  table `$2C900`, input-dir table `$2C9B0`): first half of the SV-001
  state machine — parachute spawn (`Slug_SpawnDrop`, `Slug_DropVariant0..4`
  creating `Turret_InitDir0..4`, `Slug_DropDescend`), idle on flat/slope,
  `Slug_AccelRightB/LeftB`, `Slug_BrakeRight/Left`, `Slug_CruiseRightB/LeftB`,
  cannon fire (`Slug_Fire*`), jump (`Slug_Jump*`), fall (`Slug_Fall*`),
  hit reaction (`Slug_Hit*`) and death (`Slug_Death*`, `Slug_DestroyedSlide*`).
  The whole Slug module `$0295A6..$030602` is now covered. 10 interior labels
  of `slug_helpers`/`slug_vehicle` promoted to globals.
  Matcher: 5,202/5,202, 279,244 B (13.32 %); real code coverage 46.6 %.
- Wave BBBB — 113 entries (5,846 B): `$0295A6..$02AE3E`
  (`slug_helpers_0295xx.s`, 29 hitbox/anim tables `$295B4..$2A0F8`, pointer
  tables `$2A024/$2A060`): the SV-001 helpers — `Slug_Init_02a0f8` /
  `Slug_InitBoss_02a1aa`, `Slug_AngleToSpriteIdx`, 17 attack tables +
  `Slug_AttackPtrTbl`, `Slug_StateByAnglePtrTbl`, terrain probes, physics,
  layout-dependent input, HP/gauge, `Slug_MarkRidden`, `Slug_NoRider`,
  `Slug_SpawnAtBossArena`.
  Matcher: 5,132/5,132, 267,432 B (12.75 %); real code coverage 44.2 %.
- Wave AAAA — 96 entries (8,788 B): `$030602..$032A02`
  (`player_tables_fx_0306xx.s`, 14 C-island RTS absorbed, 44 data blocks):
  the player grenade (`PlayerGrenade_Spawn/SpawnB/SpawnFromVehicle`,
  layout-dependent throw angle, `ExplodeGround/ExplodeAir`), generic enemy
  shots (`EnemyShot_Straight/Bounce`), the vehicle launch/drop sequence
  (`VehicleLaunch_Init/Fall/Glide/Crash*`), Slug/explosion effects (`Fx_*`,
  `SlugFx_Exhaust*`, `Fx_SpawnDustPair`), player icons (`PlayerIcon_Pow/
  Bubble/FreeFallP1/P2`), the remaining `Chain3_*` helpers and the static
  player tables (`Player_Hitbox*`, `Player_Vel*Tbl`, `Player_GroundTbl*`,
  `Player_WeaponAmmoTbl` 999/999/999/999/150).
  Matcher: 5,019/5,019, 261,586 B (12.47 %); real code coverage 43.1 %.
- Wave ZZZ — 63 entries (10,258 B): `$02DD20..$030602`
  (`slug_vehicle_02ddxx.s`, 7 CCR C islands absorbed, 4 data blocks): the
  SV-001 Metal Slug vehicle state machine `Slug_*` (IdleEnter/Idle/
  IdleAngled/Jump/Hunker/PlayerMount/Drive/TurnToDrive/Brake/Stall/Accel/
  Knocked/Decel/Cruise/SetSpeed), the 80-pointer state table
  `Slug_StatePtrTbl_02e582`, damage sprites (`Slug_Damage*`,
  `Slug_WheelAnim`), destruction (`Slug_Destroyed`, `Slug_KillInit`,
  `Slug_SelfDestructAttack`, `Slug_BlastAttack`, `Slug_ExplodeFx`,
  `SlugFx_*`, 28-template `SlugFx_ExplosionAnim_02f6c0`) and the three-link
  `Chain3_*` entities with their templates and debug HUD.
  Matcher: 4,923/4,923, 252,798 B (12.05 %); real code coverage 41.3 %.
- Wave YYY — 82 entries (5,230 B): `$03C62A..$03DA98`
  (`player_fire_shells_03c6xx.s`, 2 border C islands absorbed, 10 data
  blocks): the player's per-weapon projectile spawners
  `PlayerFire_Pistol/HMG/Shotgun/Rocket/Flame_*` (one entry per firing
  direction; flame also has `_SpreadN_M` fan pairs), shell casings
  (`ShellCasing_Pistol/Rocket`), scene-3 debris (`Scene3Debris_*`),
  `Player_DebugMarker`, `Player_SpawnFx3D8FA`, the Slug cannon arm overlay
  (`SlugCannon_ArmOverlay_03d944`) and 3 residual `PlayerArm_*` handlers.
  Matcher: 4,867/4,867, 242,582 B (11.57 %); real code coverage 39.3 %.
- Wave XXX — 152 entries (8,224 B): `$03A60A..$03C62A`
  (`player_arm_air_death_crouch_03a6xx.s`, clean region, no islands): the
  remaining 47 weapon-arm overlay handlers `PlayerArm_*` (air: Jump/Fall/
  AirShoot*/JumpShoot*/FallShoot*/…ShootDown*; death: DeathA..H which also
  flag the arm entity dead; SpawnFall; crouch: CrouchEnter/Idle/Shoot/
  Crawl/Grenade/Melee/Reload; melee: MeleeC, AirMeleeA/B) and their 105
  `PlayerArm_SpriteTbl_*` 10-pointer tables. Every handler address was
  cross-checked against the player animation tables at `$2796xx..$279Fxx`
  (5 handlers are unreferenced). Matcher: 4,787/4,787, 237,368 B
  (11.32 %); real code coverage 38.3 %.
- Wave WWW — 125 entries (7,270 B code + data): `$0388F0..$03A60A`
  (`player_arm_weapon_fx_0388xx.s`, 24 spurious C islands absorbed, 47
  data blocks): the crouch actions `Player_CrouchThrowGrenade/Melee/Reload`,
  the dropped-weapon entity (`DroppedWeapon_*`), the spawn parachute
  (`Parachute_Open/Swing/Release/FallAway`), the paired duck sensors
  (`DuckTrigger_*`, set player +$88 bit0), the fall-death splash fx
  (`PlayerDeathFx_*`) and the player's weapon-arm overlay task
  (`PlayerArm_Spawn_0394a8` created by `PlayerEntitySpawn`; dispatches on
  the parent's anim id and calls the arm handler stored at +$74; 31
  `PlayerArm_<pose>` handlers each indexing a 10-pointer
  `PlayerArm_SpriteTbl_*` = 5 weapons x 2 players). Matcher: 4,635/4,635,
  229,144 B (10.93 %); real code coverage 36.7 %.
- Wave VVV — 39 entries (8,466 B): `$036632..$0388F0`
  (`player_air_death_crouch_0366xx.s`, 23 spurious C islands absorbed):
  the player's air / death / crouch sub-machines — `Player_JumpStart` /
  `Player_JumpAir` (air control, wall bounce, ring grab via
  `TargetRing_*` -> `Player_HangRing`), knockback (`Player_Knockback*`,
  `Player_Fall_Physics`), `Player_SlugJumpOff`, `Player_SpawnFreeFall`,
  the 7 death handlers targeted by `Player_StateTable68` (`Player_Death_*`,
  `Player_DeathPit`, `Player_Death_Despawn`) and the crouch set
  (`Player_CrouchEnter/Idle/Exit`, `Player_CrawlRight/Left`,
  `Player_CrouchShoot`, `Player_CrouchWeaponEmpty`). Matcher: 4,534/4,534,
  221,874 B (10.58 %).
- Wave UUU — 36 entries (9,050 B): `$0342C4..$036632`
  (`player_states_0342xx.s`, 4 gaps closed, 3 spurious C islands
  absorbed): the player's ground state machine — `Player_Stand`,
  `Player_WalkRight/Left` and the `WalkLoop` second phase with their
  `_Shoot` / `_ShootUp` pose variants, `Player_TurnRight/Left`,
  `Player_Melee` (knife), `Player_ThrowGrenade_Stand/Walk/WalkLoop`,
  `Player_RideSlug` (mount the SV-001 from slot `$100580`) and the shared
  tails (`Player_Stand_Tail`, `Player_Walk_Tail`) that chain hit / fire /
  ground tests. Four unreferenced duplicate bodies documented as dead code.
  Matcher: 4,518/4,518, 213,408 B (10.18 %); real code coverage 33.5 %.
- Wave TTT — 66 entries (5,354 B): `$032A02..$0342C4`
  (`player_core_032axx.s`, 38 gaps closed): the player core — weapon /
  ammo setters with the per-weapon default table (`Player_SetWeaponAndAmmo`,
  `Item_GiveAmmo_ToPlayer`, `Item_GiveBombs_ToPlayer`, clamp 999/99),
  invulnerability blink (`Player_InvulnBlinkStep`), per-weapon music
  table, pad input helpers mirrored by facing (`Input_*ByFacing`,
  `Input_FireByMode/JumpByMode` honouring the `$106F2A` button layout),
  the per-frame action selector (`Player_ActionSelect`), grenade throw
  entry points (`Player_ThrowGrenade*` -> `$28Dxxx`), the spawn sequence
  (`Player_SpawnStart` -> fall / parachute -> land -> `Player_Idle`), the
  drowning/death gate and the idle / crouch / reload states. Three
  embedded tables (`Player_StateTable68_03338a`, `PlayerStateLUT`,
  weapon music). Plus the PAUSE fix-layer text island (`$013D20`) and a
  curated ROM zone map in `tools/measure_coverage.py --zones` /
  `docs/COVERAGE.md` (real code coverage 31.8 %). Matcher: 4,485/4,485,
  204,358 B (9.74 %).
- Wave SSS — 23 entries (2,546 B): `$18D152..$18DB78`
  (`player_grenade_18d1xx.s`, 2 gaps closed): the only code block in the
  upper 1 MiB bank (CPU `$28Dxxx`) — the player's grenade subsystem
  (`Grenade_*`: three throw variants dispatched from `$033346..$033358`,
  ballistic flight with air drag, bounce/heavy variants gated by the
  global flag `$1081AE`, explosion with music `$1027`, smoke child) plus
  five embedded animation tables via `--data`. Six spurious thunk islands
  (`JsrAbsThunk_18d56c/57e/746/766/9d4`, `SetTaskHandler_18d6f0`) were
  tails of real functions and got absorbed. Matcher: 4,414/4,414,
  198,938 B (9.49 %).
- Wave RRR — 96 entries (9,196 B): `$09A0BC..$09C608`
  (`items_score_crates_09a0xx.s`, 28 gaps closed): a generic aimable
  gun with sprite child (`Gun_*`, spawned by `Airship_Wait`), the pickup
  items of templates 284..294/316/317 (`Item_*`: ammo, weapon, weapon
  swap, bombs, 21-variant food, POW, combo timer, static blit) driven by
  the 30-entry spawn table at `$9A5F4` (`Item_SpawnFromParent`), floating
  score digits (`Score_Popup_*`, `Score_Digit_*`, combo level in
  `$10E488/$10E489`), parachutes (`Chute_*`), the thrown crate with
  debris and chute followers (`Crate_*`, `Entity_IntegrateVelFrac`),
  thrown objects with shadows (`Thrown_*`) and `Flag_Init`. Six embedded
  data blocks via `--data`. Matcher: 4,397/4,397, 196,440 B (9.37 %).
- Wave QQQ — 125 entries (9,252 B): `$09773C..$099F3A`
  (`hiscore_memcard_mobs_0977xx.s`, 96 gaps closed): the high-score
  table (`HiScore_*`: 10 x 12-byte records at `$100002`, defaults from
  `$2F53B2`, insert/rank helpers, three attract task templates), the
  3-letter name editor (`NameEntry_*`, alphabet `$2F54B2`, timeouts,
  profanity filter), memory-card load/save dialogs (`MemCard_*`, BIOS
  `$C00468` ops 2/3/4, file name "METAL SLUG"), the logo scene
  (`LogoScene_*`, template `$98720`), 16 background-mob templates
  (`Mob_Tmpl174..222_*` with shared Walk/Hit/Flee/Patrol/Drop states),
  the 16-slot trail ring at `$10E3BE` (`Trail_*` reset/advance/lookups)
  and the option menus (`OptionsMenu_*`, `OptionSelect2_*`,
  `DebugCursor_*`). Two embedded data tables emitted via `--data`; two
  false `NopCCR` C islands removed (tails of `movem.w ...,0x3c0000`);
  `gen_asm_region.py` now emits signed `moveq` immediates. 36 defsyms
  promoted; +28 island RTS; 73 call sites renamed. Matcher: 4,301/4,301,
  187,244 B (8.93 %).
- Wave PPP — 92 entries (7,678 B): `$08F6D2..$0916B8`
  (`gameover_continue_08f6xx.s`, 50 gaps closed): player-slot anchor
  masks (`PlayerSlot_*`, `Anchor_GetWorldPos*`), the Game Over sequence
  (`GameOver_Boot/Spawn/Wait/Final/WaitCredit/Continue*`, task added at
  `$1300`), its ~40 child effects (`GO_Letter_V0..V13`, `GO_Sprite_*`,
  `GO_Prop_*`, `GO_Zoom`, `GO_Shake`, `GO_Flash`, `GO_Banner*`,
  `GO_Glow*`, `GO_Figure*`, `GO_Scroller*`, `GO_Particle`) and the
  Continue screen (`Continue_Tpl`, `Continue_Tmpl227` = `$E8000[227]`,
  fix-layer text with BCD countdown from `$10FDDA`). 18 defsyms
  promoted; +17 island RTS; 32 call sites renamed. Matcher: 4,178/4,178,
  178,004 B (8.49 %).
- Wave OOO — 84 entries (3,980 B): `$08E4E4..$08F6D2`
  (`critters_rings_08e4xx.s`, 54 gaps closed): small critters
  (`Bobber_*`/`Leaper_*`/`Runner_*`, Mission-VM templates 182/183/184),
  the 4-stage nest (`Nest_*`/`Nest2_*`, templates 185/187) with its
  `Swarmer_*` children (atan2 + sine steering), props (`Static_Tmpl188`,
  `CamProp_Tmpl189/190`, `Lob_Tmpl191`, `Shard_V0..V2`), the grunt-physics
  helpers (`Phys_*`, `Snd_ByParam9A_*`, `Prio_Set8018`) and the three ring
  buffers `$10E2F2`/`$10E33A`/`$10E362` (`Ring_*`, `ZoneRing_*`,
  `PosRing_FindNear`, `TargetRing_*`, `Turret8_SndByState`).
  `gen_asm_region.py` now emits `moveq` immediates signed. 19 defsyms
  promoted; +4 island RTS; 144 call sites renamed (the 10 NNN forward
  defsyms resolved). Matcher: 4,086/4,086, 170,326 B (8.12 %).
- Wave NNN — 80 entries (4,442 B): `$08D17A..$08E4E4`
  (`grunts_capsule_08d1xx.s`, 67 gaps closed): latched on-screen tests
  (`Screen_InBounds*`), 8.8 fixed-point position integrators
  (`Pos_Integrate*`), the mission-end capsule/beacon (`Capsule_*` ->
  `MissionEnd_*`, spawned by `SceneB_Init`/`SceneC_Init`), the grunt
  soldier family (`Grunt_*`/`Grunt2_*`: Mission-VM templates
  153/154/157/158/160/162/180/181, random-behaviour tables `$2F3712`/
  `$2F3722`, carrier, hopper, hit-and-launch, runner, direction map
  loader), `Sentry_*` and the `Swinger_*` oscillator. 22 defsyms promoted;
  +15 island RTS; +10 forward defsyms (grunt physics helpers in `$8EFxx`/
  `$8F0xx`). Matcher: 4,002/4,002, 166,346 B (7.93 %).
- `tools/wave_apply.py`: applies a `gen_asm_region.py` report to
  `registry.py`/`symbols.py` (REGISTRY block, defsym promotion, island RTS
  and forward defsyms) and emits the old->new rename list for call sites.
- Wave MMM — 67 entries (4,990 B): heterogeneous region `$08BA04..$08D17A`
  (`cutscene_anim_08baxx.s`, 39 gaps closed): bouncing/burst projectiles
  (`Proj_Bounce_V1/V2`, `Proj_Burst_*`, `Proj_Shell` with embedded
  `Hitbox_08bb8c`/`SpriteMap_08bbde`), the 8-byte animation-script
  interpreter (`Anim_ScriptStep*`, fix-layer text variants), the 4 slot
  icons (`Icon_Base`, `Icon_Slot1..4[_Lit]`, `Icon_Anchor_*` — the task-adds
  of `Anim_State_F1_08C008`), the cutscene machinery (`Cut_Watcher_*`
  parallel task started by `MissionDriver_Init`, `Cut_Fade`, fix-layer text
  panel `Fix_TextRow_*`, `Cut_Dropper_*`, `Cut_Item`, Mission-VM templates
  244/245/246/248/320) and the mission $0B/$0C scene bootstraps
  (`SceneB_Init -> Stage2..6 -> Tail`, `SceneC_Init`). `Proj_Bounce_08b9ba`
  grows to 74 B (absorbs a split `movea.l`). 20 defsyms promoted; +13
  island RTS; +3 forward defsyms. Matcher: 3,922/3,922, 161,904 B (7.72 %).
- `tools/gen_asm_region.py`: labels promoted to globals because another
  gap references them are now also emitted by name from their own entry
  (previously GAS failed to resolve the local `.L`).
- Wave LLL — 98 entries (10,690 B): **scene-5 cluster** closing all 43
  gaps in `$088A56..$08BA00` (`scene5_airship_088axx.s`). Spawnable entries
  come from spawn list 5 (`$097422`) and Mission-VM templates
  `$E8000[168..170]`, `[254..257]`, `[258]`, `[270]`. The landing airship
  (`Airship_Wait -> Landed -> Hover -> Depart`, spiral descent, ground
  probing, trooper drop, camera lock publish), its 8-way turret
  (`Turret8_*` with rotating barrel, bullets and ejected casings), the two
  towers (`S5_TowerA_*` / `S5_TowerB_*`: random HP, 3 damage stages,
  `TowerPort_*` children, wreck flag/smoke/sparks, lamp sprite callbacks
  `SprCb_Lamp*`), the camp (`S5_Tent*`, `S5_Depot*`, `S5_Bunker`,
  `S5_Camp_Spawn`), breakable props, barrel row and falling rocks, and the
  projectile family (`Proj_Thrown`, `Proj_Drop_V0..V3 -> _Common`,
  `Proj_Bounce` driven by the embedded pointer table `Proj_Bounce_HitTable_08b944` (+ `Hitbox_08b950`)).
  20 defsyms promoted; +16 island RTS; +4 forward defsyms. Matcher:
  3,854/3,854, 156,910 B (7.48 %).
- `tools/gen_asm_region.py`: new `--data START-END` option emits embedded
  data tables inside `.text` as `.dc.w` (first used for
  `Proj_Bounce_HitTable_08b944`); `scripts/bootstrap_sandbox.sh` now looks for
  the ROM zip in `/home/user/uploaded_files/` and `/mnt/aidrive/mslug_rom/`.
- Wave KKK — 84 entries (9,334 B): **scene-4 fortress cluster** closing
  all 6 gaps in `$0865BE..$088A56` (`fort_scene4_0865xx.s`). Every
  spawnable entry is referenced from spawn list 4 (`$0972CC`, 20-byte
  records `{$0100,x,y,handler,...}` behind `JumpTable_096B9C`) or list 7
  (`$0975A2`); `Heli_InitTmpl_087b26` is template `$E8000[269]`. The
  fortress container (`Fort_Init_V0..V4` -> `Fort_Init_Common`,
  `Fort_Destroyed`, per-variant `Fort_SpawnChildren_V0..V4`), its pillboxes
  (`Pillbox_*`: 6 init variants by `+0x21` nibble, 3 damage stages,
  collapse/fragment, damage propagated to the parent via
  `Entity_PropagateDamageToParent_08848c`), crates (`Crate_*`), the troop
  hatch cycle (`Hatch_WaitClosed -> Opening -> SpawnTroops -> WaitOpen ->
  Closing`, final open when the fortress dies), breakable props and an
  indestructible signboard, the 13-piece crate wall
  (`CrateWall_Spawn13` + `CrateWall_Piece00..12` -> `_Common`), the armored
  car (`ArmoredCar_*` with random HP `$2C0628`, two damage stages, turret
  child, blast box and wreck), the helicopter (`Heli_*` with random HP
  `$2C05A6`, rotor animation `Heli_RotorAnim_0883ec`, trooper dropper), the
  scattering debris, the quad prop and the 5-stage barricade. Hitbox
  "pulse" helpers `Entity_HitboxPulseTable/Saved` (formerly
  `PcThunkTarget_088438/08846a`). 4 forward defsyms promoted; +1 island RTS
  (`SetTaskWRts_088436`), +1 forward defsym (`Sub_00088A64`). Matcher:
  3,756/3,756, 146,220 B (6.97 %).
- `tools/gen_asm_region.py`: pc-relative/branch targets that fall in a
  future gap without a symbol are now emitted as `Sub_XXXXXXXX` forward
  defsyms (reported as "refs forward") instead of raw hex, so drafts link
  and verify without manual edits.
- `scripts/bootstrap_sandbox.sh`: looks for `mslug.zip` in
  `/home/user/uploaded_files` and `/mnt/aidrive/mslug_rom`, and persists a
  copy to AI Drive (sudo fallback) so sandbox resets don't require a
  re-upload.
- Wave JJJ — 65 entries (4,742 B): **Mission 4 entities + boss spawn
  helpers** closing all 33 gaps in `$08512C..$0865BE`
  (`m4_carrier_boss_helpers_0851xx.s`). Mission-4 templates
  `$E8000[35..40]`: the carrier (`M4_Carrier_*`: init/approach/fight/
  wreck with `MissionWatch_Spawn_04429E` over aux list `$EC6C8`), its
  turret (`M4_Turret_*`: edge check, shake, knockback, death), soldier
  droppers, the double wheel (`$2E7BDE/$2E7BEE` frames), the 20-segment
  rail, per-player water bodies spawning splashes, debris, and the
  "camera floor" that walks the aux height table `$EC882` to publish the
  scroll limit in `$10816A/$10816E`. `Flight_*` helpers used by Wave III's
  flight cycle (random wobble, hitbox params, altitude check). Mission-3
  boss helpers (`Boss_*`): random drop spawner, phase jingles, sine bob,
  list-driven spawners over 8-byte records (`$2EAF1C/$2EAF7E/$2EAFE2`),
  45/4/8/9-child loops, a hand-unrolled 10-escort spawner (362 B), six
  blit-table loaders and the boss shadow init. 25 forward defsyms
  promoted to real symbols; +7 island-RTS defsyms. Matcher: 3,672/3,672,
  136,886 B (6.53 %).
- `tools/gen_asm_region.py`: verified draft-`.s` generator for an
  unmatched range (gap walking, entry splitting, project-style GAS,
  cross-entry/mid-island symbols, byte-exact self-check, `--registry`
  output). Wave JJJ was produced with it: 65/65 entries byte-exact on
  the first pass, semantic pass done by hand on top.
- `scripts/bootstrap_sandbox.sh`: one-shot toolchain + deps + ROM setup.
- `TaskHandler_083c02` promoted from a local label of `TaskHandler_083be2`
  to a global symbol (it is a `lea pc` handler target from `$0860F6`).
- Wave III — 22 entries (2,192 B): **rescue squad and flight cycle**
  closing all 12 gaps in `$084836..$08512C` (`rescue_squad_0848xx.s`).
  Includes the squad-step handlers driven by sprite-list table `$2E77CA`
  (indexed by `+0x21<<1`, `$FFFFFFFF` sentinel), the child handler
  spawned from Wave HHH (`Sub_0008495E`, promoted from forward defsym to
  real symbol: snd `$85`, random timer from `$2C0218`, double armor past
  scroll `$4F0`), a 4-stage fall sequence (sprites `$2B5B92..$2B604A`,
  landing snd `$1040`), a parachutist follower, a rescued-POW reward
  handler (score `$2000`), a 3-way type selector over `+0x98`, and the
  transport flight cycle (snd `$1074`/`$1075`, climb/dive/glide sprite
  alternation, landing spawn of `$8512C`). New island-RTS defsyms
  `SetHandlerRts_084898/_0848dc/_084b22/_084b98/_084bd0/_084c24` (+6)
  and `Jsr5B6Rts_084c5c` (+12); removed 9 forward defsyms that became
  real symbols and added 7 new forward defsyms
  (`Sub_00085EE8..Sub_00086504`). Matcher: **3,607/3,607 functions,
  132,144 B (6.3011% of P ROM)**.
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
