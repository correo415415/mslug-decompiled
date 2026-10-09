# Metal Slug 1 — registro de progreso

**MD5 P ROM procesado esperado:** `816b3f74c76b3373993407615f1850fe`
**Toolchain oficial:** `m68k-linux-gnu-gcc` (Debian/Ubuntu, GCC 13.3.0) en
modo bare-metal 68000 (`-mcpu=68000 -nostdlib -nostartfiles -ffreestanding
-fno-builtin -fomit-frame-pointer -Os`).
**Assembler:** `m68k-linux-gnu-as` vía GCC con `-Wa,--register-prefix-optional`.
**Diff visual:** `simonlindholm/asm-differ` vendored en `tools/asm-differ/`
(soporta m68k oficialmente).

## Estado del matcher

```
MATCHED : 8062/8062 funciones
BYTES   : 541,182/541,182 (registrados)
ROM     : 541,182/2,097,152  (25.8056%)
```

> **Wave GGGGG** (55 entradas, 2 320 B; 22 huecos, 7 `--data`, 3 `--entry`) —
> `$05CADE..$05E000` en `input_thunks_debug_hex_atan_luts_05cade.s`.
> Quincuagésima wave. Nombres en `docs/waves/ggggg_names.txt`, args en
> `docs/waves/ggggg_args.txt`. 21 thunks `InputEvtThunk_*` (máscara/canal/
> contexto `$10E200`/`$10E206` → backends `InputMask_*`), `InputEvt_ToggleChain`
> (detección de flanco vs `$10E20C`), HUD hex de debug (`Debug_HexDrawToFix4/8`,
> `HexDigit_FixTileTable_05d864`, `Bin16_ToBcd4`), `Noise_LookupByIndex` +
> `NoiseLut256_05d956`, LUTs `AtanLog_Table_05de18`/`AtanExp_Table_05df18` de
> `Atan2_Angle256`, `Entity_CheckOnScreenBox/EnterScreen/LeaveScreenWide` +
> `ScreenBox_Default`, `ListCursor_Step/LoadEntry`, `Spawn_ChildFromDesc`,
> `SpriteBlock20x14_Setup/SetupDup`. Promovido `Sprite_HexFormat4_05D6C2__L05d6d0`.
> Cobertura: 541,182 B (25.81 %), CODE 98.4 %, zona core 99.1 %, 54 huecos /
> 8,234 B. Siguientes: `$051000/$052000` (≈2.2 KB), `$0133B0..$013D6A`
> (2,082 B), `$000400..$002F30` (≈3.9 KB).

> **Wave FFFFF** (33 entradas, 3 738 B; 15 huecos, 5 `--data`, 2 `--entry`) —
> `$055B96..$056ACC` en `enemy_projectiles_grenade_mortar_roller_055b96.s`.
> Cuadragésimo novena wave. Nombres en `docs/waves/fffff_names.txt`, args en
> `docs/waves/fffff_args.txt`. Proyectiles enemigos: granada lanzada
> (`Grenade_SpawnFromThrower` → `Grenade_Task` con `Grenade_TrajTable_055c08`
> {vx,ay,vy,timer} por tipo `$5C(a6)`, velocidad polar en modos de escena
> `$106F2B` 1/2/3, `Grenade_Explode` agua/fuego), bola rebotante (`Bounce_Task`,
> `Bounce_Rest/Rest2/Fizzle/Explode`), obús de mortero (`Mortar_ApplyDrag`,
> `Mortar_SpawnFromParent`, `Mortar_Shell_Task/Explode`, `Mortar_HitboxList*`,
> `SpriteMap_Mortar`), rodillo (`Roller_SpawnFromParent`, `Roller_Task`,
> `Roller_Explode`, `Roller_HitboxList*`, `SpriteMap_Roller`), 7 thunks
> `Entity_CmpDepthToParent_*` y `Soldier_PhysicsBox_056ac4`. Cobertura: 538,872 B
> (25.70 %), CODE 97.9 %, zona core 98.1 %, 76 huecos / 10,544 B. Siguientes:
> `$05C000..$05E000` (22 huecos ≈2.3 KB), `$051000/$052000` (≈2.2 KB),
> `$0133B0..$013D6A` (2,082 B), `$000400..$002F30` (≈3.9 KB).

> **Wave EEEEE** (71 entradas, 5 290 B; 38 huecos, 7 `--data`, 5 `--entry`) —
> `$05934E..$05A9D6` en `result_ending_gunner_walker_05934e.s`. Cuadragésimo
> octava wave. Nombres en `docs/waves/eeeee_names.txt`, args en
> `docs/waves/eeeee_args.txt`. Módulos: pantalla de resultados de misión
> (`ResultText_Strings_05934e`, `Result_DrawDigits`, `Result_PlayerPanel_Init` →
> `Result_Roll{Continues,Prisoners}` → `Result_PrintScore` → `Result_HiScoreEntry`),
> secuencia final (`Ending_Show{Mission,All,Over}`, `Ending_WipeAllOver`,
> `Ending_Seq_*`, `EndingOrbit_Parent/Child`, `Str_PeaceForever`), soldado
> artillero `Gunner_*` (Search/Fire/Reload + `Gunner_Shell`) y `Gunner2_*`
> (`Gunner2_AimFromAngle`, Track por stick del jugador o Target_AngleToPlayer,
> `Gunner2_Shell` con `Shell_HitboxList_A/B/C`), `Walker_*`, `GunnerAnim_Table`,
> `FadeLut_16x16_05a8ba`, `VRAM_FixAutoclear_Reset_05a9ba`. `gen_asm_region.py`:
> rangos `--data` byte a byte (`.dc.b`) para labels en offsets impares. 27
> renombres propagados. Zona `$024E10..$05E000` al 96.5 %; CODE total 97.2 %
> (91 huecos, 14 282 B).
>
> **Wave DDDDD** (79 entradas, 4 374 B; 47 huecos) — `$046000..$048000`
> en `enemy46_drops_fix_banners_046260.s`. Cuadragésimo séptima wave. Nombres en
> `docs/waves/ddddd_names.txt`. Módulos: Enemy46 fases C/D + `Enemy46_RandomPause`
> (cierra la máquina de vehicle_deploy_045f2c.s), drops de enemigos por template
> (`Drop_SpawnRandom_046322`, `Drop_Spawn_Tmpl4F/F2/1B/6D/2C/2B/3A_12F`,
> `Drop_SpawnFromTable`, `Drop_SpawnThrown`, matriz `$28DC4E`), `Fade_WhiteFlash_*`,
> `SceneC_Load_*`, pantalla CONTINUE grande (`ContinueDigits_P1P2_Task`,
> `ContinueBig_Init/Countdown/ClearAndExit`, `Continue_IsStartP1/P2/Any`), banners
> del fix layer (`FixBanner_MissionStart/Complete_Blit`, `MissionNumBanner_*`,
> `MissionStart_*`, `MissionComplete_*`, `TimeUp_Banner_Task`), `BigText/SmallText_
> Typewriter`, `Font_*GlyphToTile`, `Fix_DrawBigNumber2Digit`, `Fix_PutString_
> PalByHighBit`. 22 renombres propagados; 2 labels promovidas; eliminadas las
> falsas islas `NopCCR_046e6c/046fd0` (eran el `movem.w` final de los blits).
> Zona `$024E10..$05E000` al 94.2 %; CODE total 96.1 % (129 huecos, 19 572 B).
>
> **Wave CCCCC** (117 entradas, 6 380 B; 6 `--data`, 39 `--entry`) — `$027400..$02A000`
> en `collmap_hitbox_script_ops_027400.s`. Cuadragésimo sexta wave. Nombres en
> `docs/waves/ccccc_names.txt`, args en `docs/waves/ccccc_args.txt`. Módulos:
> resto del motor de movimiento/colisión (`Entity_MoveAndCollide_D/E/F`,
> `Entity_StepB/C/D/Fall_UpdatePos`, `Entity_SlopeResolve/Commit`,
> `CollMap_LookupTile` + `CollMap_TestSolid*/TestPlatform*`, probes de suelo,
> `Entity_*VelLatch`, `Entity_ProbeSwapGravity_A/B/C`), hitboxes entidad-entidad
> (`Hitbox_RunList` + `Hitbox_ListOpTable` de 5 ops sobre jugadores/slugs/pools
> `$100800`/`$1008A0`, `Hitbox_TestBoxes`, `Hitbox_Overlap{Rect,Point,PointA4}`,
> `Hitbox_ApplyDamage`, `Hitbox_DefaultShapes`) e intérprete de scripts de
> sprite (`Script_OpcodeTable_028cf0`: 32 opcodes `ScriptOp00_Wait` ..
> `ScriptOp1F_Sound2222`, llamado desde `script_dispatch.s`). 14 renombres
> propagados; 3 labels locales promovidas a globales. Zona
> `$024E10..$05E000` al 92.3 %; CODE total 95.3 % (176 huecos, 23 946 B).
>
> **Wave BBBBB** (88 entradas, 8 874 B; 9 `--data`, 1 `--entry`) — `$024E10..$027400`
> en `hud_inputrec_entity_move_024fb8.s`. Cuadragésimo quinta wave. Nombres en
> `docs/waves/bbbbb_names.txt`, args en `docs/waves/bbbbb_args.txt`. Módulos:
> `Input_Poll_LiveMode` + `InputRec_*` (grabación/replay RLE del input para el
> demo del attract; modos `$106ECA` 0/1/2), `PlayerSlotDesc_*` (32 registros de
> 16 B {tmpl, entidad, task, flags}), tarea HUD por jugador `HUD_Task_*` /
> `HUD_State_*` (INSERT COIN → PUSH START → spawn → muerte → CONTINUE/GAME
> OVER) con dibujado al fix layer `HUD_Draw*`/`HUD_Msg_*` (strings ASCII en
> `$2785B8..`), créditos BCD `$1081BF/C0` + BIOS `$C00450`, y motor de
> movimiento/colisión `Entity_MoveAndCollide_A/B/C`, `Entity_FloorProbe`,
> `ClampVelocity`. 10 renombres propagados (task_handlers.c, jsr_pc_thunks.c,
> entity_probe_*.s). `gen_asm_region.py`: fallback `rawinsn` para `movem`
> abs.l. Código real 94.0 %; zona `$024E10..$027400` al 100 %. Siguiente:
> huecos de `$027400..$02A000` y páginas `$046000/$052000/$056000/$059000`.
>
> **Wave AAAAA** (65 entradas, 23 648 B; todo datos, byte-exacto) — `$0916C8..$0967B4`
> en `scene_scripts_0916c8.s` y `$096BBC..$097730` en `attract_sprite_lists_096bbc.s`.
> Cuadragésimo cuarta wave. Generadores `tools/scene_script_dump.py` y
> `tools/attract_sprites_dump.py` (capstone; verifican enlazando con defsyms de
> SYMBOLS+REGISTRY). Contenido: `SceneDescTable_0916C8[16]` `{script.l, entities.l}`,
> 14 scripts de la VM de escena (`SceneScript_*`, opcodes $00..$16 con callbacks
> 68000 embebidos tras `lea d(pc),a1/a0 ; rts`), listas `SceneEntities_*` (14 B,
> ctx de cámara $1080E0/$108064/$107FE8/$106F6C), tablas `SceneTrig_*` (op $11),
> colas CCR `ChildRank_CmpByte10_0967A4/_097720` como código, 8 listas
> `AttractSprites_List0..7_*` (registros de 20 B `{flags,x,y,tmpl.l,params[10]}`).
> Zona `$083000..$09C608` al 100 %; código real 92.2 %. CI matcher (runner
> self-hosted) en verde con releases `matcher-*`. Siguiente: huecos de
> `$024E10..$05E000` (Wave BBBBB: `$024E10..$027400`).
>
> **Wave ZZZZ** (192 entradas, 18 154 B; 1 `--data`, 6 `--entry`) — `$07A002..$083000`
> en `boss2_crab_carrier_soundtest_07axxx.s`. Cuadragésimo tercera wave.
> Nombres en `docs/waves/zzzz_names.txt`, args en `docs/waves/zzzz_args.txt`
> (strings del sound test `$7A7E8..$7A954`; entradas forzadas por plantillas
> $E8000 y refs absolutas externas). Módulos: Crew_Hostage_*/Crew_Captor_*,
> SoundTest_Init/Run, M2Boss_* (tmpl 132: Body/Arm/Turret/Shell/Smoke/Debris/
> Wreck/Burst), Crab_* (tmpl 138..140: Claw/Leg/Shard/Patrol), Carrier_* (tmpl
> 129..131: Hull/Cockpit/Mark/Hatch/Cannon/Spawner/Trooper/Gunner/Rider). 27
> renombres; 10 labels promovidos en squad_deploy_module. Código real 87.6 %;
> zona `$05E000..$083000` al 100 %. Siguiente: huecos de `$083000..$09C608`.
>
> **Wave YYYY** (251 entradas, 15 600 B; 11 `--data`, 76 `--entry`) — `$076000..$07A000`
> en `props_fx_explosions_platforms_crew_076xxx.s`. Cuadragésimo segunda wave.
> Nombres en `docs/waves/yyyy_names.txt`, args en `docs/waves/yyyy_args.txt`
> (tablas de sprites, tabla de tablones `$772A8`, FallPath `$78BE0`, jump table
> PathScript `$78FC0`; entradas forzadas por refs absolutas de scripts de misión
> a Explosion_Var01..32 y a los spawners de tablones). Módulos: ScriptedProp_
> hijos, Frag_*, Platform_*, MovingPlatform/BridgePlank, MuzzleFx_*, Spawner_*,
> Explosion_*/Smoke_*/Debris_*, Breakable_Tmpl23..26, PathScript_* (VM de 11
> ops), AutoDemo_*, Crew_Tmpl125..128. Código real 84.0 %; zona `$05E000..$083000`
> al 88.0 %. Siguiente: `$07A000..$083000`.
>
> **Wave XXXX** (177 entradas, 15 662 B; 46 `--data`) — `$071FFC..$076000`
> en `m5tank_finalboss_helpers_scriptedprop_072xxx.s`. Cuadragésimo primera
> wave de `gen_asm_region.py`. Nombres en `docs/waves/xxxx_names.txt`, args en
> `docs/waves/xxxx_args.txt` (LUT `$74460`, sprites `$7466E`/`$75270`, 34 blobs
> del script inline `$749E8..$74F58`, bloque cero, hitbox `$75F34`).
>
> * **FinalBoss_\*** helpers: cierre de los forward refs de WWWW (Wreck A/B,
>   Spark, Smoke A/B, LimbPart Init/Damaged/Critical, Pick*Pattern, spawners).
> * **M5Tank_\*** (tmpl 118, slot 4 `$EDC36`): Drive/Turn/Retreat/Pause/
>   Phase2/Phase3/Die/Sink/Explode; hijos Hull/Cabin/Launcher/Gun/Turret;
>   Rocket Launch/Fly/Home/Explode, Casing, Muzzle A..E, Frag.
> * **M5Missile_Tmpl77** (tmpl 119): velocidad/altura desde +$98/+$99.
> * **MiniScript_\*** (8 ops) + **ScriptedProp_\*** (tmpl 151, slots 0/2):
>   prop destructible por fases con script inline de 34 pasos.
> * Fix `gen_asm_region.py`: target de `movem.l d(pc),regs` (base PC+4).
> * Matcher 7113/7113, 452,812 B (21.59 %); código real 80.9 %.

> **Wave WWWW** (221 entradas, 15 122 B; 1 `--data`) — `$06DFE8..$071FFC`
> en `gunship_m5boss_finalboss_06exxx.s`. Cuadragésima wave de
> `gen_asm_region.py`. Nombres en `docs/waves/wwww_names.txt`, args en
> `docs/waves/wwww_args.txt` (hitbox `Gunship_Hitbox` `$6EB84..$6EBD0`).
>
> * **FireBurst_\*** / **Walker_\*** / **Frag_\*** / **Spawn_\***: cierre de
>   los forward refs de VVVV (`Sub_0006Exxx` → nombres).
> * **Gunship_\*** (tmpl 75): path `$2D4A2E[+$98]`, Crew + Gunner (Aim/Fire/
>   Reload/Wait), Die → 4 WreckPiece (`jmp $6DCE0`), Shell, Explode.
> * **M5Boss_\*** (tmpl 111/112/113): Intro con S5Gate/S5Airship, Idle →
>   PickAttack → Hover/AttackA-C/Charge/Barrage; hijos Bomb/Missile/Shell/
>   Spark/Soldier/Grenadier/Grenade/Rocket/TurretCar; DispatchBy8C.
> * **FinalBoss_\*** (tmpl 114/115/116): Left/Right + 5 Limb + Head
>   (respawn) + Cannon/Casing/Flame/Beam/Debris; fin con `$1071` + `$518`.
> * Matcher 6936/6936, 437,150 B (20.84 %); código real 77.8 %.

> **Wave VVVV** (179 entradas, 15 616 B; 29 `--data`) — `$06A000..$06DFE8`
> en `bazooka_rocketvehicle_walker_06axxx.s`. Trigesimonovena wave de
> `gen_asm_region.py`. Nombres en `docs/waves/vvvv_names.txt`, args en
> `docs/waves/vvvv_args.txt` (29 tablas inline de 4 punteros de sprite).
>
> * **Tank_\*** cola de UUUU (ataque por variante, snd `$E/$1A`).
> * **Bazooka_\*** / **BazookaB_\*** / **BazookaCrew_\*** (tmpl 86/87,
>   90/91, 92/93 ← `Mortar_ToJump6A7D6`): Idle/Walk/Turn/Alert/Attack/Die
>   con arma hija **BazookaWeapon_\*** (espejo de estado `$2CA644`,
>   MuzzleFlash, Rocket `$2CA4B0`, impacto `$1025`).
> * **AllyBazooka_\*** (tmpl 64/65, lo crea `TaskHandler_0849ba` del
>   escuadrón de rescate): avanza a `$110`, ráfagas, muere con el padre.
> * **RocketVehicle_\*** (tmpl 88/89): eslabones Chain3 (`$30696/$30704/
>   $3076A`), fase de rueda `+$8A→+$97` → 29 tablas de poses, Rider hijo,
>   Rocket/MuzzleFlash, Die/Wreck/Corpse/DetachLinks.
> * **Walker_\*** (tmpl 66..74): Init/Approach/Attack(ráfagas)/Die/Explode
>   (16 × `Frag_Scatter` vía `Entity_SpawnLoop16_06E412`).
> * **Frag_\*** / **FireBurst_\***: Shell/Debris(`$6DBD4`)/Smoke/Spark/
>   Scatter (sin/cos `$2C072C/$2C07AC`), chorro `$6DF32` (→ WWWW).
> * Matcher 6715/6715, 421,728 B (20.11 %); código real 74.7 %.

> **Wave UUUU** (180 entradas, 15 488 B; 1 `--data`) — `$066000..$06A000`
> en `barrel_paratrooper_shield_tank_066xxx.s`. Trigesimoctava wave de
> `gen_asm_region.py`. Nombres en `docs/waves/uuuu_names.txt`, args en
> `docs/waves/uuuu_args.txt` (tabla de scripts de animación `$66CD8..$670B6`).
>
> * **FloatBarrel_\*** / **FloatMine_\***: cola de `Barrel_Tmpl8D_065f40`
>   (Submerge/Surface/Drift/Bob/Splash/Pop/Shard; mina con HP `$64`).
> * **Paratrooper_\*** (tmpl 94/95): spawner con cadencia `$2B889A` y
>   umbral de scroll; paracaidista Init → Land → WalkA/B → Leave; Frag/Smoke.
> * **ShieldSoldier_\*** / **ShieldSoldierB_\*** (tmpl 108/109; también
>   desde `M4_PlatformSpawn_085134`) + **Shield_\*** hijo (absorbe golpes
>   frontales; `ShieldLost` → `jmp $58F82`).
> * **S5Gate_\*** / **S5Airship_\***: puerta y dirigible de escena 5 sobre
>   `$106F5E/$106F60`, gancho de cámara `$106F6C`, DropPow (`Pow_EntryB`).
> * **Tank_\*** (tmpl 53/54/55): Drive/Idle/Turn/Brake/AimPlayer/FireBurst
>   → Die/Wreck; Driver (Bail/ToSoldier `$58F82/$5724E`), Turret, Missile
>   guiado (`$5E070`), `SetSpriteByIndex` con asserts `trap #15`.
> * Matcher 6536/6536, 406,112 B (19.36 %); código real 71.7 %.

> **Wave TTTT** (210 entradas, 15 242 B; sin `--data`) — `$062000..$066000`
> en `sniper_camper_mortar_062xxx.s`. Trigesimoséptima wave de
> `gen_asm_region.py`. Nombres en `docs/waves/tttt_names.txt`, args en
> `docs/waves/tttt_args.txt`.
>
> * **Cola de LateProp** (`$62008..$620B6`, `$626F0..$62922`): TakeHit/
>   HPCheck (daño `$2870A`, flash `$5E770`), StepX/ClampX/StepAnim, Register
>   (`$43FAC`), SpawnDebris2 (`$77C7E`), SpawnPair/Child A..D.
> * **TurretCar_Gunner_0620da**: artillero hijo del coche-torreta (`$4CA64`).
> * **Sniper_\*** (`$6212A..`): Init → Aim (Atan2 `$5E018`) → Fire (Shell/
>   Muzzle/Flash) → Flee como soldado (`$58F82/$58FE2`) / Die; Grenade,
>   SmokeA/B, Spark. `Facing_SignDelta_0626b8` / `Matches77`.
> * **MultiStage_Decal_06293e**: hijo de `Prop_MultiStage_054cf2`.
> * **Camper_\*** (`$62A4E..$631D0`): soldado atrincherado (Idle/Watch/Alert/
>   Fire/Fire2/Rearm/Die/Explode), ToSoldier(Flee) vía `$4A0D4`+`$5724E/
>   $58FC2`, DropItem `$9A7CC`, Music `$2352`.
> * **Tent_\*** (`$631D0..`): escombros de la tienda de escena 5 (`$89202`).
> * **Mortar_Tmpl3C_06361e** (tmpl 60/61): Active/Die/Wreck, Shell/ShellB/
>   ShellAim, Crew/CrewHit/CrewToSoldier (`$57226`), `ToJump6A7D6`.
> * **Cannon_Tmpl3E_063ec6** (tmpl 62/63): Idle → Acquire `$5E086` → Fire
>   A/B/C → Rearm; HitCheck A/B; Explode/Die.
> * **Hostage_\*** (tmpl 76..85, `$64550..$64D98`): rehén/POW (Walk/Pose/
>   Kneel/Stand/Shoot/Run/Fall/Thrown/Die/Free), contador `$10E276..$10E27B`
>   (`CountRead_065d4a`/`CountInc_065d7a`), música `$105D`, `trap #15` assert.
> * **Patrol_\*** (`$64D98..$65F40`): soldado de patrulla (sprites `$2C6510`
>   por `+$20&$F`), PlayerNear/AimAtPlayer/FollowParent, Register0..4.
>   **Scene3Prop_\*** (hijo de `Scene3Debris_SpawnPrio4_03d72a`).
> * **Barrel_Tmpl8D_065f40** (tmpl 141): snd `$179`, HP 1, sprite `$2C756C`;
>   continúa en `$066000+` (Wave UUUU).
> * Matcher 6356/6356, 390,624 B (18.63 %); código real 68.6 %.

> **Wave SSSS** (221 entradas, 15 144 B; 10 `--data`) — `$05E000..$062000`
> en `late_props_turrets_05exxx.s`. Trigesimosexta wave de
> `gen_asm_region.py`. Nombres en `docs/waves/ssss_names.txt`, args en
> `docs/waves/ssss_args.txt`. Fix de herramienta: `exg.l` → `exg`.
>
> * **Helpers de runtime tardío** (`$5E000..$5EB98`, >200 call-sites):
>   `Atan2_Angle256_05e018`, `Target_AcquireNearestPlayer_05e086` (caja
>   opcional, P1/P2 por máscara de vivos y |dx|), `Players_AliveMask`,
>   `Player_GetEntity_05e3a2` (handler $FFFFFFFF/$52A/$400/$2AE3E = libre),
>   `Parent_CopyPos*`, `Dist_Approx`, `Rng_Seed_05e998` (LCG 32 words en
>   `$10E230`) / `Rng_Mask_05ea1c`, `Hit_ClassifyAttack_05e6a4` (34 clases,
>   `trap #15` = assert), `HitSoundTable`, `Hud_WriteTimerCounters`,
>   `Fix_DrawMessageRow_05eae4` (jmp desde `$150E`, 30 filas en `$5EB98`).
> * **DebugColl_\*** (`$5EFCA..$5F384`): tarea de depuración de colisión
>   (DIP `$100001` bit1), dibuja el nibble del byte `$43F02` en pantalla.
> * **TowerSoldier_\*** / **HutOccupant_\*** / **HutDoor_\*** (`$5F384..
>   $5FD78`): hijos de `Prop_TowerBase_04dce6` / `Prop_Hut_04d8f2`; el
>   ocupante huye como soldado `$58F82` si el padre muere.
> * **Breakable_\*** (tmpl 31..39), **Sign_\*** (40..49), **ItemProp**
>   (276..280), **Obstacle_\*** (110/111/094/095), **Crate_\*** (122..124),
>   **AimTurret_\*** (142: Atan2 → `$5E23A`, `$13C0E`, escombros `$77C7E`,
>   ítem `$9A7CC`), **GroundNest** (50..52), **LateProp_\*** (`$61AA2..`,
>   helpers en `$062000+` pendientes), `HomingMarker_Targets_060576`.
> * Matcher 6146/6146, 375,382 B (17.90 %); código real 65.6 %.

> **Wave RRRR** (126 entradas, 12 874 B; 5 `--data`) — `$03DA98..$040EF2`
> en `results_pow_squadleader_03daxx.s`. Trigesimoquinta wave de
> `gen_asm_region.py`. Nombres en `docs/waves/rrrr_names.txt`, args en
> `docs/waves/rrrr_args.txt`.
>
> * **Pantalla de resultados** (`Results_*`): tarea `$3DBC8` (instalada por
>   `Attract_InitTaskAdd`), una `Results_PlayerColumn` por jugador con fases
>   Score→Wait→Commit→Bonus→Total→Pause→WaitAll→Winner, premio
>   (`PrizeSprite/Fall/Land`), roster de POWs (`RosterPickA/B`, `RosterDrawA/B`),
>   banners A/B/C, `DrawFrame/DrawLabels` en VRAM fix, `PollStart*`.
> * **POW** (`Pow_*`): crédito de rescate por jugador (`$106F4C/$106F4E`),
>   `RectOverlap`, tablas `AnimTblA/B/C`, estados Tied/Freed/Idle/Walk/Jump/
>   Crouch/Fall/Land/WaitRider/Rescued→Bow→Run→Exit, `Despawn`.
> * **Líder de escuadrón** (`SquadLeader_*`): Spawn/Enter/Turn/Hover/
>   Formation/Swoop/PickAttack/Dive/Regroup/Reform/Circle, órdenes +$84,
>   `HitCheck`, `Death/DeathDone/Respawn`.
> * `SlugCannon_ArmOffsetCurve` (tabla de 65 pares del brazo en el Slug).
> * Matcher: 5925/5925, 360,238 B, 17.18 %; código real 62.6 %.

> **Wave QQQQ** (97 entradas, 7 396 B; 7 `--data`) — `$04FA50..$051914`
> en `allen_oneil_04fa50.s`. Trigesimocuarta wave de `gen_asm_region.py`.
> Nombres en `docs/waves/qqqq_names.txt`, args en `docs/waves/qqqq_args.txt`.
>
> * Jefe **Allen O'Neil** (`Allen_*`): `AcquireTarget` (+$72 objetivo),
>   `Physics`, `Entity_StepMoveWithProbe`, decisiones `Check*` evaluadas en
>   `Allen_Main`, saltos (`JumpDecide/JumpToward/JumpHigh/Airborne/Land`),
>   `KnifeAttack`, `FireMGHigh/Level` + `Allen_Bullet`, `ThrowGrenade` +
>   `Allen_Grenade/GrenadeExplode`, barra de vida de 5 tramos (`$296E72`
>   vía `$2C30`), `Death/DeathFall/DeathLand/DeathFade`, `TouchSensor` ×4.
> * 19 spawners `Barrier/Gatehouse/Fortress_SpawnPiece*` (máscara de bit
>   en +$21, tablas de blit +$70/+$74) usados por la Wave PPPP.
> * Blits por estado de torre/búnker/nido (OOOO), `MemCard_*` glue,
>   `Players_*`, `Nibbles_*`, `PlayerState_FlagTable`.
> * Matcher: 5799/5799, 347,364 B, 16.56 %; código real 60.0 %.

> **Wave PPPP** (40 entradas, 5 296 B) — `$04E580..$04FA50`
> en `props_fortress_04e5xx.s`. Trigesimotercera wave de
> `gen_asm_region.py`. Nombres en `docs/waves/pppp_names.txt`, args en
> `docs/waves/pppp_args.txt`.
>
> * Props de la misión del fuerte: `Prop_Barrier*` (activa/dañada/restos,
>   poste, luz), `Prop_Gatehouse*` (intacta/dañada/restos, bloqueador,
>   tejado, puerta) y `Prop_Fortress*` (espera de scroll → activa → restos,
>   bloqueador, soporte de torreta, lateral). Se encadenan con `$44022`
>   (posiciones absolutas) y `MissionWatch_Spawn $4429E` (listas
>   `$E9348/$E93B0/$E9442`) según umbrales de scroll `$A10/$A20/$A70`.
> * Misceláneos: `Prop_Roof*` (`$E840C`), `FixBlink3_PhaseA/B`,
>   `Prop_Sensor*`, `Prop_Crate`, `Debris_Bouncer/Hop/Roll`, `Prop_SignA/B`
>   (`$E8414/$E8418`), `Prop_Boat/BoatBlast/BoatDebris` (`$E8410`),
>   `FixTile_Set11C2`.
> * Las subs de setup `$4FB8A..$5017A` que invocan quedan para QQQQ.
> * Matcher: 5702/5702, 339,968 B, 16.21 %; código real 58.6 %.

> **Wave OOOO** (64 entradas, 6 384 B; 2 `--entry`) — `$04CBD4..$04E580`
> en `turret_car_props_04cbxx.s`. Trigesimosegunda wave de
> `gen_asm_region.py`. Nombres en `docs/waves/oooo_names.txt`, args en
> `docs/waves/oooo_args.txt`.
>
> * Vehículo-torreta (`TurretCar_*`, creado desde `$070EC0`): base con
>   ángulo 0..31 (`Idle/Track/Recoil`, pasos `AngleStep16/8`), hijo cuerpo
>   (`Body`, HP por dificultad, muerte → padre `+$21 = $FF`), conductor
>   (`Driver/DriverPanic/DriverFlee`), cañón guiado (`Cannon`: histórico de
>   16 ángulos +$88..+$97, `GroundProbeUp`, dispara `Shell` cada 8 frames),
>   helpers de ángulo/offset/offworld.
> * Props de misión 3ª tanda (`Prop_Static/Lamp/Hut/Tower/TowerFlag/Bunker/
>   Bridge/Nest/Shed/Barrier` + fases/wrecks), misma plantilla que GGGG/MMMM,
>   con MissionWatch `$4429E` y blits `$5022A`; registros de spawn en
>   `$096BFE..$096C8A`/`$0975F8..$097670`.
> * Matcher: 5662/5662, 334,672 B, 15.96 %; código real 57.5 %.

> **Wave NNNN** (53 entradas, 3 814 B; 1 `--data` para la tabla de 6
> punteros `$4CB44`) — `$04BB9A..$04CBD4` en `gun_platform_04bbxx.s`.
> Trigesimoprimera wave de `gen_asm_region.py`. Nombres en
> `docs/waves/nnnn_names.txt`, args en `docs/waves/nnnn_args.txt`.
>
> * Emplazamiento de cañón enemigo (plantillas `$E8214..$E8220`):
>   `GunPlatform_Spawn_04bb9a` (4 variantes +$70, hijos según tablas
>   `$2902E8..$2902F4`), ciclo `Rearm/Aim/FireA/FireB` con parámetros por
>   dificultad, hijos `Hatch*` (escotilla por estado del padre), `Shield*`
>   (HP propio, mata al padre), `RiderA/B_*` (tiradores humanos con sprites
>   del soldado/POW que mueren vía HumanDeath `$4A146/$4A154`),
>   `GunPlatform_Gun_04c776` (ataque `$290D70`, sigue al padre con `$5E506`),
>   destrucción `HitCheck/Destroyed/DestroyedWithWreck/Wreck/FlyingPart`.
> * `SpawnStream_ReadNext_04cac4` / `SpawnStream_Dispatch_04cb88`: lector del
>   stream de spawn de 16 B en `$1081B2` por scroll; `SpriteSetPtrTbl6_04cb44`.
> * Matcher: 5598/5598, 328,288 B, 15.65 %; código real 56.3 %.

> **Wave MMMM** (41 entradas, 4 746 B; sin `--data` ni `--entry`) —
> `$053F96..$055258` en `props_mission_053fxx.s`. Trigésima wave de
> `gen_asm_region.py`. Nombres en `docs/waves/mmmm_names.txt`.
>
> * Segunda tanda de props destructibles (misma plantilla que Wave GGGG:
>   `$2942A` template, HP +$66, `$2870A` -> flash `$5E770`, `$28758` ->
>   música + escombros `$77C7E` + puntos `$51A28` + registro `$43FAC`):
>   `Prop_Building` (3 fases), `Prop_Column`, `Prop_CompoundWall` (2
>   variantes, 3 fases) + entidad compuesta `Prop_Compound_Spawn/PhaseA/
>   PhaseB/Done/TriggerSpawn`, `Prop_NeonSign` con actualización del fix
>   layer (`NeonSign_Fix*`, `NeonSign_Tiles*` vía `$2C26`, flag `$1081B1`),
>   `Prop_Stall`, `Prop_Fragile` (3 fases), `Prop_Breakable`,
>   `Prop_HitStages` (sprite por impacto `$298C46`), `Prop_Small`,
>   `Prop_ScaledHP` (HP por dificultad), `Prop_MultiStage` (5 variantes, 4
>   fases, hijo `$6293E`), `FixBlink2_PhaseA/B`, `Prop_Blocker`.
> * Referenciados por los registros de spawn de misión `$096FAE..$097166`.
>   Cierra el bloque `$0527BA..$055258`.
> * Matcher: 5545/5545, 324,474 B, 15.47 %; código real 55.5 %.

> **Wave LLLL** (25 entradas, 2 880 B; 1 `--data` para las tablas de
> estados de muerte, 2 `--entry`) — `$049430..$049FC4` en
> `pow_hang_0494xx.s`. Vigesimonovena wave de `gen_asm_region.py`.
> Nombres en `docs/waves/llll_names.txt`, args en `docs/waves/llll_args.txt`.
>
> * POW colgado de la cuerda: `PowHang_SpawnVariants_04954a` (amplitudes
>   +$7A/+$7C por parámetro y dificultad, hijo `PowHang_RopeSpawn_0497ac`),
>   `PowHang_Swing/Struggle` (física `SwingStep` con `$13C0E`, anim por
>   ángulo `$5E136`, gotas `DropFxTimer` -> hijo `$48B56`), protocolo
>   padre/hijo +$78/+$79 con la cuerda (`RopeIdle/RopeStruggle/RopeCut/
>   RopeBroken`), `PowHang_Freed_049742` (cuerpo `$4ACFE` + ítem `$48CA4`).
> * POW en caída (plantilla `$E81B8`): `PowFall_Spawn_049baa` + sombra/
>   ataque `PowFall_Shadow_049ca4`.
> * Callbacks de scripts de animación (`0800 <addr>` en `$18E612/$18E6CA/
>   $18EC56`), checks de slot/golpe, `HumanDeath_StateTbls_049d8a` (4×34
>   punteros) + `HumanDeath_StateTblPtrs_049faa`,
>   `HumanDeath_HitCheckUnlessCutscene_049fba`.
> * Promovida la etiqueta interior `PowItem_Toss_048ba0__L048bca` en
>   `pow_helpers_048axx.s`. Cierra el bloque `$0478FC..$04BB8E`.
> * Matcher: 5504/5504, 319,728 B, 15.25 %; código real 54.6 %.

> **Wave KKKK** (42 entradas, 6 990 B; 3 `--data` para tablas de sprites
> y punteros, 4 `--entry`; parche del generador) — `$049FF2..$04BB8E` en
> `human_death_049fxx.s`. Vigesimoctava wave de `gen_asm_region.py`.
> Nombres en `docs/waves/kkkk_names.txt`, args en `docs/waves/kkkk_args.txt`.
>
> * Módulo compartido de muerte de humanos (lo llaman las colas de estado
>   del soldado y del POW tras `$2870A`): `HumanDeath_Dispatch_049ff2`
>   elige el estado en las 4 tablas `$49FAA[kind]` (34 punteros cada una,
>   hueco anterior) por el tipo de daño +$58; estados `TumbleBack/Fwd`
>   (5 sprites por tabla), `Collapse`, `Knockdown`, `InitBurst`/`BurstLand`,
>   `Launched` (escala 8.8 en +$5C -> +$32/+$33, "vuela hacia la cámara"),
>   `Burning`/`BurnedDown` + `FlameChild`; hijos `SpawnCorpseA/B`,
>   `SpawnBloodSplash` (A/B), `SpawnSmokePair`; helpers de física
>   (`PhysicsAir/Ground/Fall`, `DampVelocity` = vel -= vel/32) y 3,8 KB de
>   tablas de sprites (`HumanDeath_SpriteTbls_04ac56`).
> * `gen_asm_region.py`: alias local `.L` para labels globales usados desde
>   su propia entrada (GAS fallaba con `bra.b` hacia atrás a un símbolo
>   global a >128 B).
> * Cobertura de código real: 54.0 % (273,040 / 505,608 B); huecos CODE:
>   1172 / 232,568 B.

> **Wave JJJJ** (45 entradas, 2 242 B, verde a la primera; 3 `--entry`
> para entradas fusionadas en el regen) — `$048A44..$049430` en
> `pow_helpers_048axx.s`. Vigesimoséptima wave de `gen_asm_region.py`.
> Nombres en `docs/waves/jjjj_names.txt`, args en `docs/waves/jjjj_args.txt`.
>
> * Helpers pc-relativos del POW: colas de estado comunes
>   (`Pow_FreeStateTail_048a44` / `Pow_TiedStateTail_048a90`), el ítem que
>   lanza al ser rescatado (`PowItem_Toss_048ba0` → `PowItem_Settle_048b34`
>   → cola común `$77F6A`), efectos (`PowFx_HitBurst`, `PowFx_DirSprite`),
>   la cuerda del prisionero atado (`PowRope_Spawn/Idle/Struggle/BrokenA/B/
>   HitCheck`, fases por +$83 del padre, bit 3 de +$13 = cuerda cortada),
>   física del balanceo (`Pow_TiedSwingStep_048f54`) y decisión por
>   distancia al player (`Pow_TargetInReach/InBox/AngleInMask`,
>   `Pow_ShouldRunAway/Wait/Turn`, `Pow_CanBeRescued`, `Pow_AtScreenEdge`,
>   tablas `$2BFE3A`/`$2BFE6A` por variante).
> * Módulo del POW `$0478FC..$049430` completo.
> * Cobertura de código real: 52.6 % (266,050 / 505,608 B); huecos CODE:
>   1182 / 239,558 B.

> **Wave IIII** (42 entradas, 4 416 B, verde a la primera; 3 `--entry`
> para funciones solo alcanzadas por el índice de templates) —
> `$0478FC..$048A3C` en `pow_prisoner_0478xx.s`. Vigesimosexta wave de
> `gen_asm_region.py`. Nombres en `docs/waves/iiii_names.txt`, args en
> `docs/waves/iiii_args.txt`.
>
> * Entidad del prisionero POW: tabla de variantes de spawn
>   (`Pow_SpawnVariantTbl_0478fc` → `Pow_SpawnInit_04797c`), estados
>   libres (`Pow_Idle/WalkToward/RunRight/RunLeft/RunAway/Wait/WalkFree/
>   Stop/Turn/Hurt/GetUp`), secuencia de rescate (`Pow_RescueStart` →
>   `Pow_RescueFaceCount` → `Pow_RescueSalute` → `Pow_RescueTurnBack` →
>   `Pow_RescueGiveItem` → `Pow_RescueThanks` → `Pow_RescueLeave`),
>   variantes ya libres (`Pow_Free*`, spawns `$0483D2/$0483E2`) y el
>   prisionero atado (`Pow_SpawnTiedVariant_048898`,
>   `Pow_TiedIdle/Struggle/Freed`).
> * Siguiente: helpers del POW `$048A44..$049430` (Wave JJJJ).
> * Cobertura de código real: 52.2 % (263,808 / 505,608 B); huecos CODE:
>   1212 / 241,800 B.

> **Wave HHHH** (20 entradas, 1 316 B, verde a la primera) —
> `$0539F0..$053F96` en `props_helpers_0539xx.s`. Vigesimoquinta wave de
> `gen_asm_region.py`. Nombres en `docs/waves/hhhh_names.txt`.
>
> * Helpers pc-relativos de la Wave GGGG: trampa de fuego
>   (`Prop_TrapFlame_053a42` prueba 3 tablas de ataque y guarda la víctima
>   P1/P2 en +$50; `Prop_BurnFollowVictim_053c64` la sigue soltando humo),
>   `Prop_PlayBreakMusicByPhase_053e0c`, `Prop_RunDebrisScriptByPhase/
>   ByPrio` (tablas `$297F50`/`$298062`/`$298074` → StateMachineRun),
>   `Prop_GateDebrisA..D`, `Prop_PickRandomItemPtr_053e9c`,
>   `Prop_SyncSpriteWithParent`, `Prop_IndestructibleChild_0539f0`.
> * Módulo de props `$0527BA..$053F96` completo.
> * Cobertura de código real: 51.3 % (259,392 / 505,608 B); huecos CODE:
>   1213 / 246,216 B.

> **Wave GGGG** (27 entradas, 4 648 B, verde tras parchear el generador)
> — `$0527BA..$0539E2` en `props_destructible_0527xx.s`. Vigesimocuarta
> wave de `gen_asm_region.py`. Nombres en `docs/waves/gggg_names.txt`.
>
> * **Generador**: `move.l #imm8,dN` ($203C) se emite como `.dc.w` crudo
>   porque GAS lo convierte a `moveq` incluso con `:l` (2 casos en
>   `Prop_Wall*`, puntos $10).
> * **Props**: patrón común init ($2942A copia template, snd, HP +$66,
>   puntos +$70, sprite) / loop ($2783A, $28D70, $2870A impacto → flash
>   $5E770, $28758 HP agotada → música $10xx + escombros $77C7E + puntos
>   $51A28 + siguiente estado, $4FA70 fuera de pantalla → $518).
>   `Prop_Sign/Wall/Large/Explosive/Tower/Gate` + `*Stage2/*Wreck`,
>   `Prop_HouseVariants_052e20` (7 variantes, spawnea la pareja de patos
>   `DuckTrigger_SpawnPairLeft_038f48`), `Prop_HutVariants_0530cc` (10),
>   `Prop_Breakable2Stage` (suelta 1..4 `PropDrop_Item`),
>   `Prop_Indestructible`.
> * **Efectos**: `PropDebris_Chunk/Flying` (hereda 2x la velocidad del
>   atacante), `FixBlink_PhaseA/B` (tiles del fix layer vía $2C26 hasta
>   que la cámara pasa de $140).
> * Templates confirmados: sus direcciones aparecen en las listas de spawn
>   `$096CB4..$096F0C` / `$09751A..$097556`.
> * Cobertura de código real: 51.0 % (258,076 / 505,608 B); huecos CODE:
>   1225 / 247,532 B.

> **Wave FFFF** (38 entradas, 4 214 B, verde tras 2 bloques de datos +
> 3 `--entry`) — `$056ACC..$057D04` en `soldier_helpers_056axx.s`.
> Vigesimotercera wave de `gen_asm_region.py`. Nombres y args en
> `docs/waves/ffff_*.txt`.
>
> * **IA**: `Soldier_Think_056b92` (646 B) elige objetivo
>   (`Soldier_FindNearestPlayer_056b38`, distancia aproximada
>   `Soldier_ApproxDist_056b12`), sondea bordes de plataforma, reparte
>   preferencias con `PosRing_FindNear $8F344` y fija los umbrales RNG
>   +$80..+$8A que consumen los estados de la Wave EEEE.
> * **Agarre**: `Soldier_PickGrabAnchor_056e4a` (máscara +$74&7 menos
>   anclas ocupadas, tabla popcount `$56ED2`), `Soldier_GrabLatch_057b06`,
>   `Soldier_GrabSlideToAnchor_057bb4`, `Soldier_GrabFollowPlayer_057cc0`,
>   `Soldier_GrabStruggleProgress_056f10` (forcejeo: +$80 sube $100 por
>   pulsación, se rompe a $B00).
> * **Spawn**: `Soldier_InitCommon_0570a8` + `Soldier_SpawnVariants_057226`
>   (24 stubs, destino de `JmpAbsThunk_06313c`), `Soldier_SpawnDispatch`,
>   `Soldier_Leap_057880` (4 parábolas por +$9D, tablas de ataque por
>   player), `Soldier_Walk_Loop_057582` (bucle principal).
> * **Datos**: dos tablas de ataque de 84 B (`Soldier_AttackTblMelee*`,
>   formato `Slug_AttackTbl*`). Módulo del soldado `$056ACC..$059342`
>   completo.
> * Cobertura de código real: 50.1 % (253,428 / 505,608 B); huecos CODE:
>   1226 / 252,180 B.

> **Wave EEEE** (54 entradas, 5 694 B, verde a la primera) —
> `$057D04..$059342` en `soldier_states_057dxx.s`. Vigesimosegunda wave de
> `gen_asm_region.py`. Nombres y args en `docs/waves/eeee_*.txt`
> (`--data 0x58DF8-0x58E08 --entry 0x57F4E`).
>
> * **Agarre al player**: `Soldier_GrabPlayer` reclama el ancla del player
>   enlazado en +$7A (`PlayerSlot_ClaimAnchor $8F85C`); `GrabStruggle*`
>   alterna animación mientras el player forcejea; `GrabBreak*` suelta (snd
>   $20 si el enlazado es `$100440`, $14B si no) y `GrabThrownA/B` lanzan al
>   soldado con parábola (+$28/+$2A, gravedad +$2E).
> * **Comportamiento**: `Soldier_Idle` elige con el RNG `$5E9B6` contra los
>   umbrales +$80..+$8A entre fidget, pasos laterales, cuchillo
>   (`Soldier_MeleeAttack`, tabla +$4C), retirada, salto, huida, rendición y
>   burla (`Soldier_TauntAnimPtrTbl_058df8`, 4 punteros).
> * **Granadas**: dos posturas (`ThrowGrenadeA/B` + Loop/Recover) y
>   `ThrowGrenadeAim` que ajusta el ángulo +$80 por distancia al player.
> * **Spawn**: `Soldier_SpawnVariantTbl_058f1e` (referida desde
>   `MeleeGuard_DeathToExtern_0427CA`) y `Soldier_Spawn*` (caída, salto
>   lateral, espera al scroll). Helpers `$056ACC..$057D04` quedan para la
>   Wave FFFF.
> * Cobertura de código real: 49.3 % (249,214 / 505,608 B); huecos CODE:
>   1245 / 256,394 B.

> **Wave DDDD** (9 entradas, 8 084 B, verde tras parchear el generador) —
> `$05AA96..$05CA2A` en `sprite_queue_render_05aaxx.s`. Vigesimoprimera
> wave de `gen_asm_region.py`. Nombres y args en `docs/waves/dddd_*.txt`.
>
> * **Generador**: `pcrel_target` ahora devuelve la base de
>   `jmp X(pc,dN.w)` (capstone da X absoluto); sin ello GAS fallaba con
>   "value too large for field of 1 byte" en las 14 jump tables indexadas.
> * **Encolado**: `SpriteDispatchJT_05AA96` (8 variantes por flip H/V y
>   cola ADD/SUB, destino del `jsr $5AA96(pc,d7.w)` de `Sprite_Dispatch`);
>   `Sprite_DispatchSplashHook_05b1b2` re-encola con flip V y pal $7F los
>   sprites sobre tile $3A (reflejo en el agua, misión 1).
> * **Render**: `SpriteQueue_SortAndRenderSCB1_05b232` (heapsort de la
>   cola ADD + inserción de la SUB, jsr desde $20CE) ->
>   `SpriteQueue_RenderRange_05b370` -> `SCB1_WriteTileColumn_05b52e` /
>   `_Term_05bf76` (Duff 4x32 sobre VRAMRW $3C0000, mod $20000);
>   `SpriteQueue_RenderSCB234_05b400` (zoom/Y/X, $8201+n) desde
>   `Vblank_FlushSpriteQueue_05c9d6` (vblank_tick_master+$88, flag $10E1EC).
> * Cabecera A-F documentada. Siguiente: `$057D04..$059342`.
>
> **Wave CCCC** (70 entradas, 11 812 B, verde a la tercera: dos tablas
> de 5 punteros `$2AE90/$2AEE4` + 8 B `$2C900` delante de código) —
> `$02AE3E..$02DD20` en `slug_states_02aexx.s` (7 bloques de datos).
> Vigésima wave de `gen_asm_region.py`. Nombres y args en
> `docs/waves/cccc_*.txt`. **Cierra el módulo del Slug `$0295A6..$030602`.**
>
> * **Spawn en paracaídas**: `Slug_SpawnDrop_02ae50` (template creado por
>   los scripts de intro `$8BD8E..$8C088` con +$98 = variante) ->
>   `Slug_DropVariant0..4` (Slug_Init + torreta `Turret_InitDir0..4` +
>   `PlayerIcon_Pow`) -> `Slug_DropDescend_02b05e` -> `Slug_IdleEnterA`.
>   `Slug_DeadHandler_02ae3e` = centinela que compara `Slug_IsAlive`.
> * **Parado / movimiento**: `Slug_IdleFlat_02b38c`, `Slug_IdleSlope_02b4d2`,
>   `Slug_SlopeIdleEnter/SlopeMount`, `Slug_AccelRightB/LeftB` (música
>   `$10AF/$10B0` por tabla), `Slug_BrakeRight/Left`, `Slug_CruiseRightB/LeftB`.
> * **Cañón**: `Slug_FireIdle/FireFlat/FireRecoil/FireMoveRight/Left/
>   FireSlope`, `Slug_FireAir*` (PlayerSlot_TestMaskCur d1=4 ->
>   `VehicleLaunch_Init` + humo `SlugFx_*`; cadencia +$8E = $1E).
> * **Salto / caída**: `Slug_JumpCrouch -> JumpLaunch (-$4A4) -> JumpAir
>   (tabla `Slug_AirSteerAccelTbl_02c900`) -> JumpLand/JumpLandSlope`;
>   `Slug_FallStart/Fall/Fall_Loop/FallFire`.
> * **Impacto / muerte**: `Slug_HitReact`, `Slug_HitLaunchA/B`,
>   `Slug_HitLandA/B` (entradas 1/2/4/5 de `Slug_StateByAnglePtrTbl`);
>   `Slug_DeathStart -> DeathExplode -> DeathFade` (handler final `$400`),
>   `Slug_DeathLaunch`, `Slug_DestroyedSlide*` (música `$10E9/$10B2`).
> * Cabecera A-F documentada. Siguiente: `$05AA96..$05CA2A`.
>
> **Wave BBBB** (113 entradas, 5 846 B, verde a la primera) —
> `$0295A6..$02AE3E` en `slug_helpers_0295xx.s` (29 tablas de hitbox/anim
> `$295B4..$2A0F8`, tablas de punteros `$2A024`/`$2A060`, 5 bloques
> menores). Decimonovena wave de `gen_asm_region.py`. Nombres y args en
> `docs/waves/bbbb_*.txt`.
>
> * **Init del SV-001**: `Slug_Init_02a0f8` (spawn normal) y
>   `Slug_InitBoss_02a1aa` (arena de jefe, via `Slug_SpawnAtBossArena_02add6`);
>   `Slug_MarkRidden`, `Slug_NoRider_02ad64`.
> * **Tablas**: `Slug_AngleToSpriteIdx_0295a6`, `Slug_Hitbox*`,
>   `Slug_AttackTbl00..10` + `Slug_AttackPtrTbl_02a024`,
>   `Slug_StateByAnglePtrTbl_02a060` (puente a los estados de ZZZ).
> * **Sondas de terreno / fisica / input por layout / HP y gauge**: el
>   resto de helpers compartidos por `Slug_*` (ZZZ) y `PlayerFire_*` (YYY).
> * Cabecera A-F documentada. Siguiente: `$02AE3E..$02DD20` (estados del
>   Slug, 1a mitad).
>
> **Wave AAAA** (96 entradas, 8 788 B, verde a la segunda: dos pares de
> punteros `$29DAxx` delante de `$31C72/$31D26`) — `$030602..$032A02` en
> `player_tables_fx_0306xx.s` (absorbe los RTS de 14 islas C; 44 bloques
> de datos). Decimoctava wave de `gen_asm_region.py`. Nombres y args en
> `docs/waves/aaaa_*.txt`.
>
> * **Granada del player**: `PlayerGrenade_Spawn_0308c2` / `_SpawnB_03093a`
>   (templates de `PlayerFire_Pistol/Flame` y `$44B68/$8B66A/$9A260`),
>   `_SpawnFromVehicle_0308b0` (jmp desde `vehicle_deploy`); angulo segun
>   layout `$106F2A` + `PlayerGrenade_AngleFromSpread_030bfe`, velocidad
>   via `$13C0E`, rebote `$27CEE`, `ExplodeGround/ExplodeAir` ->
>   `Fx_GroundBurst_031c72` / `Fx_AirBurst_031cca`.
> * **Proyectiles enemigos** `EnemyShot_Straight_030c14` / `_Bounce_030c70`
>   (`mission_spawn_boss`, `turret_boss2`).
> * **Caida del vehiculo** `VehicleLaunch_Init_0311c0` / `_InitDrop_0318e6`
>   (indice +$94 en `OffsetTblA/B`, musica $1084, fases +$72 por layout
>   con `Popcount4_0323b4`), `_Fall`, `_Glide`, `_Crash(A/B/C)` (`$10A2D1`,
>   musica $10F2), `_Despawn`, `_ReleaseParent`.
> * **Fx**: `SlugFx_ExhaustOrDrop/Exhaust`, `Fx_Sparkle/SmokePuff/DustCloud/
>   GroundBurst(B)/AirBurst/Spark/Dust/Smoke*`, `Fx_SpawnDustPair_031e5e`
>   (10 variantes, desde `Slug_AccelRight`), `Entity_SpawnSpark*IfBit1`.
> * **Iconos**: `PlayerIcon_Pow_031fca` (snd $1AD), `PlayerIcon_Bubble_03207c`
>   (snd $1B0), `PlayerIcon_FreeFallP1/P2_032112/032142` (snd $A2/$A3, desde
>   `Player_SpawnFreeFall_036c8c`).
> * **Tablas del player** (antes `Sub_000324BC..Sub_000329E8`):
>   `Player_GroundTblA/B`, `Player_VelYTbl(B)/VelXTbl`, `Player_AttackTblA/B`,
>   `Player_Hitbox{Stand,Crouch,Melee,Grenade,Air,Knockback,Death,Slug}`,
>   `Player_WeaponAmmoTbl_0329d4` (999x4 + 150), `Player_WeaponFlagTbl`,
>   `Player_WeaponStateByteTbl_032412`, `Entity_ClearCollisionCb_0329f8`.
> * Resto de Chain3: `Chain3_InitAlt_030696`, `Chain3_PickLink_030704`,
>   `Chain3_LinkCmpField82`, `Chain3_LinksYDeltaIsStep`, `Entity_CmpDepthWithLink8(B)`.

> **Wave ZZZ** (63 entradas, 10 258 B, verde a la segunda: `$02F886` era
> datos) — `$02DD20..$030602` en `slug_vehicle_02ddxx.s` (absorbe 7 islas
> CCR). Decimoseptima wave de `gen_asm_region.py`.
>
> * **Maquina de estados del SV-001** (slot `$100580`): `Slug_IdleEnterA/B/C`,
>   `Slug_Idle`, `Slug_IdleAngled(B)`, `Slug_Jump(B)` (snd $196..$199),
>   `Slug_Hunker`, `Slug_PlayerMount` (recorre `$100440/$1004E0`),
>   `Slug_Drive(B)`, `Slug_TurnToDrive(Alt)`, `Slug_DriveAlt`, `Slug_Brake`,
>   `Slug_Stall`, `Slug_AccelRight/Left`, `Slug_Knocked`, `Slug_DecelA/B`,
>   `Slug_CruiseRight/Left`, `Slug_SetSpeed`. Patron comun: angulo del
>   terreno `Sub_0002A958` -> +$80, sprite por angulo (`$2B0DC8/$2B0C30`),
>   anim por arma/direccion `Sub_0002A9A0` (`$2793xx..$2796xx`).
>   `Slug_StatePtrTbl_02e582` = 80 punteros (despachador `$2A078`).
> * **Dano**: `Slug_DamageSpriteTblA/B/C`, `Slug_DamagePtrTbl/Data`,
>   `Slug_ResetDamageIdx` (destino de `JsrPcThunk_02ff1c`),
>   `Slug_UpdateDamageSprite` (por HP +$66), `Slug_WheelAnim`.
> * **Destruccion**: `Slug_Destroyed_02fc70` (musica $10E9 -> $10AF, +$92 =
>   $30, +$36 = $600, bset #2,+$8D), `Slug_KillInit`, `Slug_SelfDestructAttack`,
>   `Slug_BlastAttack`, `Slug_ExplodeFx` (snd $19A..$19E), `SlugFx_Smoke/Fall`,
>   tabla de 28 templates `SlugFx_ExplosionAnim_02f6c0`.
> * `Chain3_Init_030002` (3 entidades circulares via
>   `Entity_Build3ChainCircular_03060A`, templates `Chain3_TplA/B/C`),
>   `Chain3_Follow/YDelta/Step/VelY/VelX/CheckSyncA/B`, `Chain3_DebugHud`
>   (`$100001` bit4 -> `$5D6C2` ids $7412/$7413/$7415).
> * Helpers: `Slug_TypeIfAir`, `Players_AnyFlag8D3(_SetC)`,
>   `Slug_ClampField92(_ClearXN)`, `Entity_CmpField10WithLink8_02ffe6(_SetXN)`.

> **Wave YYY** (82 entradas, 5 230 B, verde a la primera) —
> `$03C62A..$03DA98` en `player_fire_shells_03c6xx.s` (absorbe
> `JsrAbsThunk_03c62a/03c8d0`). Decimosexta wave de `gen_asm_region.py`.
>
> * **Spawners de proyectil por arma** `PlayerFire_<arma>_<dir>`: Pistol
>   (template `$3093A`, musica $10F7), HMG (`$9C4D4`, $10F6, decrementa
>   +$82/+$85), Shotgun (`$9C25E`, $10F3), Rocket (`$9BEC2`, $10F5, crea
>   casquillo `ShellCasing_Rocket`), Flame (`$308C2`, $10F4; variantes
>   Neg/0/Pos de desviacion y `_SpreadN_M` con dos llamas). Comun: a2 =
>   player, abortan si +$82 == 0 (`movem.l (a7)+,a0; rts`), +$98/+$99 =
>   angulo/signo, +$9A rafaga, +$9B desviacion.
> * `ShellCasing_Pistol_03d396` (snd $184), `Scene3Debris_*` (solo
>   `$106ECE == 3`), `Player_DebugMarker_03d842` (si `$10FD8F`),
>   `Player_SpawnFx3D8FA`, `SlugCannon_ArmOverlay_03d944` (brazo sobre el
>   Slug `$100580`, tabla `SlugCannon_ArmSpriteTbl_03da02`).
> * 3 handlers de brazo residuales: `PlayerArm_SlugRideA/B`, `PlayerArm_Fall2`.

> **Wave XXX** (152 entradas, 8 224 B, verde a la primera, region sin
> islas) — `$03A60A..$03C62A` en `player_arm_air_death_crouch_03a6xx.s`.
> Decimoquinta wave de `gen_asm_region.py`.
>
> * Los 47 handlers de brazo/arma `PlayerArm_*` restantes: aire
>   (`PlayerArm_Jump_03a60a`, `PlayerArm_Fall_03a656`, `AirShootA..C`,
>   `JumpShootA..D`/`FallShootA..D`, `JumpShootDownA..C`/`FallShootDownA..C`),
>   muerte (`PlayerArm_DeathA..H` + `bset #0,+$13` para que
>   `PlayerArm_Spawn` libere el brazo), `PlayerArm_SpawnFall_03bea0`,
>   agachado (`CrouchEnterA/B`, `CrouchIdleA/B`, `CrouchShoot`, `CrawlA/B`,
>   `CrouchGrenade`, `CrouchMelee`, `CrouchReload`), melee (`MeleeC`,
>   `AirMeleeA/B`). Cada uno elige tabla por +$72 (modo de disparo) y
>   carga el sprite con `$28CD4`; 105 tablas `PlayerArm_SpriteTbl_*`.
> * Todas las direcciones verificadas contra las tablas de anim del player
>   `$2796xx..$279Fxx` (campo -4); 5 handlers sin referencia.

> **Wave WWW** (125 entradas, 7 270 B de codigo + 47 bloques de datos,
> verde a la primera) — region `$0388F0..$03A60A` en
> `player_arm_weapon_fx_0388xx.s` (absorbe 24 islas C espurias).
> Decimocuarta wave de `gen_asm_region.py`.
>
> * **Agachado** (cierre de VVV): `Player_CrouchThrowGrenade_0388f0`,
>   `Player_CrouchMelee_038a28`, `Player_CrouchReload_038ae6`.
> * **Entidades hijas del player**: `DroppedWeapon_Spawn_038be4` (arma que
>   cae al perder municion; rebote, spawnea `$77D88`), `Parachute_Spawn_038cf6`
>   (paracaidas del spawn: Open -> Swing por vel X del padre -> Release/
>   FallAway), `DuckTrigger_SpawnPair*_038f12/038f48` (par de sensores por
>   slot que marcan +$88 bit0 del player), `PlayerDeathFx_Splash/Ripple/Alt`.
> * **Overlay de brazo/arma** `PlayerArm_*`: `PlayerArm_Spawn_0394a8` es el
>   template que `PlayerEntitySpawn` crea con `jsr $4AE`; sigue al padre
>   (`PlayerArm_FollowParent_03937c`), despacha por anim id del padre (+$70)
>   e invoca el handler guardado en +$74 (puntero -4 de cada tabla de anim
>   `$2796xx`). 31 handlers `PlayerArm_<pose>` (Stand/Walk/WalkLoop/
>   ShootStand*/WalkShoot*/Melee*/Turn*/Crouch*/SpawnLand*/JumpMelee), cada
>   uno con su `PlayerArm_SpriteTbl_*` de 10 punteros (5 armas x 2
>   jugadores; indice `PlayerArm_WeaponTableIndex_03933a`).
> * Resto de handlers de brazo (`$3A60A..$3C6C4`) -> siguiente wave.

> **Wave VVV** (39 entradas, 8 466 B, verde a la primera) — region
> `$036632..$0388F0` en `player_air_death_crouch_0366xx.s` (22 huecos;
> absorbe 23 islas C espurias). Decimotercera wave de `gen_asm_region.py`.
>
> * **Aire**: `Player_JumpStart_036914` (anim $25/$26, vel Y $9CD, +$90 =
>   5 frames de hold) -> `Player_JumpAir_036a70` (control aereo por tabla
>   `Sub_000324D0[layout]`, clamp `$267F4`, suelo `$27B66`, anilla
>   `TargetRing_FindPending` -> `Player_HangRing_036638`), golpe
>   `Player_Knockback_036d64` -> `Player_Fall_Physics_036e42`,
>   `Player_SlugJumpOff_036796`, `Player_SpawnFreeFall_036c8c`,
>   `Player_AirActionSelect_037168`.
> * **Muerte**: targets de la tabla de 68 (`Player_Death_Generic` x30,
>   `Player_Death_Fall` x29, `_FallSpawnFx`, `_Alt`, `_PrioE000`, `_Debug`,
>   `_Timed`), snd $1053 + musica $1123/$1087, cadaver `$78840`,
>   `Player_DeathPit_037b8e`, `Player_Death_Despawn_037c1a`.
> * **Agachado**: `Player_CrouchEnter_037c74` -> `Player_CrouchIdle_03827a`
>   (anim $30, hitbox $32598), `Player_CrawlRight/Left` (±$120),
>   `Player_CrouchShoot_03873c`, `Player_CrouchWeaponEmpty_038086`,
>   `Player_CrouchExit_037ec2`.
> * Etiquetas nuevas en ficheros previos: `Player_SpawnLand_SetInvuln1E_033ec2`,
>   `Player_SpawnLand_Reset_033ede`, `Player_RideSlug_Frame_03652e`.

> **Wave UUU** (36 entradas, 9 050 B, verde a la primera) — region
> `$0342C4..$036632` en `player_states_0342xx.s` (4 huecos; absorbe las
> islas C `JsrPcThunk_034d2c/0354ec` y `SetTaskHandler_034b30`, que eran
> colas internas). Duodecima wave de `tools/gen_asm_region.py`.
>
> * **Maquina de estados en suelo**: `Player_Stand_034704` (anim $10) con
>   `Setup/InputMove/Tail`; `Player_WalkRight/Left` (anim $11, vel ±$300,
>   facing +$3A) -> `Player_WalkLoopRight/Left` (anim $12, giro en sitio
>   con `eori +$3A`); variantes `_Shoot`/`_ShootUp` via poses
>   `Player_WalkShootPose/WalkShootUpPose/WalkLoopShoot*Pose` (anim $22/$23).
> * `Player_TurnRight/Left` (anim $31), `Player_Melee` (cuchillo, anim $33,
>   +$4C = `Sub_00032638`, `$283CA`), `Player_ThrowGrenade_Stand/Walk/
>   WalkLoop`, `Player_RideSlug_0364a2` (slot Slug `$100580`, invuln $3C).
> * Patron comun: +$7C/+$7E fase, +$8C bits1-3, tabla sprite por +$72/+$78
>   via `$28CD4`, +$60 = $32500, bucle de frame `Player_FrameCommon` +
>   `$27A92` + `Player_CheckDeathOrState21`, switch de `Player_ActionSelect`
>   (d1: $FF Melee / 3 Grenade / 4 `Sub_0003873C` / 1 ShootUp / else Shoot).
> * Nueva etiqueta global `Player_Idle_StateCheck_034046` (entrada
>   secundaria de `Player_Idle`, usada por WalkLoop).
> * 4 cuerpos sin xrefs ($034438, $0344F2, `_Alt_0358aa`, `_Alt_035a0a`).

> **Wave TTT** (66 entradas, 5 354 B, verde a la primera) — region
> `$032A02..$0342C4` en `player_core_032axx.s` (38 huecos, 3 tablas
> `--data`, 1 `--entry`). Undecima wave de `tools/gen_asm_region.py`.
>
> * **Nucleo del jugador**: slots `$100440`/`$1004E0`; campos +$71 arma,
>   +$82 municion (clamp 999), +$80 bombas (clamp 99), +$87 timer de
>   invulnerabilidad, +$8C/+$8D bits de estado, +$78 nibble de direccion.
> * `Player_SetWeaponAndAmmo` (tabla de municion por arma `$329D4`),
>   `Item_GiveAmmo_ToPlayer` / `Item_GiveBombs_ToPlayer` (callers de RRR),
>   `Player_InvulnBlinkStep` (paleta por `OpcodeOffsetTable_0329EE`),
>   `Player_WeaponMusicTable_032d28` + `Player_PlayWeaponMusicIfFlag`,
>   `Player_PlayLifeMusic` ($1123/$1087).
> * **Input**: `Input_ForwardByFacing/BackwardByFacing` (espejo por +$3A),
>   `Input_JumpOrFire`, `Input_FireByMode/JumpByMode` (layout `$106F2A`),
>   `Player_ReadDirNibble/ReadFireNibble`, `Player_ActionSelect_0330d0`
>   (d1 = $FF hit / 3 fire / 4 / 1 / 0) consumido por `Player_Idle_Tail`.
> * **Granadas**: `Player_ThrowGrenade/_Back/_Down` (+$80--, `$5EAB6` con
>   `JmpAbsThunk_033346`, `$5DD02`, `$517FE`), `Player_JmpGrenadeBounce/Down`.
> * **Spawn**: `Player_SpawnStart_0336dc` -> `Player_SpawnByMode` (caida
>   `$279F8A` / paracaidas `$279B2C`) -> `Player_SpawnLand/LandB` ->
>   `Player_SpawnLand_Done` -> `Player_Idle_033d64`; `Player_DeathGate_0334c6`
>   (muerte si `$106E92`==0, ahogado en escena 1 con `$27DB2`->$40).
> * Estados: `Player_Idle` ($279828), `Player_Crouch/CrouchB`
>   ($27973E/$27981E), `Player_Reload` ($2796F8/$279702); comun
>   `Player_FrameCommon_032ff2`, suelo `$5DD56` con hitbox `$324C6/$324BC`.
> * Tabla `Player_StateTable68_03338a` (68 ptrs -> 7 handlers `$37684..
>   $37B00`), `PlayerStateLUT_03349A`. Correcciones: tamano real de
>   `PlayerRoute_PublishState_033522` (80 B), defsym `Probe_Bit3At100001_End`
>   -> `Player_DeathGate_0334c6`.
> * Extra: isla **PAUSE** `$013D20..$013D6A` (`fix_pause_text_013d20.s`) y
>   mapa curado de zonas en `tools/measure_coverage.py --zones` /
>   `docs/COVERAGE.md` (codigo real cubierto 31.8 %).

> **Wave SSS** (23 entradas, 2 546 B, verde a la primera) — region
> `$18D152..$18DB78` en `player_grenade_18d1xx.s` (2 huecos, 5 tablas
> `--data`, 8 `--entry`). Decima wave de `tools/gen_asm_region.py`.
>
> * Es el **unico bloque de codigo del banco alto** (archivo `$18Dxxx` =
>   CPU `$28Dxxx`; el resto de `$09C608..$200000` son datos salvo las
>   islas C ya registradas en `$19C95A/$19CB64`).
> * **Grenade_***: granadas del jugador (item 291 Bombs). Tres lanzamientos
>   despachados por la triada `jmp $28D876/$28D9DC/$28D7AA` en
>   `$033346..$033358` (`Grenade_ThrowHeavy` con rebote unico y
>   `Grenade_HeavyFall`, `Grenade_ThrowBounce` con timer +$5C=18 y
>   `Grenade_BounceFall`, `Grenade_ThrowDown`), mas `Grenade_Throw/ThrowB`
>   (via `Grenade_SpawnCopyA/B` + `$517FE`) -> `Grenade_Throw_Common`
>   (snd `$7C`/`$14C` por jugador, prio `$D000`) -> `Grenade_Fly` (freno
>   aereo vel X>>4, `$27D50`; `$1081AE` selecciona `$27BC8`).
>   `Grenade_Explode{,_B,_C}` -> `Grenade_Explode_Common` (musica `$1027`,
>   snd `$D`, humo `Grenade_Smoke_Init/Run`), `Grenade_ExplodeBig` (snd
>   `$178`, mapa `$29E76C`).
> * 5 tablas de animacion de 10 B/registro (`Grenade_AnimSpin/SpinAlt/
>   ExplodeBig/Explode/Smoke`, terminadores `$1D00`/`$1600`/`$0100+ptr`).
> * **6 islas espurias absorbidas**: `JsrAbsThunk_18d56c/18d57e/18d746/
>   18d766/18d9d4` y `SetTaskHandler_18d6f0` eran colas de funciones
>   reales (`jsr X.l; rts` finales); eliminadas de los .c y del registro.

> **Wave RRR** (96 entradas, 9 196 B, verde a la primera) — region
> `$09A0BC..$09C608` en `items_score_crates_09a0xx.s` (28 huecos, 6 tablas
> `--data`, 5 `--entry`). Novena wave de `tools/gen_asm_region.py`.
>
> * **Gun_***: cañon orientable (angulo 8.8 en +$70, mapas por octante
>   `$2F6540`, hijo `Gun_Child` con poses +$73) que registra su boca en el
>   anillo de targets (`$8F3A6/$8F3BE`) y dispara rafagas de 3
>   (`Gun_FireShell` -> `Gun_Shell`, `Gun_FireBullet` -> `$3093A`); caller
>   `Airship_Wait_0893ac`.
> * **Item_*** (E8000 284-291/293/294/316/317): `Item_SpawnTable_09a5f4`
>   {hitbox, pickbox A/B, 30 handlers} + `Item_SpawnFromParent` (C=1 si
>   crea); Ammo1..4, AmmoSeq (fade `$2F7A28`), Weapon/WeaponSwap/Bombs
>   (`$32C12/$2A28E/$32C7E/$2A2BA`), Food (21 mapas, Wait/Blink/Rot/
>   Thrown/Bounce, combo `$10E488/$10E489`), Pow, ComboTimer, Static.
>   `Item_Taken_Snd{A..D}` + `Item_Taken_Rise`.
> * **Score_Popup_*/Score_Digit_***: digitos flotantes por kind 0..4 o
>   valor BCD (`$51A44`), snd por jugador (+$68), Lift/Hold/Blink.
> * **Chute_A/B/C**, **Crate_*** (caida `$27D50`, `Crate_Debris` con seno,
>   seguidores de paracaidas, `Entity_IntegrateVelFrac_09c072`),
>   **Thrown_*** (sombras a 3/8 y 3/4) y **Flag_Init** (`$10A2D1`).

> **Wave QQQ** (125 entradas, 9 252 B, verde a la primera) — region
> `$09773C..$099F3A` en `hiscore_memcard_mobs_0977xx.s` (96 huecos
> cerrados, 2 tablas `--data`). Octava wave de `tools/gen_asm_region.py`.
>
> * **HiScore / NameEntry**: tabla de ranking en `$100002` (10 x 12 B:
>   tag, glifo de rango, score u32, 3 letras) con defaults en `$2F53B2`;
>   plantillas `HiScore_Tpl_Frame/Loader/StaticLogo` (desde
>   `AttractHandler_*`), `HiScore_InsertScore` (desplaza filas, C=1 si entra
>   en el top-10), `HiScore_TryEnter_P1/P2` (desde `$598D2/$598DC`), editor
>   `NameEntry_*` (alfabeto `$2F54B2`, timeouts 900/300 f, filtro
>   `CensorName`).
> * **MemCard**: dialogos load/save (`$51636/$5168A`) con BIOS `$C00468`
>   (op 2/3/4) y `$C0046E`; fichero "METAL SLUG" (`$981E8`); strings fix
>   `MemCard_Str_*` terminadas en `$FFFF`.
> * **LogoScene** (`$98720`, `AttractHandler_10002C`): 8 piezas + centro
>   con rampa de brillo `$2F5560`, 200 frames.
> * **Mobs** plantillas E8000 174/175/176/178/195/196/201/202/207-211/
>   220-222: cuerpo comun (`$267E2`, prio `$8000`, HP 1) y estados
>   Walk/Hit/Flee/Patrol/TurnAround/Pause/Drop/Sit/RunLeft; tablas de
>   mapas `$2F558E..$2F55B6`, dispatch `$2F55A6`.
> * **Trail ring** `$10E3BE` (16 x 12 B; head/tail `$10E47E/$10E480`):
>   `Trail_RingReset` (SceneLoader_Main), `Trail_RingAdvance`
>   (SceneScriptVM), `FindByKeyRange/FindNearest/LookupById`
>   (callers en `$26Bxx..$277xx`), variante `Entity_TrailRecord_Alt`.
> * **Opciones**: `OptionSelect2_Tpl_099b06` (2 filas, resultado
>   `$106ED5`), `OptionsMenu_Tpl_099ba6` (`$1700`: dificultad `$10FD8B`,
>   vidas `$10FD88`, creditos `$10E486`, modo 2P `$10FD92`),
>   `DebugCursor_*`.
> * Correcciones: `NopCCR_099f0a/099f34` eliminadas (colas de `movem.w`
>   a `$3C0000`); labels `Entity_TrailRecord_HasId/StoreSlot` exportadas;
>   `gen_asm_region.py` emite `moveq` con signo. Regiones
>   `$0916C8..$0967B4` (SceneDescriptor[256] + scripts) y
>   `$096BBC..$097730` (listas de spawn) identificadas como datos.

> **Wave PPP** (92 entradas, 7 678 B, verde a la primera) — region
> `$08F6D2..$0916B8` en `gameover_continue_08f6xx.s` (50 huecos cerrados,
> 19 `--entry`). Septima wave de `tools/gen_asm_region.py`.
>
> * **Mascaras de slot y anclas** (`PlayerSlot_*`, `Anchor_*`): +$96/+$97
>   del slot de jugador son mascaras por paridad de frame (`$106F28&1`);
>   `Anchor_GetWorldPos` lee un descriptor {tabla, dx, dy} en +$70 y
>   devuelve la posicion mundial de un ancla; `PlayerSlot_Try/ClaimAnchor*`
>   reservan anclas (bits 0..2) — usados por la IA del jugador ($57Bxx) y
>   los bosses (`Boss_Descend`, `Boss2_TickParent`).
> * **Game Over** (`GameOver_*`, tarea anadida en `$1300` tras poner
>   `$106ECE/$106ECF=$FF`): `Boot` carga la escena $A y la paleta
>   `$2F4A32`; `Spawn` crea ~40 hijos (letras `GO_Letter_V0..V13`, figuras,
>   scrollers, partes) y sube con `GameOver_IntegrateY`; `Wait/Wait2/Final`
>   encadenan jingles `$10DF/$10DE`, `WaitCredit` espera `$10A2CF` y
>   `Continue*` recorre la tabla `$2F4BF8` y muestra el texto.
> * **Efectos** (`GO_*`): `GO_Sprite_Right/Left/Mid/MidB` (entrada
>   deslizante + tabla `$2F4C48`), `GO_Prop_A/B/C`, `GO_Zoom` (decaimiento
>   a la mitad por frame + `$2F4B24`), `GO_Shake` (`$2F4B48`), `GO_Flash`,
>   `GO_Banner*`, `GO_Glow/_Drift` (brillo +$32/+$33), `GO_Figure*`,
>   `GO_Scroller*` (wrap en x=$110), `GO_Particle` (RNG `$5E9B6`),
>   helpers `GO_ParentState2x_IsN`.
> * **Continue** (`Continue_*`): `Continue_Tpl` (anadida por
>   `AttractHandler_2Task_0010F2`), `Continue_Tmpl227` (`$E8000[227]`,
>   primera tarea de `Init_ModeToggle_001260`), texto fix-layer con
>   `Fix_BlitRect/Row/Str` (`$5DA9C/$5DA56/$5DAD8`), cuenta atras BCD de
>   `$10FDDA`, filas `$2F4CB8`, titulo `$2F4C98`.
> * 18 defsyms promovidos; +17 RTS mid-isla; 32 call-sites renombrados.

> **Wave OOO** (84 entradas, 3 980 B, verde a la primera) — region
> `$08E4E4..$08F6D2` en `critters_rings_08e4xx.s` (54 huecos cerrados).
> Sexta wave de `tools/gen_asm_region.py` (5 `--entry`); fix del generador:
> `moveq #$FF` se emite con signo (`#-1`), GAS rechazaba el inmediato.
>
> * **Bichos sencillos** (plantillas Mission VM 182/183/184): `Bobber_*`
>   oscila en y con aceleracion ±4 entre y>=$1D0 e y<=$160; `Leaper_*`
>   salta con vel y -($40+rand) y acel 4; `Runner_*` corre en x. Todos
>   usan la **fisica de grunt** (`Phys_FacingFromParam`, `Phys_VelXFromParam`
>   = +$99<<5, `Phys_GroundKill`, `Phys_PlayerNearX`, `Phys_ScrollTarget`/
>   `_ScrollReached`, `Prio_Set8018`, `Snd_ByParam9A_*`) — los 10 defsyms
>   forward de NNN quedan resueltos y renombrados (144 call-sites).
> * **Nido de 4 etapas** (`Nest_*`, plantilla 185 + spawn desde `$4E274`):
>   HP $64 por etapa, mapas de dano `$2F41F2 -> $2F4202 -> $2F4284 ->
>   $2F4306 -> $2F4388`, crea 3 `Swarmer_*` (atan2 `$5E018` + seno
>   `$13C0E` hacia el padre, huyen a magnitud $300 si el padre esta
>   golpeado); `Nest2_*` (187) variante con flash; `Nest_DieIfParentGone`
>   (jsr pc desde las etapas). `Swarmer_Tmpl186` version suelta.
> * **Props**: `Static_Tmpl188`, `CamProp_Tmpl189/190` (siguen a la camara
>   con `$4407A`; el 189 lo crea `MissionEnd_SpawnDropper`), `Lob_Tmpl191`
>   (vel y = -(+$99<<5), aterriza en y<=$190), esquirlas `Shard_V0/V1/V2`
>   (tabla `$1E9A42`, caen hasta la y del abuelo).
> * **Rings** (`$10E2F2` zonas / `$10E33A` posiciones / `$10E362`
>   targets; cabecera {tail, head, scan_end, count} + datos en +8):
>   `Ring_Reset`/`Ring_Compact`, `Rings_InitAll` (desde
>   `SceneLoader_Main_043568`) / `Rings_CompactAll`; `Zone_Tmpl152` +
>   `ZoneRing_Push/PushOffset/HitTest` (rects 8 B en mundo);
>   `PosRing_FindNear` (±$40/±$30, usado por `MeleeGuard_Think`);
>   `TargetRing_NewId/Register/FindPending/InRange/ClaimById/ClaimByKey`
>   (registros 6 B {x,y,key,state}, usados por `Turret8_*` y la IA del
>   jugador en `$36xxx`); `Turret8_SndByState`.
> * 19 defsyms promovidos; +4 RTS mid-isla. `Subsystem_AudioSceneInit_08F158`
>   renombrado a `Rings_InitAll_08f158` (no era audio).

> **Wave NNN** (80 entradas, 4 442 B, verde a la primera) — region
> `$08D17A..$08E4E4` en `grunts_capsule_08d1xx.s` (67 huecos cerrados).
> Quinta wave de `tools/gen_asm_region.py`; nuevo `tools/wave_apply.py`
> aplica la salida del generador a `registry.py`/`symbols.py` y produce la
> lista de renombres para los call-sites.
>
> * **Test de pantalla con latch** (`Screen_InBounds{X,XWide,Y}_Latch[ed]`):
>   bit7 de +$13 se arma al entrar en pantalla; despues, salir => C=1
>   (muere). **Integradores 8.8** (`Pos_IntegrateX88/Y88/XY88[_Accel]`):
>   fraccion en +$26/+$27, velocidad en +$28/+$2A, aceleracion +$2C/+$2E.
> * **Capsula de fin de mision** (`Capsule_*`): tarea creada por
>   `SceneB_Init`/`SceneC_Init` (MMM): vuela, desciende, se abre con pulso
>   de brillo (+$32/+$33), sube/baja, y `Capsule_CheckMissionEnd` lanza
>   `MissionEnd_ScrollOut` (mision $B, `$106F5C>=$42C0`) o
>   `MissionEnd_Marker` (mision $C, `>=$4280`) -> `ScrollUp` -> `Wait`
>   (spawn de `Cut_Dropper`) -> `Idle`.
> * **Soldados rasos** (`Grunt_*`, `Grunt2_*`): plantillas Mission VM
>   153/154/157/158/160/162/180/181; comportamientos al azar via tablas
>   `$2F3712`/`$2F3722` (`Grunt_PickRand_*`), porteador que suelta un
>   `Grunt_Dropped`, saltador `HopDown/HopUp`, golpeado y lanzado con seno
>   (`Grunt_Hit_*`), corredor, mapa por direccion
>   (`Grunt_LoadMapByDir` = `$2F3CDC[(+$99>>4)&7]`); `Sentry_*` y
>   oscilador `Swinger_*` (amplitud en +$74). La fisica comun esta en
>   `$8EFCE/$8F002/$8F010/$8F02C/$8F040/$8F070/$8F084/$8F0D0/$8F108`
>   (hueco siguiente, 10 defsyms forward).
> * 22 defsyms promovidos; +15 RTS mid-isla.

> **Wave MMM** (67 entradas, 4 990 B, verde a la primera) — region
> heterogenea `$08BA04..$08D17A` en `cutscene_anim_08baxx.s` (39 huecos
> cerrados). Cuarta wave de `tools/gen_asm_region.py` (3 tablas `--data`,
> 2 `--entry`); fix del generador: los labels promovidos a globales por
> referencias desde otro hueco se usan tambien desde su propia entrada.
>
> * **Proyectiles**: `Proj_Bounce_V1/V2` (despacho por
>   `Proj_Bounce_HitTable_08b944` + `Table_LookupPointerBounded $772`),
>   `Proj_Burst_*` (dos tablas de 4 punteros identicos elegidas con RNG —
>   placeholders), `Proj_Shell_08bc0c` con `Hitbox_08bb8c`/`SpriteMap_08bbde`
>   embebidos; `Proj_Tmpl_InitHitbox[Probe]` encadenados desde las plantillas
>   168..170 de LLL. `Proj_Bounce_08b9ba` (LLL) pasa a 74 B al absorber el
>   `movea.l #-1,a1` que estaba partido en `$8BA00`.
> * **Interprete de script de animacion** (`Anim_ScriptStep*`): registros de
>   8 B {dur, val, b4, b5}; variantes Fix_A/Fix_B escriben caracteres en la
>   capa fix (via `$47872` o directo en `$3C0000`) y spawnean objetos.
> * **Iconos de ranura** (`Icon_Base`, `Icon_Slot1..4` + `_Lit`,
>   `Icon_Slot_Run_08c4a0`, `Icon_Anchor_*`): 4 slots en y=$12B con mascaras
>   +$8A=$10/$20/$40/$80 contra los bytes +$92/+$93 del padre; el ancla
>   elige script por idioma (`$10FD83/$10FD92`) y deja caer un objeto
>   (`Icon_Anchor_Drop`). Son los 6 task-adds de `Anim_State_F1_08C008`.
> * **Cutscene** (`Cut_*`): plantillas 244/245/246/248/320 del Mission VM,
>   `Cut_Watcher_*` = tarea paralela arrancada por `MissionDriver_Init`
>   (vigila `$106F5C`, arma `$10E2EF`), `Cut_Fade`, panel de texto fix
>   (`Fix_TextRow_Draw/Clear/DrawOne`, `Cut_TextPanel[_Type]`, `Cut_Banner`),
>   `Scroll_StepVelX/Y`, `Cut_Dropper_*`, `Cut_Item`.
> * **Escenas B/C** (`SceneB_Init` -> `Stage2..6` -> `Tail`, `SceneC_Init`):
>   arranque de misiones $0B/$0C desde el dispatcher de modo (`$164C`,
>   `$1688`): `SceneLoader_Main`, velocidades de scroll `$106F60` por
>   umbral de `$106F5C` y por numero de jugadores (`$106EAE`).
> * 20 defsyms promovidos; +13 RTS mid-isla; +3 forward (`$8D24C`,
>   `$8D2D4`, `$8D3B4`).

> **Wave LLL** (98 entradas, 10 690 B, verde a la primera) — **cluster de la
> escena 5**: cierra los 43 huecos de la region `$088A56..$08BA00` en
> `scene5_airship_088axx.s`. Tercera wave de `tools/gen_asm_region.py`
> (92 entradas automaticas + 6 fronteras `--entry` + primera tabla de datos
> embebida con `--data 08B944-08B9A2`), analizada semanticamente a mano.
>
> * **Origen de los spawns**: lista 5 (`$097422`) de `JumpTable_096B9C`
>   (`$088F74`, `$088F8E`, `$0890CC`, `$089E8E`, `$08A006`, `$0893AC`,
>   `$08A10C`, `$08A74E`, `$089C44`, `$089E2C`, `$089C58`) y plantillas del
>   Mission VM `$E8000[168..170]` (`Proj_Tmpl168/169/170`), `[254..257]`
>   (`Proj_Drop_V0..V3`), `[258]` (`S5_Bunker_088ff4`) y `[270]`
>   (`Proj_Thrown_08b258`). Los callbacks `SprCb_Lamp2On/Off` y
>   `SprCb_Lamp3On/Off` cuelgan de la tabla de animacion de sprites `$1EE1xx`.
> * **Dirigible de desembarco** (`Airship_*`): espera `$106F54 >= $280`,
>   snd `$CF`, crea la torreta `Turret8_*` y un `Airship_PlayerTracker` por
>   jugador; desciende en espiral (`Airship_SpiralDescent_08b718`) sondeando
>   el suelo; al aterrizar (`$10E39C=0`, `$106F5E=-1`, `$106F60=$8000`)
>   suelta soldados (`Airship_DropSoldier_08b82c`, antes `PcThunkTarget_*`)
>   y publica el lock de camara; flota hasta cam x >= `$680` y despega con
>   snd `$1026` dejando rastro (`Entity_TrailRecord_099812`).
> * **Torreta de 8 direcciones** (`Turret8_*`): Init -> Aim -> Track
>   (`Turret8_RotateStep_08b5c8`, tablas `$2EE318/$2EE3F0/$2EE4C8`) -> Fire
>   (`Turret8_FireBullet_08b626`) -> Cooldown; canon hijo y casquillos con RNG
>   (`Turret8_Casing_*`).
> * **Torres A/B** (`S5_TowerA_*` / `S5_TowerB_*`): snd `$1DC`+`$D3`, 3-4
>   portillas `TowerPort_*`, soldado `$77228`, HP aleatoria `$2C06AA` al
>   llegar cam x a `$7D0`/`$950`, etapas de dano en HP `$29A`/`$14D`,
>   destruccion con score `$5000`, 6 explosiones `$7808A`, bandera
>   `Wreck_FlagSet_08ae0c` (`$10E39E=1`), humo `Wreck_SmokeRise` y chispas
>   `Wreck_SparkBurst`/`Wreck_Spark`.
> * **Campamento y props**: `S5_PropStatic_A/B/C`, `S5_Bunker`,
>   `S5_Camp_Spawn` -> `S5_Tent` -> `S5_Tent_Ruin`, `S5_Depot` ->
>   `S5_Depot_Wreck` -> `S5_Depot_Idle` (antes `TaskHandler_089398`),
>   `S5_Crate`, `S5_PropSolid`, fila de barriles, lluvia de rocas.
> * **Proyectiles**: `Proj_Thrown` (parametros +$9A..+$9D), `Proj_Drop_V0..V3`
>   -> `Proj_Drop_Common_08b3b4` (snd por cam x vs `$670`), `Proj_Bounce_08b9ba`
>   con tabla de punteros `Proj_Bounce_HitTable_08b944` + `Hitbox_08b950` (datos en `.text`,
>   emitidos como `.dc.w`).
> * 20 defsyms promovidos; +16 RTS mid-isla; +4 forward (`$8BA0C`, `$8BA52`,
>   `$8BB34`, `$8BB5E`, proxima wave). Callees pendientes: `$9A300`, `$38F14`,
>   `$997E2`, `$78908`, `$631D0`, `$8F3A6/$8F3BE/$8F69C`, `$8F002/$8F010`,
>   `$280C6`, `$5E3A2`, `$3093A`, `$5DD5C`.

> **Wave KKK** (84 entradas, 9 334 B, verde a la primera) — **cluster de la
> fortaleza de la escena 4**: cierra los 6 huecos de la region
> `$0865BE..$088A56` en `fort_scene4_0865xx.s`. Segunda wave producida con
> `tools/gen_asm_region.py` (79 entradas automaticas + 5 fronteras forzadas
> con `--entry` para separar las variantes `Fort_Init_V1..V4` y
> `Heli_InitTmpl`), analizada semanticamente a mano.
>
> * **Origen de los spawns**: todas las entradas con `movea.l +$3C(a6),a1;
>   jsr $2942A` estan en la **lista 4** (`$0972CC`) o la **lista 7**
>   (`$0975A2`) de `JumpTable_096B9C` (registros de 20 B
>   `{$0100, x, y, handler.l, 0, 0, FFFF x3}`), que hasta ahora figuraba
>   como "sub-dispatcher attract pendiente" (Wave HH#2) — en realidad es la
>   **tabla de listas de spawn por escena** (8 escenas: `$096BBC`,
>   `$096CAE`, `$096FA8`, `$09718A`, `$0972CC`, `$097422`, `$097500`,
>   `$0975A2`); las listas 0-3 apuntan a los clusters `$04D6EC..`,
>   `$052902..`, `$053F96..` y `$0834A4..` (Waves GGG/HHH), la 5 al hueco
>   `$088F74..$08A74E` (proxima wave). `Heli_InitTmpl_087b26` es ademas la
>   plantilla `$E8000[269]`.
> * **Fortaleza** (`Fort_*`): 5 variantes de init (+$20) que spawnean sus
>   casamatas y cajas (`Fort_SpawnChildren_V0..V4`, 270-330 B cada una,
>   desenrolladas) y una trampilla `Hatch_Init_V<n>`; HP `$140`, hitbox
>   `$2EC41E`, muerte con tres pares de escombros, `Fort_BlitWreck` y
>   `MissionWatch_Spawn_04429E` sobre la lista aux de +$7C.
> * **Casamatas** (`Pillbox_*`): +$21 = `$00/$30/$60/$01/$31/$61` (nibble
>   alto = fila de sprites `$2EB2C0`, bit0 = lado), HP `$28` con copia en
>   +$80 para `Entity_PropagateDamageToParent_08848c` (el dano a la casamata
>   se resta tambien a la fortaleza), 3 etapas de dano, colapso con
>   fragmento (`ori.b #$F` en +$21 del padre) o explosion con score `$100`.
> * **Trampilla** (`Hatch_*`): ciclo cerrada (90 f) -> abriendose -> spawn de
>   tropas (`MissionWatch_Spawn` +$7C, avanza con
>   `Entity_CmpPrioWithSibling_086552`) -> abierta (80 f) -> cerrandose;
>   apertura final con +$8C cuando muere la fortaleza.
> * **Muro de 13 cajas**, **blindado** (`ArmoredCar_*`, HP aleatorio
>   `$2C0628`+`$2E4`, torreta hija que copia el sprite del padre por +$20,
>   score `$5000`), **helicoptero** (`Heli_*`, HP aleatorio `$2C05A6`+`$17C`,
>   rotor `Heli_RotorAnim_0883ec` alternando +$16/+$18 cada 4/16 frames,
>   lanzador de soldados `$77228`/`$77F6A`), **escombro** con velocidad
>   aleatoria `$5EA1C`, **atrezzo** (3 destruibles, 1 cartel indestructible
>   con HP `$7FFF`, cuadruple) y **barricada** de 5 etapas.
> * Helpers: `Entity_HitboxPulseTable_088438` / `_Saved_08846a` (hitbox
>   activa 1 de cada 4 frames; antes `PcThunkTarget_*`),
>   `Entity_PropagateDamageToParent_08848c`.
> * Rarezas: `movea.l #-1,a0` pisado por `lea`, `addi.w #0,+$24(a0)` x4,
>   `jmp $518; rts` x7, `bra.w` a la instruccion siguiente, `jsr $434DC`
>   (rts puro), codigo muerto tras `bra.w` en `Barricade_Stage5`.
> * symbols.py: -4 defsyms forward promovidos, +1 RTS mid-isla
>   (`SetTaskWRts_088436`), +1 forward (`Sub_00088A64`).
> * Herramientas: `gen_asm_region.py` emite ahora refs forward
>   `Sub_XXXXXXXX` para targets pc-rel en huecos futuros;
>   `scripts/bootstrap_sandbox.sh` localiza el zip en `uploaded_files`/AI
>   Drive y lo persiste.

> **Wave JJJ** (65 entradas, 4 742 B, verde a la primera) — **entidades de
> la Mision 4 + helpers de spawn del boss de la Mision 3**: cierra los 33
> huecos de la region `$08512C..$0865BE` en
> `m4_carrier_boss_helpers_0851xx.s`. Primera wave producida con el nuevo
> `tools/gen_asm_region.py` (borrador byte-exacto 65/65 a la primera) y
> analizada semanticamente a mano encima.
>
> * **Templates de la M4** (`$E8000[35..40]` = `$859D4/$857F2/$858D0/
>   $8577A/$84F26/$85190`, `tmpl=$107..$10C` en `MissionStream_Slot03`):
>   el transporte `M4_Carrier_*` (snd `$1C5`, HP de `$2C029A`, fases
>   approach/fight con daño `$2870A`, muerte `$28758`, wreck con
>   `MissionWatch_Spawn_04429E` sobre la lista aux `$EC6C8`), su torreta
>   `M4_Turret_*` (bordes `$2D0/$310`, shake `+0x75`, knockback con flag de
>   direccion en bit0 de `+0x78`, muerte hasta `y<=$20`), los spawners de
>   soldados `$483E2`, la rueda doble (frames `$2E7BDE/$2E7BEE` por
>   `+0x88` mod 4), el rail de 20 segmentos (`$2EAA10[+0x21<<2]`), el agua
>   por jugador (caja `$2EB10E`, chapoteo `M4_SplashFx`), el escombro
>   (`$2E7B2E/$2E7B3E`) y el **suelo de camara** `M4_CamFloor_*` que
>   recorre la tabla aux de alturas `$EC882` y publica `$10816A/$10816E`
>   cuando ambos jugadores estan por debajo (`M4_PlayersBelowY`).
> * **`Flight_*`** (`$85EE8..$85FB0`): los 4 helpers pendientes de la
>   Wave III (bamboleo aleatorio 1/32, pasos `+-$100`, hitbox `$60/$C0`,
>   altitud entre `+0x98<<4` y `+0x99<<4`).
> * **`Boss_*`** (`$85FB0..$8656E`): los 20 helpers `jsr pc` pendientes
>   de las Waves GGG/HHH — spawner aleatorio 1/8 de 1..3 hijos `$3FEC6`,
>   jingles de fase (`$1026`/`$1032`), bob senoidal (`$13C0E`), spawners
>   por lista de registros de 8 B `{dx,dy,+0x38,+0x21,|=+0x3A}` en
>   `$2EAF1C`/`$2EAF7E`/`$2EAFE2` (los 7 primeros via `$6FE`, el resto via
>   `$4AE`), bucles de 45/4/8/9 hijos, `Boss_SpawnTenEscorts` (362 B,
>   desenrollado a mano), 6 cargadores de tabla de blit y
>   `Boss_Shadow_Init` (3 entradas, `$10E39E=1`).
> * Rarezas: `jmp $518; rts` con rts muerto (x4), `bra.w` doble en
>   `Boss_Shadow_Init`, `move.w d0,d0` muerto, `movea.l #-1,a0` pisado
>   por `lea` (x3). El label `.L83c0c-0xA` de `TaskHandler_083be2` se
>   promovio a `TaskHandler_083c02` (target `lea pc` de `$0860F6`).
> * symbols.py: -25 defsyms forward promovidos, +7 RTS mid-isla
>   (`SetHandlerRts_085482/_085acc`, `SetTaskWRts_085606/_0856a8`,
>   `JsrAbsRts_085d02/_08604e/_0863d4`).

> **Wave III** (22 entradas, 2 192 B, verde a la primera) — **escuadron de
> rescate y ciclo de vuelo**: cierra los 12 huecos de la region
> `$084836..$08512C` en `rescue_squad_0848xx.s`.
>
> * **Pasos del escuadron** (`$84836..$8492A`): snd `$A9`, sprites de la
>   tabla `$2E77CA[(+0x21<<1)<<2]` con centinela `$FFFFFFFF`, sincronia
>   con el padre y avance con retroceso `-$10` + `bset` bit6 del par
>   `$2E993C`.
> * **Handler del hijo del Wave HHH** (`$8495E..$84AAA`, `Sub_0008495E`
>   convertido de defsym a simbolo real): snd `$85`, timer aleatorio de
>   `$2C0218` (`$799DE`), montaje de pareja `$84AAA`/`$6AA14`, armadura
>   doble a partir de scroll `$4F0` (`$106F50` -> `$2E915A`) y muerte
>   con snd `$1028` y blitter `$2EACC4`.
> * **Secuencia de caida** (`$84B24..$84C50`): 4 etapas con sprites
>   `$2B5B92..$2B604A`, snd `$1040` al tocar suelo y liberacion del
>   abuelo (`+0x21=$FF` via doble `$C(a0)`).
> * **Paracaidista y rescatado** (`$84C5E..$84DB0`): snd `$4`, sprite
>   `$2E75C0` siguiendo al padre; premio `$2000` con helper futuro
>   `Sub_00086504`.
> * **Selector por tipo** (`$84DB0..$84F4E`): listas/pares/colas por
>   `+0x98` (0/1/2), bucle de spawn multiple segun `+0x99`, y arranque
>   del vuelo (snd `$1B1`, `+0x38=$2000`).
> * **Ciclo de vuelo** (`$84F5E..$8512C`): snd `$1074`/`$1075`, sprites
>   `$2E7828`/`$2E7866`/`$2E78A4`, helpers futuros `Sub_00085EE8`/
>   `Sub_00085F08`/`Sub_00085F44`/`Sub_00085F60`, aterrizaje cuando
>   scroll `$106F5C<=$108` spawneando `$8512C` via `$4AE` + `$5DD02`.
>
> Nuevos defsyms de RTS de islas: `SetHandlerRts_084898`/`_0848dc`/
> `_084b22`/`_084b98`/`_084bd0`/`_084c24` (+6) y `Jsr5B6Rts_084c5c`
> (+12). Se eliminaron 9 defsyms forward convertidos en simbolos reales
> (8 `TaskHandler_*` + `Sub_0008495E`) y se añadieron 7 forwards nuevos
> (`Sub_00085EE8..Sub_00086504`).

> **Wave HHH** (28 entradas, 3 110 B, verde a la primera) — **fases finales
> del miniboss y transiciones de oleada**: cierra los 5 huecos de la region
> `$083BE2..$084828` en `miniboss_finale_083bxx.s`, continuacion directa
> del modulo del Wave GGG.
>
> * **Entradas dobles con jingle** (`$83BE2..$83D6E`): snd `$166`/`$AA`
>   convergen en `$83C0C` (sprites `$2E6EFC`/`$2E6F12`, gate `x>$140`,
>   muerte con snd `$10A6` y par `$2E9A0C`); snd `$A6` + sprite `$2E6F28`
>   con transicion via snd `$102E` a `TaskHandler_083d2a`.
> * **Fase de disparo y premio** (`$83D6E..$83F26`): variantes segun
>   `+0x21` (sprites `$2E6F54`/`$2E6F6A`), snd `$1027`, helpers futuros
>   `Sub_000863E4`/`Sub_000863F2`; bucle de score `$2000` con el global
>   interno `TaskHandler_083e8c` reinstalado cruzando entradas.
> * **Maquina bit-scan** (`$83F26..$8414C`): gate `x<=$140`, blitter
>   `$2EACB8` + snd `$1030`, score `$500`, `bset` del indice en el padre
>   y escaneo de la tabla `$2EAA60[+0x21<<4]` (2x `StateMachineRun`,
>   helpers `Sub_000864B6`/`Sub_000864D0`) hasta `+0x21==$11`.
> * **Desenlace y explosion** (`$8414C..$84408`): cadena de sprites
>   `$2E6FE0..$2E7048`, score `$1000`, pares `$2E9B00`/`$2E9ACA`,
>   blitters `$2EACB8`/`$2EAE76`, epilogo con snd `$A9` y spawn del hijo
>   `$8495E` (hueco futuro) via `$4AE` + `$5DD22`.
> * **Transicion de oleada** (`$84410..$84504`): `$10E39C=3`, instala
>   `TaskHandler_0845b8` (referencia cruzada entre huecos de la misma
>   wave), snd `$1033`, spawn absoluto `$86586` (hueco futuro), pares
>   `$2E98D8`/`$2E98EA` y contador `+0x21>=9` -> `$10E39A=2`.
> * **Cierre y sincronizacion padre/hijo** (`$8450C..$84828`): guardian
>   con `+0x70=$100`/`+0x66=$7FFF` y probe de `$106F28`, parpadeo de
>   `$10A2D1`; tres etapas que esperan bit3 de `+0x13` del padre,
>   relanzamiento aleatorio con tabla `$2E7556[+0x21<<2]` y selector
>   `$2EB02C[rnd&$F<<2]`, y despawn final (`$FFFF` + par `$2E9ADC`).
>
> Nuevos defsyms de RTS de islas: `SetHandlerRts_08440e`/`_0844be`/
> `_08450a` (+6) y `Jsr5B6Rts_084834` (+12). Se eliminaron 3 defsyms
> forward (`TaskHandler_084410`/`_0844c0`/`_08450c`) al convertirse en
> simbolos reales, y se añadieron 17 forwards nuevos hacia `$8495E` y
> `$860E4..$86586`.

> **Wave GGG** (24 entradas, 2 374 B, verde a la primera) — **modulo de
> miniboss con secuencia de estados**: cierra los 6 huecos de la region
> `$083262..$083BDA` en `miniboss_module_0832xx.s`. Con esta wave el
> proyecto **cruza el 6% de la P ROM**.
>
> * **Helpers cortos** (`$83262..$832E0`): montaje de par
>   `$2E5ADC`/`$2E5A10` via `$77C7E` + snd `$102F`, carga de lista
>   `$2E545E[+0x5C]` en `+0x4C`, comparador de prioridad `+0x10` con el
>   sibling `+0x8` (cae en islas `ClearXN_0832e0`/`SetXN_0832e6`), y
>   `Sub_00083270` que setea `$10A2D0=4`/`$10A2D1=0` y `+0x20=$77` en el
>   padre. `LeaList_083262` es el prefijo del thunk `JsrAbsThunk_083268`.
> * **Tres variantes de entrada** (`$832EC..$834A4`): snd `$A4`, sprite
>   `$2E6CDC`, drift `+-$80` segun `x>=$A0`; una con empuje aleatorio
>   cada 4 frames (`$5E9B6`/`$5E9E4`) y otra usando el helper futuro
>   `Sub_00086050`.
> * **Fase de combate** (`$834A4..$83612`): snd `$A7`, sprite `$2E6C00`,
>   blitter de fila (`$43FAC` + lista `$2EAC8C`, snd `$1037`), muerte
>   hacia `TaskHandler_08354e` (score `$100` via `$51A28`) o
>   `TaskHandler_083596` (snd `$AB`).
> * **Secuencia de huida** (`$835F2..$83926`): 3 variantes (`+0x21`=0/1/2)
>   que convergen en `TaskCont_08364e` (global interno): snd `$AA`,
>   spawn de pareja `$2E9A1E`/`$2E9A30`, retroceso `-$3` con vel `+-$40`,
>   flash `$F0` (`$5E722`), snd `$1054` en t=`$50` y transformacion final
>   (`$2E987E`, 30 frames).
> * **Variantes protegidas y spawner** (`$8396A..$83BDA`): parpadeo
>   `+0x44` con timer aleatorio reinstalando `TaskHandler_0839a2` (global
>   interno), versiones acorazadas `$8000` y handler hijo `$83B92` que
>   copia x/y del padre y se autodestruye si `x<-$80`.
> * **symbols.py**: +2 RTS de islas (`Jsr5B6Rts_083b90` en +12 — la isla
>   `Jsr5B6ThenJmpScheduler_083b84` mide 14 B — y `JsrAbsRts_083be0`),
>   +4 forward refs a helpers futuros (`$85FB0`/`$86050`/`$86076`/`$863BE`).

> **Wave FFF** (36 entradas, 2 456 B, verde a la primera) — **helpers y
> handlers de escape del escuadron paracaidista**: cierra los 17 huecos
> de la region `$08283C..$08325A` en `para_squad_helpers_082cxx.s`,
> completando todo el modulo del escuadron iniciado en Waves EEE.
>
> * **Continuacion del escape** (`$8283C..$82924`): `TaskHandler_08283c`
>   retorna a la cola de frame del modulo EEE via `bra.w` cross-file —
>   se promovio el label local `.L827fe` de `para_squad_module_0818xx.s`
>   a global `ParaSquad_FrameTail_0827fe`. `TaskHandler_082884` spawnea
>   la pieza `$77E10`; `$828D2` marca `+0x48=-1` y encadena con
>   `jmp TaskHandler_056204`; `TaskHandler_0828ee` es el hijo (snd `$193`,
>   sprite `$2E6728`, hereda `+0x5E` del padre).
> * **Handlers de los hijos** (`$8292C..$82C74`): disparo de venganza
>   con jitter aleatorio (`$5E9B6`), respawn desde tabla `$2E588E` con
>   clamp de suelo (`$440D0`, `d0=$E4C/d1=$68`), salto balistico con
>   seno/coseno (`$2C07AC`/`$2C072C`, `muls #$2000, asr.l #8`), caida en
>   paracaidas (vel `$10/$C0`, sprite `$2E6AEA`) y fila aleatoria
>   (`$2E596A`/`$2E594A` con sentinel, HP `$800`, `jmp $6DCE0`).
>   `TaskHandler_082c54` (global interno) recorta `+0x45` del padre segun
>   `x>$120` y encola via `jmp FUN_00000518`.
> * **Helpers jsr-pc** (`$82C7C..$82F8A`): ring de historia de anclaje
>   `+0x7E` (mod 16), copias de anclaje del padre, relay de flags
>   bit6/7<->bit0(`+0x5A`), dificultad -> `+0x73=$FF` (tablas
>   `$2BE098`/`$2BE11A`), probes de bounds que caen en las islas C
>   `SetXN_*`/`ClearXN_*` ya matcheadas, maquina de flags `+0x72` y
>   punteria vertical `(altura-$24)>>3` con clamp 0..6.
> * **Spawners** (`$82F90..$8325A`): reacquire de target
>   (`$5E338`/`$5E0D4` con indireccion `+0x7C==$FF`), spawn de pieza +
>   escolta `TaskHandler_082720`, spawn de hijos `$8292C`+`$82A66`
>   (snd `$10FF`), bucle de 10 tropas `TaskHandler_082480`, 3 oleadas
>   `TaskHandler_082562` (`+0x5E`=0/1/2), flag global `$10A2D1`, montaje
>   de pares `$2E5A22`/`$2E597A` (`$77C7E`) y escolta `$77FD6` con jitter.
>   `LeaList_083254` (6 B) es el prefijo del thunk `JsrAbsThunk_08325a`.
> * **symbols.py**: los 23 defsyms forward de Wave EEE se eliminaron al
>   convertirse en simbolos reales; +6 `SetHandlerRts_*` nuevos.

> **Wave EEE** (23 entradas, 3 862 B, verde a la primera) — **modulo de
> escuadron con lider movil** (transporte + tropas): cierra los 23 huecos
> de la region `$081816..$082834` en `para_squad_module_0818xx.s`.
>
> * **`TaskHandler_081908`** (924 B, la entrada mas grande de la wave):
>   el lider — spawnea escolta + 2 piezas + 6 hijos en cadena (heredando
>   `+0x7C` como slot), avanza por tabla de sprites `$2E541E` con
>   sentinel `$FFFFFFFF`, y muere con jingle `$1071` congelando el input
>   (`$106ED3`) + timer `$B4` + pieza `$2E5AEE` a mitad de cuenta.
> * **Pieza con humo** (`TaskHandler_081db0`, 568 B): dispara de a 3 con
>   cadencia por bit-gate de frame (`$106F28 & 7`) y HP por dificultad
>   (`+0x73` elige tabla 2D `$2BE322`/`$2BE3A4`); al agotarse hace
>   `bset #5` en `+0x72` del padre.
> * **Pieza movil** (`TaskHandler_082052`, 936 B): dos entradas segun
>   lado (flip `+0x3a`), ciclo de oleadas con rafagas de a 5 spawneando
>   hijos `$828EE`, y despiece final soltando el escolta `$77FD6`.
> * **Tropas**: movimiento por tablas 2D anidadas `$2E54A2[fila][paso]`,
>   caida desde el techo, dispersion con venganza de angulo aleatorio
>   (`$5E9B6 & $F` → tripletas vel/angulo en `$2E560E`) y rebote.
> * **Escape final** (`TaskHandler_08267c`): mueve DOS escoltas `$77F22`
>   por tick con lista de deltas en tripletas; `TaskHandler_082720`
>   aplica knockback heredado del atacante (`+0x50`).
> * Matching: 11 defsyms `*Rts_*` nuevos (incl. `ClrRamWordRts_081caa`),
>   5 defsyms forward promovidos a simbolos reales, 23 refs forward a
>   los helpers pc-relativos del bloque `$82C7C..$831DA` (proxima wave).
>
> **Wave DDD** (29 entradas, 4 102 B, verde a la primera) — **los handlers
> de muerte y escape del modulo "Squad Deploy"**: cierra los 23 huecos de
> la region `$080736..$08180E` en `squad_death_handlers_0807xx.s`; junto
> con Wave CCC, toda la region `$07FBD2..$08180E` queda 100% decompilada.
>
> * **Tabla de despacho de muerte a 2 niveles** en `$2E3DC0` (8 punteros
>   a arrays de 8 handlers): segun arma y tipo elige entre voltereta con
>   rebote (`Squad_DeathTumble_080bd6`, curva `$2E22FA`), derrape con
>   freno (`Squad_DeathSkid_080cdc` / `Squad_DeathSkidBrake_080d92`),
>   arco parabolico (`Squad_DeathArcJump_080e48`), saltito hacia atras
>   (`Squad_DeathHopBack_080b1a`) o despiece en gibs
>   (`Squad_DeathPieces_080f32` / `TaskHandler_081018`).
> * **Esquirlas/gibs**: `Squad_ShardSprites_08134c` (742 B, la entrada
>   mas grande) + 4 plantillas `Template_0812xx` dan a cada fragmento
>   sprite, velocidad radial y gravedad; `Squad_GibGate_081332` corta el
>   ciclo cuando `Squad_BoundsCheck_080efa` detecta salida de pantalla.
> * **Escape del comandante**: grito `$171` + saltitos con la **curva de
>   campana `SquadCurve_BellTable_08175e`** (32 words `0100,00F0..0180..
>   0110` — tabla de datos embebida en codigo, registrada como entrada
>   propia) y estela de fragmentos (`Squad_SpeedTailFrags_0817d0`).
> * Rarezas de matching: `bgt.b` cross-section en `$8098E` hacia la isla
>   C `ClearXN_08097c` (GAS no puede emitirlo → `.dc.w 0x6eec` crudo);
>   el hueco `$8166E..$817CA` se partio en 4 entradas para exponer el
>   target interno `$8179E`; el label local `.L80704` de Wave CCC se
>   promovio a global `Squad_CmdrTick_080704` (target de `bra.w`
>   cross-file); 20 defsyms forward de waves previas pasaron a ser
>   simbolos reales y 9 `SetHandlerRts_*` nuevos se añadieron.
>
> **Wave CCC** (23 entradas, 2 714 B, verde a la primera) — **el modulo
> "Squad Deploy"**: comandante que despliega un escuadron de hasta 8
> soldados con gestion de slots por bitmask. Cierra los 23 huecos de la
> region `$07FBD2..$08072E` en `squad_deploy_module_07fbxx.s`; la cadena
> de handlers tejida a traves de las 26 islas `SetTaskHandler_*`/`SetC_*`/
> `ClearC_*`/`SetTaskW_*` ya matcheadas en C queda completa.
>
> * **`Squad_Mgr8Slots_0803e8`** (280 B): el corazon del modulo —
>   inicializa el array de recompensas `+0x80..+0x8E` a `$FFFF` y DOS
>   bitmasks de ocupacion en `+0x77`/`+0x78` ("alguna vez vivo" /
>   "vivo ahora"). Doble pasada de `btst`: primero busca un slot NUNCA
>   usado y luego cualquier slot muerto — respawn con memoria de bajas.
>   Cadencias y cupo desde tablas 2D `$2BEA3E`/`$2BEAC0`/`$2BEB42`
>   (decodificador `$799DE`).
> * **Hijos con 2 plantillas de init**: por tabla de offsets `$2E3EBC`
>   (pares de punteros de 8 B por fila, coords + timing en `+0x32/+0x33`)
>   o posicion fija desde `$2BEED0`; salto con fisica `$13C0E`
>   (vel `$400`, angulo `$88`), sombra `$723DA`, y al morir notifican al
>   padre (`Squad_NotifyParent_0803d2`: `addq.b #1,$20(a0)`).
> * **Sub-modulo "hatch"** (`$7FC1A`): 3 variantes de fila (sprites
>   `$2E1DCA`/`$2E1E5C`/`$2E1EEE`, offset lateral -$48/0/+$40 en
>   `+0x70`) que CAEN en un cuerpo comun; cuando el padre llega a
>   estado 3 monta las piezas `$2E2AE8`/`$2E2AFA` via `$77C7E`.
> * **Seguimiento con curvas**: `Squad_TrackArc`/`FollowLeader` usan las
>   tablas `$2E22FA`/`$2E2320` (indice = vel/4, clamps ±$90 con rebote
>   `$B800`/`$4800`) y `Squad_BobOscillate` la onda seno `$2C072C`
>   (fase `+0x74` mod $80, amplitud *4/256).
> * Idioms notables: `lea $ffff.w,a0` como ENTITY_NIL corto (4 B) donde
>   otros modulos gastan 6; `bpl.b` corto hacia atras en `Squad_AIDecide`;
>   los `bcc.w` colgantes de cada estado saltan al RTS interno (+6) de la
>   isla `SetTaskHandler` siguiente (13 defsyms `SetHandlerRts_*` nuevos).

> **Wave BBB** (27 entradas, 916 B, verde a la primera) — **las tablas
> de angulo de punteria + el backend de lectura de input**: cierra
> ENTERO el hueco `$05D316..$05D6AA` entre `ClearXN_05d310` y
> `JsrPcThunk_05d6aa` en `input_aim_tables_05d316.s`.
>
> * **19 `AimAngleTable_*`** (32 B c/u): las "arrays tables" reales que
>   consume `Ent_AimUpdate_045022`/`Ent_AimInit_045412` (Wave XX) —
>   entrada = `tabla[(estado & $F)*2]` = word de angulo objetivo
>   ($10000 = vuelta completa, `$FFFF` = sin cambio). Identificadas por
>   arma via las refs del codigo matcheado: `$5D326` pistola/default,
>   `$5D346` rocket (abanico 40/50/30), `$5D3A6`+`$5D4C6` HMG, y el
>   juego de 8+4 tablas del lanzallamas/shotgun (una por direccion:
>   20/40/60/80/A0/C0/E0 mas 30/50/B0/D0 y diagonales suaves D8/A8).
> * **`AimDirRows_05D316` + 3 `SpawnRows_*`**: filas de 4 bytes
>   terminadas en `$FF`; las SpawnRows son los destinos de la tabla de
>   punteros de 5 longs en `$02A5B8`.
> * **`InputLayout_ReadField2_05D5B6`/`ReadField0_05D616`**: versiones
>   completas del idioma InputMask de Wave U — eligen buffer de input
>   por player (`$6d(a6)`), cortocircuitan sin replay, y extraen del
>   layout el campo de bits 2-3 o 0-1. **BUG DE SNK hallado en
>   `$05D628`**: la rama P1 de ReadField0 hace `movea.l $72(a1),a2`
>   ANTES del `lea $100300,a1` (orden invertido respecto a sus 4
>   gemelas): lee el puntero con el `a1` viejo. Asi quedo compilado.
> * **`InputCtx_DemoOverride_05D674`**: por fin decompilado el "probe
>   real" pendiente desde Wave U (defsym forward `Sub_00005D674`,
>   llamado por los 3 `InputMask_ReadCtxSwitchPlayer_*`): si el modo
>   replay `$10E39D` esta armado redirige `a2` a los buffers de
>   grabacion `$10E2E2`/`$10E2E8`.
> * `DebugHex_SetupA1_05D6A0`: cabecera de 10 B que cae por
>   fall-through en el thunk `$05D6AA` -> volcado hex de debug.

> **Wave AAA** (17 entradas, 43 736 B, verde a la primera) — **LOS
> STREAMS DE BYTECODE DE LA MISSION VM**: el mayor salto de cobertura
> del proyecto (+65 % de los bytes matcheados en una sola wave, de
> 3.18 % a 5.27 % de la ROM). Es la region contigua `$0E8524..$0F2FFC`
> en un solo archivo `mission_streams_0e8524.s`: **todos los datos de
> eventos de mision del juego** — que enemigo aparece, donde y cuando,
> en las 6 misiones.
>
> * Son los 12 destinos de la tabla de punteros
>   `MissionStreamPtrs_044266` (Wave XX) mas el **stream de debug**
>   `$0F260E` (override cuando `$10FD83` esta armado, ver
>   `MissionDriver_Init_0442E6`). Slot 9 es un stream vacio de 4 B
>   (solo el op `$0C` de fin), igual que el nulo `$044262`.
> * El volcado NO es un blob: esta **estructurado registro a registro**
>   con un parser que replica los strides exactos de
>   `MissionVM_SkipOp_0446B6` (op `$00` spawn 18 B, `$01` spawner
>   periodico 10 B + hijo anidado, `$02`/`$03` bloques hasta marcador
>   `$0D`/`$0E`, `$04`/`$0A`/`$0B` esperas con 2-3 hijos, `$05..$09`
>   pausas de 4 B). Cada linea lleva comentario con direccion, opcode
>   decodificado (plantilla de enemigo, umbral de scroll, posicion XY,
>   flags) y sangria que refleja el anidamiento real. Los 13 streams se
>   recorren completos **sin un solo opcode invalido** y cada
>   terminador `0C 00 FF FF` cae exactamente donde empieza el
>   siguiente stream — la validacion mas fuerte posible de que el
>   formato deducido en Wave XX es correcto.
> * Cuatro streams (M1/M4/M5/M6) llevan **bloques aux** empaquetados
>   tras su terminador: tablas de words (waypoints/alturas terminadas
>   en `$FFFF`) referenciadas como inmediatos desde el codigo de los
>   enemigos de esa mision (78 referencias localizadas; los origenes
>   van comentados linea a linea, p. ej. `$04DB74 -> $0E92B2`).
> * Curiosidades: M2 tiene **doble terminador** `0C 00 FF FF` (bytes
>   muertos del compilador de niveles de SNK); el stream de debug es
>   casi identico al slot 12 y termina en `...0D 00 0C 00 FF FF` —
>   un marcador `$0D` huerfano delante del fin.

> **Wave ZZ** (17 entradas, 4 216 B, verde a la primera) — **el banner
> de mision "MISSION START" / "MISSION COMPLETE"**, buscado expresamente
> como wave de *funciones grandes*: incluye las **4 rutinas mas grandes
> decompiladas hasta ahora** (`BannerLayout_CompleteFinal_07B598` 968 B,
> `BannerLayout_StartFinal_07AF50` 830 B, `BannerLayout_Complete_07B28E`
> 778 B, `BannerLayout_Start_07ACD0` 640 B). Rellena LOS 9 huecos del
> cluster `$07A970..$07BA28` en un solo archivo
> `banner_mission_07a970.s`:
>
> * Las letras del rotulo **caen una a una desde arriba**: cada layout
>   es una secuencia lineal de spawns via el scheduler `$4AE` — una
>   llamada por letra con su glifo (+`$20`), fila (+`$21`), orden de
>   caida (+`$74`) y posicion final X/Y (+`$70`/+`$72`). Los glifos
>   indexan las tablas de sprites `$2DF684` (letras) / `$2DF71C`
>   (digitos); la escena `$106ECE==5` (mision final) usa las variantes
>   "Final" con la fila extra del numero de mision.
> * `BannerLetter_Drop_07AAE0`/`BannerDigit_Drop_07AB5C`: cuenta atras
>   por letra y caida con sonido `$1BE`; `BannerLetter_Seek_07B960`
>   dirige cada letra a su destino (delta<<6 como velocidad),
>   `_Approach` frena cuando la distancia (`$5E23A`) baja de `$30`, y
>   `_Blink` parpadea y suena `$1AA` al aterrizar (latch +`$7A`).
> * Los roots `BannerStart_Boot_07A970`/`BannerComplete_Boot_07A9F0`
>   esperan el contador de letras aterrizadas del contenedor (+`$21`)
>   y al cerrar (`Banner_Close_07AA94`) las letras salen volando:
>   `BannerLetter_FlyOff_07AC4A` rampa un contador a `$7FFF`, calcula
>   el angulo de escape hacia `($A0,$190)` via `$5E018` y lo convierte
>   a velocidad con `$13C0E`.
>
> Nuevos simbolos: 5 mid-isla (`SetHandlerRts_07a9ee/07aa76/07aa92/`
> `07abf2/07ac48`). Total: 1 059 simbolos.

> **Wave YY** (41 entradas, 2 576 B, verde a la primera) — **torreta con
> punteria de 5 direcciones + Boss2/Miniboss2 + despliegue de vehiculo +
> Enemy46**. Rellena los 6 primeros huecos tras el megabloque
> (`$04580C..$046258`). Dos archivos:
>
> 1. `turret_boss2_04580c.s` (26 entradas, 1 788 B) — la **torreta**
>    que dispara al ras del suelo con **5 inicializadores de direccion**
>    (`Turret_InitDir0..4`, cada uno apuntando a su tabla de angulos
>    `$2895x`) despachados por la jump-table de datos
>    `Turret_InitTable_045CD6` (5 punteros); nucleo de seguimiento y
>    suavizado de angulo (`Turret_Track`/`Turret_AimSmooth`, mismo idiom
>    de punteria que Wave XX); cola comun `Turret_Tail_045B9C` que
>    spawnea `Boss2Shot` en el pool `$100800` via el scheduler `$4AE`
>    (entrada global mid-funcion `Turret_SpawnShot_045BE6`). El bloque
>    **Boss2/Miniboss2** replica el patron boss de Wave XX: explosion,
>    caida, flags, muerte con despacho por tabla; el mini-jefe se monta
>    (`Attach`), cabalga (`Ride`) y muere con salto de impulso vertical
>    aleatorio via `$5DCA4` (`Hop` + etiqueta global interna
>    `Miniboss2_HopKill_045EEA`). Hitbox del proyectil en
>    `Boss2_HitboxTable_045DD4` (dato, 16 B, variante B en `$45DDC`).
>
> 2. `vehicle_deploy_045f2c.s` (15 entradas, 788 B) — **despliegue de
>    vehiculo**: tabla de datos `VehicleAnim_Table_045F3A` (dos guiones
>    de animacion de 80 B con cabeceras `0x03`/`0x04` y terminador
>    `FFFF FFFF`), lanzamiento con calculo de angulo via `$13C0E`
>    (`Vehicle_Launch`), vuelo balistico con bifurcacion a dos choques
>    (`Vehicle_Flight` → `CrashA`/`CrashB`), dos comparadores de
>    profundidad que retornan flags via los islotes `ori.b #$11,ccr;rts`
>    (`SetXN_04611e`/`SetXN_04613a`) y la maquina de estados del
>    **Enemy46** (`Boot`→`PhaseA`→`Move`→`PhaseB`→`Tail`), cuya cola
>    referencia dos rutinas de huecos futuros (`Fn_00046260`,
>    `Fn_000463C2`) resueltas por defsym — candidatas naturales para la
>    proxima wave.
>
> Nuevos simbolos: 5 mid-isla (`Jsr5B6Rts_045dd2`, `SetHandlerRts_045f2a`,
> `SetXN_04611e`, `SetXN_04613a`, `SetHandlerRts_04625e`) + 2 forward
> (`Fn_00046260`, `Fn_000463C2`). Total: 1 054 simbolos.

> **Wave XX** (66 entradas, 5 356 B, verde a la primera — NUEVO RECORD DE
> WAVE) — **LA VM DE EVENTOS DE MISION + spawner de enemigos + punteria
> del jugador**. Rellena LOS 25 huecos entre las islas C de
> `$04422A..$045806`, soldando el megabloque contiguo
> **`$040EF2..$045806` (~18.7 KB sin cortes)**. Tres archivos:
>
> 1. `mission_event_vm_04422a.s` (33 entradas, 1 536 B) — la VM de
>    eventos de mision: cada escena tiene un stream de bytecode (tabla
>    de 14 punteros `$044266`, indexada por `$106ECE`, con override de
>    debug a `$F260E` via `$10FD83`). `MissionDriver_Loop_044368`
>    compara cada frame el umbral del registro actual contra el progreso
>    del scroll `$106F5C` y despacha **13 opcodes** (`MissionVM_ExecOp`,
>    jump-table en `$0443E4`): spawn directo/periodico, pausa/reanuda,
>    esperas por posicion de players (Y, X, ambos ejes) y por conteo de
>    enemigos vivos `$106E88`. `MissionVM_SkipOp_0446B6` es el
>    **iterador paralelo** que salta registros sin ejecutarlos (segunda
>    jump-table de strides) — lo usan las esperas para descartar spawns
>    que el jugador dejo atras y el op `$03` para elegir 1 de N ramas
>    con `$5E9E4` (rand) apilando punteros. HALLAZGO: los guards
>    `nop;nop;cmpi;nop;trap #15` tras cada fetch son **asserts de
>    desarrollo de SNK** (op > `$0C` = stream corrupto) que quedaron
>    compilados en la ROM final.
> 2. `mission_spawn_boss_0448a6.s` (20 entradas, 1 684 B) —
>    `Spawn_FromStream_0448A6` materializa enemigos desde registros de
>    18 B (flags de gate por dificultad via `$2AC6A`/`$106ED1`, XY
>    relativo al scroll o absoluto, plantilla en la tabla `$E8000`,
>    8 B de parametros a `$98(a0)`) sobre el pool `$100800`, con alta en
>    el contador de vivos `$106E8A`; mas la **state-machine del jefe**
>    (intro→wait→engage→rearm→active→phase-fire→descend, con proyectil
>    `BossShot` de vel `$D000` + rand `$5DCA4` y timeout de 60 frames) y
>    la montura del **miniboss** (attach por bits de `$6B/$8C`, ride con
>    colision rect `$28C20` contra `Boss_HitboxTable_044D30`).
> 3. `ent_aim_input_044f8a.s` (13 entradas, 2 136 B) — **el nucleo de la
>    punteria**: `Ent_AimUpdate_045022` (**1 008 B, la funcion mas
>    grande de la wave**) resuelve el angulo objetivo segun el arma
>    `$106F2A` via las tablas de angulos `$5D326..$5D546` — HMG por
>    bit5, shotgun con 8 tablas segun altura/estado (`$5CEF8`), y para
>    el lanzallamas una **matriz de transicion de punteria diagonal**
>    (pares de bits `$30/$90/$60/$C0` de `$83(a6)` conmutan la tabla
>    activa `$84(a6)` para suavizar el giro 8-direccional). El angulo
>    `$7C(a6)` se integra con **easing con friccion** (paso = delta>>3 −
>    fric>>5 acumulado en `$7E`, modo snap `$800`, sentido por smi/not).
>    `Ent_InputSample_04546E` muestrea el pad con **mascaras por arma**
>    (`$30`/`$F0`/`$10`) desde el contexto del player o el neutral
>    `$5CC08` (attract), `Ent_FireGate_045648` lleva la cadencia
>    (cooldown 24/6/2 frames segun arma y latch), y
>    `Ent_GroundProbe_04572C` sonda el mapa de colision (`$280C6`) con
>    offsets por pose (`$29D752`). 7 defsyms mid-island nuevos (rts
>    internos de islas C) + 19 externals nuevos en `symbols.py`.

> **Wave WW** (7 funciones, 1 624 B, verde a la primera) — **LA VM DEL
> SCROLL**: `SceneScriptVM_Frame_0437DA` (1 288 B, una de las funciones
> individuales mas grandes del proyecto), el interprete de bytecode que
> ejecuta cada frame el script de la escena activa (PC en `$10815C`,
> cargado por `SceneLoader_Main` desde la tabla `$916C8`): **23 opcodes**
> despachados por jump-table pc-relativa `jmp $4381C(pc,d0.w)` de `bra.w`,
> donde el **op `$16` DESBORDA la tabla** aterrizando directamente en su
> handler inline (`$043874`) — patron nuevo. El op `$02` cede el frame:
> integra scroll saturado via `Scroll_ClampToRange_043E3A` (deteccion de
> cruce de limite por XOR de signos + islas `ClrWD0`), acopla la
> velocidad Y al delta X en tramos diagonales (`slope*(maxX-posX)>>8`),
> actualiza el high-water de progreso segun el modo de eje `$108178` y
> sale por `bra.w` a `CameraApplyAll4_043D86`. Completa la ola: el
> chequeo CCR de llegada al borde (`$043CE2`, cae en islas Clear/SetC),
> los 2 helpers del **mapa de colision de 4 KB** en `$106F6C+$7C`
> (indice de 12 bits con celdas 8x8 en bloques de 2 columnas — el MISMO
> buffer que limpia `Buffer_ClearBlock1024L` y resetea el op `$0B`), la
> pareja de presets de velocidad del smoothing en **C puro GCC-derivado**
> y 3 bumps de `rts` finales ya emitidos pero registrados cortos. El
> megabloque contiguo llega a **`$040EF2..$04422A` (~13.1 KB)**.

> **Wave VV** (41 funciones, 4 482 B, verde a la primera en sus 3 partes) —
> NUEVO RECORD de oleada y cierre del megabloque contiguo
> **`$040EF2..$0434C2` (~9.6 KB)**: extiende el bloque escuadron de TT/UU
> con `$0422EA..$0434C2`, 11 fragmentos tejidos entre 13 islas C ya
> matcheadas. Parte 1: 4 spawners de hijos + la familia **MeleeGuard**
> (handler doble A/B con NOP de alineacion, 5 estados, IA con throttle
> 1/32). Parte 2: el **Charger** (init con dispatcher tejido a traves de
> 4 islas SetTaskHandler consecutivas — patron NUEVO —, 3 marchas, ataque
> de 5 golpes con **continuaciones diferidas en `+0x78`** como inmediato
> de 32 bits, windups A/B espejo) + el **Skirmisher** minimalista. Parte 3:
> set de animaciones de **16 rumbos** (1 332 B de datos transcritos y
> verificados por script, con terminador HOLD `$1600` nuevo) + handler
> dirigido por RAM global + clones tri-estado con rama invertida (`bcs`).
> Forense: TRES `bra.w` muertos de plantilla, `moveq` muerto en `$042C86`,
> 3 simbolos nuevos de **rts a mitad de isla**, el unico `bsr.b` de la
> wave (fuerza fusion de secciones) y `jsr X(pc)` vs `bsr.w` a la misma
> distancia. Ver "Wave VV en detalle".

> **Wave UU** (31 funciones, 3 246 B, verde a la primera) — anterior RECORD
> de oleada: cierra por completo el hueco `$040F00..$041C12` entre el
> handler de muerte `Jsr5B6ThenJmpScheduler_040ef2` y el subsistema
> escuadron de Wave TT. Parte 1: la maquina de estados de los 8 MIEMBROS
> del escuadron (handler `$40F00` instalado por `Squad_SpawnEight`) —
> punteria con atan2 y giro por pasos, animacion ciclica temporizada por
> templates medidos, protocolo lider<->miembro completo en accion (poll /
> write-back / tag con latch anti-rebote `+0x86`), retroceso por impacto
> con i-frames. Parte 2: los hijos derivados — par escoltado (con la
> funcion mas grande del wave, `PairChild_HandlerB` de 308 B), trio con
> vuelo ORBITAL y seguimiento de objetivo, caidas (zigzag por tabla de
> registros de 6 bytes, drop con jitter RNG) y las secuencias de muerte.
> **11 usos del idioma "branch a mitad de isla"** (`SetHandlerRts_*` /
> `JsrAbsRts_*`), un `bcs.w` DIRECTO a la funcion C ya matcheada `$40EF2`
> y un **DEAD STORE forense** en `$041A96` (`movea.l #-1,a0` pisado por
> `lea $28610A.l,a0`: plantilla editada a mano por Nazca). Promueve los
> 6 handlers pc-rel nombrados en Wave TT a codigo real. Ver "Wave UU en
> detalle".

> **Wave TT** (32 funciones, 1 634 B, verde a la primera) — el cluster
> contiguo mas grande decompilado hasta ahora: el subsistema "escuadron"
> COMPLETO (`$041C1A..$0422E4`, formacion de 8 entities con vuelo
> ondulatorio senoidal — los enjambres voladores del juego). Incluye
> calculo de target de formacion (tabla `$286124`), guiado con atan2 de
> tabla (`$5E018`) y giro limitado por turn-rate, trio NO factorizado de
> "bobs" senoidales (tabla seno `$2C072C`), despachador de estados con
> jump table `$28633C`, cadena de seleccion de templates de animacion
> con variante espejada (`+0x7C == $FF`), protocolo de estado compartido
> lider<->miembros (poll + writeback tipo CAS cooperativo sobre el array
> `+0x80` de la estructura compartida `+0xC`), spawner de los 8 miembros
> (tabla `$28615C`, handler `$40F00`), spawners de par escoltado (clon
> par #11) y de trio con patron ciclico, conversor sincos heading->
> velocidad (`$2C072C/$2C07AC`) y moduladores de brillo por onda
> triangular. **Dos nuevos ASSERT `trap #15` nop-patched** (`$041D74`:
> `ASSERT(paso != 0)`; `$041E56`: `ASSERT(estado < 6)`), confirmando la
> macro de Nazca descubierta en Wave SS. Promueve 5 placeholders y
> nombra 6 handlers pc-rel nuevos. Ver "Wave TT en detalle".

> **Wave SS** (12 entradas de registro, 1 384 B brutos / +1 366 B netos) —
> segunda oleada priorizada por TAMANO con `tools/rank_candidates.py`.
> Nucleo del sistema de colision hitbox-vs-hitbox
> (`Entity_HitboxCollide_028A96` 114 B + `Hitbox_OverlapTestXY_028B14`
> 268 B, con **assertions `trap #15` nop-patched** — primera evidencia
> directa de la macro de assert del build de desarrollo de Nazca);
> cierre del bloque boot `$0009B4..$000B8A` (`ScriptSlotPairTable_0009B4`
> 200 B datos + `TaskSlots_BootInstall_000A7C` 270 B, instalador de los
> 12 TCBs estaticos alcanzado via la tabla (TCB,handler) de `$178000`);
> el "callee $1CD4" de MM#3 (`TaskList_ChangeAndRunEight_001CD4` 96 B);
> sincronizador de sub-entities (`Entity_CopyAnimFromLeader_06E2BC`
> 66 B); y el cluster de glifos 16x16 del Fix Layer (`$099F3A..$09A0B3`,
> 6 funciones, 370 B). Absorbe 3 FPs (#49–#51) y promueve 8 placeholders
> de `symbols.py`. Ver la seccion "Wave SS en detalle" y `CHANGELOG.md`.

> **Wave RR** (5 funciones, 516 B) — primera oleada seleccionada
> puramente por TAMANO via `tools/rank_candidates.py` en vez de
> popularidad de callers. Par de rutinas de deteccion de solape de caja
> rectangular con espejado por facing (`Entity_CheckActiveBoxOverlap_072C98`
> 144 B, `Entity_CheckBoxOverlapWithSelector_0798AC` 164 B, ambas escriben
> en `target->+0x8E`); relink+wrap del scroll de `camera[0]`
> (`Camera0_RelinkAndWrapScroll_06896A` 112 B, subsistema JJ ya conocido);
> y un constructor de grupo de sub-entities enlazadas desde lista de
> templates de 12 B/nodo (`EntityGroup_SpawnLinkedFromTemplateList_065C94`
> 84 B + helper `Entity_MirrorDeltaByFacing_065D32` 12 B). Promueve 4
> `PcThunkTarget_*` de `tools/symbols.py` (`072c98`, `0798ac`, `06896a`,
> `065c94`) a definicion canonica; actualiza los 8 thunks correspondientes
> en `src/jsr_pc_thunks.c`. Ver `CHANGELOG.md` para el resumen en ingles y
> los `.s` individuales para el analisis campo-a-campo completo.

> Toolchain reproducido con **m68k-linux-gnu-gcc 14.2.0** (Debian trixie) sin
> regresiones frente al codegen documentado con 13.3.0: las 2 980 funciones
> heredadas de las oleadas A–U reensamblan byte-a-byte con ambas versiones.

## Metodología

Modelo dual C + ASM, análogo a `n64decomp/sm64` y `zeldaret/oot`:

- `src/*.c` — funciones cuya salida de GCC bare-metal coincide byte-a-byte
  con la ROM. Cubre casi todo lo mecánico: stubs `rts`, thunks
  `jsr abs.l;rts`, wrappers de scheduler, dispatchers de estado.
- `asm/*.s` — funciones semánticas del juego cuya secuencia exacta no es
  rederivable con GCC (convenciones de paso por registros absolutos,
  retorno por CCR, epílogos compartidos entre funciones). Cada `.s`
  documenta la firma C conceptual y las evidencias forenses encontradas.
- `asm/non_matchings/` — dumps literales por dirección para funciones aún
  no analizadas, integradas al build para que la ROM final coincida
  bit-a-bit mientras se decompilan. No cuentan como decompiladas.

Ninguna oleada se cierra hasta que `python3 tools/match_batch.py` da verde.

## Waves

| Wave | Familia | Funciones | Bytes | Formato |
|---|---|---:|---:|---|
| HH | Scene loader + attract sub-helpers + FixLayer batch | 12 | 1 036 | ASM |
| A | Semánticas base | 5 | 122 | C |
| B | Stubs `rts` triviales | 37 | 74 | C |
| C | Return constante | 4 | 16 | C |
| D | SetTaskW | 44 | 264 | C |
| E | SetTaskB | 16 | 96 | C |
| F | CCR helpers | 1 029 | 6 174 | C |
| G | StateDispatchStub | 270 | 3 780 | C |
| H | SetTaskHandler | 731 | 5 848 | C |
| I | JsrAbsThunk | 422 | 3 376 | C |
| J | JsrPcThunk | 164 | 984 | C |
| K | JmpAbsThunk (tail-call) | 15 | 90 | C |
| L | JmpToScheduler | 48 | 384 | C |
| M | Jsr5B6ThenJmpScheduler | 87 | 1 218 | C |
| N | ClrRamByte + LeaA1Plus4 | 26 | 188 | C |
| O | GlobalFlag + JmpScheduler | 6 | 88 | C |
| P | BIOS entry points + arranque | 9 | 210 | C |
| Q | IRQ handlers | 5 | 162 | C |
| R | Scheduler central | 7 | 318 | C |
| **S** | **Entity/Sprite helpers (asm 68000)** | **4** | **164** | **ASM** |
| **T** | **Probe/revert cluster + task/draw/script helpers (asm 68000)** | **16** | **868** | **ASM** |
| **U** | **InputMask event dispatchers cluster ($5CDFC..$5D1D9)** | **53** | **990** | **ASM** |
| **V** | **Entity/Sprite helpers priorizados por callers (asm 68000)** | **9** | **482** | **ASM** |
| **W** | **Sprite slot allocator + hex formatters + Fix Layer blit + entity allocator (asm 68000)** | **16** | **2 496** | **ASM** |
| **X** | **Pipeline decimal display + entity finder + Wave I FP absorbido (asm 68000)** | **5** | **304** | **ASM** |
| **Y** | **Scheduler central + arranque post-BIOS + colas + constructores de entities (asm 68000)** | **11** | **750** | **ASM** |
| **Z (batch 1)** | **Sprite dispatch dual + probe cluster $027xxx + LFSR RNG + helpers varios (asm 68000)** | **12** | **804** | **ASM** |
| **Z (batch 2)** | **Probes 2-intentos + dispatchers por-jugador + BCD add clamp + clones list-apply + handlers zona baja (asm 68000)** | **14** | **752** | **ASM** |
| **AA (batch 1)** | **Cluster dispatcher estado por-jugador `$051914..$051AA3` (asm 68000)** | **4** | **294** | **ASM** |
| **AA (batch 2)** | **Cluster debug HUD hex-counter `$047482..$047675` (asm 68000)** | **5** | **500** | **ASM** |
| **BB (batch 1)** | **Procesador START del sistema Neo Geo `$024E38..$024FB5` (asm 68000 + tabla)** | **4** | **374** | **ASM** |
| **BB (batch 2)** | **State publishers per-entity `$057044..$057225` (asm 68000)** | **11** | **272** | **ASM** |
| **CC (batch 1)** | **Cluster coord/camera $043EDA..$0440E3 + LFSR self-seed + attract-init (asm 68000)** | **14** | **380** | **ASM** |
| **CC (batch 2)** | **Handlers gemelos apply-camera-with-clipping + blitters MMIO batch (asm 68000)** | **4** | **622** | **ASM** |
| **DD** | **Helpers heterogéneos alta prioridad: Clipping_Test + Input snapshot + VRAM autoclear + VBlank tick + Task freelist init (asm 68000)** | **5** | **670** | **ASM** |
| **EE (batch 1)** | **Cluster dispatcher-tabla de estados attract/title `$001260..$001AB5` (asm 68000)** | **3** | **622** | **ASM** |
| **FF (batch 1)** | **Cluster attract handlers restantes `$001744..$001AF7` (asm 68000)** | **9** | **472** | **ASM** |
| **FF (batch 2)** | **Helper geométrico Geom_Proj_Clamp `$0436DE` (asm 68000)** | **1** | **252** | **ASM** |
| **GG (batch 1)** | **Cluster attract state handlers `$096xxx` (asm 68000)** | **7** | **452** | **ASM** |
| **GG (batch 2)** | **Máquina de estados de animación `$08Cxxx` (asm 68000)** | **6** | **688** | **ASM** |
| **HH (batch 1)** | **Scene loader + camera smoothing `$043xxx` (asm 68000)** | **4** | **504** | **ASM** |
| **HH (batch 2)** | **Sub-helpers attract `$096xxx` (asm 68000 + tabla)** | **6** | **404** | **ASM** |
| **HH (batch 3)** | **FixLayer_QuadBatch + PlayerCtx_Reset (asm 68000)** | **2** | **128** | **ASM** |
| **II (batch 1)** | **Fix Layer backends `$05DBxx` + slot helper (asm 68000)** | **4** | **176** | **ASM** |
| **II (batch 2)** | **Callees camera/list/ctx pendientes (asm 68000)** | **6** | **236** | **ASM** |
| **JJ (batch 1)** | **Cluster aplicación de cámara `$043DAA` (asm 68000)** | **4** | **144** | **ASM** |
| **JJ (batch 2)** | **Asignador de sprites hardware `$0139xx` (asm 68000)** | **6** | **240** | **ASM** |
| **KK (batch 1)** | **Callees pendientes camara/sprites (asm 68000)** | **3** | **218** | **ASM** |
| **KK (batch 2)** | **Probes de colision + handler MMIO `$051Cxx` (asm 68000)** | **4** | **524** | **ASM** |
| **LL (batch 1)** | **Callees por-celda del cluster colision `$051Bxx`/`$051Dxx` + pubcleaner `$052712` (asm 68000)** | **3** | **282** | **ASM** |
| **MM (batch 1)** | **Scheduler bootstrap + bytecode virtual continuation-passing `$000Exxx-$0010xx` (asm 68000)** | **5** | **486** | **ASM** |
| **MM (batch 2)** | **Super-tabla dispatch `$000B92` (datos-en-.text, 191 u32 BE)** | **1** | **764** | **ASM** |
| **MM (batch 3)** | **Handlers scheduler `$00109C..$00125E` (8 handlers attract phase1+phase2)** | **8** | **438** | **ASM** |
| **NN–PP** | **(sin detalle registrado en esta tabla — ver `git log`/`CHANGELOG.md`)** | **6** | **756** | **ASM** |
| **QQ** | **PlayerEntity_InitAuxState (#1) + Entity_ApplyFadeShade compartido (#2)** | **2** | **202** | **ASM** |
| **RR** | **Box-overlap x2 (facing-mirrored) + Camera0 relink/wrap-scroll + EntityGroup spawn-linked-from-template-list + mirror helper — 1a oleada priorizada por TAMANO (`tools/rank_candidates.py`)** | **5** | **516** | **ASM** |
| **SS** | **Colision hitbox (caller + test AABB con asserts trap#15) + boot block $0009B4..$000B8A (tabla + instalador TCBs) + re-arme 8 TCBs + sync sub-entity + cluster glifos 16x16 Fix Layer — 2a oleada por TAMANO** | **12** | **1 384** | **ASM** |
| **TT** | **Subsistema "escuadron" completo $041C1A..$0422E4: formacion 8 entities con vuelo senoidal (target de formacion + guiado atan2 + trio de bobs + despachador de estados + templates de animacion espejables + protocolo estado compartido poll/writeback + spawners de 8/par/trio + sincos + shade) — cluster contiguo mas grande decompilado, 2 asserts trap#15 nuevos** | **32** | **1 634** | **ASM** |
| **UU** | **Maquina de estados de miembros e hijos del escuadron $040F00..$041C12: 13 estados de miembro (punteria atan2, anim ciclica, protocolo lider/miembro con latch, hit-recoil) + 18 handlers de hijos (par escoltado, trio orbital con tracker, caidas zigzag/drop, 6 secuencias de muerte) — cierra el hueco completo hasta Wave TT; 11 branch-a-mitad-de-isla, dead store forense $041A96** | **31** | **3 246** | **ASM** |
| **VV** | **MeleeGuard + Charger + Skirmisher + Heading16 $0422EA..$0434C2: 4 spawners, guardia melee (handler doble A/B, 5 estados), Charger (dispatcher tejido por 4 islas SetTaskHandler, 5 golpes, continuaciones diferidas +0x78, windups espejo), Skirmisher, set de anims de 16 rumbos (1 332 B datos verificados por script) — cierra el megabloque $040EF2..$0434C2 (~9.6 KB); 3 rts-a-mitad-de-isla nuevos, 3 bra.w muertos, moveq muerto, unico bsr.b** | **41** | **4 482** | **ASM** |
| **TOTAL** | | **3 247** | **52 910** |  |

### Wave VV en detalle — MeleeGuard, Charger, Skirmisher y Heading16

Wave VV cierra el megabloque de gameplay contiguo mas grande del
proyecto: con `$0422EA..$0434C2` matcheado, **todo `$040EF2..$0434C2`
(~9.6 KB) es continuo** entre codigo C ya matcheado y las tres oleadas
ASM TT/UU/VV. Eran 11 fragmentos entre 13 islas C (SetTaskW/
SetTaskHandler/JsrAbsThunk/CCR helpers).

**Parte 1 (`melee_guard_cluster_0422xx.s`, 24 fn, 1 516 B):** cuatro
spawners que crean hijos con los handlers de Wave UU (DropPair con
offset `indice*32-64`, FxChild con handler externo `$78042`,
GlideAttacker con sfx `$109B`, FinalPoseChild que hereda velocidad);
los helpers CCR `Entity_CmpDepthToParent_042390` y el par
`TargetFacingTest` (con DOS `bra.w Stub_000423EA` muertos de plantilla
en `$0423BE` y `$0423E0`); y la familia **MeleeGuard**: tracker
tri-estado con gate, timers de engage/cooldown/vida que TERMINAN
cayendo o saltando a mitad de la isla `SetTaskW_042476`, puerta de
ataque con 3 salidas a `SetHandlerRts_0424a8`, IA con throttle 1/32
sobre `$106F28`, handler doble A/B (NOP de alineacion en `$0425DE`,
stats re-escalados con tablas `$2B74EC`/`$2B746A` y `moveq #-1` como
sentinela `$FFFF`) y 5 estados con cola comun `TailDespawn`. Los
estados Approach/Retreat usan `bsr.w TailDespawn + rts` donde el resto
usa `bra.w`: dos plantillas de cola conviviendo.

**Parte 2 (`charger_states_0429xx.s`, 12 fn, 1 374 B):** el **Charger**.
Su Init (240 B) estrena el patron mas llamativo de la wave: el
dispatcher de marcha esta TEJIDO a traves de cuatro islas
`SetTaskHandler` consecutivas (`$429FC/$42A0C/$42A1A/$42A22`) — cada
rama del if-else cae en la isla que instala su estado. El unico
`bsr.b` de la wave (`61 82`, `$042AA6`) obliga a fusionar el helper
`ColumnDeltaTest` con las 3 entradas de marcha en UNA seccion. El
ataque (292 B) despacha 5 golpes por `+0x9c` y usa **continuaciones
diferidas**: guarda `.Lat_anim` en `+0x78` como inmediato de 32 bits
(reloc a label local) o `$00059042` (hex) y desvia a los windups A/B,
que saltan a `+0x78` al terminar (A calma `+0x76`, B enfada; A llama
al tracker con `bsr.w`, B con `jsr X(pc)` — 4EBA — a la MISMA
distancia). Restos muertos: `bra.w .Lat_anim` en `$042BCC` y
`moveq #0,d0` en `$042C86`. El **Skirmisher** replica el tracker
tri-estado INLINE (sin helper) sobre `+0x70`.

**Parte 3 (`heading16_cluster_042exx.s`, 5 entradas, 1 592 B):** set de
animaciones de **16 rumbos** (22.5 grados): 1 332 B de datos SIN header
`$0900` (frames puros `{dur,attr,tile,$FFFF}`, diagonales con attr
`$0209` = flip), cada tabla loopeando a la IDLE comun `$43354`, que
termina en el opcode **`$1600` (HOLD)** — primer avistamiento de este
terminador. La transcripcion se genero y verifico POR SCRIPT contra la
ROM (assert por frame y por puntero). El handler (`$0433BE`) es un
sprite dirigido por RAM global (`$106F29` control / `$106F28` rumbo)
con despawn al salir por la derecha. Cierran la wave dos clones
tri-estado con la primera rama INVERTIDA (`bcs.w` en vez de `bcc.w`)
y el clon del comparador de profundidad con su pareja de islas CCR.

**Hallazgos arquitectonicos Wave VV:**

- **Rts a mitad de isla**: 3 simbolos nuevos (`SetTaskWRts_04247a`,
  `SetHandlerRts_0424a8`, `JsrAbsRts_04290a`) — el codigo salta al
  `rts` FINAL de islas C ya matcheadas, no a su entrada.
- **Dispatcher tejido por islas**: patron nuevo donde un if-else de 4
  ramas se implementa cayendo en 4 islas `SetTaskHandler` consecutivas.
- **Continuacion diferida en `+0x78`**: direccion de codigo como
  inmediato de 32 bits; dos plantillas (reloc local vs hex externa).
- **Terminador `$1600` (HOLD)** en tablas de animacion: congela el
  ultimo frame sin loop.
- **3 `bra.w` muertos + 1 `moveq` muerto**: plantillas editadas a mano.
- `clr.w d1` (Charger_Init) vs `moveq #0,d1` (MeleeGuard_Handler) para
  el MISMO calculo: codegen distinto en plantillas gemelas.

### Wave UU en detalle — miembros e hijos del escuadron

Wave UU completa el mapa del subsistema escuadron cerrando el cluster
`$040F00..$041C12` (31 funciones, 3 246 B, dos archivos) que quedaba
entre `Jsr5B6ThenJmpScheduler_040ef2` (el handler de muerte, C) y el
arranque de Wave TT en `$041C1A`. Con esto, **todo `$040EF2..$0422E4`
(5,1 KiB contiguos) esta decompilado**. Verde a la primera pasada.

**Parte 1 — `asm/squad_member_states_040fxx.s`** (13 fn, 1 268 B): la
maquina de estados de los 8 miembros creados por `Squad_SpawnEight`:

| Funcion | Dir | B | Rol |
|---|---|---|---|
| `SquadMember_Handler_040F00` | $040F00 | 130 | handler inicial (sonido por bit-id, init, estado $80) |
| `SquadMember_OnStateChange_040F82` | $040F82 | 108 | transicion por orden nueva; 2a entrada `RunStateB_040FB0` |
| `SquadMember_AnimCycleIdle_040FEE` | $040FEE | 204 | anim ciclica temporizada por 2 pares de templates medidos |
| `SquadMember_SetPose2_0410BA` | $0410BA | 10 | prologo: pose 2 y cae en AimTrack |
| `SquadMember_HoldPose_0410C4` | $0410C4 | 42 | espera tras publicar orden $83 |
| `SquadMember_AimTrackTarget_0410EE` | $0410EE | 168 | punteria: atan2 `$5E018` + giro por pasos + flag `+0x7D` del lider |
| `SquadMember_AlignHeading_041196` | $041196 | 98 | giro a pose 2 a media velocidad (toggle de frame) |
| `SquadMember_PoseFromHeading_0411F8` | $0411F8 | 134 | pose por heading (tabla `$28618C`), publica $83 |
| `SquadMember_AckAndHold_04127E` | $04127E | 62 | ack incondicional + orden $83 |
| `SquadMember_HitRecoil_0412BC` | $0412BC | 64 | retroceso por impacto, resetea orden a 0 |
| `SquadMember_FrameTail_0412FC` | $0412FC | 36 | cola comun (dano -> i-frames + HitRecoil); 2a entrada `FrameTailFull_041316` |
| `SquadMember_AlignHeadingB_041328` | $041328 | 98 | clon B (pose 8); 2a entrada `KeepPose_04132E` |
| `SquadMember_PoseFromHeadingB_041390` | $041390 | 114 | clon B (tabla `$2861A4`) — **PAR DE CLONES #12** |

**Parte 2 — `asm/squad_children_handlers_0414xx.s`** (18 fn, 1 970 B):

| Funcion | Dir | B | Rol |
|---|---|---|---|
| `SquadChild_SwoopPhysics_041408` | $041408 | 122 | picado con drag recortable |
| `SquadChild_FlipTouchdown_04148A` | $04148A | 200 | aterrizaje: 2 efectos espejados via eori del flip, rebote, dano |
| `SquadChild_TouchdownIdle_04155A` | $04155A | 28 | espera en suelo hasta cambio de senal `+0x21` |
| `SquadChild_DieToScheduler_041586` | $041586 | 28 | muerte via cola `$77FD6` |
| `SquadChild_DespawnNoLink_0415A2` | $0415A2 | 36 | despawn silencioso (`ori.w #0` MUERTO) |
| `PairChild_HandlerA_0415C6` | $0415C6 | 88 | hijo A del par; 2a entrada `InstallRun_0415F2` compartida |
| `PairChild_HandlerB_041626` | $041626 | 308 | **la mayor del wave**: template espejado por el `+0x7C` DEL LIDER, guiado dual `$27BC8`/`$27CEE` |
| `PairChild_DeathCry_041762` | $041762 | 74 | grito $1022; 2a entrada `DeathPlain_041788` |
| `SquadChild_DropSpawnAtTop_0417AC` | $0417AC | 104 | spawner con jitter RNG; **`bcs.w` directo a `$40EF2`** |
| `SquadChild_DropRun_04181C` | $04181C | 44 | per-frame de la caida |
| `SquadChild_ZigzagFall_041850` | $041850 | 194 | zigzag por registros de 6 B (`idx*6` = `2n+n` doblado); test de limites condicionado por contador |
| `SquadChild_GlideAttack_04191A` | $04191A | 156 | planeo con vida medida sobre `$2BBFF6` |
| `SquadChild_GlideDeath_0419BE` | $0419BE | 14 | grito + despawn |
| `TrioChild_HandlerA_0419CC` | $0419CC | 48 | comparte TODO el bucle con PairChild A (via `bra`) |
| `TrioChild_HandlerB_0419FC` | $0419FC | 176 | vuelo orbital `$78F8A` + shade de Wave TT; 2a entrada `RunTail_041A7A` |
| `TrioChild_OrbitTracker_041AB4` | $041AB4 | 260 | seguimiento orbital con timeout $B4 y periodo por tabla `$2BC302` |
| `TrioChild_OrbitDeath_041BB8` | $041BB8 | 14 | clon de GlideDeath |
| `SquadChild_FinalPose_041BC6` | $041BC6 | 76 | pose final antes del despawn |

**Hallazgos arquitectonicos Wave UU:**

1. **DEAD STORE forense en `$041A96`** (`TrioChildB_RunTail`): `movea.l
   #-1,a0` inmediatamente pisado por `lea $28610A.l,a0`. Todos los demas
   release `$5DD56` del cluster pasan `a0 = -1`; aqui Nazca edito la
   plantilla a mano para pasar un puntero real y olvido borrar la carga
   anterior. Es la evidencia mas clara hasta ahora de codigo-por-plantilla
   editado manualmente.
2. **El idioma "branch a mitad de isla" escala**: 11 usos en un solo
   wave (`SetHandlerRts_041326/041488/04157c/041624/041760/04184e/
   041918/0419bc/041ab2/041c18` + `JsrAbsRts_041558`). La cola de cada
   estado sale por `bcc/beq` al `rts` INTERNO de la isla
   `SetTaskHandler` adyacente en vez de duplicar un `rts` propio: 2 B
   ahorrados por estado, confirmando que las islas se generaron JUNTO
   con los estados (no despues).
3. **`bcs.w Jsr5B6ThenJmpScheduler_040ef2` directo** en DropSpawnAtTop:
   primer branch condicional del proyecto cuyo destino es una funcion C
   ya matcheada (en vez del patron `lea/move.l` de autoinstalacion).
4. **Doble dereferencia del abuelo** en el trio: `movea.l 0xc(a6),a0;
   movea.l 0xc(a0),a0` — los hijos B del trio laten (`+0x92`++) sobre la
   estructura del ABUELO (el spawner), dos niveles arriba, y consultan
   su bit de aborto. Jerarquia de 3 niveles confirmada.
5. **Encodings verificados** (mismo criterio Wave SS/TT): tablas de
   punteros cargadas con `movea.l #imm` (207c) y templates con `lea
   abs.l` (41f9); `move.w #0,0x88(a6)` (3d7c) en `$041B18` donde el
   resto del proyecto usaria `clr.w`; dos `ori.w #0,0x38(a6)` muertos
   (capa 0 explicita de la plantilla).

### Wave TT en detalle — el subsistema "escuadron" completo

Siguiendo la directriz de ir a por **funciones principales mas grandes**,
Wave TT abre y cierra de una vez el cluster de gameplay contiguo
`$041C1A..$0422E4` (32 funciones, 1 634 B, dos archivos), el subsistema
completo de los enjambres voladores en formacion. **Verde a la primera
pasada del matcher** (record del proyecto para una oleada >1,6 KiB),
gracias a la verificacion previa de encodings introducida en Wave SS.

**Parte 1 — `asm/squad_wave_motion_041cxx.s` (20 funciones, 824 B):**

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---------|-----------|-------|-------------|
| 1 | `Squad_ComputeTargetPos_041C1A` | `$041C1A` | 56 | Target de formacion: tabla `$286124[(estado&15)*4]`, transform `$440D0`, clamp X `$110` → `+0x8A/+0x8C`. Promueve `PcThunkTarget_041c1a` |
| 2 | `Squad_InitFormationSlot_041C52` | `$041C52` | 68 | Init de vuelo: slot RNG 0..3, turn-rate RNG de `$286154`, heading inicial = atan2(`$5E018`). Cae en `SetTaskB_041c96` |
| 3 | `Squad_PickSwoopState_041C9C` | `$041C9C` | 62 | Decision de picado: estado 8/9 o A/B segun Y del objetivo (`$5E1EA`) + bit RNG. Cae en `JsrPcThunk_041cda` |
| 4 | `Squad_SteerTowardTarget_041CE0` | `$041CE0` | 120 | Guiado: distancia Manhattan vs umbral d5, giro ±2·turn-rate hacia atan2, refresco sincos. CCR via islas `ClearXN_041d58`/`SetXN_041d66` |
| 5 | `Squad_HaltVelocity_041D5E` | `$041D5E` | 8 | Llegada: vel = 0, return true |
| 6 | `Squad_TurnRateStepClamp_041D6C` | `$041D6C` | 44 | `+0x36 += d1` con saturacion en d0. **ASSERT trap#15 nop-patched `d1 != 0` en `$041D74`** |
| 7 | `Squad_TurnRateClampHi_041D9E` | `$041D9E` | 12 | Rama de clamp superior del anterior |
| 8–10 | `Squad_BobYFast/Wide/Narrow` | `$041DB6/DC/$041E02` | 38+38+34 | **Trio clonado no factorizado** de bob senoidal (tabla `$2C072C`): fase +$80/+$10/+$10, escala >>8/>>6/>>7. Promueve 2 placeholders |
| 11 | `Squad_BobYApply_041E24` | `$041E24` | 24 | Cola comun: delta → `+0x8E`, aplica a Y, test `$28D70`, revierte segun carry |
| 12 | `Squad_BobYRestore_041E42` | `$041E42` | 6 | Rama carry del test |
| 13 | `Squad_StateDispatch_041E4E` | `$041E4E` | 34 | **ASSERT trap#15 `estado < 6` en `$041E56`** + instala handler de `$28633C[estado]` en `(a6)` |
| 14–17 | `SquadAnim_*Select` | `$041E70..$041F32` | 58+52+38+22 | Cadena de seleccion de template de animacion (`$2BC62E..$2BC9BC`), cada uno con variante espejada si `+0x7C == $FF`, salida por islas `JsrAbsThunk_041exx/041fxx` |
| 18 | `Squad_TagSharedBit_041F3A` | `$041F3A` | 14 | `bset` del bit-id propio (`+0x85`) en `+0x21` de la estructura compartida (`+0xC`) |
| 19 | `Squad_PhaseStepToTarget_041F48` | `$041F48` | 48 | Heading → objetivo con paso ±4 o ±1 (**2 entradas**: `$041F48/$041F50`), modulo 256, CCR via islas |
| 20 | `Squad_ApplyLeaderDelta_041F84` | `$041F84` | 48 | Integra deltas s8 del registro de miembro (+bob del lider `+0x8E`) y hereda `+0x7C` |

**Parte 2 — `asm/squad_spawn_states_041fxx.s` (12 funciones, 810 B):**

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---------|-----------|-------|-------------|
| 21 | `Squad_SpawnEight_041FB4` | `$041FB4` | 66 | Crea los 8 miembros: bucle sobre `$28615C` (registros dx,dy,dz,bit-id), `Task_Alloc($4AE)` con handler `$40F00` + copy-transform `$5DD02` |
| 22 | `Squad_PollSharedState_041FF6` | `$041FF6` | 74 | Detecta cambio del estado publicado en `(+0xC)[$80+(bit-id&7)]`, cachea en `+0x80/+0x81`, instala `$40F82` o `$2863BC[cmd]` si bit 7. Promueve placeholder |
| 23 | `Squad_WriteBackState_042040` | `$042040` | 42 | Writeback tipo **CAS cooperativo**: publica `+0x80` solo si el array aun contiene su eco `+0x81`. Promueve placeholder |
| 24 | `Squad_DepthToScaleIdx_04206A` | `$04206A` | 58 | Cuantizador: clamp `[$70..$C0]`, −$68, /8, clamp 0..$A, fuerza indice PAR (`andi #$FE`) |
| 25 | `Squad_SinCosVelocity_0420A4` | `$0420A4` | 64 | heading+amplitud (min `$100`) → vel: seno `$2C072C`, coseno `$2C07AC` (= seno+64), `muls` + `asr.l #8` |
| 26 | `SquadPair_SpawnJoinTail_0420E4` | `$0420E4` | 4 | Trampolin `bra.w` a SpawnPlain |
| 27 | `SquadPair_SpawnFlagged_0420E8` | `$0420E8` | 22 | Preambulos del par (**2 entradas**: `+0x7F`=1 / `+0x7F`=0) |
| 28 | `SquadPair_SpawnCore_0420FE` | `$0420FE` | 132 | Sonido `$1064`, offsets `$2861D4[heading/2]`, crea 2 hijos (handlers `$415C6/$41626`), hereda flags, padre → comando `$87`. Cae en `JsrPcThunk_042182` |
| 29 | `SquadPair_SpawnPlain_042188` | `$042188` | 126 | **Clon par #11** de SpawnCore: mismo cuerpo, epilogo distinto (hijo B `+0x7F`=0, `+0x88`=1, sin comando al padre) |
| 30 | `TrioSpawner_PatternedPair_042206` | `$042206` | 124 | Sonido `$10A3`, patron ciclico `$28631C[(+0x9A)&15]` → offsets `$286310`, 2 hijos (handlers `$419CC/$419FC`), hijo B hereda fase en `+0x9A` |
| 31 | `Entity_ShadeBySine_042282` | `$042282` | 72 | Onda triangular de `+0x34` (doble pliegue 255→127→63, zona muerta <$18) → shade `+0x32/+0x33` = `$80+2v`, integra en `+0x38` |
| 32 | `Entity_SineToStep_0422CA` | `$0422CA` | 26 | Pliegue corto + /8 → paso 0..15. Cae en `SetTaskW_0422e4` |

**Hallazgos arquitectonicos Wave TT:**

- **2 nuevos bloques ASSERT `trap #15` nop-patched** (total 6 del
  proyecto): `ASSERT(paso != 0)` en `$041D74` y `ASSERT(estado < 6)`
  delante del jump table `$28633C` en `$041E56`. Mismo encoding exacto
  `cmp/bcc/nop/nop/cmp/nop/trap#15` que Wave SS: la macro de assert de
  Nazca se usaba tambien en el codigo de gameplay, no solo en colision.
- **Protocolo de estado compartido lider<->miembros**: la estructura
  apuntada por `+0xC` mantiene un array de 8 bytes en `+0x80` (1 por
  miembro, indexado por bit-id `+0x85 & 7`). El miembro hace *poll*
  (`$041FF6`) cacheando valor y eco, y *writeback* (`$042040`) solo si
  el array aun contiene su propio eco — un compare-and-swap cooperativo
  sin atomicidad, correcto porque el scheduler seria las tasks. Estados
  con bit 7 son comandos directos despachados via `$2863BC`.
- **Variante espejada de templates**: todas las selecciones de template
  de animacion comprueban `+0x7C == $FF` y eligen entre dos tablas
  separadas 0x82 B (`$2BC62E`/`$2BC6B0`, etc.) — assets duplicados
  pre-espejados en ROM en lugar de flip por hardware.
- **Coseno = seno + 64 entradas**: `$2C07AC = $2C072C + $80`, la tabla
  de 256 words se reutiliza desfasada un cuarto de periodo.
- **Trio de bobs no factorizado** (38+38+34 B casi identicos) y **clon
  par #11** (`SpawnCore`/`SpawnPlain`, 132/126 B): mas evidencia de
  macros/inline expandidos por el ensamblador de Nazca sin factorizar.
- 6 handlers pc-rel nuevos nombrados en `symbols.py`:
  `SquadMember_Handler_040F00`, `SquadMember_OnStateChange_040F82`,
  `PairChild_HandlerA_0415C6`, `PairChild_HandlerB_041626`,
  `TrioChild_HandlerA_0419CC`, `TrioChild_HandlerB_0419FC` — proximos
  candidatos naturales (el cluster `$040F00..$041C12` que queda por
  encima es la logica de los handlers de miembro).

### Wave SS en detalle — segunda oleada priorizada por TAMANO

Cierra los 8 primeros puestos de la cola de `tools/rank_candidates.py`
tras RR. 12 entradas de registro (11 funciones + 1 tabla de datos),
1 384 B brutos, +1 366 B netos tras descontar los 18 B de los 3 FPs
absorbidos. Promueve 8 placeholders de `symbols.py` a definicion
canonica.

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `Entity_CopyAnimFromLeader_06E2BC` | `$06E2BC` | 66 | Copia 9 campos (pos, anim, flags) de la entity "lider" (`a6->+0x50`) al destino `a0`. La copia de `+0x30` aparece DOS veces (store muerto que un compilador habría eliminado — evidencia de asm a mano). 4 callers, 2 de ellos `JsrPcThunk_*` matcheados. |
| 2 | `Entity_HitboxCollide_028A96` | `$028A96` | 114 | Caller del barrido de colisión: filtra self-hit, descriptor NIL, tipo != $80 e inmunidad (bit 3 de `flags69`); invoca #3 con los extents (+6 de cabecera) y propaga el resultado como bits dinámicos (`bset d7/d6`) en los `flags69` de ambas entities. Sale por las islas `ClearXN_028b08`/`SetXN_028b0e` (que siguen matcheadas aparte). |
| 3 | `Hitbox_OverlapTestXY_028B14` | `$028B14` | 268 | Test AABB entity-vs-entity con espejado por facing (bit 0 de `flags3a`, eje X) y flip vertical (bit 1, eje Y). Calcula además la intersección `[max(min), min(max)]` y el lado relativo (`slt d7`/`sge d6`) que #2 consume. **Contiene 4 bloques de ASSERT `trap #15` nop-patched** (ver hallazgo mayor abajo). Absorbe FPs #49–#51. |
| 4 | `ScriptSlotPairTable_0009B4` | `$0009B4` | 200 | Tabla de datos-en-.text: DOS sub-tablas de pares word (id, script) terminadas en `$FFFF`, consumidas por `Sub_00002B58` desde 3 handlers del scheduler (`lea $9b4(pc), a0`). Cada par asigna `$1CE00 + script*64` al slot `$1082C8 + id*32`. |
| 5 | `TaskSlots_BootInstall_000A7C` | `$000A7C` | 270 | Instalador boot de los 12 TCBs estáticos `$100xxx` vía `Task_InstallHandler_0000050E` (símbolo nuevo): 8 con handler idle `RtsStub_0400`, 3 con thunks reales y `$1001C0` con `SchedulerBootstrap_Boot_000E8E` (MM#1) por PC-rel — cierra el círculo del arranque. Arranca 3 tasks al vuelo y enlaza los pares player/partner (`$10044C`/`$1004EC` = campo `+0xC`). SIN caller directo: se alcanza vía la tabla (TCB, handler) de `$178000`. Fall-through en `SetTaskHandler_000b8a`. |
| 6 | `TaskList_ChangeAndRunEight_001CD4` | `$001CD4` | 96 | Batch de `Task_ChangeAndRun_0626` sobre los 8 TCBs de gameplay. Es el "callee $1CD4" documentado en MM#3 (7 callers). Fall-through en `JsrAbsThunk_001d34` (la 9ª operación implícita es `FUN_000005B6`). |
| 7 | `FixGlyph16_DrawCursorA_099F3A` | `$099F3A` | 76 | Dibuja un glifo 16x16 (bloque 2x2 de fix tiles, base `$4B22`) vía puerto LSPC `$3C0000`, en la celda leída de la tabla `a6->+0x80[a6->+0x78]` — cursor de menú en posición variable. |
| 8 | `FixGlyph16_DrawCursorB_099F86` | `$099F86` | 76 | Clon byte-a-byte de #7 con tile base `$4B40` (estado alternativo del cursor). 9º par de clones no factorizados. |
| 9 | `FixGlyphRun_Draw2F61F0_099FD2` | `$099FD2` | 24 | Prepara `a2 = $2F61F0 + idx*8`, `a1 = $72EB`, `d1 = 4` y cae por fall-through en `JsrAbsThunk_099fea` (renderizador de tiras de glifos `$4784C`). |
| 10 | `FixGlyph16_DrawDigit72EF_099FF2` | `$099FF2` | 74 | Dígito en celda fija `$72EF`: tile `$4B60 + 2*(a6->+0x73)`. |
| 11 | `FixGlyph16_DrawDigit72F3_09A03C` | `$09A03C` | 74 | Gemela de #10 en `$72F3` con `a6->+0x74`. 10º par de clones. |
| 12 | `FixGlyphRun_DrawPad2P_09A086` | `$09A086` | 46 | Solo si byte BIOS `$10FD83 == 2`: elige tira ROM por `a6->+0x76` y cae en `JsrAbsThunk_09a0b4`. La salida temprana hace `bne.w` al **rts INTERNO** del propio thunk matcheado (`JsrAbsRts_09a0ba`, símbolo nuevo). |

**Falsos positivos absorbidos Wave SS (3 nuevos, 51 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 49 | `ClearXN_028b7c` | N | `Hitbox_OverlapTestXY_028B14` (SS#3) | Rama "sin solape en X" `andi.b #$EE, ccr; rts` |
| 50 | `ClearXN_028c14` | N | `Hitbox_OverlapTestXY_028B14` (SS#3) | Rama "sin solape en Y" |
| 51 | `SetXN_028c1a` | N | `Hitbox_OverlapTestXY_028B14` (SS#3) | Rama "solape confirmado" `ori.b #$11, ccr; rts` |

**HALLAZGO FORENSE MAYOR — macro ASSERT con `trap #15` nop-patched:**

`Hitbox_OverlapTestXY_028B14` contiene CUATRO bloques idénticos:

```
    cmp.w   dY, dX
    blt.w   .Lok          ; ASSERT(min < max)
    nop
    nop
    cmp.w   dY, dX        ; (repite el cmp — resto de macro)
    nop
    trap    #15           ; breakpoint de debugger si falla
.Lok:
```

Primera evidencia directa del proyecto de una **macro de assert del
build de desarrollo de Nazca**: los `nop` son instrucciones del cuerpo
original de la macro parcheadas/condicionalmente ensambladas en la
release (probablemente un volcado de contexto para el monitor),
dejando el esqueleto `cmp/trap`. `trap #15` era el vector clásico de
breakpoint de los ICE/monitores 68000. Confirma definitivamente que el
binario final se ensambló desde fuentes con macros de debug activables.

**Otros descubrimientos arquitectónicos Wave SS:**

1. **Tabla (TCB, handler) en `$178000`** documentada por primera vez:
   contiene el par `($100120 -> $000A7C)` que explica cómo se alcanza
   SS#5 sin ningún `jsr`/`bra` en todo el ROM (mismo mecanismo threaded
   `jmp (a0)` de MM#1). Pendiente de registrar como tabla completa
   cuando se delimite su extensión.
2. **Campo `TCB->+0xC` = "partner"**: SS#5 enlaza `$100440->+0xC =
   $100300` y `$1004E0->+0xC = $1003A0` — pares task de jugador +
   partner, consistente con el uso de esos 4 TCBs en SS#6.
3. **Variante nueva del idioma "fall-through a thunk matcheado"**
   (11ª aparición del fall-through): SS#12 no cae sobre el INICIO del
   thunk vecino sino que salta con `bne.w` a su ÚLTIMO opcode (el
   `rts` en `$9A0BA`) como salida temprana. Nuevo símbolo
   `JsrAbsRts_09a0ba` en `symbols.py`.
4. **Puertos LSPC por `movem.w d0-d1, $3C0000`**: el par
   (dirección, dato) se escribe en un solo `movem` que cubre
   `$3C0000/$3C0002` — idioma compartido por las 4 funciones de glifo
   del cluster y los blitters de CC#2.
5. **9º y 10º pares de clones no factorizados** (#7/#8 y #10/#11):
   refuerza la hipótesis de macros ASM pesadas en el código original.

**Verde a la primera pasada completa del matcher** (0 iteraciones de
fix tras el primer `match_batch.py` de la oleada — récord del proyecto
para una oleada >1 KiB; las lecciones de RR sobre `jsr (pc)` explícito
y case-sensitivity de GAS se aplicaron preventivamente).

Estado tras SS: **3 143 / 3 143, 43 556 B, 2.0769 % ROM total**.
Incremento vs baseline post-RR: +9 funciones netas (12 nuevas, 3 FPs
absorbidos), +1 366 B netos, +0.0651 pp de cobertura ROM.

### Wave RR en detalle — primera oleada priorizada por TAMANO

Primera oleada del proyecto seleccionada con `tools/rank_candidates.py`
(cola ordenada por `score = eff_size * (1 + log2(callers))`, priorizando
TAMANO neto sobre popularidad de callers) en vez de
`tools/scan_unmatched_callees.py` (cola por popularidad, usada en todas
las oleadas S..QQ anteriores). Cierra 5 funciones que sumaban 516 B y
promueve 4 placeholders `PcThunkTarget_*` de `tools/symbols.py` a
definicion canonica.

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Entity_CheckActiveBoxOverlap_072C98` | `$072C98` | 144 | 1 | Comprueba si la entity `a1` cae dentro de la "caja activa" apuntada por `a6->+0x94`, con espejado horizontal segun `a6->flags3a` (bit 0 = facing). Escribe el resultado compuesto en `a1->+0x8E`. |
| 2 | `Entity_CheckBoxOverlapWithSelector_0798AC` | `$0798AC` | 164 | 1 | Variante hermana de #1: la caja llega por parametro explicito `a2` (no via `a6->+0x94`) y el chequeo final se resuelve con un **selector de 4 funciones** en `$2DF4AA` indexado por `box->+8 & 3`, invocado con `jsr (a3)` indirecto. |
| 3 | `Camera0_RelinkAndWrapScroll_06896A` | `$06896A` | 112 | 3 | Re-ancla `camera[0]` al banco VRAM `$20A000` y envuelve (`wrap`) el contador de scroll `$106F50` cada 320 px (`$140`, un ancho de pantalla completo) — mecanismo de fondo infinito. Sin `rts` propio: cae por fall-through en `SetXN_0689da`; la salida temprana salta a `ClearXN_0689e0`. |
| 4 | `Entity_MirrorDeltaByFacing_065D32` | `$065D32` | 12 | 1 | Micro-helper: niega `d0` si `a6->flags3a & 1` (facing izquierdo). Retorno via CCR (`ClearXN_065d44` / `SetXN_065d3e`). |
| 5 | `EntityGroup_SpawnLinkedFromTemplateList_065C94` | `$065C94` | 84 | 5 | Itera una lista de templates de stride 12 B terminada en centinela `0xFFFFFFFF`; por cada entrada reserva un task enlazado (`Task_AllocFromFreeList` + `Entity_CopyTransform`) y espeja su `delta_x` con #4 (`Entity_MirrorDeltaByFacing_065D32`). |

**Sin falsos positivos absorbidos** — las 5 funciones eran nuevas, sin
colas previamente contabilizadas como thunks independientes.

**Descubrimientos arquitectónicos Wave RR:**

1. **Campo `Entity->+0x8E`** ("resultado de deteccion") y **`Entity->+0x94`**
   ("puntero a caja activa") documentados por primera vez, vía #1/#2. No
   incorporados aún a `include/mslug.h` — pendiente de un segundo consumidor
   que confirme el layout antes de promoverlos a la struct compartida.
2. **Selector de 4 function-pointers en `.text`** (`$2DF4AA`) — cuarta
   tabla embebida del proyecto (tras `StartInputTable` BB#4, state
   descriptors EE#1, `JumpTable_096B9C` HH#2), aquí indexada por 2 bits de
   flags de la caja en vez de por estado del scheduler.
3. **Case-sensitivity de GAS reconfirmada por enésima vez**: `ClearXN_0689e0`/
   `SetXN_0689da`/`ClearXN_065d44` son minúsculas en `symbols.py` pese a que
   la dirección hexadecimal del nombre sugiere mayúsculas; ambos ficheros
   nuevos fallaron el link en la primera pasada por este motivo exacto.
4. **`jsr Simbolo(pc)` explícito obligatorio para PC-rel de 16 bits**:
   `jsr Entity_MirrorDeltaByFacing_065D32` (sin `(pc)`) ensambla como
   `4EB9` abs.l (6 B) en vez de `4EBA` PC-rel (4 B) del original, desplazando
   +2 B todo lo que sigue y recortando el `rts` final al extraer con
   `objcopy` el tamaño registrado. Mismo idioma ya documentado en Wave HH#2.

**Fixes iterativos aplicados (2 iteraciones hasta verde):**

1. `undefined reference to ClearXN_065D44` / `ClearXN_0689E0` — corregido a
   minúsculas (`ClearXN_065d44`, `ClearXN_0689e0`, `SetXN_0689da`).
2. Byte mismatch en `EntityGroup_SpawnLinkedFromTemplateList_065C94` (colas
   desde la mitad de la función, `rts` final recortado) — causado por el
   `jsr` sin `(pc)` del punto 4 anterior; corregido añadiendo `(pc)`.

Estado tras RR: **3 134 / 3 134, 42 190 B, 2.0118 % ROM total**. Incremento
vs baseline pre-RR: +5 funciones netas (5 nuevas, 0 FPs), +516 B netos,
+0.0246 pp de cobertura ROM.

### Wave MM en detalle (batch 1) — scheduler bootstrap + bytecode virtual

Corazon del scheduler-by-script-table del arranque BIOS. Interpreta la
super-tabla dispatch $000B92..$000E8E (760 B, 6 sub-tablas u32 BE con
centinelas $FFFFFFFF, 186 entradas, 93 handlers unicos) como bytecode
virtual continuation-passing.

| # | Simbolo | Direccion | Bytes | Descripcion |
|---|---|---|---:|---|
| 1 | `SchedulerBootstrap_Boot_000E8E` | `$000E8E` | 338 | Setup MMIO inicial (scroll limits $10E1E4/E6, RTC clear) + selector 5-way por bytes de config BIOS `$10FDAE` / `$10FDAF` (mode2/mode2_alt/title/demo/hardstart) que instala en `$70(a6)` el head de la sub-tabla dispatch correspondiente (6 targets: `$BE6`, `$BDA`, `$BCE`, `$B92`, `$BBA`, `$E6E`). Cae por fall-through al mainloop tail (`$000F98`) que incrementa `$10007C` (frame counter global), invoca video/audio hooks + `Pubcleaner_10A2Cx` (LL#1) + BIOS VBlank, instala `SchedulerLoopA_000FC6` como (a6).entry y ejecuta el **bucle dispatch A**: intérprete threaded que deref el cursor, salta sobre centinelas $FFFFFFFF via `*(cursor+4)`, y hace `jmp (a0)` al handler. |
| 2 | `SchedulerDispatch_LoopB_000FE0` | `$000FE0` | 30 | **Punto de re-entrada de todos los handlers de la super-tabla**: cada handler termina con `bra.w $FE0`, este bloque avanza cursor +=4 (con auto-salto sobre centinelas via `*(cursor+4)`), re-publica en $70(a6), y salta cross-section a `SchedulerLoopA_000FC6` via `bra.b`. 7a aparicion del fall-through, **1a como hub compartido entre >2 handlers**. |
| 3 | `SchedTail_JsrCD4_001020` | `$001020` | 6 | Handler-tail micro: `bsr.w $1CD4; bra.b $FE0`. Un handler entero en 6 bytes. |
| 4 | `SchedTail_JsrD3C_001026` | `$001026` | 6 | Gemelo byte-a-byte del anterior con destino $1D3C. **8o par de clones no factorizados del proyecto**. |
| 5 | `AttractHandler_10002C` | `$00102C` | 106 | Handler grande de attract: incrementa frame_ctr, refresca video+BIOS+Pubcleaner_10A2Cx (LL#1), asigna task_tpl $98720 via ThunkTarget_0004ae (Task_Alloc), resetea key latches, pinta el marco HUD Fix Layer via `FixLayer_QuadBatch_046AC6` (HH#3), setea `$106ED2=-1` (hud_dirty), salta al bucle B. Fase 2 (`$00107E`) testea `$106ED2` y cae por fall-through directo al thunk ya matcheado `JsrPcThunk_001096` (Wave J). Tamano real 106 B (no 112) - los 6 B finales son cuerpo del thunk vecino. |

**Sin FPs absorbidos** — los 5 rangos estaban delimitados por 4 anclajes
matcheados vecinos (`JsrPcThunk_001096`, `JsrPcThunk_0010ec`,
`SetTaskHandler_00116a`, `Init_ModeToggle_001260`) tras aplicar por 4a
vez la leccion metodologica de Wave II (cruce contra registry antes de
abrir cluster).

**Descubrimientos arquitectonicos Wave MM batch 1:**

1. **Super-tabla dispatch `$000B92`** documentada por primera vez. 6
   sub-tablas u32 BE (T1: 13 entries, T2: 105 entries, T3: 4, T4: 4,
   T5: 40, T6: 20) separadas por centinelas $FFFFFFFF en $BC6, $D6E,
   $D82, $D96, $E3A. 186 entries totales, 93 handlers unicos.
   Se registrara como `BootDispatchTable_000B92` (760 B .long array)
   en Wave MM batch 2.

2. **Bytecode virtual continuation-passing** documentado por primera vez.
   El bucle A (`$000FC6..$000FDE`) es un interprete threaded de 4
   instrucciones:

   ```
   .loop:
     a0  = *cursor                       ; cursor = $70(a6)
     if (*a0 == $FFFFFFFF):
         a0 = *(a0 + 4)                  ; auto-salto a sig. sub-tabla
         *cursor = a0                    ; persist
     a0 = *a0                            ; deref handler ptr
     jmp (a0)                            ; execute; handler bra $FE0
   ```

   El bucle B (`$000FE0..$000FFE`, 30 B) es el hub de re-entrada:
   avanza cursor +=4 y salta cross-section a bucle A. GCC no genera
   este patron - usaria un switch/table + call convencional.

3. **Idioma "fall-through a thunk matcheado"** documentado por primera
   vez. `AttractHandler_10002C` termina con `bne.w $096; jsr $5B6.l;
   bra.w $FE0` y por fall-through directo cae en el cuerpo de
   `JsrPcThunk_001096` (Wave J), que sirve como "salida alternativa" en
   6 bytes. 8a aparicion del fall-through del proyecto, **1a hacia una
   funcion ya matcheada por wave anterior**.

4. **8o par de clones no factorizados del proyecto** — `SchedTail_JsrCD4`
   y `SchedTail_JsrD3C` son gemelos byte-a-byte salvo por el destino del
   `bsr.w`. Ambos son handlers-tail de 6 bytes.

5. **`$70(a6) = cursor persistente**` — offset del TCB que aloja el
   cursor de la super-tabla. Todos los handlers leen/escriben este
   offset como si fuera la variable global de estado del interprete.

6. **Idioma "GAS m68k PC-rel absoluto"** documentado por primera vez
   como *leccion tecnica del matcher*. `lea.l 0xBE6(pc), a0` con GAS
   m68k emite el literal `$0BE6` como displacement (constante literal
   en el opcode), NO como direccion absoluta a resolver. Para obtener
   el opcode ROM (`41FA FCE2`) hay que:

   - Declarar el simbolo como **external no definido** en el `.s`:
     `lea.l BootTblEntry_BE6(pc), a0`.
   - Anadir la direccion real a `symbols.py` como
     `0x00000BE6: "BootTblEntry_BE6"`.

   GAS emite entonces reubicacion `R_68K_PC16` que el linker calcula
   correctamente. Los `.set XXX, 0xNNN` NO funcionan aqui porque GAS
   los trata como constantes literales locales, sin reubicacion. Regla
   aplicable a todos los futuros PC-rel a targets externos al `.s`.

**Fixes iterativos aplicados (5 iteraciones hasta verde):**

1. `moveq #0xFF, d0` fuera de rango signed 8-bit → cambiado a
   `moveq #-1, d0` (mismo byte `70 FF`).
2. `bra.b` cross-section fallaba → promovida `.Lsched_boot_loop_a` a
   `SchedulerLoopA_000FC6` como simbolo global, activando `R_68K_PC8`.
3. Overlap con funciones matcheadas → recortadas `SchedulerDispatch_LoopB`
   de 64→30 B (colision con `Global_SetDualFlagFrom10FD82_000FFE`) y
   `AttractHandler_10002C` de 112→106 B (colision con `JsrPcThunk_001096`).
   Total batch **486 B** (no 526).
4. PC-rel con literales numericos emitia `41FA 0BE6` en vez de `41FA FCE2`
   → 6 externals nuevos `BootTblEntry_XXX` en `symbols.py`. **Leccion
   metodologica** documentada.
5. Etiqueta cruzada → `beq.w $F52` apuntaba a `.Lsched_boot_title`
   erroneamente colocada en $F1A; renombrada a
   `.Lsched_boot_hardstart_10FEC5` y colocada en $F52 (donde vive el
   `move.b #1, $10FEC5.l`).

### Wave KK en detalle (batch 1) — callees pendientes camara/sprites

Cierra 3 de los 5 callees pendientes documentados al final de Wave JJ. La
cuarta función candidata (`$05A88A` `SpriteSubsystem_Reset`) se retiró del
batch tras detectar por overlap del linker que cae **dentro** de
`VRAM_FixLayerAutoclear_05A824` (Wave DD, 150 B, `$05A824..$05A8B9`): lo
que parecía el reset del subsistema de sprites es en realidad la segunda
mitad de aquella función. **Segunda aplicación exitosa de la lección
metodológica de Wave II** (cruzar rango contra registry antes de abrir
cluster).

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `BlitterTile_2D_043E8C` | `$043E8C` | 78 | Blit 2D de un tile en el buffer local de un sistema de cámara. **Target de tail-jump** (`bra.w $43E8C`) de los tres hooks de cámara JJ#1 cuando el probe pasa y el enlace no es NULL. Doble bucle con `dbra` sobre filas y columnas, cascada de 4 `add.w dX,dX` intercalados como `<<2` hand-coded, máscaras `$F80`/`$7C` como aritmética modular del tile-map. |
| 2 | `Integrator_XY_051B80` | `$051B80` | 40 | Integrador de coordenadas 2D. Suma incrementos `d0`/`d1` a los acumuladores long (`+$4`, `+$8` del struct sprite), publica el word alto como delta visible (`(a0)`, `$2(a0)`). Idioma `swap/sub.w/move.w` repetido sin factorizar entre X e Y. Es el `Transform_Publish` invocado por `CameraApplyOne_043DAA` (JJ#1). |
| 3 | `TransformCommit_MMIO_051F30` | `$051F30` | 100 | Commit de la transformación. Gate por bit 0 de `$C(a0)` (rama corta con `rts` propio), calcula 4 valores intermedios (`$2A/$2C/$2E/$30`), y llama al dispatcher `Fn_00001F4A` pasándole por `a0` el **handler inline** en `$051F94` (patrón "call by continuation"). Es el `Transform_Commit` de `CameraApplyOne_043DAA` (JJ#1). |

**Sin FPs absorbidos** — los 4 rangos del batch estaban limpios (auditoría
previa con `scan_unmatched_callees.py`).

**Descubrimiento arquitectónico Wave KK batch 1:**

1. **Idioma `call by continuation`** documentado por primera vez. En
   `TransformCommit_MMIO_051F30`, en lugar de invocar directamente al
   handler `$051F94`, el código hace:

   ```asm
   movem.l a0/a6, -(a7)              ; save frame ptrs
   movea.l a0, a6                     ; a6 = struct sprite
   lea.l   TileMap_HandlerInline_051F94(pc), a0  ; a0 = ptr handler
   jsr     Fn_00001F4A                ; dispatcher generico
   movea.l a6, a0                     ; restore a0
   movem.l (a7)+, a0/a6
   ```

   `Fn_00001F4A` es un dispatcher que ejecuta el handler apuntado por `a0`
   con `a6` como contexto activo. Es la variante 68000 del "trampoline"
   clásico. GCC no genera este patrón: usaría punteros a función
   convencionales. **Añadir a `include/mslug.h` como documentación del ABI
   interno.**

2. **Segunda aplicación de la lección de Wave II** (overlap linker detecta
   que `$05A88A` cae dentro de una función ya matcheada). El caller
   `Scratch_Alloc_01390E` (JJ#2) hace `jsr` al **punto de entrada interno**
   `$05A88A`, no al inicio de la función contenedora `$05A824` — idioma
   "multiple entry points" del asm hand-coded ya visto en Wave AA#3 y HH#1.

### Wave KK en detalle (batch 2) — probes de colisión + handler MMIO

Cierra los tres probes CCR referenciados por los hooks de cámara JJ#1
(`Probe08/82/F6`) y el handler inline pasado por `a0` a `Fn_00001F4A`
desde `TransformCommit_MMIO` (KK#1). **7 FPs absorbidos** en un solo batch
(récord del proyecto por batch, previamente 6 en HH#2).

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `Collision_ProbeRange_051C08` | `$051C08` | 120 | Probe de rango completo. Recorre `d4` iteraciones con `dbra`, invoca `$51D84` (colisión) y aplica `$51BA8`/`$51DE2` por celda actualizando dos tile-maps locales (`+$32`, `+$52`). Retorno CCR bilateral: `ori.b #$1, ccr; rts` (colisión) vs `rts` puro (no colisión). Absorbió `Stub_00051C80` (FP #48) y `SetC_051c7a` (FP #42). |
| 2 | `Collision_ProbeX_051C82` | `$051C82` | 110 | Probe de una sola columna X con **cache en `$1E(a0)`** que salta el probe si la posición no cambió. Aplicación directa sin bucle. Absorbió `SetC_051cea` (FP #43) y `ClearC_051cf0` (FP #44). |
| 3 | `Collision_ProbeY_051CF6` | `$051CF6` | 136 | **Clon estructural** de ProbeX con ejes X/Y intercambiados y bucle `dbra` interno (por eso 26 B más que ProbeX). Cache en `$20(a0)`. **6º par de clones no factorizados del proyecto** (tras BB#2, Z#5/#6, HH#2, II#1, II#2 y JJ#2). Absorbió `SetC_051d78` (FP #45) y `ClearC_051d7e` (FP #46). |
| 4 | `TileMap_HandlerInline_051F94` | `$051F94` | 158 | Handler MMIO pasado por `a0` a `Fn_00001F4A` desde `TransformCommit_MMIO_051F30` (KK#1). Recorre `[tile_row_start, tile_row_end]` publicando cada celda en el puerto VRAM `$3C0000/$3C0002` con `<<11` compuesto vía `moveq #$B, d4; lsl.w d4, dX`. Absorbió `SetV_05202c` (FP #47). |

**Falsos positivos absorbidos Wave KK batch 2 (7 nuevos, 48 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 42 | `SetC_051c7a` | N | `Collision_ProbeRange_051C08` (KK#2) | Rama colisión `ori.b #$1, ccr; rts` |
| 43 | `SetC_051cea` | N | `Collision_ProbeX_051C82` (KK#2) | Rama colisión del probe X |
| 44 | `ClearC_051cf0` | N | `Collision_ProbeX_051C82` (KK#2) | Rama sin colisión `andi.b #$FE, ccr; rts` |
| 45 | `SetC_051d78` | N | `Collision_ProbeY_051CF6` (KK#2) | Rama colisión del probe Y |
| 46 | `ClearC_051d7e` | N | `Collision_ProbeY_051CF6` (KK#2) | Rama sin colisión del probe Y |
| 47 | `SetV_05202c` | N | `TileMap_HandlerInline_051F94` (KK#2) | `addi.w #$800, d3` dentro del bucle (mal clasificado como helper CCR) |
| 48 | `Stub_00051C80` | B | `Collision_ProbeRange_051C08` (KK#2) | `rts` de la rama sin colisión alcanzada por `bcc.w` |

**Descubrimientos arquitectónicos Wave KK batch 2:**

1. **6º par de clones no factorizados del proyecto** (ProbeX/ProbeY). El
   cuerpo es idéntico salvo por los cuatro pares `X↔Y`: `$1E↔$20` (cache),
   `(a0)↔$2(a0)` (flags de flip), `$1A↔$1C` (width↔height), `d4↔d5`
   (contador de eje). ProbeY usa además bucle `dbra` interior (por eso es
   26 B más grande). Confirma definitivamente la hipótesis de macros ASM
   pesadas del código original.

2. **Récord de FPs absorbidos en un batch**: 7 (previamente 6 en HH#2). El
   cluster $051Cxx está densamente poblado de epílogos CCR bilaterales
   (SetC/ClearC + rts) que Wave N clasificó como helpers independientes
   por su tamaño compacto (6 B c/u), pero forman parte de la semántica
   "return via CCR" de cada probe.

3. **FP #47 recategorizado**: `SetV_05202c` NO era un helper CCR sino
   `addi.w #$800, d3` dentro del bucle del handler MMIO. El opcode
   `023C 0800` (Wave N asumió `ori.b #$?, ccr`) es en realidad `addi.w
   #$800, d3` (`0643 0800`). Es el primer FP del proyecto que Wave N
   clasificó erróneamente por decodificación hint.

4. **Cache de posición en probes X/Y**: los offsets `$1E(a0)` y `$20(a0)`
   guardan la última posición probada para saltarse el probe cuando el
   sprite no se ha movido. Es una optimización hand-coded que GCC no
   generaría por sí mismo (requiere semantica de dominio).

5. **Layout struct sprite ampliado** (más de 20 offsets confirmados):
   `$04/$08` = acumuladores X/Y long, `$16/$18` = offset base, `$1A/$1C`
   = width/height, `$1E/$20` = cache X/Y, `$22/$24` = pos tile, `$26/$27`
   = flag byte, `$28` = sprite ID, `$2A/$2C` = tile_row_start/end,
   `$2E/$30` = hw_x_offset/hw_y_offset, `$32/$52` = dos tile-maps locales.

### Wave JJ en detalle (batch 1) — cluster de aplicación de cámara `$043DAA`

Cierra el subsistema de cámara iniciado en Wave CC#1 (coordenadas) y
continuado en II#2 (`Reset4CameraLongs_043D6C`, `CameraApplyAll4_043D86`).
`CameraApplyOne_043DAA` es precisamente el callee que `CameraApplyAll4`
invoca 3 veces por `bsr.w` y una cuarta por fall-through.

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `CameraApplyOne_043DAA` | `$043DAA` | 74 | Aplica la transformación de escala/posición a UN sistema de cámara. Gate por `$E(a0)` (sistema activo), producto 16×16→32 con reescalado a punto fijo 8.8 (`muls.w` + `asl.l #8`), dos sub-hooks condicionados por bits 0/1 de `$72(a0)`, y commit final. Absorbió `JsrAbsThunk_043dec` (FP #40). |
| 2 | `CameraHook_Probe08_043DF4` | `$043DF4` | 26 | Hook con probe `$51C08`. **Exporta el `rts` compartido** en `$043E0C` que reutilizan los otros dos hooks. |
| 3 | `CameraHook_Probe82_043E0E` | `$043E0E` | 22 | Hook con probe `$51C82` (bit 0 de flags). Salta al `rts` de Probe08; su propio `rts` final es código muerto. |
| 4 | `CameraHook_ProbeF6_043E24` | `$043E24` | 22 | Hook con probe `$51CF6` (bit 1 de flags). Clon byte-a-byte de Probe82 salvo el probe. Mismo `rts` muerto. |

**Falso positivo absorbido Wave JJ batch 1 (1 nuevo, 40 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 40 | `JsrAbsThunk_043dec` | I | `CameraApplyOne_043DAA` (JJ#1) | Cola `jsr $51F30.l; rts` (Transform_Commit) |

**HALLAZGO FORENSE MAYOR — epílogo compartido cruzado:**

La comparación byte-a-byte de los tres hooks revela que **B y C reutilizan
el `rts` del hook A** como salida temprana:

```
$043E0C   rts              <- epílogo del hook A
$043E14   bcc.b $43E0C     <- hook B salta al rts de HOOK A
$043E1C   beq.b $43E0C     <- idem
$043E22   rts              <- epílogo propio de B: INALCANZABLE
$043E2A   bcc.b $43E0C     <- hook C salta al rts de HOOK A
$043E32   beq.b $43E0C     <- idem
$043E38   rts              <- epílogo propio de C: INALCANZABLE
```

Los `rts` finales de B y C son **código muerto** preservado por la macro que
emite el epílogo incondicionalmente (la instrucción previa es siempre un
`bra.w $43E8C` incondicional). Esta es la **primera evidencia directa del
proyecto** del idioma «epílogos compartidos entre funciones aparentemente
independientes» enunciado como hipótesis fundacional, y además en su forma
*cruzada* (B y C dependen de A).

Consecuencia práctica: los `bcc.b`/`beq.b` de B y C deben ensamblarse contra
un símbolo **global** exportado desde la sección de A
(`CameraHook_SharedRts_043E0C`), no contra una etiqueta local.

Detalle adicional: el hook A usa `bcc.w`/`beq.w` (4 B) mientras B y C usan
`bcc.b`/`beq.b` (2 B) para el *mismo salto lógico*. Es un artefacto de
ensamblado en una sola pasada: en A el destino aún no estaba resuelto.

### Wave JJ en detalle (batch 2) — asignador de sprites hardware `$0139xx`

Cierra el trío de callees que `SceneLoader_Main_043568` (HH#1) invoca en sus
dos pasadas sobre el bytecode de escena, y que hasta ahora solo existían como
externals en `symbols.py`. **Descubrimiento arquitectónico mayor**: es el
asignador de los 381 sprites hardware del Neo Geo.

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `Scratch_Alloc_01390E` | `$01390E` | 68 | Particiona los 381 sprites en **dos pools que crecen en sentidos opuestos** y apaga todos los sprites antes de la escena. Fija los 6 globales del asignador en `$10E1F4..$10E1FE`. Absorbió `JsrPcThunk_01394c` (FP #41). |
| 2 | `Spawn_TypeB_013952` | `$013952` | 46 | Reserva del pool B (crece desde 0 hasta `$10E1F6`). **Exporta `SpawnTail_01396E`**, la cola de 6 instrucciones que comparte con TypeA. |
| 3 | `SpriteTrapGuard_013980` | `$013980` | 2 | Guarda de desbordamiento `trap #$F` compartida por ambos spawners. |
| 4 | `Spawn_TypeA_013982` | `$013982` | 26 | Reserva del pool A (crece desde la marca hasta 380). Termina con `bra.b` a la cola **compartida** de TypeB. |
| 5 | `SpriteRange_DisableAll_01399C` | `$01399C` | 34 | Apaga el rango `[d0,d1]` escribiendo 0 en SCB3 (altura 0 = no dibujado). Escritura MMIO pareada con `movem.w`. |
| 6 | `SpriteRange_InitChain_0139BE` | `$0139BE` | 64 | Arma `[first,last]` como **cadena hardware**: cabeza no-sticky (`$000`), resto sticky (`$040` = bit 6 de SCB3), shrink neutro (`$FFF`) en SCB2. Núcleo del objeto multi-sprite. |

**Falso positivo absorbido Wave JJ batch 2 (1 nuevo, 41 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 41 | `JsrPcThunk_01394c` | J | `Scratch_Alloc_01390E` (JJ#2) | Cola `jsr $1399C(pc); rts` |

**Descubrimientos arquitectónicos Wave JJ batch 2:**

1. **Modelo de sprites hardware del Neo Geo reconstruido.** El sistema tiene
   381 sprites (`0..$17C`) descritos en cuatro bancos accedidos por el puerto
   MMIO `$3C0000`: SCB1 (tile map), SCB2 `$8000..$81FF` (shrink), SCB3
   `$8200..$83FF` (Y + sticky bit 6 + altura) y SCB4 (X). El **bit sticky**
   encadena un sprite al anterior, permitiendo objetos anchos compuestos por
   N sprites contiguos. `SpriteRange_InitChain` implementa exactamente eso.

2. **Dos pools opuestos.** `Scratch_Alloc` reparte el banco en un pool B que
   crece desde 0 y un pool A que crece desde `380 − head − mid`. El bytecode
   de escena (HH#1) manda `type == 0` a TypeA y el resto a TypeB, repartiendo
   los objetos entre los dos extremos del banco (típicamente decorado en uno
   y actores en el otro).

3. **Segundo caso de código compartido de la oleada, ahora no solo epílogo.**
   `Spawn_TypeA` termina con `bra.b $01396E`, una dirección **dentro del
   cuerpo de `Spawn_TypeB`** (offset +$1C). No comparten dos bytes de `rts`
   sino **seis instrucciones completas**, incluida la llamada al inicializador
   de cadena. Comparten además la guarda `trap #$F`, alcanzada con `bhi.w`
   (4 B) desde TypeB y `bhi.b` (2 B) desde TypeA — de nuevo el artefacto de
   ensamblado en una pasada.

4. **Idioma `movem.w` como escritura MMIO pareada** reconfirmado por cuarta
   vez (CC#2, DD, II#1, JJ#2): `movem.w d0/d2, (a0)` con `a0 = $3C0000`
   escribe dirección y dato en un único opcode. GCC no lo genera.

### Wave II en detalle (batch 1) — Fix Layer backends `$05DBxx` + slot helper

Continúa el cluster Fix Layer iniciado en Wave W (que ya cerró `Fix_BlitRect
_05DA9C` y `Fix_BlitStream_05DAD8`, los dos backends base) y explotado por
Waves CC#2 y HH#3. Wave II#1 cierra los 3 helpers contiguos restantes más
`SlotExtractCoords_05E2D8`, que hasta ahora sólo existía como external
`Fn_00005E2D8` invocado por `SelectPositive_TwoSlots_0967C0` (HH#2).

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `FixLayer_MultiRowBlit_05DB1A` | `$05DB1A` | 34 | ? | Recorre lista de tile codes terminada en `$FFFF`, invocando el backend fila `$5DA56` por entrada y avanzando el cursor VRAM `$40` B (= 2 filas de tile-map). Tiene un **`rts` INTERMEDIO** en `$05DB26` (rama "lista vacía") seguido de la continuación del bucle. `jsr $5DA56(pc)` PC-rel 16-bit (4 B). |
| 2 | `CompareField10_CCR_05DB3C` | `$05DB3C` | 28 | ? | Probe CCR bilateral: compara byte `$10` del contexto activo contra el del contexto enlazado en `$8(a6)`. `C set` = self < linked. Primer probe del proyecto que compara **dos contextos enlazados**, lo que identifica `$8` como puntero parent/sibling del context struct. Absorbió `ClearXN_05db4c` y `SetXN_05db52` (FPs #33/#34). |
| 3 | `InstallListPubHead_05DB58` | `$05DB58` | 18 | ? | Instala descriptor de lista en `$3C(a6)` y publica su tamaño (`$6(a0)`) en `$46(a6)`, delegando en `$5DBC2` para reiniciar el cursor. Fija el layout del context struct: `$3C` = list_ptr, `$46` = list_size. Absorbió `JsrPcThunk_05db64` (FP #35). |
| 4 | `SlotExtractCoords_05E2D8` | `$05E2D8` | 96 | 2 | Extrae `(x, y)` de un slot player con 3 guardas de validez (`$FFFFFFFF`/`$52A`/`$400`), gate por modo global `$106ECE`, y test de bit 7 de `$5B(a0)` para validar Y. Precarga sentinela con `moveq #-1`. **Los paths A y B son byte-a-byte idénticos** — 4º par de clones no factorizados del proyecto. Fija el layout de PlayerSlot: `$22`=x, `$24`=y, `$5B`=flags. |

**Falsos positivos absorbidos Wave II batch 1 (3 nuevos, 35 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 33 | `ClearXN_05db4c` | N | `CompareField10_CCR_05DB3C` (II#1) | Rama greater-equal `andi.b #$EE, ccr; rts` |
| 34 | `SetXN_05db52` | N | `CompareField10_CCR_05DB3C` (II#1) | Rama less-than `ori.b #$11, ccr; rts` |
| 35 | `JsrPcThunk_05db64` | J | `InstallListPubHead_05DB58` (II#1) | Cola `jsr $5DBC2(pc); rts` |

**Descubrimiento clave Wave II batch 1:** `$05DA9C` y `$05DAD8` ya estaban
matcheados desde Wave W (`Fix_BlitRect` / `Fix_BlitStream`), detectado por
colisión de secciones del linker. Se consolidó sin duplicar entradas: la
documentación semántica ampliada de Wave II se mantiene, pero el registry
conserva las entradas originales de Wave W. **Lección metodológica**: antes
de abrir un cluster nuevo hay que cruzar el rango de direcciones contra el
registry completo, no sólo contra la cola de `scan_unmatched_callees.py`.

### Wave II en detalle (batch 2) — callees pendientes camera / list / ctx

Cierra 6 funciones que son **callees directas de código ya matcheado**, lo
que las convierte en cierres naturales del grafo de llamadas:

- `Reset4CameraLongs_043D6C` ← `SceneLoader_Main_043568` (HH#1, `bsr.w`)
- `ListCursor_Reinit_05DBC2` ← `InstallListPubHead_05DB58` (II#1, `jsr pc`)
- `PlayerCtx_InitExtended_025012` ← `PlayerCtx_ResetTwoBlocks_024FEC` (HH#3, `bcs.w`)

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `Reset4CameraLongs_043D6C` | `$043D6C` | 26 | Desactiva los 4 sistemas de cámara escribiendo `0` en el campo `+$E` de cada base (`$106F6C`/`$107FE8`/`$108064`/`$1080E0`, stride `$7C`). Cuatro `clr.l abs.l` literales pese al stride regular. |
| 2 | `CameraApplyAll4_043D86` | `$043D86` | 36 | Bucle de 4 iteraciones **desenrollado** que aplica `$043DAA` a los 4 sistemas: 3 por `bsr.w` y **la 4ª por FALL-THROUGH**, reutilizando el `rts` del callee. 6ª aparición del idioma fall-through y **1ª como última iteración de bucle** en vez de salida alternativa. |
| 3 | `ListCursor_Reinit_05DBC2` | `$05DBC2` | 26 | Reinicia cursor de lista y blitea la entrada actual vía `$5DA56`. Fija el layout de ListEntry: `$0`=tile, `$2`=cols, `$4`=rows, y `$22(a6)`=puntero VRAM cargado con `movea.w` (sign-extend, válido porque el Fix Layer vive en `$7000..$74FF`). Absorbió `JsrAbsThunk_05dbd4` (FP #38). |
| 4 | `ListCursor_ReinitClipped_05DBDC` | `$05DBDC` | 36 | Contrapartida **"borrar"** de la anterior: guarda de lista vacía (`$FFFF`) y relleno con tile `$FF` vía `Fix_BlitRect` (`$5DA9C`). Par simétrico pintar/borrar. Absorbió `JsrAbsThunk_05dbf8` (FP #39). |
| 5 | `CompareField10_CCR_05DC00` | `$05DC00` | 28 | **Clon byte-a-byte** de `CompareField10_CCR_05DB3C` (II#1) situado `$C4` B más adelante. 5º par de clones no factorizados del proyecto. Absorbió `ClearXN_05dc10` y `SetXN_05dc16` (FPs #36/#37). |
| 6 | `PlayerCtx_InitExtended_025012` | `$025012` | 84 | Rama "player_count ≥ 2" de HH#3. Marca `ctx_mode = MULTI`, indexa la tabla de buffers `$E7C00[]` con `count*4` (dos `add.w d0,d0`), limpia 4 campos de estado reutilizando `d0=0`, y si el flag de build `$025118` está armado hace un `memset(buffer, $FF, 512)` byte a byte con `dbra`. |

**Falsos positivos absorbidos Wave II batch 2 (4 nuevos, 39 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 36 | `ClearXN_05dc10` | N | `CompareField10_CCR_05DC00` (II#2) | Rama greater-equal del probe clon |
| 37 | `SetXN_05dc16` | N | `CompareField10_CCR_05DC00` (II#2) | Rama less-than del probe clon |
| 38 | `JsrAbsThunk_05dbd4` | I | `ListCursor_Reinit_05DBC2` (II#2) | Cola `jsr $5DA56.l; rts` |
| 39 | `JsrAbsThunk_05dbf8` | I | `ListCursor_ReinitClipped_05DBDC` (II#2) | Cola `jsr $5DA9C.l; rts` |

**Descubrimientos arquitectónicos Wave II batch 2:**

1. **CORRECCIÓN de la hipótesis de Wave HH#2 sobre la jump-table `$096B9C[8]`.**
   HH#2 la documentó como "8 handlers de un sub-dispatcher attract pendiente"
   y la propuso como objetivo principal de Wave II. **Era incorrecto**: el
   volcado de `$096BBC` interpretado con stride `$14` revela entradas de
   20 B con centinela `$FFFF` — exactamente el formato que iteran
   `AttractCuller_Cam0/Cam1` (`cmpi.w #$FFFF, (a0,d4.w)` + `addi.w #$14`).
   Los 8 punteros apuntan a **TABLAS DE DATOS**, no a código.
   `ClampAndLookup8_096B7E` es por tanto un **table-of-tables**: dado un
   índice de escena attract (0..7) devuelve la lista de sprites de esa
   escena, y `$21(a6)` es el **índice de escena**, no un opcode de dispatch.
   Conforme a las reglas del proyecto, esos datos NO son decompilables y no
   entran en el registry.

2. **Sexta aparición del idioma fall-through, primera como iteración de bucle.**
   Hasta ahora (Waves T, DD, EE#3, GG#2, HH#3) el fall-through a la función
   vecina era siempre una *salida alternativa*. En `CameraApplyAll4_043D86`
   es la **última iteración de un bucle desenrollado**: ahorra 4 B del
   `bsr.w` y 2 B del `rts` propio. La función no tiene epílogo.

3. **Quinto par de clones no factorizados** (`CompareField10_CCR` ×2, a `$C4`
   B de distancia). Junto con los paths A/B de `SlotExtractCoords` (II#1),
   la Wave II aporta **dos casos nuevos**, elevando el total del proyecto a
   cinco y consolidando definitivamente la hipótesis de macros ASM pesadas.

4. **Layout del context struct (a6) ampliado**: `$8` = puntero parent/sibling
   (comparado por `CompareField10`), `$10` = campo de prioridad/orden,
   `$22` = puntero VRAM (word con sign-extend), `$3C` = list_ptr,
   `$46` = list_size. **Añadir a `include/mslug.h`.**

5. **Flag de build en ROM**: `PlayerCtx_InitExtended` lee `$025118` — un byte
   dentro del propio rango de código `$025xxx` — como flag de configuración.
   Es una constante de build embebida en el binario, no una variable RAM.
   GCC la habría resuelto en compilación y eliminado la rama muerta.

### Wave HH en detalle (batch 1) — scene loader + camera smoothing `$043xxx`

Ataque al **top #1 de la cola priorizada** (`SceneLoader_Main_043568`, 8 callers,
374 B): el punto de entrada canónico de carga de escena del juego. Descubrimiento
arquitectónico mayor — el ROM contiene una **tabla de descriptores de escena en
`$916C8[256]`** (parejas `{config_ptr, entry_script_ptr}` de 8 B cada una), y
cada `entry_script` es un mini-bytecode de entradas de 14 B (`{u8 type; u8 subop;
u32 template_ptr; u8 payload[8]}`) terminado por `type==2`. El loader hace dos
pasadas sobre el bytecode: en la 1ª cuenta entradas por tipo para llamar al
asignador `$1390E`, en la 2ª spawnea entities vía `$51ABE` inicializando flags
fijos (`$74/$76=$100, $72=0, $78=0L`), y finalmente ejecuta 5 subsystem-init
hooks (`$7707C/$8F158/$3EE3A/$997B8/$4CB5C`). El cluster incluye 3 helpers
contiguos: `Camera_ResetSmoothing_0434EA` (stub 12 B), `Camera_SmoothingIntegrate
_0434F8` (integrador con damping asimétrico y clamp saturado ±$80000, publica
delta hi-word en `$108182`, 104 B), y la **entry alternativa** `SceneLoader
_ByIndex_043562` (6 B, `clr.w d0; bra.w $43574` que salta al `move.l` INTERNO
de la función larga — patrón dual-entry hand-coded).

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Camera_ResetSmoothing_0434EA` | `$0434EA` | 12 | 1 (SceneLoader) | Stub reset simétrico de los dos slots long del smoothing accumulator de camera-Y en `$10817A/$10817E`. Referenciado por `bsr.w` PC-rel desde SceneLoader (`$0436A0`). Comparte idioma "reset explícito con clr.l abs.l" con Attract_SoftReset (FF#6). |
| 2 | `Camera_SmoothingIntegrate_0434F8` | `$0434F8` | 104 | ? | Integrador de smoothing camera-Y con damping asimétrico: si `\|v\| < $4000` aplica `v/8` (damping fuerte), si no damping nulo. Clamp saturado bilateral a `[-$80000, +$80000]` con 2 ramas `move.l #const, d2` inline (GCC habría factorizado). Publica delta hi-word en `$108182.w` (leído por Geom_Proj_Clamp FF#2 en fase 2). |
| 3 | `SceneLoader_ByIndex_043562` | `$043562` | 6 | 1+ | **Entry alternativa** a SceneLoader_Main. Fuerza `d0=0` y hace `bra.w $43574` para saltar al `move.l (a0, d0.w), $10815c.l` INTERNO de la función larga, saltándose los 3 `andi.w/lsl.w/lea` iniciales. Patrón dual-entry hand-coded (ya visto en Wave AA#3 Player_Dispatch). |
| 4 | `SceneLoader_Main_043568` | `$043568` | 374 | 8 | **Punto de entrada canónico de carga de escena.** Indexa `scene_table[idx]` en `$916C8`, publica `cfg_ptr` en `$10815C`, hace 2 pasadas sobre el mini-bytecode del entry_script (14 B/entrada, terminado en `type==2`), inicializa contexto de camera+HUD (25+ slots limpiados/publicados), y ejecuta 5 `jsr abs.l` finales a subsystem-init hooks. Absorbió `JsrAbsThunk_0436d6` (FP #30). |

**Falso positivo absorbido Wave HH batch 1 (1 nuevo, 30 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 30 | `JsrAbsThunk_0436d6` | I | `SceneLoader_Main_043568` (HH#1) | Cola `jsr $4CB5C.l; rts` del 5º subsystem-init hook |

**Descubrimientos arquitectónicos Wave HH batch 1:**

1. **Tabla `scene_table[256]` en `$916C8`** identificada como raíz del sistema
   de carga de escenas. Cada entrada de 8 B contiene `{void *config_ptr, Entry
   *entry_script}`. Los 256 índices sugieren que MSLUG1 tenía slots reservados
   para todo el catálogo de escenas (título, cutscenes, niveles, sub-áreas,
   game-over, etc.), aunque muchos probablemente son NULL. **Añadir a
   `include/mslug.h` como `extern SceneDescriptor scene_table[256];`**.

2. **Estructura `struct SceneEntry`** identificada por primera vez: 14 B por
   entrada, layout `{u8 type; u8 subop; u32 template_ptr; u8 payload[8]}`.
   El `type==0` invoca spawner "tipo A" (`$13982`) y `type==0xFF` (o cualquier
   otro no-cero) invoca "tipo B" (`$13952`); `type==2` es el terminador. Los
   templates apuntados por `template_ptr` son los mismos identificados en Wave
   Y#10/#11 (Entity_Build3ChainCircular/Entity_Build4FromTemplates).

3. **Convención "5 subsystem-init hooks al final del scene setup"** documentada.
   El orden fijo (`$7707C/$8F158/$3EE3A/$997B8/$4CB5C`) sugiere que MSLUG1
   tenía un macro `SCENE_INIT_HOOKS` que expandía la lista literal en ASM.
   Ninguna oleada futura debe intentar refactorizar estos 5 `jsr abs.l` en
   un loop sobre array de function-pointers — es literal.

4. **Idioma "damping asimétrico con umbral"** (`Camera_SmoothingIntegrate`)
   documentado por primera vez: `if (|v| < $4000) v /= 8;`. Es el corazón
   del smooth-follow del jugador — permite que la camera Y responda rápido
   cuando el jugador salta a distancia grande, pero se asiente suave cerca
   del target. Patrón replicable como macro `DAMP_ASYM d, threshold, shift`.

5. **Entry alt `SceneLoader_ByIndex_043562` cae al MEDIO de la función larga.**
   Esta es la segunda función del proyecto con entry-point que salta al
   interior de otra (la primera fue Wave AA#3). Refuerza la hipótesis de que
   MSLUG1 tenía múltiples "aliases" de scene-loader según el sitio de llamada
   (uno para "scene 0 hardcoded" y otro para "scene por índice").

### Wave HH en detalle (batch 2) — sub-helpers attract `$096xxx`

Cierra 6 sub-helpers del cluster attract que arrastran los 7 handlers de Wave
GG batch 1. `SelectPositive_TwoSlots` era el **top #3 de la cola** (6 callers,
uno por cada `ATTRACT_STATE` handler). Los cullers Cam0/Cam1 son clon
estructural byte-a-byte salvo por el slot de camera (`$106F50` vs `$106F5C`) —
tercer par de "clones no factorizados" del proyecto (tras Waves BB#2 y Z#5/#6).
Se recupera la **tercera tabla embebida en `.text`** (jump-table de 8 punteros
en `$096B9C`, tras StartInputTable BB#4 y state descriptors EE#1).

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `SelectPositive_TwoSlots_0967C0` | `$0967C0` | 62 | 6 | Min-magnitud componentwise sobre P1 (`$100440`) y P2 (`$1004E0`). Llama `$5E2D8` dos veces (una por slot), preserva P1 en `d2/d3`, y hace 4 tests `tst.l/bpl.w/move.l` para elegir por componente el valor positivo entre ambos jugadores. Semántica: "toma el jugador VIVO para calcular la coord del follower attract". |
| 2 | `AttractCuller_Cam0_0969C2` | `$0969C2` | 76 | 1 | Culler+blit de la lista de sprites attract (14 B/entrada, centinela `$FFFF`). Test de viewport contra `(camera0_x_hi + $140)`, blit vía `$5DCCE`. Loop hasta encontrar centinela. El `jsr $096B7E(pc)` a la jump-table es PC-rel 16-bit (4 B) — hand-coded, GCC habría emitido abs.l (6 B). |
| 3 | `AttractCuller_Cam1_096A0E` | `$096A0E` | 76 | 1 | **Clon byte-a-byte de Cam0** salvo por `$106F5C` (camera1) en lugar de `$106F50` (camera0). Tercer par de clones no-factorizados del proyecto. |
| 4 | `Viewport_CoordToScreen_096A5A` | `$096A5A` | 38 | ? | Conversión coord viewport → screen aplicando camera global. Doble `asr.l #8, dX` para extraer hi_word con sign-extend (GCC habría emitido `asr.l #16` en una sola instr). Y-flip Neo Geo (`$180 - Y`) idéntico al de Wave CC batch 1. |
| 5 | `DebugTriggers_TwoBits_096B24` | `$096B24` | 90 | 6+ | Dos triggers debug gated por DIP bits 2/7 en `$100001`. Cada uno: `probe1() && probe2() && Task_Alloc(template)`. Los dos bloques son literalmente idénticos salvo bit probado y template — macro `DEBUG_TRIGGER n, template` expandida 2x. Absorbió `JsrAbsThunk_096b76` (FP #31). |
| 6 | `ClampAndLookup8_096B7E` (+tabla) | `$096B7E` | 30+32 | 2 | Clamp asimétrico `d0 = min(d0, 7)` + fetch de puntero en jump-table embebida de 8 entradas (`$096B9C..$096BBB`). Devuelve handler en `a0` — el `rts` funciona como retorno de valor, no como salto indirecto. La tabla apunta a 8 handlers en `$096BBC..$0975A2` (sub-dispatcher attract pendiente). |

**Falso positivo absorbido Wave HH batch 2 (1 nuevo, 31 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 31 | `JsrAbsThunk_096b76` | I | `DebugTriggers_TwoBits_096B24` (HH#2) | Cola `jsr $4AE.l; rts` del segundo bloque debug-trigger |

**Descubrimientos arquitectónicos Wave HH batch 2:**

1. **Idioma `jsr d16(pc)` (opcode `4EBA`) vs `jsr abs.l` (opcode `4EB9`)**
   documentado. Los dos cullers (Cam0/Cam1) usan `jsr ClampAndLookup8_096B7E(pc)`
   con desplazamiento PC-rel 16-bit (4 B) porque el target está a `+438`/`+362`
   bytes del caller (dentro del rango ±32 KiB). GCC bare-metal siempre emite
   abs.l (6 B) porque no sabe que hay proximidad. **Solución técnica del
   matcher**: usar sintaxis explícita `jsr TARGET(pc)` en GAS. Añadir esta
   convención a los notes forenses del proyecto para futuras oleadas.

2. **Tercera tabla embebida en `.text` del proyecto**: `JumpTable_096B9C[8]`
   con 8 punteros de 4 B a handlers en `$096BBC..$0975A2`. La convención
   "tabla justo después del `rts` sin alineamiento adicional" es la misma
   que se vio en `StartInputTable_024FA6` (BB#4) y en los state descriptors
   de `$000BA2` (EE#1). GCC habría emitido `.section .rodata` separada.

3. **Convención `ClampAndLookup8` devuelve handler en `a0` con `rts`**: no
   es un `jmp indirecto` disfrazado — el caller (Cam0/Cam1) usa `a0`
   directamente como base de la deref `cmpi.w #$FFFF, (a0, d4.w)`. Es
   micro-optimización 68000: `rts` desde jsr ahorra el push+pop de un
   `jmp (a0)` intermedio.

4. **Sub-dispatcher attract pendiente de descubrir**: los 8 targets de la
   tabla `$096B9C[]` son handlers de un pipeline que hasta ahora no había
   emergido. Su rango (`$096BBC..$0975A2` ≈ 2 470 B) sugiere que la próxima
   oleada (Wave II) podría atacar el cluster completo como continuación
   natural de HH#2.

### Wave HH en detalle (batch 3) — FixLayer_QuadBatch + PlayerCtx_Reset

Cierra los 2 helpers restantes de la cola priorizada: `FixLayer_QuadBatch_046AC6`
(top #4, 4 callers, 90 B) y `PlayerCtx_ResetTwoBlocks_024FEC` (top #10, 3
callers, 38 B). Ambos son heterogéneos entre sí pero comparten el patrón
de "helper corto con muchos callers" ideal para cierre de oleada.

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `FixLayer_QuadBatch_046AC6` | `$046AC6` | 90 | 4 | 4 batches al backend fill-tilemap `$5DA9C` con parámetros fijos (a1, d0=tile $20, d1=cols, d2=rows). Batches #1/#2 pintan la barra HUD superior (dos mitades izq+der), #3/#4 pintan dos columnas laterales (izq y der). Es el "frame overlay" del HUD. Los 4 batches son la misma macro `FILL_FIXLAYER a1, d0, d1, d2` expandida sin factorizar. Absorbió `JsrAbsThunk_046b18` (FP #32). |
| 2 | `PlayerCtx_ResetTwoBlocks_024FEC` | `$024FEC` | 38 | 3 | Reset simétrico de dos slots player-context scratch (`$106EB0..$106EB5` y `$106EB6..$106EBB`, 6 B cada uno) con `clr.l (a0)+; clr.w (a0)`. Gate por `d0` (player_count): si `< 2` hace `bcs.w` a `$025012` (funcion vecina, no cerrada aquí = fall-through al pipeline multi-jugador), si `≥ 2` marca `ctx_mode = SINGLE` en `$106ECA` y retorna. |

**Falso positivo absorbido Wave HH batch 3 (1 nuevo, 32 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 32 | `JsrAbsThunk_046b18` | I | `FixLayer_QuadBatch_046AC6` (HH#3) | Cola `jsr $5DA9C.l; rts` del batch #4 (columna derecha) |

**Descubrimientos arquitectónicos Wave HH batch 3:**

1. **Layout Fix Layer HUD** identificado: el marco superior es 40 cols × 2
   filas dividido en dos mitades (`$7000` y `$701E`), y las columnas
   laterales son 1 col × 32 filas (`$7000` y `$74E0`). Los offsets
   confirman el layout estándar de tile-map 40×32 tiles del Neo Geo Fix
   Layer. El tile de fill `$20` es el índice del tile transparente/negro
   del `sfix.sfix` (verificable con extracción de sfix + visualización).

2. **Contador de callers de `$5DA9C` sube a 6+**: los thunks Wave I ya
   contabilizaban `JsrAbsThunk_05dbf8` y `JsrAbsThunk_09813c` (2 usos),
   más los 4 usos internos de FixLayer_QuadBatch, más los 12 usos ya
   identificados en Wave CC batch 2 (fix_blit_batch_046b20.s). Total
   ~18 callers → `$5DA9C` es el backend fill-tilemap CANÓNICO del Fix
   Layer del proyecto. Excelente candidato para Wave II próxima oleada.

3. **Convención "fall-through hacia función vecina como salida alternativa"
   reconfirmada por 5ª vez** (Waves T, DD, EE#3, GG#2, HH#3). El `bcs.w
   $025012` de PlayerCtx_ResetTwoBlocks salta a otra función distinta pero
   contigua en el ROM. Esta convención implica que MSLUG1 usaba `.text`
   sin alineamiento entre funciones (densidad byte-a-byte confirmada) y
   que las "salidas alternativas" son el idioma preferido para representar
   returns con estados distintos sin duplicar código.

### Wave GG en detalle (batch 1) — cluster attract state handlers `$096xxx`

Ataque a los 7 targets concretos de `Dispatcher_ModeTable_001922` (EE#3):
son los handlers apuntados por `move.l #$967FE/$96840/$96882/$968C4/$96906/$96948/$9698A, ($100260)`,
que EE#3 publica cuando decodifica el estado attract. Con Wave GG cierran
el circuito completo del subsistema attract/title.

| # | Símbolo | Dirección | Bytes | State | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Attract_State0_Handler_0967FE` | `$0967FE` | 66 | 0 | Full template (`move.b #n, d0; move.b d0, $21(a6); jsr $43568; clr flag $10e39e; clr $70(a6); lea self+2, a1; move a1,(a6); jsr $967c0; jsr $436de; jsr $969c2; jsr $96a80; jsr $96b24; set $10e39a; rts`). |
| 2 | `Attract_State1_Handler_096840` | `$096840` | 66 | 1 | Idem con `n=1`. |
| 3 | `Attract_State2_Handler_096882` | `$096882` | 66 | 2 | Idem con `n=2`. |
| 4 | `Attract_State3_Handler_0968C4` | `$0968C4` | 66 | 3 | Idem con `n=3`. |
| 5 | `Attract_State4_Handler_096906` | `$096906` | 66 | 4 | Idem con `n=4`. **3 refs** de EE#3 (states 4, 8, 9 comparten este handler). |
| 6 | `Attract_State5_Handler_096948` | `$096948` | 66 | 5 | Idem con `n=5`. |
| 7 | `Attract_State7_Handler_09698A` | `$09698A` | 56 | 7 | Template **compacta**: omite `move.w #$0, $70(a6)` y `jsr $969c2(pc)`. Solo 8 instr semánticas vs 12 del full template. |

**Sin falsos positivos absorbidos en Wave GG batch 1** (el cluster estaba limpio, ninguna cola registrada como thunk independiente).

**Descubrimientos arquitectónicos Wave GG batch 1:**

1. **Circuito completo del subsistema attract cerrado.** Con EE (dispatcher-tabla) + FF (backbone + handlers + geom_proj) + GG (7 state handlers concretos) tenemos las tres capas completas: `Dispatcher_ModeTable_001922` (EE#3) → tabla `$100260` → uno de estos 7 handlers → tail común (`jsr $967c0 + $436de + $96a80 + $96b24`).

2. **Macro `ATTRACT_HANDLER n [, no_timer_reset, no_969c2_call]` documentada.** 7 handlers estructuralmente idénticos con parámetros distintos = expansión de macro ASM. La macro admite dos parámetros opcionales que sólo el handler #7 ejerce. GCC nunca emitiría 7 copias literales de 66 B con diferencias mínimas.

3. **Patrón "self-loop handler" reconfirmado.** El `lea $<self+2>(pc), a1; move.l a1, (a6)` reinstala el propio handler como continuación del task, formando un bucle sobre sí mismo hasta que otro evento cambie el estado. Ya visto en Wave EE#3 (dispatcher-tabla), reafirmado aquí en la implementación de cada estado.

4. **Case-sensitivity `ThunkTarget_096a80` reconfirmada como convención GAS**. GAS m68k con `--register-prefix-optional` es case-sensitive en símbolos externos. El proyecto usa minúsculas para `ThunkTarget_XXXXX` según la práctica de `symbols.py`. Documentado en el `.s` para evitar la ronda de link-fail que hemos tenido en 3 oleadas consecutivas (EE, FF, GG).

5. **Primera confirmación explícita de 7 callers de Geom_Proj_Clamp**. Los 7 handlers son 7 de los 13 callers identificados en la topología FF#2. Los 6 restantes viven en `$08Cxxx` (batch 2).

### Wave GG en detalle (batch 2) — máquina de estados de animación `$08Cxxx`

Ataque a los 6 callers restantes de `Geom_Proj_Clamp_0436DE` (FF#2).
Cluster homogéneo con encadenamiento por "self-replace handler" a lo
largo de una máquina de estados F1 → F2 → F3 → F4 → F5 → F6 con LUT
lookup, timer decrement y colas comunes.

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `Anim_State_F1_08C008` | `$08C008` | 342 | Initializer: sprite setup completo (position `$A0/$1C3`, flags `$4000`, `bset #6, $12(a6)`), 6 task-adds via PC-rel `lea` con templates `$8C2B8/322/37E/3DA/436/5B2`, publica `$335A6` en slot P1 (`$100440`), y arranca LUT-driven state machine con clamp `$FF`. Transita a F2 cuando `$34(a6) > $3F`. |
| 2 | `Anim_State_F2_08C15E` | `$08C15E` | 78 | LUT sobre `$2C072C[$34(a6)*2]`, actualiza `$32(a6)`, incrementa `$34` por 4. Cuando `$34 > $3F`, publica `$32 = $FF` y transita a F3. |
| 3 | `Anim_State_F3_08C1AC` | `$08C1AC` | 62 | Probe `Sub_0008BC74` (CCR-C); si `C=1` llama MMIO blitter `$5DA9C(#$7084, #$2320, #$20, #$19)` (VRAM $7084, tile-id, W×H), reset `$34=0`, transita a F4. |
| 4 | `Anim_State_F4_08C1EA` | `$08C1EA` | 80 | LUT sobre `$2C07AC[]` con bias `-1` (bcc/clr), fase decrement. Transita a F5. |
| 5 | `Anim_State_F5_08C23A` | `$08C23A` | 92 | LUT sobre `$2C07AC[]` para `$33(a6)`. Al terminar (`$34 > $3F`): dispara SFX `Sub_00002308(#$80)` + fade `ThunkTarget_05239e(#2)` + timer `$70=$3C`, transita a F6. |
| 6 | `Anim_State_F6_08C296` | `$08C296` | 34 | Final: decrementa `$70(a6)`, cuando ≤0 clear `$106ED2`. Tail acortado (2 jsr en lugar de 3). |

**Falsos positivos absorbidos Wave GG batch 2 (6 nuevos, 37 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 32 | `JsrAbsThunk_08c156` | I | `Anim_State_F1_08C008` (GG#B/1) | Cola `jsr $28D70.l; rts` tail F1 |
| 33 | `JsrAbsThunk_08c1a4` | I | `Anim_State_F2_08C15E` (GG#B/2) | Cola `jsr $28D70.l; rts` tail F2 |
| 34 | `JsrAbsThunk_08c1e2` | I | `Anim_State_F3_08C1AC` (GG#B/3) | Cola `jsr $28D70.l; rts` tail F3 |
| 35 | `JsrAbsThunk_08c232` | I | `Anim_State_F4_08C1EA` (GG#B/4) | Cola `jsr $28D70.l; rts` tail F4 |
| 36 | `JsrAbsThunk_08c28e` | I | `Anim_State_F5_08C23A` (GG#B/5) | Cola `jsr $28D70.l; rts` tail F5 |
| 37 | `JsrAbsThunk_08c2b0` | I | `Anim_State_F6_08C296` (GG#B/6) | Cola `jsr $436DE.l; rts` tail F6 |

**Descubrimientos arquitectónicos Wave GG batch 2:**

1. **6 FPs simultáneos en un mismo cluster es el récord del proyecto.** El escáner Wave I registró cada cola `jsr $XXX.l; rts` de los 6 handlers como thunks independientes. La absorción confirma que Wave I sistemáticamente sobre-cuenta los tail-calls comunes. Es el mismo fenómeno visto en clusters Y, Z, DD, EE — pero aquí a mayor escala (6 de 6 handlers absorbieron una cola cada uno).

2. **Máquina de estados F1→F6 con "self-replace handler" en cadena.** Cada estado se auto-reemplaza en `(a6)` por el siguiente cuando su condición de salida se cumple. El scheduler central (`Scheduler_MainLoop_000656`, Wave Y#1) llama al puntero en `(a6)` cada frame, y el mismo handler cambia el puntero al siguiente estado sin re-encolar. Es la implementación del patrón State pattern en 68000 puro.

3. **LUT-driven animation identified.** Las tablas `$2C072C` y `$2C07AC` son lookup tables word-indexed por `$34(a6)*2`. `$2C072C` tiene rango con clamp a `$FF`, `$2C07AC` con bias `-1`. Modelan probablemente **curvas de brillo/fade** para el "attract animation" que se muestra al iniciar el juego. La transición F5 con `SFX $80 + fade #2 + timer $3C` sugiere que es el "reveal effect" del logo de Metal Slug.

4. **Slot P1 (`$100440`) confirmado como registro del handler player.** F1 publica `$335A6` en ese slot, tal como `Attract_DoubleCheck_400_Publish_001846` (FF#7) publicaba `$2575C/$25766` en los slots `$100300/$1003A0`. Los tres slots forman parte del mismo sistema de "player entry handlers".

5. **MMIO blitter `$5DA9C(a1_vram, d0_tile, d1_w, d2_h)` prototipo inferido.** F3 lo llama con VRAM `$7084`, tile-id `$2320`, W=$20, H=$19 — parámetros consistentes con un `sprite_blit_rect(vram_addr, tile_id, width, height)`. Confirma la firma ya sospechada en Wave S (`Fix_BlitRectToFixLayer`) y Wave T (`Sprite_MultiBlitClippedX`).

### Wave FF en detalle (batch 2)

Ataque a `$0436DE` (helper geometrico con 13 callers desde `$08Cxxx` y
`$096xxx`), la funcion mas alta prioridad NO-attract de la cola.

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Geom_Proj_Clamp_0436DE` | `$0436DE` | 252 | 13 | Helper geometrico de proyeccion 2D con clamp `[-$80000, +$80000]`. Estructura: 2 entradas efectivas (fase X + fase Y) que comparten sub-rutina interna `.Lreduce` (invocada por `bsr.w` desde `$4371E` y `$43742`). Aritmetica 32-bit con `swap`/`clr.w` (shift-left-16 idiomatico) + `exg d0,d2` / `exg d1,d3` / `exg d1,d0` (reordenar sin RAM) + `push d7; asr.l #1; add.l (a7)+` (ceildiv sin link/unlk). Dos `rts` consecutivos (`$437D4` valor calculado, `$437D8` overflow). Cae por diseño en `$0437DA` (siguiente funcion del ROM). Absorbio `Ret0_000437D6` (FP #31). |

**Falsos positivos absorbidos Wave FF batch 2 (1 nuevo, 31 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 31 | `Ret0_000437D6` | B/Ret | `Geom_Proj_Clamp_0436DE` (FF batch 2) | Cola `moveq #0, d0; rts` de la rama overflow (`.Loverflow`) |

**Descubrimientos arquitectónicos Wave FF batch 2:**

1. **Pipeline geometrico identificado.** `Geom_Proj_Clamp_0436DE` es el helper de proyeccion que consumen 13 funciones desde `$08Cxxx` (bloque de sprites de fondo/parallax) y `$096xxx` (bloque de proyectiles con parallax). Su firma toma dos rectangulos (`d0/d1` = pos, `d2/d3` = size) y devuelve la posicion proyectada en pantalla tras aplicar la camara actual (`$108164` = camera_x, `$108166` = camera_y con offset `$10817A`) y clamp `[-$80000, +$80000]`. Es un pilar del sistema de render.

2. **Idioma `swap; clr.w` documentado por primera vez.** GCC usaria `lsl.l #16, dX` (4 B por reg) o cambiaria el tipo del temporal. El codigo hand-coded emite `swap dX; clr.w dX` (4 B tambien, pero mas legible como "convertir word alta a long alineado"). Aparece 5 veces consecutivas en `.Lreduce` sobre `d0/d2/d4/d5/d6`. Es una firma clara de ensamblador escrito a mano.

3. **`exg` sin sufijo confirmado como sintaxis obligatoria GAS m68k.** GAS `--register-prefix-optional` rechaza `exg.l dX,dY` con "Unknown operator". La instruccion `EXG` es intrinsecamente 32-bit en 68000 y GAS no acepta sufijos. Documentado en el `.s` como convencion del proyecto (afecta a cualquier oleada futura que use `exg`).

4. **`ceildiv(x, 2)` sin link/unlk.** El fragmento `move.l d7, -(a7); asr.l #1, d7; add.l (a7)+, d7` en `$043782..$043788` calcula `d7 + d7_original/2` (equivalente a `d7 * 1.5`) usando la pila como scratch temporal SIN entrar en un stack frame. GCC en 68000 usaria `link a6, #-N` + `move` + `unlk`, un overhead grande. El original ahorra 6 bytes con este patron.

5. **Falso positivo `Ret0_000437D6` idéntico al patrón `Ret0` de Wave B.** Confirmacion adicional de que la Wave B (return-constant stubs) sistematicamente barre colas de funciones grandes como si fueran funciones independientes. Es el 15to Ret0/Stub absorbido en el proyecto.

## Repack a formato MAME (Wave FF)

Wave FF cierra tambien la **herramienta de repack** solicitada, que permite
regenerar un `201-p1.bin` cargable por MAME/FBNeo/RetroArch a partir de la
P-ROM procesada del build. La transformacion es la involucion inversa de
`scripts/setup.sh` (byte-swap word-a-word + bank-swap de 1 MiB).

### `scripts/pack_prom.sh`

Uso tipico:

```bash
$ bash scripts/pack_prom.sh
[REPACK] P ROM in : 816b3f74c76b3373993407615f1850fe   (build/mslug_prom.bin)
         MAME out : b6804bc6be580c80d43d187f6f9d2e7c   (build/repack/201-p1.bin)
         size     : 2,097,152 bytes (2048 KiB)
```

Los dos MD5 quedan documentados oficialmente:

- **`b6804bc6be580c80d43d187f6f9d2e7c`** = ROM formato MAME (cartucho, entrada de `setup.sh`, salida de `pack_prom.sh`).
- **`816b3f74c76b3373993407615f1850fe`** = P-ROM procesada (formato CPU, salida de `setup.sh`, entrada de `pack_prom.sh`, referencia byte-a-byte del matcher).

### Bug fix `scripts/setup.sh`

Detectado durante Wave FF y corregido: la version historica comparaba la
salida contra un MD5 hardcoded (`816b3f74...`) pero el mensaje decia
"expected 816b3f74..." incluso cuando la ROM cruda ya tenia ese mismo
MD5 (i.e., el usuario ya trajo la P-ROM procesada). El script ahora:

1. Detecta si la entrada YA es la P-ROM procesada (MD5 `816b3f74...`) y
   la copia directamente sin volver a aplicar el swap (que la
   corromperia).
2. Documenta ambos MD5 (entrada y salida) para eliminar la ambiguedad
   historica que confundio a la orquestacion en la Wave EE→FF.

Ambos flujos ahora funcionan igual:

```bash
# Caso A: usuario tiene ROM cruda (formato MAME, b6804bc6...)
$ cp mame_dump/201-p1.bin rom/ && bash scripts/setup.sh
[OK] input  MD5 = b6804bc6...
       output MD5 = 816b3f74...

# Caso B: usuario tiene P-ROM ya procesada (formato CPU, 816b3f74...)
$ cp ya_procesada.bin rom/201-p1.bin && bash scripts/setup.sh
[NOTE] input ya es formato CPU (MD5=816b3f74...)
       copiando directamente sin re-swap
```

### Flujo end-to-end verificado (Wave FF)

Round-trip completo probado con ROM MAME real:

```
MAME 201-p1.bin  ──setup.sh──▶  build/mslug_prom.bin  ──match_batch──▶  3077/3077 OK
     │                              │                                    35210 B
     │                              │                                    1.6789 % ROM
     ▼                              ▼
b6804bc6...                    816b3f74...
     ▲                              │
     │                              │
     └──── pack_prom.sh ────────────┘   (involucion perfecta, cmp = IDENTICAL)
```

Verificado con dos ROMs MAME independientes: `mslug.zip` original y una
segunda copia (ambos con MD5 `b6804bc6...`) — ambos re-generados por
`pack_prom.sh` byte-a-byte identicos a los originales.

### Wave FF en detalle (batch 1)

Cierra el cluster attract/title `$001260..$001AF7` iniciado en Wave EE.
9 handlers en un solo fichero `asm/attract_cluster_batch_ff.s` porque
todos comparten firma arquitectonica (registrados en la tabla de
descriptores `$000BA2..$000E8A`).

| # | Símbolo | Dirección | Bytes | Refs de tabla | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Attract_InitBIOS_001744` | `$001744` | 132 | 2 (+3 al probe) | Post-BIOS init con dos entradas: primaria hace `BIOS_FIX_CLEAR + palette fade + task-adds $46682/$59B6A + seal $106ECE/CF/D2 + probe $1E0A + tail`. Secundaria (`$00179A`) hace probe path con gate sobre `$106ED2` (armed → `rts` inmediato; libre → `jsr $5B6/$2B58/$212E + tail`). |
| 2 | `Attract_InitTaskAdd_3DBC8_0017C8` | `$0017C8` | 30 | 12 | Handler minimo mas usado del cluster: seal `$106ED2=$FF`, task-add `$3DBC8`, init pesado `$46AC6`, tail. |
| 3 | `Attract_InitShow27_TaskAdd_0017E6` | `$0017E6` | 44 | 1 | Dispara opcode `$27` sobre `InputGuardCall219c` (Wave A#4) — primer caller no-thunk registrado, task-add `$46608`, marca `$21(a6)=$FF`. |
| 4 | `Attract_SetTimers2_And_Gate21_001812` | `$001812` | 26 | 1 | Probe + timers `$45=$2, $44=$2` + gate `$21(a6) != 0 → rts`. Fall-through a #5. |
| 5 | `Attract_TailChain_1CD4_1DA4_00182C` | `$00182C` | 12 | fall-through | Encadena `PcThunkTarget_001CD4 + Sub_00001DA4 + tail`. Continuacion natural de #4. |
| 6 | `Attract_SoftReset_10FDAF_001838` | `$001838` | 14 | 3 | Thunk tail-call: `move.b #1, $10FDAF; jmp $85E.l` (SoftReset del BIOS). |
| 7 | `Attract_DoubleCheck_400_Publish_001846` | `$001846` | 68 | 12 | Handler mas referenciado. Doble-check `$100300==$400` y `$1003A0==$400`, publica `$2575C/$25766` cuando aplica. Patron `cmpi.l #$400, addr.l` de 10 B/lado incompatible con GCC. |
| 8 | `Attract_WaitStateBackbone_00188A` | `$00188A` | 80 | ≥11 `bra.w` | **Backbone comun** del cluster. Target de todos los `bra.w $188A` de `Dispatcher_ModeTable_001922` (EE#3) y del path init de F2. Emite el bucle "wait state loop" completo (`InputQueue + palette fade + task-adds + seal + BIOS_FIX_CLEAR`) y termina con `.byte 0x4e, 0xb9` (opcode `jsr abs.l` sin operando — los 4 bytes del operando son los bytes literales al inicio de `Init_EntitySpawn_0018DA` de Wave EE#2, formando la instruccion completa `jsr $46AC6.l` a nivel fisico). Fall-through a EE#2 offset +4. |
| 9 | `Attract_PostStart_Cleanup_001AB6` | `$001AB6` | 66 | fall-through | Cleanup post-Start alcanzado desde `.Lone_path` de EE#3. Probe bit-0 de `$100001`, `jsr $5D288`, repatch de `(a6)` (handler del task actual) con `$F76(pc)`, gates `$106ED6/D2`. Absorbio `ClrRamWord_001af0` (FP #30). |

**Falsos positivos absorbidos Wave FF batch 1 (1 nuevo, 30 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 30 | `ClrRamWord_001af0` | N | `Attract_PostStart_Cleanup_001AB6` (FF#9) | Cola `clr.b $106ED6; rts` de la rama `.Lff12_clr_and_rts` |

**Descubrimientos arquitectónicos Wave FF batch 1:**

1. **Instrucción compartida entre F9 y EE#2** (idioma "operand-splicing"). El `jsr $46AC6.l` que cierra el backbone F9 tiene sus 2 bytes de opcode (`4E B9`) al final de F9 y sus 4 bytes de operando (`00 04 6A C6`) al inicio de EE#2. Es la primera vez que documentamos oficialmente este idioma en el proyecto: dos secciones `.text.X` distintas contienen mitades de una misma instruccion 68000. GCC no puede emitir esto porque cada funcion tiene su propia seccion. La construccion requirio dividir F9 en 80 B (14 instr semanticas + 2 B de opcode literal `.byte 0x4e, 0xb9`) y aceptar que EE#2 arranca con 4 B literales que no son opcode valido en si mismo pero son operando de la instrucción física final de F9.

2. **`Attract_WaitStateBackbone_00188A` es el corazon del subsistema attract.** Con 11+ ramas entrantes desde EE#3 y contando la ruta de F2, es el codigo mas caliente del subsistema. Su sequencia fija (`InputQueue → snapshot state → palette fade → publish → task-adds → seal → BIOS_FIX_CLEAR → jsr init pesado`) es la operacion de "quiesce" que se ejecuta al final de cada transicion de estado attract.

3. **Handler `$001846` es el "gate del gameplay start"**: sus 12 refs de tabla y su logica (activar handler `$2575C/$25766` cuando el slot P1/P2 alcanza `$400`) sugieren que es donde se decide el "P1 press start" -> spawn player. Combinado con los flags `$10FDB6/B7` (Wave BB1) y el analisis dual en EE#3 rama post-Start, tenemos la ruta completa Start-button → attract-exit → gameplay-enter.

4. **Renombrado `Label_00188A → Attract_WaitStateBackbone_00188A`** propagado a `symbols.py` (1 entrada) y a `asm/dispatcher_mode_table_001922.s` (13 usos). Consolidacion natural que refleja el hallazgo arquitectonico.

5. **`Attract_InitShow27_TaskAdd` (FF#3) es el primer caller no-thunk registrado de `InputGuardCall219c` (Wave A#4).** Confirma que la funcion Wave A que hasta ahora solo tenia callers desde thunks (Waves H/I) es realmente parte del pipeline de arranque del subsistema attract. El opcode `$27` sera un dato a rastrear en waves futuras.

### Wave EE en detalle (batch 1)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Init_ModeToggle_001260` | `$001260` | 148 | tabla `$000BA2` | Handler de un slot del scheduler. Alterna dos submodos por bit-0 de `$10007B` (mode-tick): even → `jsr $24FEC(0); jmp $1940(pc)`, odd → `jsr $24FEC(1); jmp $199A(pc)`. Fallback (bit-0 no matcheado) hace `bsr $1DB8; tst $106ED2` con bifurcación reset-vs-continue. Contiene el patrón "sentinel `move.w #$FFFF, d0`" que GCC no emite (usaría `moveq #-1`). Absorbió `JsrPcThunk_0012ee` (FP #28). |
| 2 | `Init_EntitySpawn_0018DA` | `$0018DA` | 72 | tabla `$000C8A/C8E/CAA/...` | Spawner mini-script. Tres `lea imm.l, a1; jsr $4AE` consecutivos (task-adds a `$28DB6A`/`$29588`/`$469E2`) todos en forma absoluta larga en lugar de PC-rel — delata expansión de macro `SPAWN_CHAIN`. Convención hand-coded: `jsr $4AE` deja el nuevo task-node en `a0`, luego `move.b #$0D, $98(a0)`. Tail al scheduler vía `bra.w $FE0`. |
| 3 | `Dispatcher_ModeTable_001922` | `$001922` | 402 | tabla `$000C8E..$000E12` | **Dispatcher-tabla de 11 estados** (0..A) del subsistema attract/title. Bloques de 30 B estructuralmente idénticos publican opcode en `$106ECE` + puntero de handler en `$100260` + `jsr $5FE` + `bra.w $188A`. Estado #6 lleva `scheduler_add($1B4C)` extra. Estados #4/#8/#9 apuntan al mismo handler `$96906` — imposible refactor GCC. La rama post-Start `$001A64` hace snapshot dual `$10FDB6/B7` en `d0/d1` con `(a4)+` auto-inc, y bifurca por combinaciones (2→`jsr $981FC; rts`; 1→fall a `$001AB6`; 0→publica `$70(a6) = $E42` + tail a `$FC6`). Absorbió `JsrAbsThunk_001aae` (FP #29). |

**Falsos positivos absorbidos Wave EE batch 1 (2 nuevos, 29 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 28 | `JsrPcThunk_0012ee` | J | `Init_ModeToggle_001260` (EE#1) | Cola `jsr $1AF8(pc); rts` de la rama `.Ltail_pending` |
| 29 | `JsrAbsThunk_001aae` | I | `Dispatcher_ModeTable_001922` (EE#3) | Cola `jsr $981FC.l; rts` de la rama `.Ltwo_path` (post-Start) |

**Descubrimientos arquitectónicos Wave EE batch 1:**

1. **Tabla de descriptores de estado en `$000BA2..$000E4A` identificada como raíz del subsistema attract/title.** Cada fila es un descriptor de 4-tupla long con punteros a `$001260/$001812/$001838/$001846/$001922/$0018DA/$001A64/$001A70` etc. — 18 punteros a labels distintos dentro del cluster `$001260..$001AB5`. Wave EE-B (siguiente batch) tendrá que atacar los tres dispatchers `$000BF2/$000C6A/$000C9A` que leen esta tabla para completar el subsistema.

2. **Backbone comun `Label_00188A`** confirmado como cabecera del "wait state loop" del cluster: los 11 estados de `Dispatcher_ModeTable_001922` terminan con `bra.w $188A`, y adicionalmente reciben `bra.w $188A` desde el interior de `$001744` (una función vecina EE-B). Es el equivalente en zona baja del `bra.w $fe0` del scheduler general — un mini-loop de espera propio del subsistema attract.

3. **Nuevo idioma "macro con parámetro opcional" detectado.** El estado #6 del dispatcher (`$0019D6`) inserta un `lea $1B4C(pc); jsr $4AE` extra antes del bloque común. Los demás 10 estados no lo tienen. La única forma de explicar esta variación sin factorización compartida es una macro ASM con parámetro opcional: `STATE_PUBLISH n, handler [, aux_handler]`. Consistente con la hipótesis general del proyecto: MSLUG1 se ensambló con macros pesadas.

4. **Convención `a0 = new task node` post-`jsr $4AE` reconfirmada por tercera vez.** Ya vista en Wave DD (`Task_FreeListInit_000410`) y Wave Y (`Init_MasterSubsystems_0020E2`), aquí `Init_EntitySpawn_0018DA` la explota literalmente: encadena tres `scheduler_add`, el último deja `a0` apuntando al nuevo nodo y publica `move.b #$0D, $98(a0)` directamente. **Añadir a `include/mslug.h` como comentario del prototipo de `Task_AllocFromFreeList` (T#4).**

5. **Segundo pipeline detectado post-Start.** La rama `$001A64..$001AB5` del dispatcher tabla implementa un análisis dual sobre `$10FDB6/B7` (las máscaras públicas de Start P1/P2 que Wave BB batch 1 ya identificó como salida de `Player_Start_Inner_024E76`). Es la conexión directa entre el pipeline de Start del hardware y el subsistema attract: cuando P1/P2 dispara `$1` la fase avanza (`.Lone_path` → fall a `$001AB6`), cuando dispara `$2` se hace snapshot dual + `rts` inmediato (`.Ltwo_path`). Se completa un eslabón arquitectónico clave del arranque del juego.

### Wave DD en detalle

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Clipping_Test_0999DE` | `$0999DE` | 24 | 2 | Test rápido con **retorno CCR-C**. Camino "accepted" via `andi.b #$FE, ccr; rts` cuando `nibble_bajo($6C(a6)) == $F`. Callee identificado en Wave CC (Handler_ApplyCameraSelf/Globals). Absorbió `ClearC_0999f0` (Wave F FP #27). |
| 2 | `Input_RisingEdgeSnapshot_05CC0E` | `$05CC0E` | 186 | 1 | Snapshot dual P1/P2 (6B c/u) desde `$106EB0/$106EB6` a `$10E200/$10E206`, detección **rising-edge** clásica (`eor;and`) sobre latch `$10FDAC`, ramificación por máscaras `$10FDB6/B7`: mask=0 IDLE skip, mask=3 clear, default copy. |
| 3 | `VRAM_FixLayerAutoclear_05A824` | `$05A824` | 150 | 1 | Reset masivo VRAM Fix Layer via autoinc port `$3C0000/$3C0002/$3C0004`: 381 words en banda estática `$8200`, 24 384 words en banda sprites `$0000` (con NOPs de timing entre writes), reset flags `$10E1F6/F8/FA` + sentinels `$FFFF` en slots `$108080+$614C/$6158`. |
| 4 | `VBlankTick_Master_001E5E` | `$001E5E` | 152 | 1 | Handler maestro del tick VBlank: publica `$4` a autoanim VDP `$3C000C`, incrementa `$106EDD/D9` (frame counters), poke a I/O output `$300001`, `jsr $226A` (Input_QueuePush), dispatch de heavy work (`jsr $137C6` + `$1EFE(pc)` + `$5C9D6`) o skip via `$106EE2` counter. |
| 5 | `Task_FreeListInit_000410` | `$000410` | 158 | 1 | Inicializador del scheduler: FASE A crea 160 nodos de `$A0 B` en `$100A80..` con `movea.l a0,a1` dentro del loop + doble `jsr $5DC1C/34` con `movem.l d0-d7/a0-a6, -(a7)`; FASE B lee 32 descriptores desde `$278000` y publica en `$100940`. Complementa Wave R (Scheduler_MainLoop) instalando los nodos que el scheduler consumirá. |

**Falso positivo #27 absorbido**: `ClearC_0999f0` (Wave F, 6 B) → cola `andi.b #$FE, ccr; rts` del camino "accepted" de `Clipping_Test_0999DE`. Total FPs del proyecto: **27**.

**Descubrimientos arquitectónicos Wave DD:**

1. **Pipeline de input completo reconstruido**: `Input_RisingEdgeSnapshot_05CC0E` establece el patrón clave `(new & ~old)` para detección de botón recién presionado en 3 instrucciones (`eor.b d0,d1; and.b d0,d1; move.b d1, +1(a1)`). Combinado con `Player_Start_Inner_024E76` (Wave BB1) y `Input_QueuePush` (Wave A/S), tenemos las 3 capas del pipeline de input del juego.

2. **Protocolo VRAM autoinc Neo Geo documentado**: `VRAM_FixLayerAutoclear_05A824` expone la triada de puertos MMIO clave: `$3C0004` = modo autoinc, `$3C0000` = registro de dirección VRAM, `$3C0002` = puerto de datos autoinc. **NOPs intercalados** en el bucle largo (12 192 iter) son idioma de sincronización con timing hardware VDP durante VBlank — imposible en GCC.

3. **Ancla del scheduler encontrada**: `Task_FreeListInit_000410` es el punto donde `$106E80` recibe el head del free-list y donde se enlazan los primeros 160+32 nodos. Descubre que el pool secundario `$100940` almacena las 32 tareas iniciales del sistema.

4. **Tabla de descriptores de tareas en `$278000`**: 32 entradas de 8 B cada una = 256 B de tabla en zona probablemente clasificada como "código" en la estimación pero que en realidad son **datos-de-configuración**. Cada descriptor lleva `{handler_ptr, task_id_ptr}`. Candidato natural a promover a símbolo `InitialTaskTable_278000` en un pase futuro.

5. **Ecuación del VBlank frame budget resuelta**: `VBlankTick_Master` implementa un **patrón de heavy-skip** típico de arcades. Cada frame incrementa contadores; si `$106ED8 == 1` significa que el heavy work del frame anterior aún está pendiente y salta el trabajo (solo `$106EE2++`). Es un throttling manual sin usar interrupciones.

### Estimación de decompilación restante

Análisis de densidad de opcodes `rts/rte/rtr` word-aligned por ventanas de 4 KB sobre toda la P-ROM (2 MiB, 512 ventanas):

```
Total P-ROM              : 2,097,152 B  (100.00%)
Bytes decompilados       :    33,890 B  ( 1.62%)  ← estado actual Wave DD
Estimación código total  :   528,384 B  (25.20%)  ← 129 ventanas 4KB con ≥3 rts
Código pendiente         :  ~494,494 B  (23.58%)
Datos (sprites/tablas)   : ~1,568,768 B ( 74.80%)  ← no requieren decomp de código
```

**Distribución del código real en la ROM:**
- `$000000..$09FFFF` (640 KB): **bloque compacto de código** — engine + arranque + game logic.
- `$0A0000..$17FFFF` (896 KB): **datos** (sprites, tilemaps, level layouts, tablas grandes).
- `$180000..$180FFF` (4 KB): **micro-isla de código** aislada (probablemente tabla de punteros mixta).
- `$190000..$1FFFFF` (448 KB): **datos puros**.

**Cobertura relativa al código real (no al ROM total):**
```
33,890 / 528,384 = 6.42 % del código estimado ya decompilado
                    ~93.6% del código pendiente por atacar
```

La cifra "1.62 %" es engañosamente baja porque **el 74.8 % de la P-ROM son datos** que no requieren decompilación de código C/ASM. En términos de **código real ejecutable**, ya está cerrado un **6.4 %**.

### Wave CC en detalle (batch 2)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Handler_ApplyCameraSelf_0440E4` | `$0440E4` | 158 | 1+ | Ruta larga con clipping y auto-desplazamiento sobre `$22/$24(a6)`. Gate re-entrancia por `$13(a6) bit 6`, `jsr $999DE` (Clipping_Test CCR-C), rama aceptada (`sub cam.x/add cam.y`) o rechazada con **signo-XOR compacto** (`sgt/spl/eor/beq`), y chequeo de override por `$106ECE`/`$106F5E`. |
| 2 | `Handler_ApplyCameraGlobals_044182` | `$044182` | 168 | 1+ | **Clon estructural gemelo** de #1 sobre coords globales `$106F38/$106F3A`. 168 B vs 158 B por `move.w abs.l` (6B) vs `move.w (a6)` (4B). |
| 3 | `FixBlit_BatchRow4x2_046B20` | `$046B20` | 186 | 1 | Blitter MMIO unroll físico de 8 tiles (4x2) al Fix Layer + bucle exterior de 16 iteraciones que hace `bls.w` a #4. `asl.w #5, d2` para pack coord Y en VRAM addr. |
| 4 | `FixBlit_BatchRow4x1_ColorInc_046BDA` | `$046BDA` | 110 | 1+ | Blitter MMIO fila 4 tiles con **incremento de atributo `+$10` por tile** (gradiente de color). Termina con `bra.b` backward al medio de #3: **primer caso de bucle mutuo entre funciones vecinas** del proyecto. |

### Wave CC en detalle (batch 1)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `FixBlit_TileByCoord_043F5E` | `$043F5E` | 78 | 1+ | Blitter tile individual al Fix Layer con pack coord→VRAM (idioma Neo Geo `andi/rol/ror` con máscaras `$1F0/$1F8/$7/$8`). |
| 2 | `Coord_LocalToScreen_04400E` | `$04400E` | 20 | 1 | Conversor local→pantalla con **Y-flip Neo Geo** (`neg.w d1; addi.w #$200, d1`) usando cámara scroll rápido `$106F70/$106F74`. |
| 3 | `Coord_ScreenToLocal_044022` | `$044022` | 20 | 1 | Inverso exacto de #2. |
| 4 | `Coord_ResetAndClearBuf_044036` | `$044036` | 18 | 1 | Resetea cámara principal + `bsr` a #12 para limpiar 4 KB. |
| 5 | `Coord_ApplyCameraDeltaToSelf_044048` | `$044048` | 22 | 1 | Aplica delta cámara principal a coords `$22/$24(a6)` (`self->x -= cam.x`, `self->y += cam.y`). |
| 6 | `Coord_ApplyCameraDeltaToA0Mark_04405E` | `$04405E` | 28 | 1 | Variante de #5 con target `a0` + marca bit 6 de `$13(a0)` = "coord procesada". |
| 7 | `Coord_ApplyCameraTerciaryToSelf_04407A` | `$04407A` | 22 | 1 | Variante de #5 con cámara terciaria `$108064/$108066` (paralaje foreground). |
| 8 | `Coord_ApplyCameraDeltaToGlobals_044090` | `$044090` | 26 | 1 | Variante de #5 sobre coords globales `$106F38/$106F3A`. |
| 9 | `Coord_ApplyCameraDeltaToD1D2_0440AA` | `$0440AA` | 18 | 1 | Aplica delta cámara a coords en registros `d1/d2` (in-out). |
| 10 | `Coord_LocalToScreenSecondary_0440BC` | `$0440BC` | 20 | 1 | Variante de #2 con cámara secundaria `$106F50/$106F54` (paralaje background). |
| 11 | `Coord_ScreenToLocalSecondary_0440D0` | `$0440D0` | 20 | 1 | Inverso de #10. |
| 12 | `Buffer_ClearBlock1024L_043EDA` | `$043EDA` | 16 | 1+ | Utility: limpia 4 KB (1024 longs) desde `$7C(a0)` con `dbra`. |
| 13 | `RNG_LFSRStep_SelfSeed_05E9B6` | `$05E9B6` | 46 | 2 | Variante "self-seeded" del LFSR global compartida con `RNG_LFSRStep_05E9E4` (Wave Z1). Retorno por CCR-N. Referenciada por `EntityState_PublishByProbeN` (BB2#3/4). |
| 14 | `AttractInit_Single_099AE2` | `$099AE2` | 26 | 1 | Inicializador attract mode single-player: publica audio channels + reset frame counter. Contrapartida de `Global_Clear10E486_099AFC` (Wave O#4). |

**Sin falsos positivos absorbidos en Wave CC.** Total del proyecto: 26 FPs.

**Descubrimientos arquitectónicos Wave CC:**

1. **Subsistema de coordenadas cámara reconstruido byte-a-byte.** Identificados los **cuatro sistemas de cámara** del juego con sus offsets exactos: `$106F6C/6E` (principal), `$106F70/74` (scroll rápido), `$106F50/54` (paralaje BG), `$108064/66` (paralaje FG). Documentado el idioma **Y-flip Neo Geo** (`neg.w d1; addi.w #$200, d1`) en las 4 conversiones locales↔pantalla.

2. **Patrón de handlers gemelos**: primer par 158 B / 168 B **casi idénticos sin factorización** del proyecto (`Handler_ApplyCameraSelf` vs `Handler_ApplyCameraGlobals`). Diferencia única: RAM local vs. RAM global.

3. **Primer bucle mutuo entre funciones vecinas** documentado: `$046B20` y `$046BDA` comparten un bucle-body distribuido, con salto adelante (`bls.w`) del uno al otro y salto atrás (`bra.b`) al medio del primero desde el segundo.

4. **Extensión del catálogo de helpers CCR-return**: descubierto `$999DE` (Clipping_Test) como el primer helper del proyecto con **retorno por flag Carry** (CCR-C), complementando los helpers CCR-N ya catalogados en Waves S/T/U/Z/BB.

5. **Combo signo-XOR compacto** (`sgt.b d4; spl.b d5; eor.b d4, d5; beq`) documentado como idioma para `if (sign(x-K) != sign(dx))` en 4 instrucciones + 1 branch.

6. **Segunda variante del LFSR global** recuperada: `RNG_LFSRStep_SelfSeed_05E9B6` comparte buffer `$10E230` y puntero `$10E270` con `RNG_LFSRStep_05E9E4` (Wave Z1).

7. **Idioma MMIO Fix Layer masificado**: +12 apariciones nuevas de `movem.w d0-d1, $3C0000.l` en Wave CC batch 2, elevando el total documentado a más de 44 usos en el proyecto.

### Wave BB en detalle (batch 2)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `EntityFrame_FrameSelectByGate3A_057044` | `$057044` | 54 | 1+ | Selector de frame por team (`$3A bit0`) que precarga `d1/d2/d3 = {4,5,16}` o `{17,18,19}`. Si `Y < $180`, hace **tail-call por `bcs.w` al helper contiguo** `EntityFrame_FrameSelectByXDist_05707A`; si `Y >= $180`, commitea `d3` en `$5C(a6)` y retorna. |
| 2 | `EntityFrame_FrameSelectByXDist_05707A` | `$05707A` | 46 | tail-call | Selector fino por distancia relativa (desplazamiento `$24`, probablemente Y pese al nombre provisional). Consume la cache `d1/d2/d3` de #1 y publica el frame final en `$5C(a6)` segun tres bandas: `> $60` → `d2`, `[-$40..+$60]` → `d1`, `< -$40` → `d3`. |
| 3 | `EntityState_PublishByProbeN_05717A` | `$05717A` | 34 | 1 | `jsr $5E9B6` con retorno via bit N del CCR: si N=1 limpia `$72 bit4`; si N=0 lo activa. En ambos casos publica `state=3` via `bset #3,$74(a6)`. |
| 4 | `EntityState_PublishByProbeN_ClearSub75_05719C` | `$05719C` | 40 | 1 | Variante de #3 con `move.b #0,$75(a6)` al final; resetea el substate tras publicar `state=3`. |
| 5 | `EntityState_SetSubstate2_0571C4` | `$0571C4` | 20 | 1 | Publica `(active=1, state=3, substate=2)` via `$72/$74/$75`. |
| 6 | `EntityState_SetSubstate1_0571D8` | `$0571D8` | 20 | 1 | Clon de #5 con `substate=1`. |
| 7 | `EntityState_SetSubstate3_0571EC` | `$0571EC` | 20 | 1 | Clon de #5 con `substate=3`. |
| 8 | `EntityState_SetState74Bit4_057200` | `$057200` | 14 | 1 | Publica `active=1` (`$72 bit4`) y `state74_bit4=1` sin substate. |
| 9 | `EntityState_SetState74Bit0_05720E` | `$05720E` | 8 | 1 | Publisher mínimo: `bset #0,$74(a6); rts`. |
| 10 | `EntityState_SetState74Bit2_057216` | `$057216` | 8 | 1 | Clon de #9 con bit 2. |
| 11 | `EntityState_SetState74Bit1_05721E` | `$05721E` | 8 | 1 | Clon de #9 con bit 1. |

**Sin falsos positivos absorbidos en Wave BB batch 2.** Los 4 dispatchers grandes de la misma zona (`$057000`, `$05702A`, `$0570A8`, `$057226`) siguen aparcando entradas falsas de Waves H/D/I dentro de su cuerpo; se reservaron para una oleada posterior de absorción/merge.

**Descubrimientos arquitectónicos Wave BB batch 2:**

1. **Cluster de state publishers per-entity recuperado como subsistema autónomo.** Por primera vez queda claro que `$72(a6)` y `$74(a6)` son dos bytes de flags de estado con semántica separada: `$72 bit4` actúa como flag principal `active/dirty`, mientras `$74 bits 0..4` codifican micro-estados. `$75(a6)` funciona como substate fino (0..3).

2. **Nuevo tail-call a función contigua**: `EntityFrame_FrameSelectByGate3A_057044` no retorna cuando `Y < $180`, sino que hace `bcs.w` al inicio exacto de `EntityFrame_FrameSelectByXDist_05707A`. Es el 12º caso documentado de cola condicional a función vecina del proyecto.

3. **Patrón de state-machine por bits consolidado**: las variantes `SetSubstate{1,2,3}` y `SetState74Bit{0,1,2,4}` muestran que muchas transiciones de enemigos no son handlers largos sino publishers de 8–20 B que solo activan combinaciones de bits en `$72/$74/$75`.

4. **Retorno via CCR-N confirmado en otro subsistema**: H03/H04 usan `jsr $5E9B6` + `bpl.w` inmediato, ampliando la evidencia de que Metal Slug 1 emplea masivamente helpers que retornan por flags del CCR en vez de por `d0`.

### Wave BB en detalle (batch 1)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `TitleModeInit_024E38` | `$024E38` | 54 | 1 | Inicializador del modo Title/Demo. Reinyecta input via `jsr $2126`, publica `StartAck_P1/P2 = 1` en `$106ECC/$106ECD`, limpia `$10007B`, hace `jsr $9773C`, y luego: si `$10FD82 != 0` hace **tail-call por `bne.w` al thunk vecino** `JsrAbsThunk_024e6e` (`jsr $99AFC; rts`); si `$10FD82 == 0`, llama a `$99AE2` y salta al `rts` del thunk. |
| 2 | `Player_Start_Inner_024E76` | `$024E76` | 272 | 1+ | Procesador dual-jugador del botón START del sistema Neo Geo. Consume el latch `$10FDB4 & 3`, usa una tabla de 4 filas embebida en `$024FA6`, actualiza las máscaras públicas `$10FDB6/$10FDB7`, rearma bits del latch con `bset`, decrementa en modo single los contadores BCD `$1081BF/$1081C0` vía `Start_Decoder`, y publica un flag final en `$10FDAF` si hubo transición. |
| 3 | `Start_Decoder_024F86` | `$024F86` | 32 | 2 | Decrementador BCD 2-dígitos con underflow: `00 -> 99`, preservando `d1` en pila. Cuarto elemento del pipeline decimal del proyecto tras X#4, Z2#4 y AA2#5. |
| 4 | `StartInputTable_024FA6` | `$024FA6` | 16 | datos | Primera tabla embebida del proyecto reconstruida byte-a-byte: 4 filas × 4 bytes indexadas por `(latch & 3) << 2`, que codifican la transición combinada de Start P1/P2 y el flag final publicado en `$10FDAF`. |

**Sin falsos positivos absorbidos en Wave BB batch 1.** El thunk vecino `JsrAbsThunk_024e6e` no se absorbe: forma parte real del control flow del cluster y sigue siendo un símbolo independiente reutilizable.

**Descubrimientos arquitectónicos Wave BB batch 1:**

1. **El latch `$10FDB4` queda identificado como el bit-latch de START P1/P2.** `Player_Start_Inner` lo consume, lo rearma con `bset`, y publica el resultado en `$10FDB6/$10FDB7` y `$10FDAF`. Ya no es un dispatcher genérico de input, sino el backend concreto del botón START del hardware Neo Geo.

2. **Primera tabla de datos embebida dentro de código** (`$024FA6..$024FB5`) recuperada y registrada explícitamente en `REGISTRY`. Demuestra que el proyecto necesitará seguir modelando `.text` con islotes de datos cuando ataquemos más zonas tempranas del ROM.

3. **Nuevo operador decimal especializado**: `Start_Decoder_024F86` añade una cuarta variante al pipeline aritmético decimal del juego: decremento BCD de 2 dígitos con wrap `00→99`, distinto de las variantes de división y suma BCD ya recuperadas.

4. **Doble uso de epílogo/vecino confirmado**: `TitleModeInit` comparte control-flow con el thunk contiguo `JsrAbsThunk_024e6e`. Es otro ejemplo claro de la organización de ROM en mini-fragmentos hand-coded que el escáner de Waves H/I puede contabilizar erróneamente como funciones independientes aunque formen parte estructural de un cluster mayor.

### Wave AA en detalle (batch 2)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Debug_DrawHexCounter_047482` | `$047482` | 156 | 1+ | Top-level del debug HUD hex-counter. Gate `cmpi.b #$1, $10FDAF.l` (mismo puerto que Debug_DrawHUDVars X#1). Si `$106E92 <= $63`, pinta 8 tiles al Fix Layer con MMIO `movem.w d0-d1, $3C0000.l` (deltas +$20/+1/-$1F/+$F/-$21). Si excede, fall-through directo a #2. |
| 2 | `Debug_DrawHexCounter_ClampBranch_04751E` | `$04751E` | 138 | fall-through | Rama gate por-registro `cmpi.b #$0, d1`. Si `d1 != 0` fall-through a #3. Si `d1 == 0`, misma estructura que #1 pero con fila fija `$4B40`. |
| 3 | `Debug_DrawHexCounter_Fallback_0475A8` | `$0475A8` | 166 | fall-through | Fallback via tabla: clamp `d0 = min($106E92, $63)`, `jsr $47656(pc)` (=#5) para decodificar cociente/resto, lookup en tabla word `$28DECC` con `d0*2` y `d1*2`, pinta 8 tiles con tile-ids resueltos. |
| 4 | `Debug_SetCounter_04764E` | `$04764E` | 8 | 1 | Setter `move.w d0, $106E92.l; rts`. |
| 5 | `Sub_Divide10_047656` | `$047656` | 32 | 1+ | **Divisor por 10 iterativo** con clamp de entrada a `$63`. Algoritmo sub-add-bmi (d0 -= 10 hasta negativo, d1 cuenta iteraciones, add.w d2,d0 restaura resto). Complementa el pipeline decimal identificado en Wave X (X#4 shift-and-subtract 32-iter generico) con la variante optimizada para operandos <100. |

**Sin falsos positivos absorbidos en Wave AA batch 2** (el cluster estaba limpio, ninguna cola registrada previamente como thunk independiente por Wave I/J/N).

**Descubrimientos arquitectónicos Wave AA batch 2:**

1. **Cluster debug HUD reconstruido byte-a-byte, 500 B contiguos.** Primera vez que se recupera un subsistema completo de debug en un solo bloque. Confirma que Metal Slug 1 (1996) llevaba HUD de debug embebido en la release comercial, gated por `$10FDAF` (mismo puerto ya visto en Wave X#1 `Debug_DrawHUDVars`), un patrón habitual de la época para dejar el código pero deshabilitado en producción.

2. **Extensión del pipeline decimal a tres divisores distintos:**
   ```
   Wave X#4  Sub_LongDivide_05D920      (32-iter shift-and-subtract, cualquier
                                         divisor, TRAP #15 en div-by-zero)
   Wave Z2#4 BCD_AddClamp99999999_051A10 (operador aritmetico BCD 8-digitos)
   Wave AA#5 Sub_Divide10_047656        (divisor por 10 iterativo sub-add,
                                         optimizado para operandos <100)
   ```
   El juego tiene **tres implementaciones especializadas** de la misma clase de operación aritmética, cada una optimizada para su patrón de uso. AA2#5 es el más pequeño (32 B) y el más específico.

3. **Triple tail-call encadenado a función contigua**: `Debug_DrawHexCounter → Debug_DrawHexCounter_ClampBranch → Debug_DrawHexCounter_Fallback`, cada uno cayendo al siguiente vía `bls.w`/`bne.w` sin `bra` explícito. Son el **10º y 11º "tail-call a función contigua" del proyecto**, tres funciones enlazadas linealmente por caídas condicionales. Idioma imposible en C.

4. **Idioma MMIO `movem.w d0-d1, $3C0000.l` consolidado como el más frecuente del proyecto**: aparece 24 veces en AA2 (8 en #1 + 8 en #2 + 8 en #3), sumadas a las 4 apariciones previas (W#3, W#4, W#6, W#7). Es el protocolo estándar de escritura al Fix Layer del Neo Geo con `address+data` en un único `movem.w` de dos palabras.

5. **Tabla de dígitos `$28DECC` publicada**: el lookup `move.w (a1,d0.w),d2` con `a1=$28DECC` implica una tabla de al menos 10 words de tile-ids para los dígitos 0..9. Candidato natural para promoción a `DigitTileTable_28DECC` en `symbols.py` y para investigar como dato estático cuando se ataque la sección de rodata.

### Wave AA en detalle (batch 1)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Player_DispatchStateBySlot_051914` | `$051914` | 134 | 1 | Variante "por slot externo" del state dispatcher. Recibe puntero a slot en `a1`, filtra por `cmpa.l` contra `$100440/$1004E0/$100580`, publica `$68(a6)` en `$6E(a1)`, y ejecuta la triada `Sub_00051862 → BCD_AddClamp99999999 → suba.w #8,a0 → Sub_00051828`. `movem.l a0-a2,-(a7)` + `move.w d1,-(a7)` para preservar registros sobre la triada. |
| 2 | `Player_BuildTableAddrOnly_05199A` | `$05199A` | 36 | 1 | Solo calcula la dirección de entrada en `StateJumpTable_05188C` (mismo `(idx&$1F)*4 + $4` que #1 y Z2#3) y la publica en `d6`. Simetría de push/pop de `a0-a2` aunque solo modifique `a2` (convención hand-coded). |
| 3 | `Player_DispatchOrLoadFromSlot50_051A28` | `$051A28` | 94 | 1 | **Dual-entry**: entry principal `$051A28` carga chain-slot desde `$50(a6)` y publica `$68(child)→$6E(a6)`; entry alterno `$051A44` parte del pipeline con `$6E(a6)` ya alimentado por el caller. Reusa el backbone P1/P2 + triada de #1 con `movem.l d0/a0-a2,-(a7)` (preserva `d0` extra). Publica `d0` en `$1081BA` antes de la triada. |
| 4 | `Player_IncCounterAt7_051A86` | `$051A86` | 30 | 1 | Incrementa `$7(a1)` con clamp inferior `bcs.w` a 9 y tail-call a `Player_CounterSaturateByte_05170C` (W#11). **Contrapartida "incremento por-slot"** del clamp global de W#11. Absorbió `JsrAbsThunk_051a9c`. |

**Falsos positivos absorbidos en Wave AA batch 1 (1 nuevo, 26 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 26 | `JsrAbsThunk_051a9c` | I (jsr_abs_thunks) | `Player_IncCounterAt7_051A86` (AA1 #4) | Cola `jsr $5170C.l; rts` tail-call clamp saturado |

**Descubrimientos arquitectónicos Wave AA batch 1:**

1. **Corrección de dimensión del "target `$051914`"**: la cola priorizada estimaba ~276 B para un único helper. El desensamblado reveló que era un **cluster contiguo de 6 funciones** cuyos huecos reales sumaban solo 294 B netos (4 nuevas + 2 ya cerradas Z2#3, Z2#4). Confirma la regla forense: los "targets grandes" del scanner suelen ser clusters mal dimensionados con vecinos ya matcheados en medio.

2. **Layout final del cluster de estado por-jugador (`$051914..$051AA3`) reconstruido byte-a-byte:**
   ```
   $051914  Player_DispatchStateBySlot          (AA1#1, "por slot externo a1")
   $05199A  Player_BuildTableAddrOnly           (AA1#2, "solo pointer en d6")
   $0519BE  Player_StateDispatch                (Z2#3,  "por a6")
   $051A10  BCD_AddClamp99999999                (Z2#4,  operador aritmetico)
   $051A28  Player_DispatchOrLoadFromSlot50     (AA1#3, "por sub-slot $50(a6)")
   $051A86  Player_IncCounterAt7                (AA1#4, "increment + clamp")
   ```
   Tres variantes del mismo dispatcher (#1 externo, Z2#3 por a6, #3 por sub-slot) más el "solo pointer" (#2) más el operador BCD (Z2#4) más el incrementador con clamp (#4). Toda la lógica de state-machine per-player del juego cabe en 398 B contiguos.

3. **Dual-entry con push tardío** (AA1#3): el entry alterno `$051A44` sirve como "shared body con arg pre-cargado". El caller que lo usa llega con `d0/a0-a2` en registros vivos y deja que la propia función empuje el snapshot al entrar. Idioma imposible en C sin `__asm__ naked`.

4. **Correlación semántica confirmada**: `Player_IncCounterAt7` (AA1#4) → `Player_CounterSaturateByte_05170C` (W#11) demuestra que el pipeline es "primero incrementar en el slot, después clampear saturado en el buffer per-player global". Extiende la cadena semántica ya descubierta en Wave X (pipeline decimal display) y Wave Z b2 (hit → score+10 → spawn).

### Wave Z en detalle (batch 2)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `ProbeTwoAttemptsCcr_05E5A8` | `$05E5A8` | 56 | 1 | Dos intentos consecutivos probe+confirm con d0=0 y d0=1 sobre `Sub_0005E3A2` y `Sub_0005E618`. Retorno por CCR. Absorbió `ClearXN_05e5ba`, `ClearXN_05e5d4` y `SetXN_05e5da`. |
| 2 | `Player_Dispatch3Slots_028998` | `$028998` | 94 | 1 | Dispatcher sobre los 3 slots de jugador `$100440/1004E0/100580` con prerrequisito `*link == $80`. Absorbió `SetXN_0289ea` y `ClearXN_0289f0`. |
| 3 | `Player_StateDispatch_0519BE` | `$0519BE` | 82 | 1 | Dispatcher de estado por-jugador con jump-table PC-rel `$5188C` de 32 entradas long-word (mask $1F, offset $4). Selector via $6E(a6) y guard $12 flag. |
| 4 | `BCD_AddClamp99999999_051A10` | `$051A10` | 24 | 1+ | **Operador aritmético BCD** del pipeline decimal del juego. 4x `abcd.b -(a2),-(a1)` con clamp a `$99999999` si overflow. Complementario del pipeline display decimal (Wave X). |
| 5 | `Entity_ProbeRevertCcr_027AFC` | `$027AFC` | 106 | 1 | Cuarta variante del cluster probe/revert `$027xxx` (tras T#7-T#15, Z1#5, Z1#6). Snapshot 4 campos, colisión PC-rel `$272A8`, dos ramas simetricas. Absorbió `ClearXN_027b3e` y `SetXN_027b60`. |
| 6 | `Entity_SpawnLoop16_06E412` | `$06E412` | 36 | 1 | Spawner de 16 entities desde template unico `$6DD5C` con contador en pila; propaga `$9B(padre)` a cada hijo. |
| 7 | `Entity_SpawnAndTag_06E224` | `$06E224` | 38 | 1 | Spawner con tag bit0(d0) publicado en `$3A(hijo)`. `movem.w d0,-(a7)` para preservar sobre 2 jsr. Absorbió `JsrPcThunk_06e244`. |
| 8 | `List_ApplyWithSentinelFF_04784C` | `$04784C` | 38 | 1 | Clon de Z1#7 con callback `Sub_00047822` (stride $40 = task node). |
| 9 | `List_ApplyWithSentinelFF_0477FC` | `$0477FC` | 38 | 1 | Clon de Z1#7 con callback `Sub_000477D4` y stride $20 (half-stride = sub-fields). |
| 10 | `Player_IncCounterAt84_032B36` | `$032B36` | 34 | 1 | Incrementa byte en `$84(slot)`; slot elegido por `$6D==1?P1:P2`. |
| 11 | `Player_IncCounterAt81_032AFA` | `$032AFA` | 34 | 1 | Clon byte-a-byte del anterior con offset $81 (contadores adyacentes en struct player_slot). |
| 12 | `Init_JsrThenTailCall_001320` | `$001320` | 18 | 1 | Init de zona baja: `jsr $46AC6.l` + publica sentinela `$FF` en `$106ED2` + `bra.w $FE0` (tail-call largo sin rts propio). |
| 13 | `Handler_TimerAndReplace_001BCC` | `$001BCC` | 104 | 1 | Handler de timer con reset condicional. Cuando counter global `$106E92` llega a 0, spawna template `$46A48` e instala handler continuación `$1C34`. Absorbió `JsrAbsThunk_001c2c`. |
| 14 | `Handler_ConditionalHitCounter_08B558` | `$08B558` | 54 | 1 | Handler condicional gated por `$58(a6)==1`: `$66 += 10` (score bonus), `Sub_00028758` (state check), bit-flag global `$106F28`, tail-call a `Entity_SpawnAndTag` (Z2#7). Absorbió `JsrAbsThunk_08b586`. |

**Falsos positivos absorbidos en Wave Z batch 2 (10 nuevos, 25 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 16 | `ClearXN_027b3e` | N (ccr_helpers) | `Entity_ProbeRevertCcr_027AFC` (Z2 #5) | Cola `andi.b #$EE, ccr; rts` rama colisión-NO |
| 17 | `SetXN_027b60` | N (ccr_helpers) | `Entity_ProbeRevertCcr_027AFC` (Z2 #5) | Cola `ori.b #$11, ccr; rts` rama colisión-SI |
| 18 | `SetXN_0289ea` | N (ccr_helpers) | `Player_Dispatch3Slots_028998` (Z2 #2) | Cola `ori.b #$11, ccr; rts` path exito |
| 19 | `ClearXN_05e5ba` | N (ccr_helpers) | `ProbeTwoAttemptsCcr_05E5A8` (Z2 #1) | Cola `andi.b #$EE, ccr; rts` exito 1er intento |
| 20 | `SetXN_05e5da` | N (ccr_helpers) | `ProbeTwoAttemptsCcr_05E5A8` (Z2 #1) | Cola `ori.b #$11, ccr; rts` path fallo total |
| 21 | `JsrAbsThunk_001c2c` | I (jsr_abs_thunks) | `Handler_TimerAndReplace_001BCC` (Z2 #13) | Cola `jsr $47482.l; rts` tail-call |
| 22 | `JsrAbsThunk_08b586` | I (jsr_abs_thunks) | `Handler_ConditionalHitCounter_08B558` (Z2 #14) | Cola `jsr $6E224.l; rts` tail-call |
| 23 | `JsrPcThunk_06e244` | J (jsr_pc_thunks) | `Entity_SpawnAndTag_06E224` (Z2 #7) | Cola `jsr PcThunkTarget_06e2bc(pc); rts` |
| 24 | `ClearXN_0289f0` | N (ccr_helpers) | `Player_Dispatch3Slots_028998` (Z2 #2) | Cola `andi.b #$EE, ccr; rts` path default |
| 25 | `ClearXN_05e5d4` | N (ccr_helpers) | `ProbeTwoAttemptsCcr_05E5A8` (Z2 #1) | Cola `andi.b #$EE, ccr; rts` exito 2do intento |

**Nuevo récord absoluto de absorciones en una sola oleada** (Wave Z batch 1 absorbió 5, batch 2 absorbe 10). Con 25 falsos positivos totales acumulados, el matcher ha catalogado ya el ~1% del espacio de FPs Wave I/J/N heredados de escaneos antiguos.

**Descubrimientos arquitectónicos Wave Z batch 2:**

1. **Operador aritmético BCD principal identificado** (Z2 #4). `BCD_AddClamp99999999_051A10` (24 B) completa el otro extremo del **pipeline decimal del juego**: complementario al pipeline display de Wave X (X#2 clamp binario, X#3 decoder, X#4 divisor). Ambos comparten la constante mágica `$99999999`. Confirma que Metal Slug maneja score/counters en BCD 8-dígitos.

2. **GAS con `--register-prefix-optional` rechaza `abcd -(aX),-(aY)`** (Z2 #4). La expresión `-(a2)` se parsea como aritmética y el modo predecrement no se reconoce. **Solución canonizada**: emitir los 2 bytes del opcode literal (`.byte 0xc3, 0x0a`) por instrucción abcd. Anotado como convención del proyecto para instrucciones raras del 68000 (junto a la ya conocida para `jsr (pc,d7.w,$34)` de Z1#1).

3. **Cluster probe/revert `$027xxx` cerrado con 8 variantes matcheadas** (T#7-T#15 + Z1#5, Z1#6, Z2#5). Estructura de la familia estabilizada:
   ```
   probe(4 campos, callback PC-rel) -> bcs/bcc -> restore + post-hook $28108 -> CCR invertido
   ```
   Variaciones sistemáticas: número de campos (2/4), sentido del branch (bcs/bcc), presencia de clamp 10-bit al $82, orden de restore (secuencial/alternado word-byte).

4. **Cadena semántica hit → spawn confirmada** (Z2 #14 → Z2 #7). `Handler_ConditionalHitCounter_08B558` termina en `jsr $6E224.l` = `Entity_SpawnAndTag_06E224`. Es la primera cadena "hit event → score+10 → spawn tagged entity" reconstruida completa del juego.

5. **Nueva pareja de patrones "reset counter + install continuation"** (Z2 #13). `Handler_TimerAndReplace_001BCC` demuestra el idioma:
   ```
   if (counter <= 0) {
     mark_field(a6, $21, $FF);
     alloc_from_template(a6, template);
     counter = 0;
     install_next_handler(a6, &continuation);
   }
   tail_call(post_hook);
   ```
   Amplía la línea de "handler cycles" ya identificada en Wave T#6.

6. **Fix del flujo en `Handler_TimerAndReplace_001BCC`**: el `bne.w` de "low nibble != 0" salta **directamente al `rts`** final (sin ejecutar `jsr $47482.l`). Etiqueta `.Lexit_rts` intercalada entre `jsr $47482.l` y `rts` — patente de asm hand-coded que rechaza los `bra` para fallthrough, prefiriendo etiquetas "encadenadas" con salidas múltiples.

### Wave Z en detalle (batch 1)

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Sprite_Dispatch_05A9D6` (dual entry con `$05A9E2`) | `$05A9D6` | 192 | 1+ | Dispatcher de sprite con dos puntos de entrada: A fuerza modo B (bit 2 encendido), B respeta d5. Convergen en `$5A9E6`, indexan cola gigante en a5=$108080 (slots ADD `$6148` / SUB `$614a`), y saltan a jump-table PC-rel `$05AA96` vía `jsr (pc, d7.w, $34)`. `move a1, usp` / `move usp, a1` como save/restore vía USP. |
| 2 | `Entity_SpawnAndPublishD0At70_05239E` | `$05239E` | 20 | 1 | Reserva entity desde template `$523EE`; publica word `d0` (preservado en pila sobre `jsr $4AE`) en `$70(a0)`. |
| 3 | `Entity_SpawnAndPublishD0At70_0523B2` | `$0523B2` | 20 | 1 | Clon byte-a-byte del anterior con template alterno `$524AA`. Confirma patrón "NO se factoriza template" del asm hand-coded. |
| 4 | `Entity_Probe_Scratch_02785C` | `$02785C` | 48 | 1 | Probe informativo simple del cluster `$027xxx`: snapshot de `$22`+`$82` en scratch a5-rel, `jsr $44182` colisión, restore incondicional, clamp 10-bit `andi.w #$3FF, $82(a6)`, post-hook comun. Absorbió `JsrPcThunk_027886`. |
| 5 | `Entity_ProbeRevertCcr_02788C` | `$02788C` | 118 | 1 | Probe/revert simetrico con snapshot de 4 campos (`$22`, `$82`, `$26`, `$27`), colisión PC-rel `$27036` y dos ramas CCR con orden de restore **distinto** (alternado word/byte). Retorno CCR invertido (`ori/andi ccr`). Absorbió `SetXN_0278d4` y `ClearXN_0278fc`. |
| 6 | `Entity_ProbeRevertCcr_027A92` | `$027A92` | 106 | 1 | Variante gemela del anterior con campo `$24` en vez de `$82` y branch inverso (`bcs` vs `bcc`). Sin clamp 10-bit: `$24` no lo necesita. Absorbió `ClearXN_027ad4` y `SetXN_027af6`. |
| 7 | `List_ApplyWithSentinelFF_047888` | `$047888` | 38 | 1 | Recorre lista de words desde `a5` hasta centinela `$00FF`; callback PC-rel `$47872` con (d4=d1_orig, a4=a1_orig, d0=elem); avanza `a4 += $40` (stride tamaño de task node del scheduler Y#1). |
| 8 | `Task_WalkTwoArenas_05DD2A` | `$05DD2A` | 34 | 1 | Ejecuta `Task_WalkList_05B6` sobre dos arenas secundarias del scheduler (`$100800` y `$1008A0`, dif `$A0` = 4 task nodes). Save/restore de a6 con `movem.l a6, -(a7)`. |
| 9 | `RNG_LFSRStep_05E9E4` | `$05E9E4` | 56 | múltiples | **GENERADOR ALEATORIO PRINCIPAL DEL JUEGO**. LFSR Fibonacci sobre buffer de 32 words en `$10E230` con head en `$10E270`. Taps -1 y -21 (mask `$3E`), feedback por `rol.w #1` + `eor.w`, salida por `mulu.w d5; swap` (high word del producto). |
| 10 | `PosRing_PushCapped_08F308` | `$08F308` | 60 | 1 | Publica `(field22, field24)` de `a6` en ring buffer `$10E33A` si `count<4`; wrap en `$20` bytes (8 tuplas); `++count`. |
| 11 | `Player_CounterSaturateByte_05170C` | `$05170C` | 64 | 1 | Incremento saturado en `$FF` de contador byte-per-player en `$10E3A2`; offset `+$18` (P1 `$106E94`) o `+$19` (P2 `$106E9C`); no-op para otros valores de `a1`. |
| 12 | `Blit_Loop48Iters_05026C` | `$05026C` | 48 | 1 | 48 iters del blitter fila `$43F5E` (T#2) con `d0` inicial 0, `d1` inicial 240, incremento `d0 += 8` = renderiza fila completa de 384 pixeles (ancho Neo Geo). Contador **en la pila**, args callee por pila explícita. |

**Falsos positivos absorbidos en Wave Z batch 1 (5 nuevos, 15 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 11 | `JsrPcThunk_027886` | J | `Entity_Probe_Scratch_02785C` (Z#4) | Cola `jsr $28108(pc); rts` |
| 12 | `SetXN_0278d4` | N (ccr_helpers) | `Entity_ProbeRevertCcr_02788C` (Z#5) | Cola `ori.b #$11, ccr; rts` rama colisión-SI |
| 13 | `ClearXN_027ad4` | N (ccr_helpers) | `Entity_ProbeRevertCcr_027A92` (Z#6) | Cola `andi.b #$EE, ccr; rts` rama colisión-NO |
| 14 | `ClearXN_0278fc` | N (ccr_helpers) | `Entity_ProbeRevertCcr_02788C` (Z#5) | Cola `andi.b #$EE, ccr; rts` rama colisión-NO |
| 15 | `SetXN_027af6` | N (ccr_helpers) | `Entity_ProbeRevertCcr_027A92` (Z#6) | Cola `ori.b #$11, ccr; rts` rama colisión-SI |

**Récord de absorciones en una sola oleada** (Wave Y absorbió 2, Wave W absorbió 4; batch 1 de Wave Z absorbe 5). El cluster probe/revert `$027xxx` fue la primera zona donde los escáneres Wave I/J/N enumeraron colas de helpers como thunks/setters independientes de forma sistemática.

**Descubrimientos arquitectónicos Wave Z batch 1:**

1. **Generador aleatorio principal identificado** (Z#9). LFSR Fibonacci con polinomio primitivo `x^32 + x^21 + 1` clásico de la época. Estado global: `struct { uint16 buffer[32]; uint16 head; }` en `$10E230..$10E270`. La salida se dispersa vía `mulu.w d5` (salt del caller) y se toma el high word del producto. Descubrimiento crítico: cualquier función del juego que necesite RNG llama aquí.

2. **Sprite dispatcher dual-entry con USP como registro extra** (Z#1). Uso de `move a1, usp` / `move usp, a1` en modo supervisor para "guardar un registro adicional" sin tocar la pila. Idioma imposible en C: la CPU en modo supervisor tiene acceso a USP como registro auxiliar. La instrucción `jsr (pc, d7.w, $34)` con desplazamiento de 8 bits requirió emitir los 4 bytes literales (`4E BB 70 34`) porque GAS no puede codificar disp8 PC-rel a un símbolo externo `--defsym`.

3. **Cluster probe/revert `$027xxx` continuando línea T#7–T#15**. Tres nuevas variantes (Z#4/Z#5/Z#6) con firma forense idéntica al cluster de Wave T: snapshot en scratch RAM a5-relativa (`-$1148..-$1143` desde `a5=$108080`), colisión callback, restore + CCR invertido. Diferencias por variante: cantidad de campos (2 vs 4), sentido del branch (bcs vs bcc), clamp 10-bit al `$82` (sí vs no).

4. **Struct Entity refinada**. Nueva evidencia consistente entre Z#5 y Z#6:
   ```c
   struct Entity {
       ...
       uint16 field22;    // coord (nunca clamped)
       uint16 field24;    // coord (nunca clamped)
       uint8  field26;
       uint8  field27;
       ...
       uint16 field82;    // coord (clamp 10-bit obligatorio tras restore)
   };
   ```

5. **Task node stride confirmado en $40 bytes** (Z#7). El bucle de `List_ApplyWithSentinelFF_047888` avanza `a4 += $40` por iteración, coincidente con el offset entre las dos arenas secundarias de Z#8 (`$100800` y `$1008A0`, dif `$A0` = 4 nodos de $40). Estructura del task node reconstruida como bloque de 64 B.

6. **Cinco absorciones simultáneas** con misma firma forense: colas `rts` con instrucción única antes fueron cuenta doble como Wave J/N. El cluster probe/revert es el punto arquitectónico donde los antiguos escáneres estaban más desincronizados con la realidad semántica.

### Wave Y en detalle

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Scheduler_MainLoop_000656` | `$000656` | 154 | 2 | Bucle central del scheduler de tareas: recorre la linked-list en `$100080`, decrementa saturadamente 3 timers (`$44/$45/$59`) con `subq.b #1 + addx.b d1,d0` (d1=0), invoca `jsr (a0)` con SP snapshot en `$106E8E`, y `bra.w $65c` (fall-through al comparator vecino como salida). |
| 2 | `Scheduler_CompareField10_0006F0` | `$0006F0` | 14 | fall-through + directos | Comparator `cmp.b $10(a0), d0` con retorno por CCR. Función contigua al bucle principal: idioma "fall-through al vecino como salida alternativa" (12° del proyecto). |
| 3 | `Copy2Bytes_10FDB6to10E3A0_0981FC` | `$0981FC` | 18 | 1 | Copia 2 bytes contiguos `$10FDB6/B7 → $10E3A0/A1` con `(a0)+/(a1)+` + `(a0)/(a1)` planos (evita `move.w`). |
| 4 | `Global_SetDualFlagFrom10FD82_000FFE` | `$000FFE` | 34 | 1 | Publica en dos globales gemelas (`$1081BF/$1081C0`) valor 0 o 4 según flag `$10FD82`. Doble branch explícito. |
| 5 | `Table_LoadPtrByIdxClamp6_04CB5C` | `$04CB5C` | 44 | 1 | "Publish default, patch on success": escribe primero sentinela `$FFFFFFFF` en `$1081B2`, luego (si idx<6) sobreescribe con `table[idx]` de la tabla de 6 long-ptr en `$04CB44`. |
| 6 | `Entity_DispatchOpcodeNibble_032D00` | `$032D00` | 40 | 1 | Doble indirección: nibble bajo de `$87(a6)` → tabla word-offset en `$329EE(pc)` → campo word del propio entity → publica en `$14(a6)`. |
| 7 | `Entity_AllocByPlayerSlot_09B9F6` | `$09B9F6` | 62 | 1 | Reserva entity hijo con idx 0/1 según si `a1==$100440` (P1) o `$1004E0` (P2). `move.l d0,-(a7)` para preservar idx sobre los 2 jsr. |
| 8 | `Init_MasterSubsystems_0020E2` | `$0020E2` | 68 | 1 | Arranque post-BIOS: 5 `jsr abs.l` a subsistemas hijos con watchdog kick `$300001` intercalado entre cada uno. Absorbió `JsrAbsThunk_00211e`. |
| 9 | `InputQueue_InitAndPushOp4_00212E` | `$00212E` | 110 | 1 | Init + priming de la cola circular de opcodes (`$108184` × 32 B, head `$1081A6`, tail `$1081A4`) con push HARDCODEADO del opcode `$04`. |
| 10 | `Entity_Build3ChainCircular_03060A` | `$03060A` | 140 | 1 | 3× (`lea tpl(pc); jsr $4AE Task_Alloc; jsr $5DD02 Entity_CopyTransform; move.l a0,slot(a6)`) + cableado circular de los 3 nodos con orientación idéntica (`$70=prev, $74=other, $78=next`). |
| 11 | `Entity_Build4FromTemplates_0818AA` | `$0818AA` | 66 | 1 | 4× (`lea tpl(pc); jsr $4AE; jsr $5DD02`) sin encadenar ni guardar los punteros — spawn batch de 4 entities independientes. Absorbió `JsrAbsThunk_0818e4`. |

**Falsos positivos absorbidos en Wave Y (2 nuevos, 10 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 9 | `JsrAbsThunk_00211e` | I | `Init_MasterSubsystems_0020E2` (Y#8) | Cola `jsr $ffe.l; rts` del arranque post-BIOS |
| 10 | `JsrAbsThunk_0818e4` | I | `Entity_Build4FromTemplates_0818AA` (Y#11) | Cola `jsr $5dd02.l; rts` del spawn batch |

**Descubrimientos arquitectónicos Wave Y:**

1. **Scheduler central del juego reconstruido byte-a-byte** (Y#1). Modelo:
   ```
   TASK_ARENA_BASE ($100080)
      │
      └→ linked-list via $8(node) → next
           │
           └→ Scheduler_MainLoop_000656:
                  reset flags → decrementa 3 timers → save SP
                  → jsr (task->handler) → cleanup flags
                  → advance a6=next → bra.w $65c (loop header)
   ```
   Globales de contabilidad: `$106E84` (`CURRENT_TASK_PTR`), `$106E8A`
   (`FRAME_TASK_MARKED_CNT`), `$106E8C` (`FRAME_TASK_TOTAL_CNT`), `$106E8E`
   (`SCHED_SP_SAVE`, snapshot para abort tipo longjmp desde handler).

2. **Arranque post-BIOS canónico** (Y#8): secuencia fija de 5 subsistemas
   con watchdog kick al puerto Neo Geo `$300001` INTERCALADO entre cada
   llamada. Patrón defensivo hand-coded para evitar reset durante fases
   largas de init. Homogeneidad de codificación (`jsr abs.l` largo incluso
   para targets que caben en abs.w) como decisión de estilo.

3. **Cola circular de opcodes especializada** (Y#9): buffer de 32 bytes con
   head/tail, deduplicación consecutiva vía last-value (`$1081AC`) y
   protocolo de recuperación "queue full → reset dedup". El helper es NO
   parámetrico: el opcode a encolar (`$04`) está literal en el código.

4. **Idioma "publish default, patch on success"** (Y#5): la sentinela
   `$FFFFFFFF` se escribe ANTES de la comprobación de rango. GCC ordenaría
   la comprobación primero. Patrón asm hand-coded que garantiza consistencia
   de estado incluso ante fallos parciales de la comprobación.

5. **Constructor de linked-list circular de 3 nodos** (Y#10) con orientación
   idéntica en los tres — patrón típico de "triada cooperativa" del Metal
   Slug (player + arma + shot-trail). Los tres punteros publicados en
   slots contiguos del padre (`$74/$78/$7C`) sugieren `struct SubEntityTrio`.

6. **Doble absorción de falsos positivos Wave I** (`JsrAbsThunk_00211e` y
   `JsrAbsThunk_0818e4`) — 9° y 10° falsos positivos del proyecto. Misma
   firma forense que los 8 previos: la cola `jsr abs.l; rts` de un helper
   semántico fue contabilizada erróneamente como thunk independiente por
   el escáner de Wave I. **Nuevo hito**: dos absorciones en una sola oleada.

### Wave X en detalle

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Debug_DrawHUDVars_096A80` | `$096A80` | 164 | 2 | HUD debug con 7 volcados hex al Fix Layer, gate `btst #0, $100001.l`. Absorbió `JsrAbsThunk_096b1c`. |
| 2 | `Decimal_Clamp99999999_05D8F2` | `$05D8F2` | 18 | 2 | Clamp a $05F5E0FF (=99 999 999) con tail-call al decoder BCD contiguo. |
| 3 | `Sub_BinToDecimalDecoder_05D904` | `$05D904` | 28 | ≥2 | Extractor BCD por division iterativa con `asl.l d5,d2` (shift variable). |
| 4 | `Sub_LongDivide_05D920` | `$05D920` | 36 | ≥1 | Divisor shift-and-subtract 32-iter con `TRAP #15` en div-by-zero. |
| 5 | `Entity_FindByKey_000190` | `$000190` | 58 | 2 | Buscador de tabla 8B/entry con watchdog kick `$300001` DENTRO del bucle. |

**Falsos positivos absorbidos en Wave X (1 nuevo, 8 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 8 | `JsrAbsThunk_096b1c` | I | `Debug_DrawHUDVars_096A80` (X#1) | Cierre `jsr $5D6C2.l;rts` |

**Descubrimientos arquitectónicos Wave X:**

1. **Pipeline "decimal display" reconstruido byte-a-byte** que atraviesa 4
   waves (W#3, X#1..X#4):
   ```
   Debug_DrawHUDVars (X#1)
      ├→ Decimal_Clamp99999999 (X#2, cap a $99999999)
      │     └tail-call→ Sub_BinToDecimalDecoder (X#3)
      │                    └→ Sub_LongDivide (X#4, ÷10)
      │                            └err→ Trap15_DivByZero (halt sistema)
      └→ Sprite_HexFormat4 (W#3, con HEX_TABLE_5D71C compartida)
   ```
   Es la primera cadena semántica completa reconstruida en el proyecto.

2. **Watchdog kick MMIO `$300001`** (X#5) — puerto oficial del Neo Geo MVS.
   Se ejecuta DENTRO del bucle de búsqueda, patrón defensivo de asm
   hand-coded imposible en C sin `volatile` + intrínseco de hardware.

3. **`TRAP #15` como abort de div-by-zero** (X#4) — idioma clásico de asm
   defensivo que aprovecha el vector table del 68000. GCC nunca emite
   `trap` como manejo de error.

4. **Noveno "tail-call a función contigua"** del proyecto (X#2→X#3): la
   rama "clamp OK" salta al inicio de la función siguiente en vez de
   tener un `rts` propio, reutilizando su cuerpo como continuación.

### Wave W en detalle

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Entity_AllocSpriteSlot_00236E` | `$00236E` | 1 594 | 8 | 47 iters desenrolladas + `jsr $29a8(pc)` sub-prólogo compartido + fallback round-robin con fall-through al epílogo. |
| 2 | `Entity_ProbeSpriteSlot_29A8` | `$0029A8` | 74 | (interno) | Bucle backward reentrando por `tst.w d3` como cabecera. |
| 3 | `Sprite_HexFormat4_05D6C2` | `$05D6C2` | 90 | 6 | Formatter 4-nibble con `movem.w d0/d2, $3C0000` MMIO + tabla ASCII PC-rel `+0x1A`. |
| 4/5 | `Sprite_HexFormat8_05D72C` / `_05D740` (dual entry) | `$05D72C` / `$05D740` | 146 | ? | Formatter 8/4-nibble con leading-zero suppression via sticky bit `0xFFFF`. |
| 6 | `Fix_BlitStream_05DAD8` | `$05DAD8` | 66 | 4 | Stream con opcodes `$FE`/`$FD` + clipping vertical `[$7000..$74FF]`. |
| 7 | `Fix_BlitRect_05DA9C` | `$05DA9C` | 60 | 3 | Blit rect doble `dbra` (Fix Layer column-major: paso $20 col, +1 fila). |
| 8 | `Entity_ProbeMoveX_09A7AA` | `$09A7AA` | 34 | 3 | Retorno por CCR + `andi.b #$EE,ccr` cleanup. Absorbió `ClearXN_09a7c6`. |
| 9 | `Entity_FlushSlotHistory_013600` | `$013600` | 36 | 3 | LIFO drain con `bra.b` al inicio de la propia función. |
| 10 | `Entity_ReserveAndSetPos_05E4B2` | `$05E4B2` | 24 | 2 | Convención de paso múltiple `d0/d1/d2 -> $38/$22/$24(a6)`. |
| 11 | `Global_Clear10E486_099AFC` | `$099AFC` | 10 | 2 | `move.b #0,abs.l` explicito para evitar RMW spurio de `clr.b`. |
| 12 | `Entity_SwapProbeCommit_028292` | `$028292` | 70 | 2 | Swap-probe-commit-or-rollback con CCR compuesto (`andi #$EE` / `ori #$11`). Absorbió `ClearXN_0282d2`. |
| 13 | `Sprite_Dispatch_05CA2A` (4 entry) | `$05CA2A` | 164 | 5 | 5-entry-point trampoline con perspective Q8.8 (`muls.w + asr.l #8`). |
| 14 | `Camera_ResetCenter_05CACE` | `$05CACE` | 16 | ? | Reset a `(0xA0, 0x178)` centro por defecto. |
| 15 | `Entity_ClearPtrSlots_05DC1C` | `$05DC1C` | 24 | 2 (via W#16) | Bucle backward unsigned `bcc.b` sobre `[$10..$9C]` con paso `-4`. |
| 16 | `Entity_AllocFromFreeList_0006FE` | `$0006FE` | 108 | 2 (+1 via W-I) | Pop free-list + doble init + insert ordenado + rama empty `beq.w` a la función CONTIGUA. Absorbió `JsrAbsThunk_000762`. |

**Falsos positivos absorbidos en Wave W (4 nuevos, 7 totales del proyecto):**

| # | Símbolo original | Wave | Absorbido por | Idioma |
|---|---|---|---|---|
| 4 | `ClearXN_09a7c6` | F | `Entity_ProbeMoveX_09A7AA` (W#8) | Epílogo `andi.b #$EE,ccr;rts` |
| 5 | `ClearXN_0282d2` | F | `Entity_SwapProbeCommit_028292` (W#12) | Epílogo `andi.b #$EE,ccr;rts` |
| 6 | `SetTaskHandler_049fea` | H | `Entity_ProbeAndInstallHandler_049FD0` (V#8) | Brazo `.Linstall_channel_b` |
| 7 | `JsrAbsThunk_000762` | I | `Entity_AllocFromFreeList_0006FE` (W#16) | Cierre `jsr $5DC34.l;rts` |

Regla forense consolidada: los CCR helpers Wave F cuyo cuerpo es puramente
el epílogo canónico `andi.b #$EE, ccr; rts` o `ori.b #$11, ccr; rts` deben
revalidarse contra helpers de dispatch que retornan por CCR contiguos.
Son sospechosos sistemáticos de ser epílogos compartidos absorbidos.

Descubrimiento arquitectónico Wave W:
- **Idioma MMIO `movem.w d?/d?, $3C0000`** confirmado en 4 helpers del cluster
  hex/blit al Fix Layer (W#3, W#4, W#6, W#7). Es el protocolo estándar
  de escritura al tile-map del Neo Geo (address+data en dos writes
  consecutivos), que GCC nunca emite porque no reconoce MMIO como destino
  válido de `movem`.
- **Fix Layer column-major** confirmado por W#7 (paso `+$20` entre columnas,
  `+1` entre filas).
- **Tabla ASCII compartida `HEX_TABLE_5D71C`** documentada como entidad de
  datos con dos referentes PC-rel (`+0x1A` en W#3, `-0x62` en W#4/W#5).
- **Rama empty tail-calls a función contigua**: en W#16, la rama
  "free-list vacio" salta a la SIGUIENTE función en `$076A` que reasigna
  `a0` a un DUMMY_ENTITY global. Idioma anti-NULL clásico de asm hand-coded.

### Wave V en detalle

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Entity_ClearFlags13Bits12` | `$0283CA` | 14 | 5 | Doble `bclr.b` sobre bits 1 y 2 de `flags13(a6)`. No colapsable a `andi.b`. |
| 2 | `Entity_ProbeSlot4c_0283D8` | `$0283D8` | 20 | 6 | Probe de `$4c(a6)` (ENTITY_NIL) con fall-through a `$283EC`. `moveq #-1,d7` no usado. |
| 3 | `Entity_CopyField68AndCall_0517FE` | `$0517FE` | 14 | 4 | Copia `field68` src→dst y delega en `$5CCC8`. Absorbió `JsrAbsThunk_051804`. |
| 4 | `Sprite_SetupSlotFromTableA/B` | `$002C26 / $002C30` | 64 | 6+6 | Dual entry-point contiguo: selector de banco (`$1CE00` vs `$14E00`) + slot en `$1082C8`. |
| 5 | `Entity_InitFields_05DC34` | `$05DC34` | 112 | 4 | Inicializa 24 campos del entity + `bset #6,$13` como flag SLOT_RESERVED. |
| 6 | `Entity_TrailRecord_099812` | `$099812` | 184 | 7 | Ring buffer 16×12 B para trails/particulas en `$10E3BE`; id counter mod 15. |
| 7 | `Entity_AddTimer0_74_0138FE` | `$0138FE` | 8 | 3 | `addi.w #$74, $1c(a6)`: incrementa timer0 con constante fija. |
| 8 | `Entity_ProbeAndInstallHandler_049FD0` | `$049FD0` | 34 | 4 | Dispatch triple por CCR con dos `lea pc+d,a1; move.l a1,(a6)`. Absorbió `SetTaskHandler_049fea`. |
| 9 | `Tbl_Decode2D_0799DE` | `$0799DE` | 48 | 4 | Decoder de tabla 2D indexado por `(flags11&3)*4 + ($106ED1&2) + (d1&7)*16`. |

**Falsos positivos absorbidos en Wave V:** `JsrAbsThunk_051804` (Wave I,
8 B) y `SetTaskHandler_049fea` (Wave H, 8 B). Ambos eran epilogos
compartidos entre helper y "thunk/setter", cosa que GCC nunca emite.
Es el tercer y cuarto caso confirmado tras `JsrAbsThunk_050248`
(absorbido por `Sprite_InvokeBlit8Params` en Wave S).

### Wave S en detalle

| # | Símbolo | Dirección | Bytes | Callers | Descripción |
|---|---|---|---:|---:|---|
| 1 | `Sprite_InvokeBlit8Params` | `$05022A` | 38 | 279 | Carga 8 parámetros (a0,a1,d0..d5) desde tabla en `a2` y salta a `$51de2`. |
| 2 | `Entity_HasLinkedSlots` | `$028D70` | 30 | 115 | Comprueba `$3c(a6)` y `$40(a6)` contra `ENTITY_NIL=0xFFFFFFFF`; retorna por CCR. Su `bne.w` cae por diseño en `Script_DispatchOpcode`. |
| 3 | `Table_LookupPointerBounded` | `$000772` | 64 | 35 | Lookup acotado de tabla de punteros con `trap #15` si el índice rebasa el límite. |
| 4 | `Entity_CopyTransform` | `$05DD02` | 32 | 19 | Copia pos_x, pos_y y tres flags desde `a6` a `a0`. |

### Wave T en detalle

| # | Símbolo | Dirección | Bytes | Descripción |
|---|---|---|---:|---|
| 1 | `Script_DispatchOpcode` | `$028D8E` | 70 | Despachador del intérprete de scripts con `trap #15` para opcodes >= `$20`. |
| 2 | `Sprite_MultiBlitClippedX` | `$043FAC` | 98 | Bucle de blit múltiple con clipping X en `[-80,+392]`. |
| 3 | `Fix_BlitRectToFixLayer` | `$05DA56` | 70 | Rectangle blit al Fix Layer `$3c0000`. |
| 4 | `Task_AllocFromFreeList` | `$0004AE` | 80 | Extrae un nodo de `$106e80`, lo enlaza a la lista viva y cae al epílogo compartido `$0004FE`. |
| 5 | `ActorCtxWrapper_02783a` | `$02783A` | 34 | Wrapper que fija `a5=$108080`, preserva `d5/d6` y delega en `FUN_00028108`. |
| 6 | `Entity_InstallHandlerAndCopyXf` | `$077C7E` | 26 | Instala un handler literal `$077C98`, copia transform y guarda `a1` en `dst->field_70`. |
| 7 | `Entity_ProbeTransformFreeCcr` | `$027CEE` | 68 | Probe/revert con retorno por CCR; absorbió `ClearXN_027d2c`. |
| 8 | `Entity_RestoreTransformSetC_027d32` | `$027D32` | 30 | Brazo hermano de T#7; restaura transform y retorna `C=1`; absorbió `SetXN_027d4a`. |
| 9 | `Entity_ProbeTransformFreeCcr_027c8c` | `$027C8C` | 68 | Segundo probe/revert del cluster; absorbió `ClearXN_027cca`. |
| 10 | `Entity_RestoreTransformSetC_027cd0` | `$027CD0` | 30 | Brazo hermano de T#9; absorbió `SetXN_027ce8`. |
| 11 | `Entity_ProbeTransformFreeCcr_027bc8` | `$027BC8` | 68 | Tercer probe/revert del cluster; absorbió `ClearXN_027c06`. |
| 12 | `Entity_RestoreTransformSetC_027c0c` | `$027C0C` | 30 | Brazo hermano de T#11; absorbió `SetXN_027c24`. |
| 13 | `Entity_ProbeTransformFreeCcr_027c2a` | `$027C2A` | 68 | Cuarto probe/revert del cluster; absorbió `ClearXN_027c68`. |
| 14 | `Entity_RestoreTransformSetC_027c6e` | `$027C6E` | 30 | Brazo hermano de T#13; absorbió `SetXN_027c86`. |

## Herramientas

| Fichero | Función |
|---|---|
| `tools/match_batch.py` | Matcher: compila `.c` y `.s`, linkea a direcciones absolutas, extrae con `objcopy` y compara byte-a-byte. |
| `tools/registry.py` | Registro maestro `(nombre, offset, size, fuente)`. |
| `tools/symbols.py` | Tabla de símbolos absolutos para `--defsym` del linker. |
| `tools/scan_unmatched_callees.py` | Cola priorizada de próximos targets ordenada por popularidad de llamadas entrantes desde código ya matcheado. |
| `tools/asm-differ/diff.py` | Diff visual side-by-side (backend m68k). |
| `tools/gen_asm_region.py` | Generador de borradores `.s` verificados byte-a-byte para una region sin matchear (recorre huecos, parte en entradas, emite GAS con el estilo del proyecto, imprime lineas de registry/symbols). |
| `scripts/bootstrap_sandbox.sh` | Instala toolchain m68k + deps Python y procesa la ROM desde un zip en un solo paso. |
| `scripts/setup.sh` | Procesa `rom/201-p1.bin` en `build/mslug_prom.bin` y verifica MD5. |
| `scripts/legacy/gen_*.py` | Generadores históricos de las Waves A–R (mantenidos por reproducibilidad). |

## Investigación previa

Se investigó exhaustivamente el toolchain original de Nazca/SNK para MSLUG1
(1996) sin encontrar fuentes públicas que lo documenten. Ni las entrevistas
disponibles ni el "Programmer's Guide" filtrado mencionan compilador o
ensamblador. El estándar de facto moderno para compilar C→Neo Geo con salida
byte-correcta es [ngdevkit](https://github.com/dciabrin/ngdevkit) (GCC 15.3
+ binutils 2.44 + newlib 4.0, target `m68k-neogeo-elf`). Como
`m68k-linux-gnu-gcc 13.3` en modo bare-metal produce el mismo codegen 68000
y está disponible como paquete Debian, se adopta como toolchain oficial del
proyecto — evitando el build de 1 h del toolchain de ngdevkit.

**Hipótesis firmemente respaldada por evidencia interna del ROM:** MSLUG1
contiene grandes bloques de ensamblador 68000 escrito a mano. Evidencias:

- Densidad de bytes sin padding entre funciones (una termina en `rts` y la
  siguiente empieza en el byte contiguo, sin alineación a 4).
- Retornos por CCR en lugar de por `d0` (`Entity_HasLinkedSlots`).
- Convenciones de paso por registros absolutos (`a2`, `a6`) incompatibles
  con el ABI GCC.
- Branches condicionales que caen a la siguiente función como "salida
  alternativa" (el `bne.w` de `Entity_HasLinkedSlots` cae dentro de
  `Sub_028d8e`).
- Solape estructural: 8 bytes del cuerpo de `Sprite_InvokeBlit8Params` habían
  sido erróneamente contabilizados por la Wave I como thunk independiente
  (`JsrAbsThunk_050248`), evidenciando reuso de epílogos.

## Historial de decisiones

- **2026-08-02** — **Wave RR** (5 funciones, 516 B, 0 FPs absorbidos).
  Primera oleada seleccionada puramente por **TAMANO** vía
  `tools/rank_candidates.py` (cola por `score = eff_size *
  (1+log2(callers))`) en vez de popularidad de callers
  (`tools/scan_unmatched_callees.py`, usado en todas las oleadas previas
  S..QQ). Cierra el par gemelo de rutinas de detección de solape de caja
  rectangular con espejado por facing (`Entity_CheckActiveBoxOverlap_072C98`
  144 B / `Entity_CheckBoxOverlapWithSelector_0798AC` 164 B, ambas escriben
  en `target->+0x8E`; la segunda resuelve el chequeo final con un
  **selector de 4 function-pointers en `.text`** en `$2DF4AA`, cuarta tabla
  embebida del proyecto), el relink+wrap del scroll de `camera[0]`
  (`Camera0_RelinkAndWrapScroll_06896A` 112 B) y un constructor de grupo de
  sub-entities enlazadas desde lista de templates de 12 B/nodo
  (`EntityGroup_SpawnLinkedFromTemplateList_065C94` 84 B + helper
  `Entity_MirrorDeltaByFacing_065D32` 12 B). Promueve 4 `PcThunkTarget_*`
  de `tools/symbols.py` a definición canónica; actualiza los 8 thunks
  correspondientes en `src/jsr_pc_thunks.c`.

  Dos fixes iterativos hasta verde: (1) case-sensitivity de GAS —
  `ClearXN_065D44`/`ClearXN_0689E0` (mayúsculas, como sugiere la dirección)
  no existen, los símbolos reales en `symbols.py` son minúsculas
  (`ClearXN_065d44`/`ClearXN_0689e0`); (2) `jsr Entity_MirrorDeltaByFacing_065D32`
  sin `(pc)` ensambló como abs.l (`4EB9`, 6 B) en vez del PC-rel original
  (`4EBA`, 4 B), desplazando +2 B el resto de la función y recortando el
  `rts` final al extraer con `objcopy` el tamaño registrado — mismo idioma
  ya documentado en Wave HH#2, corregido añadiendo `(pc)` explícito.

  Sesión interrumpida por un reset de sandbox a mitad de análisis;
  restaurada extrayendo `201-p1.bin` de `mslug.zip` (MD5
  `b6804bc6be580c80d43d187f6f9d2e7c`), regenerando `build/mslug_prom.bin`
  vía `scripts/setup.sh` (MD5 `816b3f74c76b3373993407615f1850fe`),
  reinstalando el toolchain m68k y las dependencias Python, y
  reconstruyendo los 5 `.s` + los cambios de `registry.py`/`symbols.py`/
  `src/jsr_pc_thunks.c` a partir del análisis ya completado antes de la
  interrupción.

  Estado tras RR: **3 134 / 3 134, 42 190 B, 2.0118 % ROM total**.
  Incremento vs baseline pre-RR: +5 funciones netas (5 nuevas, 0 FPs),
  +516 B netos, +0.0246 pp de cobertura ROM.

- **2026-07-26** — **Escaner priorizado: bug fix critico + mejoras**.
  Reescritura de `tools/scan_unmatched_callees.py`. Bug descubierto en
  la clasificacion de aristas de llamada: el escaner original no
  distinguia entre `M68K_AM_ABSOLUTE_DATA_LONG` (opcode 4EB9, 6 B) y
  `M68K_AM_PCI_DISP` (opcode 4EBA, 4 B) - ambos tienen op.type=MEM con
  base_reg=0. Consecuencia: cada thunk `JsrPcThunk_XXXXXX` (~164
  funciones) hacia `jsr $XXX(pc)` con displacement +4 se contabilizaba
  como si llamase a `$000004` (Reset PC vector del 68000). El Top-1 de
  la cola priorizada era el vector de Reset con 6 callers falsos.

  Ademas el escaner tenia un **Path 2** (parseo naive de `op_str`) que
  duplicaba las aristas legitimas de `jsr abs.l` inflando popularidad
  2x, y no filtraba la zona baja `$000..$3FF` (vectores 68000 en
  $000..$0FF + cabecera cartucho Neo-Geo en $100..$3FF, todo datos).

  Correcciones aplicadas:

  1. Discriminacion rigurosa por `op.address_mode`:
       * ABSOLUTE_DATA_LONG (17)  -> target = op.mem.disp
       * ABSOLUTE_DATA_SHORT (16) -> target = signo-ext(op.mem.disp)
       * PCI_DISP (11)            -> target = ins.address + 2 + disp
       * BRANCH_DISPLACEMENT (19) -> target = op.br_disp.disp (resuelto)
  2. Eliminado Path 2 duplicado.
  3. Filtros de plausibilidad `--min-addr` (default $400) y `--min-size`
     (default 20 B). Modo debug `--show-vectors` para desactivar.
  4. Reporte con desglose de aristas por modo
     (`abs.l/abs.w/pc-rel/bsr/bra`).
  5. Opcion `--json` para volcar el reporte a fichero.

  Resultado con proyecto en estado MM#3 (3 121 / 40 716 B):

    - Antes: Top-1 = `$000004` (Reset vector, 6 callers falsos).
    - Ahora: Top-1 = **`$033522`** (18 callers reales, ~28 B).
    - 1 300 aristas descartadas al filtro `<$400` (basura).
    - 25 aristas descartadas al filtro `<20B` (candidatos triviales).
    - **74 candidatos plausibles finales** (vs 152 mezclados con basura).

  Sin regresiones en el matcher (`3 121 / 3 121, 40 716 B, 1.9415 %`).

- **2026-07-26** — **Wave MM batch 3** (8 funciones, 438 B, 0 FPs absorbidos).
  Oleada de handlers de codigo apuntados por la super-tabla dispatch
  $000B92 (MM#2) en la zona `$00109C..$00125E`. Cierra completamente el
  gap del cluster BIOS bootstrap. Todos los handlers salvo
  `AttractPhase2_Probes5D0_00122E` terminan con `bra.w $FE0` (re-entry
  al bucle B) - patron threaded continuation-passing.

  Descubrimientos: **patron dual "phase1 + phase2"** documentado como
  idioma del scheduler (9a aparicion del fall-through), 2a y 3a
  aparicion del idioma "fall-through a matcheada previa"
  (SchedTail_JsrDB8_0010E8 -> JsrPcThunk_0010ec Wave J;
  AttractPhase2_Gate106ED5_001148 -> SetTaskHandler_00116a Wave H).
  **SchedTail_JsrDB8_0010E8 (4 B)** es la 2a funcion mas pequena del
  proyecto tras los stubs rts (2 B): un unico bsr.w como cuerpo.

  Sin fixes iterativos: verde a la primera pasada gracias a la leccion
  tecnica de MM#1 sobre PC-rel (todos los simbolos externos en
  `symbols.py`, ningun `.set XXX, 0xNNN` local).

  Estado tras MM#3: **3 121 / 3 121, 40 716 B, 1.9415 % ROM total**.
  Incremento vs baseline pre-MM#3: +8 funciones netas, +438 B netos,
  +0.0209 pp de cobertura ROM.

- **2026-07-26** — **Wave MM batch 2** (1 registro de datos, 764 B, 0 FPs).
  Registro de la **super-tabla dispatch `$000B92..$000E8E`** como datos-
  en-.text: `.long` array de 191 u32 BE (186 handlers + 5 sentinels
  $FFFFFFFF), organizado en 6 sub-tablas (T1..T6). Interpretada como
  bytecode virtual continuation-passing por el scheduler bootstrap
  (Wave MM#1). **1a entrada de datos-en-.text del proyecto** registrada
  como registro autonomo del REGISTRY.

  Descubrimientos: **super-tabla dispatch $000B92** ya matcheada al 100 %;
  cierre del grafo hacia atras confirmado: 91 de 93 handlers unicos ya
  matcheados por waves anteriores (97.8 %). Los 6 externals ad-hoc
  `BootTblEntry_XXX` de MM#1 se promueven a labels internos globales del
  `.text` (4a aplicacion del patron "promocion a definicion canonica").

  Sin fixes iterativos: verde a la primera pasada (datos puros sin
  ambiguedad codegen).

  Estado tras MM#2: **3 113 / 3 113, 40 278 B, 1.9206 % ROM total**.
  Incremento vs baseline pre-MM#2: +1 registro, +764 B netos, +0.0364 pp.

- **2026-07-26** — **Wave MM batch 1** (5 funciones, 486 B, 0 FPs absorbidos).
  Oleada del **corazon del scheduler bootstrap** del arranque BIOS.
  Descubierta la **super-tabla dispatch `$000B92`** (760 B, 6 sub-tablas
  u32 BE con centinelas $FFFFFFFF, 186 entradas, 93 handlers unicos) y
  el **bytecode virtual continuation-passing** que la interpreta como
  DSL. `SchedulerBootstrap_Boot_000E8E` (338 B) es el punto de entrada
  canonico: setup MMIO + selector 5-way por `$10FDAE`/`$10FDAF` + bucle
  dispatch A con `jmp (a0)`. `SchedulerDispatch_LoopB_000FE0` (30 B) es
  el hub compartido de re-entrada de todos los handlers.

  Descubrimientos: **super-tabla dispatch $000B92** (1a documentacion),
  **bytecode virtual continuation-passing** (1a documentacion), 7a
  aparicion del idioma fall-through (**1a como hub multi-handler**), 8a
  aparicion del fall-through (**1a hacia funcion matcheada por wave
  anterior**), 8o par de clones no factorizados (SchedTail_JsrCD4/D3C).
  **Leccion tecnica del matcher**: GAS m68k con `.set XXX, 0xNNN` NO
  emite reubicacion PC-rel; hay que usar externals no definidos en
  `symbols.py` (regla aplicable a todos los futuros PC-rel externos).

  Fixes iterativos: 5 iteraciones hasta verde. `moveq` signed 8-bit,
  `bra.b` cross-section, overlap con funciones matcheadas (recorte de
  40 B), PC-rel con literales, etiqueta cruzada.

  Estado tras MM#1: **3 112 / 3 112, 39 514 B, 1.8842 % ROM total**.
  Incremento vs baseline pre-MM: +5 funciones netas (5 nuevas, 0 FPs),
  +486 B netos, +0.0232 pp de cobertura ROM.

- **2026-07-26** — **Wave LL batch 1** (3 funciones, 282 B, 0 FPs absorbidos).
  Oleada de **cierre de callees por-celda del cluster de colision** de
  Wave KK batch 2: los 2 externals residuales (`Fn_00051BA8`,
  `ThunkTarget_051de2`) mas el `ThunkTarget_052712` invocado desde el
  cluster attract (Wave FF). Los 3 simbolos vivian como aliases
  `--defsym` en `symbols.py`, promovidos ahora a simbolos canonicos en
  `.text` siguiendo el patron inaugurado por Wave KK#1
  (`TransformCommit_MMIO_051F30`).

  Descubrimientos: **7o par de clones no factorizados del proyecto**
  (`CellCommit_MMIO` vs bucle interior de `TileMap_HandlerInline`),
  **idioma "cursor persistente"** y **idioma "clear-by-move.b-of-
  zeroed-register"** documentados por primera vez. Cierre completo del
  grafo de Wave KK batch 2 (0 externals pendientes). 3a aplicacion
  exitosa del patron "cerrar grafo antes de abrir cluster" (tras II y
  KK#1).

  Fix iterativo: 2/3 en primera pasada; 1 byte mal en el `dbra d5` del
  bucle interior de `CellCommit_MMIO_051DE2` porque la etiqueta caia
  tras la `lea.l (a1, d1.w), a2` en vez de delante. Movida la etiqueta
  --> matcher verde. Documentado el idioma "lea dentro del bucle para
  refresh de la direccion base".

  Estado tras LL#1: **3 107 / 3 107, 39 028 B, 1.8610 % ROM total**.
  Incremento vs baseline pre-LL: +3 funciones netas (3 nuevas, 0 FPs),
  +282 B netos, +0.0134 pp de cobertura ROM.

- **2026-07-25** — **Wave KK** (7 funciones, 742 B, 2 batches, 7 FPs absorbidos).
  Oleada de **cierre de callees del cluster de camara/sprites** de Wave JJ,
  centrada en los 5 targets pendientes documentados al cierre de aquella
  oleada. Estado tras KK: **3 104 / 3 104, 38 746 B, 1.8476 % ROM total**.

  **Batch 1** (callees pendientes camara/sprites, 3 funciones, 218 B):
  `BlitterTile_2D_043E8C` (78 B, target de tail-jump de los 3 hooks de
  camara JJ#1), `Integrator_XY_051B80` (40 B, `Transform_Publish`) y
  `TransformCommit_MMIO_051F30` (100 B, `Transform_Commit`).
  **Segunda aplicacion exitosa de la leccion de Wave II**: la 4a funcion
  candidata (`$05A88A` `SpriteSubsystem_Reset`) se retiro del batch al
  detectar por overlap del linker que cae dentro de `VRAM_FixLayerAutoclear
  _05A824` (Wave DD). Sin FPs absorbidos. **Primer idioma `call by
  continuation`** documentado: `TransformCommit_MMIO` pasa un handler
  inline via `a0` a un dispatcher generico (`Fn_00001F4A`).

  **Batch 2** (probes de colision + handler MMIO, 4 funciones, 524 B):
  `Collision_ProbeRange_051C08` (120 B), `Collision_ProbeX_051C82` (110 B),
  `Collision_ProbeY_051CF6` (136 B) y `TileMap_HandlerInline_051F94`
  (158 B). **6º par de clones no factorizados del proyecto** (ProbeX/ProbeY,
  gemelos con ejes X/Y intercambiados). **Record de FPs absorbidos en un
  solo batch: 7** (FPs #42-#48), superando el record previo de HH#2 (6).
  Uno de los FPs (#47 `SetV_05202c`) es el primer caso del proyecto donde
  Wave N clasifico erroneamente por decodificacion hint: era `addi.w
  #$800, d3` dentro del bucle del handler MMIO, no un helper CCR.

  **Descubrimientos clave**:
    - **Idioma `call by continuation`** documentado por primera vez
      (`TransformCommit_MMIO` -> `Fn_00001F4A` -> handler inline).
    - **Sexto par de clones no factorizados** del proyecto: ProbeX/ProbeY.
      Refuerza definitivamente la hipotesis de macros ASM pesadas.
    - **Cache de posicion en probes X/Y** (`$1E(a0)`/`$20(a0)`): optimizacion
      hand-coded para saltar el probe cuando el sprite no se movio.
    - **Layout struct sprite ampliado**: mas de 20 offsets confirmados en
      esta oleada.

  **Metodologia validada por 2a vez consecutiva**: overlap detectado por
  el linker impide error de duplicacion (`$05A88A` dentro de `$05A824`,
  Wave DD). La leccion de Wave II queda consolidada como parte estable del
  flujo de trabajo del proyecto.

  Incremento vs baseline pre-KK: +4 funciones netas (7 nuevas -
  3 desplazadas al conteo por FPs), +704 B netos (742 registrados - 42 B
  de los 7 FPs), +0.0336 pp de cobertura. **Segundo salto de +0.03 pp en
  oleadas consecutivas** (JJ +0.0177 pp, KK +0.0336 pp).

- **2026-07-25** — **Wave JJ** (10 funciones, 384 B, 2 batches, 2 FPs absorbidos).
  Oleada centrada en **evidencia forense de código compartido**: los dos
  batches aportan, cada uno, un caso directo del idioma «funciones que
  comparten cuerpo o epílogo» enunciado como hipótesis fundacional del
  proyecto pero nunca antes demostrado de forma tan explícita.

  **Batch 1** (cluster de aplicación de cámara `$043DAA`, 4 funciones, 144 B):
  `CameraApplyOne_043DAA` — el callee que `CameraApplyAll4_043D86` (II#2)
  invoca 3× por `bsr.w` y 1× por fall-through — más sus tres hooks de probe.
  **Hallazgo**: los hooks B y C reutilizan el `rts` del hook A como salida
  temprana, y sus propios `rts` finales son código muerto preservado por la
  macro. Primera evidencia directa de **epílogo compartido cruzado**.
  Absorbió el FP #40.

  **Batch 2** (asignador de sprites hardware `$0139xx`, 6 funciones, 240 B):
  cierra el trío de callees de `SceneLoader_Main_043568` (HH#1). Reconstruye
  el **modelo de sprites hardware del Neo Geo**: 381 sprites repartidos en
  dos pools opuestos, armados como cadenas mediante el bit sticky de SCB3.
  **Hallazgo**: `Spawn_TypeA` salta con `bra.b` al *interior* de
  `Spawn_TypeB`, compartiendo seis instrucciones completas de cola — no solo
  el epílogo — además de la guarda `trap #$F`. Absorbió el FP #41.

  **Metodología validada**: la lección de Wave II (cruzar el rango de
  direcciones contra el registry *completo* antes de abrir un cluster)
  evitó trabajo duplicado al detectar que `$05DA56` ya estaba matcheado
  desde Wave W como `Fix_BlitRectToFixLayer`.

  **Nota técnica recurrente**: en ambos batches aparece la asimetría de
  anchos de branch (`bcc.w` 4 B vs `bcc.b` 2 B para el mismo salto lógico,
  `bhi.w` vs `bhi.b` hacia la misma guarda). Se explica por ensamblado en
  una sola pasada: cuando el destino aún no está resuelto el ensamblador
  reserva la forma larga. Es un marcador forense fiable de ASM hand-coded.

  Estado tras JJ: **3 104 / 3 104, 38 042 B, 1.8140 % ROM total**. Incremento
  vs baseline pre-JJ: +8 funciones netas (10 nuevas − 2 FPs absorbidos),
  +370 B netos (384 registrados − 14 de FPs), +0.0177 pp de cobertura.
  **Superada la barrera del 1.8 % de la ROM.**

- **2026-07-25** — **Wave II** (10 funciones, 412 B, 2 batches, 7 FPs absorbidos).
  Oleada de **cierre de grafo**: en lugar de abrir un cluster nuevo, se atacan
  las callees directas de código ya matcheado en HH#1/HH#3/II#1, cerrando los
  extremos sueltos del pipeline Fix Layer / camera / player-context.

  **Batch 1** (Fix Layer backends `$05DBxx` + slot helper, 4 funciones, 176 B):
  `FixLayer_MultiRowBlit_05DB1A` (lista `$FFFF` con `rts` intermedio),
  `CompareField10_CCR_05DB3C` (primer probe que compara dos contextos
  enlazados por `$8(a6)`), `InstallListPubHead_05DB58` (fija `$3C`=list_ptr,
  `$46`=list_size) y `SlotExtractCoords_05E2D8` — que era el external
  `Fn_00005E2D8` llamado por `SelectPositive_TwoSlots_0967C0` (HH#2) y cuyos
  paths A/B resultan **byte-a-byte idénticos** (4º par de clones). Fija el
  layout de PlayerSlot (`$22`=x, `$24`=y, `$5B`=flags). Absorbió FPs #33-#35.

  **Batch 2** (callees camera/list/ctx, 6 funciones, 236 B):
  `Reset4CameraLongs_043D6C` y `CameraApplyAll4_043D86` (bucle de 4
  iteraciones desenrollado con la última por fall-through),
  `ListCursor_Reinit_05DBC2` + `ListCursor_ReinitClipped_05DBDC` (par
  simétrico pintar/borrar), `CompareField10_CCR_05DC00` (5º par de clones)
  y `PlayerCtx_InitExtended_025012` (rama multi-jugador de HH#3, con flag de
  build leído desde la propia ROM en `$025118`). Absorbió FPs #36-#39.

  **Descubrimientos clave**:
    - **Corrección de la hipótesis de HH#2**: los 8 targets de la jump-table
      `$096B9C[]` NO son handlers de código sino **tablas de datos** (listas
      de sprites attract, stride `$14`, centinela `$FFFF`).
      `ClampAndLookup8_096B7E` es un *table-of-tables* y `$21(a6)` es el
      índice de escena attract. No decompilables → fuera del registry.
    - **Sexta aparición del idioma fall-through**, primera como última
      iteración de un bucle desenrollado (`CameraApplyAll4`).
    - **Dos nuevos pares de clones no factorizados** (total del proyecto: 5).
    - Layout del context struct (a6) ampliado: `$8`, `$10`, `$22`, `$3C`, `$46`.
    - **Lección metodológica**: `$05DA9C`/`$05DAD8` ya estaban matcheados
      desde Wave W y se detectó por colisión de secciones del linker. Antes
      de abrir un cluster hay que cruzar el rango contra el registry
      completo, no sólo contra `scan_unmatched_callees.py`.

  Estado tras II: **3 096 / 3 096, 37 672 B, 1.7963 % ROM total**. Incremento
  vs baseline pre-II: +3 funciones netas (10 nuevas − 7 FPs absorbidos),
  +366 B netos (412 registrados − 46 de FPs), +0.0174 pp de cobertura.

- **2026-07-25** — **Wave HH** (12 funciones, 1 036 B, 3 batches, 3 FPs absorbidos).
  Cierra tres frentes independientes de la cola priorizada:

  **Batch 1** (cluster `$043xxx`, 4 funciones, 504 B): `SceneLoader_Main_043568`
  (top #1 de la cola, 8 callers, 374 B) — el punto de entrada canónico de
  carga de escena del juego. Descubierta la **tabla `scene_table[256]` en
  `$916C8`** y la **`struct SceneEntry` de 14 B**. Incluye 2 helpers de
  camera-smoothing contiguos (`Camera_ResetSmoothing_0434EA`,
  `Camera_SmoothingIntegrate_0434F8`) y la **entry alternativa dual**
  `SceneLoader_ByIndex_043562` (patrón hand-coded ya visto en Wave AA#3).
  Absorbió `JsrAbsThunk_0436d6` (FP #30, cola `jsr $4CB5C.l; rts` del 5º
  subsystem-init hook).

  **Batch 2** (sub-helpers attract `$096xxx`, 6 funciones, 404 B):
  `SelectPositive_TwoSlots_0967C0` (top #3, 6 callers) + los dos cullers
  clónicos `AttractCuller_Cam0/Cam1` (76 B cada uno, byte-a-byte idénticos
  salvo por el slot de camera) + `Viewport_CoordToScreen_096A5A` +
  `DebugTriggers_TwoBits_096B24` (macro DEBUG_TRIGGER expandida 2x) +
  `ClampAndLookup8_096B7E` con **tercera tabla embebida en `.text`** del
  proyecto (jump-table de 8 punteros). Documentado el idioma `jsr d16(pc)`
  (opcode `4EBA`) que GCC no emite y requiere sintaxis explícita
  `jsr TARGET(pc)` en GAS. Absorbió `JsrAbsThunk_096b76` (FP #31).

  **Batch 3** (FixLayer_QuadBatch + PlayerCtx_Reset, 2 funciones, 128 B):
  `FixLayer_QuadBatch_046AC6` (top #4, 4 callers, macro FILL_FIXLAYER
  expandida 4x que pinta el marco HUD del Fix Layer) y
  `PlayerCtx_ResetTwoBlocks_024FEC` (top #10, 3 callers, reset simétrico
  de 2 slots player-scratch con fall-through a `$025012` como salida
  alternativa). Absorbió `JsrAbsThunk_046b18` (FP #32).

  **Descubrimientos arquitectónicos clave**:
    - Sistema de carga de escenas completo (scene_table + entry_script
      bytecode + 5 subsystem-init hooks) — el corazón del arranque de
      cada nivel/pantalla del juego.
    - Idioma `jsr d16(pc)` vs `jsr abs.l` con solución técnica del matcher
      documentada para futuras oleadas.
    - Tercer par de "clones no factorizados" del proyecto (Cam0/Cam1)
      reafirmando la hipótesis de macros ASM pesadas del código original.
    - Contador de callers de `$5DA9C` sube a ~18, convirtiéndolo en
      candidato principal para Wave II.

  Estado tras HH: **3 093 / 3 093, 37 306 B, 1.7789 % ROM total, ~7.54 %
  del código real estimado**. Incremento vs baseline pre-HH: +9 funciones,
  +1 004 B netos (1 036 registrados − 32 de 3 FPs absorbidos), +0.0479 pp
  de cobertura ROM.

- **2026-07-24** — **Wave DD** (5 funciones, 670 B, asm 68000 puro, 1 FP absorbido).
  Batch heterogéneo de helpers de alta prioridad: `Clipping_Test_0999DE`
  (callee CC identificado, retorno CCR-C), `Input_RisingEdgeSnapshot_05CC0E`
  (pipeline de input con rising-edge detection), `VRAM_FixLayerAutoclear_05A824`
  (protocolo autoinc VRAM Neo Geo con NOPs de timing), `VBlankTick_Master_001E5E`
  (handler maestro del tick con dispatch de heavy work), `Task_FreeListInit_000410`
  (ancla del scheduler: 160 nodos + 32 tareas iniciales). Dos fixes iterativos
  aplicados durante la oleada: (1) `movea.l a0, a1` movido dentro del loop en
  Task_FreeListInit para que `bne.b` salte al `+$10` correcto; (2) bloques
  `Lcopy/Lclear` invertidos en Input_RisingEdgeSnapshot (mask=0 = IDLE skip,
  no copy; default fallthrough = copy). FP #27 absorbido: `ClearC_0999f0` →
  cola de Clipping_Test. **Primera estimación cuantitativa del código pendiente
  publicada**: `~494,494 B` de código real por decompilar (93.6% del total
  estimado). Estado tras DD: **3 068 / 3 068, 33 890 B, 1.6160 % ROM total,
  6.42 % del código real estimado**.


- **2026-07-24** — **Wave CC batch 2** (4 funciones, 622 B, asm 68000 puro).
  Cerrados los 2 handlers gemelos apply-camera-with-clipping en `$0440E4/
  $044182` (158/168 B casi idénticos, diferencia única RAM local vs global)
  y el cluster de 2 blitters MMIO al Fix Layer en `$046B20/$046BDA` (186+
  110 B) con **primer bucle mutuo entre funciones vecinas** del proyecto.
  Documentado nuevo idioma **signo-XOR compacto** (`sgt/spl/eor/beq`), el
  helper CCR-C `$999DE` (Clipping_Test) como primer retorno por flag Carry,
  y extendido el idioma MMIO Fix Layer (+12 apariciones). Estado tras CC2:
  **3 064 / 3 064, 33 226 B, 1.5843 % ROM**.

- **2026-07-24** — **Wave CC batch 1** (14 funciones, 380 B, asm 68000 puro).
  Subsistema completo de coordenadas cámara↔pantalla reconstruido en
  `$043EDA..$0440E3` con **4 sistemas de cámara** identificados por su
  offset exacto en RAM y documentado el idioma **Y-flip Neo Geo**
  (`neg.w d1; addi.w #$200, d1`). Incluye `FixBlit_TileByCoord_043F5E`
  (blitter tile individual con pack coord→VRAM), 10 helpers coord con
  aritmética cámara-principal / -secundaria / -terciaria, y utility
  `Buffer_ClearBlock1024L_043EDA` (limpia 4 KB con `dbra`). En dos ficheros
  adicionales: `RNG_LFSRStep_SelfSeed_05E9B6` (segunda variante del LFSR
  global, retorno CCR-N, callee de BB2#3/4) y `AttractInit_Single_099AE2`
  (audio channels + frame counter, contrapartida asimétrica de Wave O#4).
  Sin FPs absorbidos. Estado tras CC1: **3 060 / 3 060, 32 604 B, 1.5547 % ROM**.


- **2026-07-24** — **Wave BB batch 2** (11 funciones, 272 B, asm 68000 puro).
  Cluster de state publishers per-entity `$057044..$057225` cerrado sin
  regresiones. Descubierto el layout estable de flags `$72/$74/$75`:
  `$72 bit4` = flag principal `active/dirty`, `$74 bits 0..4` = selector
  de micro-estado, `$75` = substate fino (0..3). Dos frame selectors
  contiguos (`$057044`, `$05707A`) comparten `d1/d2/d3`; el primero hace
  **tail-call condicional** al segundo cuando `Y < $180`, lo que corrige
  el único mismatch de la oleada (`bcs.w #+$8` salta al inicio exacto de
  la función vecina, no al `rts` local). Los 4 dispatchers grandes de la
  misma zona (`$057000`, `$05702A`, `$0570A8`, `$057226`) quedan aparcados
  para una futura oleada de absorción de falsos positivos Wave H/D/I.
  Estado tras BB2: **3 046 / 3 046, 32 224 B, 1.5366 % ROM**.

- **2026-07-24** — **Wave BB batch 1** (4 funciones, 374 B, asm 68000 puro + tabla).
  Cluster START del sistema Neo Geo `$024E38..$024FB5` cerrado:
  `TitleModeInit_024E38`, `Player_Start_Inner_024E76`,
  `Start_Decoder_024F86` y la tabla embebida `StartInputTable_024FA6`.
  Se identifica definitivamente `$10FDB4` como latch de START P1/P2, y
  se recupera la **primera tabla de datos incrustada** del proyecto (4×4 B)
  integrada en `.text`. `TitleModeInit` comparte control-flow con el thunk
  vecino `JsrAbsThunk_024e6e` (tail-call por `bne.w` al thunk y `bra.w`
  al `rts` del mismo), confirmando otro caso de cluster con epílogo
  compartido. `Start_Decoder` añade una cuarta variante al pipeline
  decimal del juego: decremento BCD 2-dígitos con wrap `00 -> 99`.

- **2026-07-24** — **Wave AA batch 2** (5 funciones, 500 B, asm 68000 puro).
  Cluster debug HUD hex-counter `$047482..$047675` cerrado en un solo
  bloque contiguo. Estructura: top-level con gate `$10FDAF==1` (AA2#1) +
  dos ramas fall-through (AA2#2 clamp branch por-registro, AA2#3 fallback
  via tabla decimal) + setter minimo (AA2#4) + divisor por 10 iterativo
  (AA2#5). El pipeline decimal del juego queda extendido a **tres**
  implementaciones especializadas de división: shift-and-subtract generico
  (X#4), operador BCD 8-digitos (Z2#4), y divisor por 10 optimizado <100
  (AA2#5). Idioma MMIO `movem.w d0-d1, $3C0000.l` triplicado (24 nuevas
  apariciones sobre las 4 previas). Triple tail-call a función contigua
  (AA2#1 -> AA2#2 -> AA2#3), 10º y 11º del proyecto. Sin falsos positivos
  que absorber (cluster limpio). **Umbral simbólico cruzado: 1.5 %
  de la ROM total.** Estado final: **3 031 / 3 031, 31 578 B, 1.5058 % ROM**.

- **2026-07-24** — **Wave AA batch 1** (4 funciones, 294 B, asm 68000 puro).
  Cluster del pipeline de estado por-jugador `$051914..$051AA3` cerrado:
  variante "por slot externo" (AA1#1), "solo pointer" (AA1#2), "por sub-slot
  $50(a6)" con dual-entry (AA1#3) y incrementador con clamp (AA1#4). El
  target `$051914` que estaba en cola priorizada como "~276 B real" era
  en realidad un cluster contiguo de 6 funciones con Z2#3 y Z2#4 ya
  matcheadas en el medio; los huecos reales sumaban 294 B. Un falso
  positivo Wave I absorbido: `JsrAbsThunk_051a9c` → cola tail-call de
  AA1#4 (26° FP del proyecto). Fix del flujo en AA1#3: el `bra.w` desde
  el entry principal debe **saltar sobre** el segundo `movem.l d0/a0-a2,
  -(a7)` (que sirve como entry alterno `$051A44`), no ejecutarlo dos
  veces. Estado final: **3 026 / 3 026, 31 078 B, 1.4819 % ROM**.


- **2026-07-23** — Reorganización del layout al estándar sm64/oot:
  `src/`, `asm/`, `asm/non_matchings/`, `include/`, `linker/`, `tools/`,
  `scripts/`, `docs/`. Integración de `asm-differ` como diff oficial.
  README, LICENSE, Makefile y `.gitignore` añadidos para publicación en
  GitHub. Estado matcher: `2930/2930`.
- **2026-07-23** — Pivote metodológico a modelo dual C + ASM. Introducidas
  las Waves S (helpers semánticos en asm 68000 puro). Primera función:
  `Sprite_InvokeBlit8Params`. Corregido falso positivo
  `JsrAbsThunk_050248`. Segunda función: `Entity_HasLinkedSlots`.
- **2026-07-24** — **Wave X** (5 funciones, 304 B, asm 68000 puro).
  Pipeline decimal display reconstruido byte-a-byte (X#1→X#2→X#3→X#4).
  `TRAP #15` como abort de div-by-zero identificado. Watchdog kick
  `$300001` dentro del bucle en X#5. Un nuevo falso positivo Wave I
  absorbido (`JsrAbsThunk_096b1c` por X#1). Hito psicológico: **3 000
  funciones matcheadas** cruzado con X#2. Estado final: 3 003 / 3 003,
  28 592 / 28 592 B, 1.3634 % ROM.

- **2026-07-24** — **Wave W** (16 funciones, 2 496 B, asm 68000 puro).
  Cluster Sprite slot allocator (1 594 B, mayor delta por-función del
  proyecto) + Probe backward (74 B) + trio de hex formatters + par de
  Fix Layer blitters + varios helpers de entity/allocation. Absorbidos
  4 nuevos falsos positivos con firma forense de reuso de epílogos entre
  helper y CCR-clearer/setter/thunk. Idioma MMIO `movem.w d?/d?, $3C0000`
  identificado como protocolo estándar de escritura al Fix Layer.
  Estado final: 2 999 / 2 999, 28 296 / 28 296 B, 1.3493 % ROM.

- **2026-07-24** — **Wave V** (9 funciones, 482 B, asm 68000 puro). Se
  extiende el catalogo de helpers de entity/sprite con dispatchers de
  handler, inicializadores de fields, un ring buffer de trails (184 B) y
  un decoder de tabla 2D. Dos nuevos falsos positivos absorbidos con la
  misma firma forense de reuso de epilogos entre helper y thunk/setter:
  `JsrAbsThunk_051804` (Wave I) y `SetTaskHandler_049fea` (Wave H).
  Toolchain migrado a `m68k-linux-gnu-gcc 14.2.0` sin regresiones.

- **2026-07-24** — **Wave Z batch 2** (14 funciones, 752 B, asm 68000 puro).
  Extensión del cluster probe/revert `$027xxx` con cuarta variante Z2#5
  (cerrando 8 variantes matcheadas entre Wave T y Z). Nuevo dispatcher
  por-jugador multi-slot `$028998` (Z2#2) con protocolo `*link == $80`.
  **Operador aritmético BCD principal identificado** (Z2#4,
  `BCD_AddClamp99999999_051A10`, 24 B): 4x `abcd.b -(a2),-(a1)` con
  clamp `$99999999`, complementario al pipeline display de Wave X.
  **Nueva convención del proyecto**: instrucciones que GAS con
  `--register-prefix-optional` rechaza (como `abcd -(a2),-(a1)`) se
  emiten con `.byte` literal (misma técnica que `jsr (pc,d7.w,$34)`
  de Z1#1). **Récord absoluto de absorciones en una oleada**: 10 FPs
  cerrados en batch 2 (16°-25° del proyecto, mayoría `ccr_helpers.c`
  del cluster probe/revert). Cadena semántica "hit → score+10 → spawn
  tagged entity" reconstruida en Z2#14 → Z2#7. Fix del flujo en
  `Handler_TimerAndReplace_001BCC` (Z2#13): `bne.w` de low-nibble salta
  al `rts` final sin ejecutar el tail-call intermedio. Estado final
  batch 2: **3 023 / 3 023, 30 792 B, 1.4683 % ROM**.

- **2026-07-24** — **Wave Z batch 1** (12 funciones, 804 B, asm 68000 puro).
  Cluster sprite dispatch dual-entry (`$05A9D6`/`$05A9E2`, 192 B con USP
  como registro extra y `jsr (pc, d7.w, $34)` a jump-table PC-rel) + par
  contiguo de spawners parametricos + 3 variantes del cluster probe/revert
  `$027xxx` (continuando linea T#7–T#15) + list-apply con centinela `$00FF`
  + walk sobre 2 arenas de tareas secundarias + **GENERADOR ALEATORIO
  PRINCIPAL DEL JUEGO** (LFSR Fibonacci con tap `-21`) + ring buffer de
  posiciones + contador saturado por-player + blit loop de fila completa
  (48 iters = 384 px). **Cinco falsos positivos absorbidos en una sola
  oleada** (record del proyecto): `JsrPcThunk_027886` (J), `SetXN_0278d4`,
  `ClearXN_0278fc`, `ClearXN_027ad4`, `SetXN_027af6` (N-ccr_helpers) —
  11°-15° del proyecto. Estado final batch 1: **3 019 / 3 019, 30 100 B,
  1.4353 % ROM**.

- **2026-07-24** — **Wave Y** (11 funciones, 750 B, asm 68000 puro).
  **Scheduler central del juego reconstruido byte-a-byte** (Y#1, 154 B):
  bucle principal de tareas con linked-list en `$100080`, decrementos
  saturados de 3 timers, SP snapshot para abort tipo longjmp desde
  handler, y fall-through al comparator vecino (`Scheduler_CompareField10`,
  Y#2) como salida alternativa. **Arranque post-BIOS canónico** (Y#8):
  5 subsistemas con watchdog kick `$300001` intercalado. **Cola circular
  de opcodes especializada** (Y#9): init + priming HARDCODEADO del opcode
  `$04`. **Constructor de trio circular** (Y#10) y **spawn batch de 4**
  (Y#11). Dos nuevos falsos positivos Wave I absorbidos: `JsrAbsThunk_00211e`
  por Y#8 y `JsrAbsThunk_0818e4` por Y#11 (9° y 10° del proyecto — primera
  oleada con doble absorción). Estado final: **3 012 / 3 012, 29 326 B,
  1.3984 % ROM**.

## Cola priorizada actualizada (post-Wave Z batch 2)

Ver `python3 tools/scan_unmatched_callees.py --top 60`. Cabeza actual
(filtrada por offset `>= $400` y tamaño ≥ 20 B, sin los ya cerrados en
batch 2):

| Prio | Offset | Callers | Tamaño | Descripción tentativa |
|---:|---|---:|---:|---|
| — | `$051914` | 1 | ~276 B real | Dispatcher de estado por-jugador multi-slot (3 slots `$100440/4E0/580`) aparcado en Y y Z. Volver a intentarlo con más slack (contexto ampliado tras `Player_StateDispatch_0519BE` matcheado). |
| — | `$047482` | ≥ 3 | ~460 B real | Dispatcher grande con clipping. Ahora es referenciado por Z2#13 (`Handler_TimerAndReplace`) — candidato natural para Wave AA. |
| — | `$0436de` | 1 | ~248 B | Helper mediano zona `$043xxx`. |
| — | `$000c6a` / `$000bf2` / `$000c9a` | 1 c/u | ~496 B c/u | Triáda de dispatchers grandes de zona `$000Cxxx` (arranque BIOS/init muy temprano). Oleada dedicada. |
| — | `$001918` / `$0018da` | 1 c/u | ~414/476 B | Handlers grandes de zona baja `$001xxx`. |
| — | `$00128e` | 1 | ~102 B | Helper aparcado en Z2 por ambigüedad de inicio; re-analizar con contexto ampliado. |

Cluster natural detectado para la **próxima Wave AA**: cerrar el par
`$047482` + `$051914` (ambos referenciados por múltiples callers ya
matcheados, con contexto completo tras Wave Z batch 2). El grupo
`$000Cxxx` (3 dispatchers de ~496 B) es Wave BB dedicada por tamaño.

## Cola priorizada actualizada (post-Wave Z batch 1)

Ver `python3 tools/scan_unmatched_callees.py --top 40`. Cabeza actual
(filtrada por offset `>= $400` y tamaño ≥ 20 B, y descartando los ya
cerrados en el batch 1 de Wave Z):

| Prio | Offset | Callers | Tamaño | Descripción tentativa |
|---:|---|---:|---:|---|
| — | `$05E5A8` | 1 | ~56 B | Helper cercano a `Entity_ReserveAndSetPos_05E4B2` (W#10) — aparcado en batch 1 por overlap con `SetXN_05e5da`; requiere absorción Wave N. |
| — | `$051914` | 1 | ~276 B real | Dispatcher de estado por-jugador multi-slot (3 slots `$100440/4E0/580`). Aparcado en Wave Y por tamaño excesivo; volver a intentarlo con más slack. |
| — | `$047482` | 1 | ~460 B real | Dispatcher grande con clipping. Requiere oleada dedicada. |

## Cola priorizada actualizada (post-Wave GG)

Cerrado el pipeline attract/title al 100% + 6 callers del helper geometrico.
Siguientes candidatos:

| Prio | Offset | Tamaño est. | Descripción |
|---:|---|---:|---|
| 1 | `$000BF2 / $000C6A / $000C9A` | ~1 488 B | **Triada dispatchers arranque BIOS** que leen la tabla `$000BA2..$000E8A` identificada en Wave EE. Contexto completo del subsistema attract → oleada natural. |
| 2 | `$047482` | ~460 B | Dispatcher grande con clipping (referenciado por Z2#13). |
| 3 | `$051914` | ~276 B | Dispatcher de estado por-jugador multi-slot. |
| 4 | `$967C0 / $969C2 / $96B24` | ~variable | Los 3 sub-helpers usados por los 7 handlers GG batch 1 — targets triviales de arrastre. |
| 5 | `$043568` | ~variable | Helper llamado por 13/13 handlers Wave GG — target de altísimo caller-count. |
| 6 | `$0335A6 / $025766 / $02575C` | ~variable | Los 3 handlers "player entry" publicados en slots `$100440/$1003A0/$100300`. |

## Cola priorizada legacy (post-Wave EE batch 1)

Ver `python3 tools/scan_unmatched_callees.py --top 40`. Cabeza actual
(filtrada por offset `>= $400`, tamaño ≥ 20 B, y descartando lo cerrado
en EE batch 1):

| Prio | Offset | Callers | Tamaño real | Descripción tentativa |
|---:|---|---:|---:|---|
| 1 | `$001744` | tabla `$000BA2`/`$000BB6` | 132 B (`$001744..$0017C7`) | Handler mini-script vecino de EE#2. `jsr $C004C2 + $52712`, task-add `$46682`, gate `$106ED0 < 6`, tail al scheduler. Wave EE-B natural. |
| 2 | `$0017C8` | tabla | 30 B | Init `$106ED2=$FF` + task-add `$3DBC8` + tail `$FE0`. Wave EE-B. |
| 3 | `$0017E6` | tabla | 40 B | Handler `bsr $1E0A` + `move.b #$FF, $21(a6)` + task-add `$46608`. Wave EE-B. |
| 4 | `$001812` | tabla `$000E4A` | 26 B | Handler compacto con gate `$21(a6) != 0 → rts`, fall-through a `$00182C`. Wave EE-B. |
| 5 | `$00182C` | fall-through | 12 B | Continuación `bsr $1CD4; bsr $1DA4; bra $FE0`. Wave EE-B (co-decompilado con #4). |
| 6 | `$001838` | tabla `$000BB6/BE6/E6A/E8A` | 14 B | Thunk `move.b #1, $10FDAF; jmp $85E.l` (tail-call). Wave EE-B. |
| 7 | `$001846` | tabla × 12 refs | 68 B | Doble-check `$100300==$400` y `$1003A0==$400`, publica handlers `$2575C/$25766`. Backbone común (segundo más referenciado tras `$188A`). Wave EE-B. |
| 8 | `$00188A` | ≥ 11 `bra.w` internos | 80 B | **Backbone "wait state"** del cluster attract. Entrada de todos los `bra.w $188A` de Wave EE#3. Wave EE-B (crítico). |
| 9 | `$001AB6` | fall-through desde `.Lone_path` | 66 B | Handler post-Start con probe `$100001` + `jsr $5D288`, gates `$106ED6/$106ED2`, cierra en `$001AF6: rts`. Wave EE-B. |
| 10 | `$0436de` | 1 | ~248 B | Helper mediano zona `$043xxx` (contiguo a subsistema cámara CC1). Wave FF candidata. |
| 11 | `$000c6a` / `$000bf2` / `$000c9a` | 1 c/u | ~496 B c/u | Triada de dispatchers grandes que **leen la tabla de descriptores $000BA2** identificada en Wave EE. Ahora tenemos el contexto completo del subsistema attract para atacarlos. Wave GG dedicada por tamaño. |
| 12 | `$047482` | ≥ 3 | ~460 B real | Dispatcher grande con clipping (referenciado por Z2#13). |
| 13 | `$051914` | 1 | ~276 B real | Dispatcher de estado por-jugador multi-slot. |

Cluster natural detectado para la **próxima Wave EE batch 2**: cerrar
las 9 funciones vecinas del cluster attract (`$001744..$001AF7`, unas
~470 B netos), que completan el subsistema junto con las 3 grandes de
EE batch 1. Todas ya tienen mapa de flujo cerrado, símbolos externos
resueltos (`Sub_00001DB8`, `Sub_00001E0A`, `PcThunkTarget_001CD4`,
etc.) y forman una unidad arquitectónica: dispatcher-tabla (EE#3) +
handlers de estado (EE-B) + backbone `$188A`. Después, **Wave FF**
sobre `$0436de` y **Wave GG** sobre la triada `$000Cxxx`.

## Cola priorizada legacy (post-Wave Y)

Ver `python3 tools/scan_unmatched_callees.py --top 40`. Cabeza actual
(filtrada por offset `>= $400` y tamaño ≥ 20 B para descartar vector table
y thunks triviales):

| Prio | Offset | Callers | Tamaño | Descripción tentativa |
|---:|---|---:|---:|---|
| 18 | `$051914` | 1 | ~134 B | Dispatcher de estado por-jugador: compara `a1` contra 3 slots (`$100440/4E0/580`), copia flag a `$6E(a1)`, salva `a0-a2`, salta a helper `$5188C(pc)`. Tamaño real 276 B tras seguir todas las ramas. |
| 19 | `$02785c` | 1 | ~48 B | Probe/collision de la familia `$027xxx` (mismo cluster que T#7–T#15). |
| 20 | `$02788c` | 1 | ~78 B | Probe/collision cercano al anterior (fall-through candidato). |
| 22 | `$047888` | 1 | ~38 B | Helper corto en la zona de `Sprite_MultiBlitClippedX` (T#2). |
| 23 | `$027a92` | 1 | ~72 B | Probe/collision cluster `$027xxx`. |
| 24 | `$05a9d6` | 1 | ~192 B | Backend de sprite dispatch (target histórico de Wave I `ThunkTarget_05a9d6`, ya expuesto en `symbols.py`). |
| 25 | `$05e9e4` | 1 | ~56 B | Helper cercano a `Entity_ReserveAndSetPos_05E4B2` (W#10). |
| 26 | `$05170c` | 1 | ~48 B | Helper cercano a `JsrAbsThunk_051804` absorbido en V. |
| 27–28 | `$05239e` / `$0523b2` | 1 | ~20 B c/u | Par de helpers cortos contiguos (fall-through natural). |
| 29 | `$05dd2a` | 1 | ~34 B | Helper cercano a `Entity_CopyTransform_05DD02` (S#4). |
| 30 | `$05026c` | 1 | ~48 B | Helper cercano a `StateMachineRun_05022A` (ya expuesto). |
| 31 | `$08f308` | 1 | ~60 B | Helper aislado en la zona `$08xxxx` (post-Y#11). |
| 32 | `$05a9e2` | 1 | ~180 B | Backend contiguo a `$05a9d6` — candidato a cluster de sprite dispatch. |

Cluster natural detectado para la **próxima Wave Z**: los tres pares
`$05a9d6`+`$05a9e2` (sprite dispatch backends), `$05239e`+`$0523b2`
(helpers contiguos cortos) y `$02785c`+`$02788c`+`$027a92` (cluster
probe/collision `$027xxx`, continuando la línea T#7–T#15).

**Aparcados por tamaño excesivo detectado durante el survey de Wave Y**
(sus `~NB` reales tras seguir todas las ramas superan los 250 B):

- `$047482` (460 B): dispatcher grande con clipping. Requiere oleada dedicada.
- `$051914` (276 B): dispatcher por-jugador multi-slot (arriba en el top).

