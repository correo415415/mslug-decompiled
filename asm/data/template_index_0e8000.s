| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching — DATOS
|  Wave VVVVV
|  Región: $0E8000..$0E8524  (1,316 B en 1 hueco(s), 1 entradas)
| ============================================================================
|
|  BORRADOR generado por tools/gen_data_region.py — volcado estructurado:
|  .fill para rachas de $00, .dc.l Simbolo para punteros a entradas
|  conocidas, .dc.w para el resto. Cada entrada empieza en una dirección
|  referenciada desde el código (o frontera de racha de ceros).
|
        .text

| ----------------------------------------------------------------------------
|  TemplateIndex_0e8000  @ $0E8000  (1,316 B)  [ref. desde código]
| ----------------------------------------------------------------------------
        .section .text.TemplateIndex_0e8000, "ax", @progbits
        .global TemplateIndex_0e8000
TemplateIndex_0e8000:
        .dc.l   MeleeGuard_DeathToExtern_0427CA| +00000  -> $0427CA
        .dc.w   0x0005,0x73a2                                               | +00004
        .dc.l   Soldier_SpawnAtGroundA_0573ae | +00008  -> $0573AE
        .dc.l   Soldier_SpawnAtGroundB_0573de | +0000c  -> $0573DE
        .dc.w   0x0005,0x736a,0x0005,0x7374,0x0005,0x7396,0x0005,0x7386     | +00010
        .dc.w   0x0005,0x729a,0x0005,0x72a6,0x0005,0x72b2,0x0005,0x72be     | +00020
        .dc.w   0x0005,0x72d2,0x0005,0x72e2,0x0005,0x72f2,0x0005,0x7302     | +00030
        .dc.w   0x0005,0x731a,0x0005,0x7326,0x0005,0x733a,0x0005,0x734e     | +00040
        .dc.w   0x0005,0x7276,0x0005,0x7282,0x0005,0x728e                   | +00050
        .dc.l   Breakable_Tmpl23_078c98       | +0005c  -> $078C98
        .dc.l   Breakable_Tmpl24_078c78       | +00060  -> $078C78
        .dc.l   Breakable_Tmpl25_078ca2       | +00064  -> $078CA2
        .dc.l   Breakable_Tmpl26_078c88       | +00068  -> $078C88
        .dc.l   MeleeGuard_Handler_04258E     | +0006c  -> $04258E
        .dc.w   0x0004,0x25b6,0x0004,0xcbea                                 | +00070
        .dc.l   HutOccupant_Init_05fa00       | +00078  -> $05FA00
        .dc.l   Breakable_Tmpl1F_05fd78       | +0007c  -> $05FD78
        .dc.w   0x0005,0xfd88,0x0005,0xfd98                                 | +00080
        .dc.l   Breakable_Tmpl22_05ff64       | +00088  -> $05FF64
        .dc.l   Breakable_Tmpl23_060050       | +0008c  -> $060050
        .dc.l   Breakable_Tmpl24_06013c       | +00090  -> $06013C
        .dc.w   0x0006,0x014c,0x0006,0x015c,0x0006,0x016c                   | +00094
        .dc.l   Sign_Tmpl28_060420            | +000a0  -> $060420
        .dc.w   0x0006,0x0440,0x0006,0x0460,0x0006,0x0480,0x0006,0x04a0     | +000a4
        .dc.w   0x0006,0x0426,0x0006,0x0446,0x0006,0x0466,0x0006,0x0486     | +000b4
        .dc.w   0x0006,0x04a6                                               | +000c4
        .dc.l   GroundNest_Tmpl32_0619ca      | +000c8  -> $0619CA
        .dc.w   0x0006,0x19d6,0x0006,0x19e4                                 | +000cc
        .dc.l   Tank_Tmpl35_068c1e            | +000d4  -> $068C1E
        .dc.w   0x0006,0x8c2a,0x0006,0x8c38                                 | +000d8
        .dc.l   TaskHandler_0832ec            | +000e0  -> $0832EC
        .dc.l   TaskHandler_08336c            | +000e4  -> $08336C
        .dc.l   TaskHandler_083426            | +000e8  -> $083426
        .dc.l   MultiStage_Decal_06293e       | +000ec  -> $06293E
        .dc.l   Mortar_Tmpl3C_06361e          | +000f0  -> $06361E
        .dc.w   0x0006,0x3626                                               | +000f4
        .dc.l   Cannon_Tmpl3E_063ec6          | +000f8  -> $063EC6
        .dc.w   0x0006,0x3ed8,0x0006,0xaa14                                 | +000fc
        .dc.l   AllyBazooka_Spawner_Tmpl41_06ac5c| +00104  -> $06AC5C
        .dc.l   Walker_Tmpl42_06d654          | +00108  -> $06D654
        .dc.w   0x0006,0xd686,0x0006,0xd6ba,0x0006,0xd660,0x0006,0xd692     | +0010c
        .dc.w   0x0006,0xd6c6,0x0006,0xd66e,0x0006,0xd6a0,0x0006,0xd6d4     | +0011c
        .dc.l   Gunship_Tmpl4B_06e52a         | +0012c  -> $06E52A
        .dc.l   Hostage_Tmpl4C_0645a8         | +00130  -> $0645A8
        .dc.w   0x0006,0x45ae,0x0006,0x462c,0x0006,0x4632,0x0006,0x461c     | +00134
        .dc.w   0x0006,0x4622,0x0006,0x45f8,0x0006,0x45fe                   | +00144
        .dc.l   Hostage_ResetState_064550     | +00150  -> $064550
        .dc.l   Hostage_SetFlags3435_06458a   | +00154  -> $06458A
        .dc.w   0x0006,0xa08a                                               | +00158
        .dc.l   Bazooka_Tmpl57_06a07e         | +0015c  -> $06A07E
        .dc.w   0x0006,0xba34                                               | +00160
        .dc.l   RocketVehicle_Tmpl59_06ba2c   | +00164  -> $06BA2C
        .dc.l   BazookaB_Tmpl5A_06a476        | +00168  -> $06A476
        .dc.w   0x0006,0xa486                                               | +0016c
        .dc.l   BazookaCrew_Tmpl5C_06a7bc     | +00170  -> $06A7BC
        .dc.l   BazookaCrew_Tmpl5D_06a7ca     | +00174  -> $06A7CA
        .dc.l   Paratrooper_Spawner_Tmpl5E_0666ae| +00178  -> $0666AE
        .dc.w   0x0006,0x66b6                                               | +0017c
        .dc.l   Pow_SpawnVariantTbl_0478fc    | +00180  -> $0478FC
        .dc.w   0x0004,0x790c,0x0004,0x791c,0x0004,0x792c,0x0004,0x793c     | +00184
        .dc.w   0x0004,0x794c,0x0004,0x795c,0x0004,0x796c                   | +00194
        .dc.l   Pow_SpawnFreeVariantB_0483e2  | +001a0  -> $0483E2
        .dc.l   Pow_SpawnFreeVariantA_0483d2  | +001a4  -> $0483D2
        .dc.l   Pow_SpawnTiedVariant_048898   | +001a8  -> $048898
        .dc.w   0x0004,0x88a0                                               | +001ac
        .dc.l   ShieldSoldier_Tmpl6C_0670d2   | +001b0  -> $0670D2
        .dc.l   ShieldSoldier_Tmpl6D_0676ba   | +001b4  -> $0676BA
        .dc.l   PowFall_Spawn_049baa          | +001b8  -> $049BAA
        .dc.l   M5Boss_Tmpl6F_06f1f6          | +001bc  -> $06F1F6
        .dc.l   M5Boss_Intro_Tmpl70_06f17e    | +001c0  -> $06F17E
        .dc.w   0x0006,0xf198                                               | +001c4
        .dc.l   FinalBoss_Tmpl72_071068       | +001c8  -> $071068
        .dc.l   FinalBoss_Tmpl73_07114e       | +001cc  -> $07114E
        .dc.w   0x0007,0x1216                                               | +001d0
        .dc.l   TaskHandler_081908            | +001d4  -> $081908
        .dc.l   M5Tank_Tmpl76_072fc8          | +001d8  -> $072FC8
        .dc.l   M5Missile_Tmpl77_073fca       | +001dc  -> $073FCA
        .dc.l   SquadLeader_Spawn_0403e4      | +001e0  -> $0403E4
        .dc.l   SquadLeader_Respawn_040d6c    | +001e4  -> $040D6C
        .dc.w   0x0006,0x0ce8,0x0006,0x0cde                                 | +001e8
        .dc.l   Crate_Tmpl07C_060cd4          | +001f0  -> $060CD4
        .dc.l   Crew_Tmpl125_079a6c           | +001f4  -> $079A6C
        .dc.l   Crew_Tmpl126_079c8c           | +001f8  -> $079C8C
        .dc.l   Crew_Tmpl127_079eb8           | +001fc  -> $079EB8
        .dc.l   Crew_Tmpl128_079ec2           | +00200  -> $079EC2
        .dc.l   Carrier_Tmpl129_07dfc4        | +00204  -> $07DFC4
        .dc.l   Carrier_Tmpl130_07e000        | +00208  -> $07E000
        .dc.l   Carrier_Tmpl131_07e03c        | +0020c  -> $07E03C
        .dc.l   M2Boss_Tmpl132_07ba7c         | +00210  -> $07BA7C
        .dc.l   GunPlatform_Spawn_04bb9a      | +00214  -> $04BB9A
        .dc.w   0x0004,0xbba4,0x0004,0xbbae,0x0004,0xbbb8                   | +00218
        .dc.l   Squad_MgrInit_0800a6          | +00224  -> $0800A6
        .dc.l   Crab_Tmpl138_07cf02           | +00228  -> $07CF02
        .dc.l   Crab_Tmpl139_07cf0c           | +0022c  -> $07CF0C
        .dc.l   Crab_Tmpl140_07dc3e           | +00230  -> $07DC3E
        .dc.l   Barrel_Tmpl8D_065f40          | +00234  -> $065F40
        .dc.l   AimTurret_Tmpl08E_060e62      | +00238  -> $060E62
        .dc.l   MovingPlatform_Tmpl143_077224 | +0023c  -> $077224
        .dc.w   0x0006,0x0778,0x0006,0x076e,0x0006,0x0a60,0x0006,0x0a56     | +00240
        .dc.l   Obstacle_Tmpl094_060a4c       | +00250  -> $060A4C
        .dc.w   0x0006,0x0b84,0x0006,0x0b7a                                 | +00254
        .dc.l   ScriptedProp_Tmpl97_075100    | +0025c  -> $075100
        .dc.l   Zone_Tmpl152_08f18c           | +00260  -> $08F18C
        .dc.l   Grunt_Tmpl153_08df4a          | +00264  -> $08DF4A
        .dc.l   Grunt_Tmpl154_08df94          | +00268  -> $08DF94
        .dc.w   0x0008,0xe0dc,0x0008,0xd852                                 | +0026c
        .dc.l   Grunt_Tmpl157_08d994          | +00274  -> $08D994
        .dc.l   Grunt_Tmpl158_08d9e2          | +00278  -> $08D9E2
        .dc.w   0x0008,0xda4c                                               | +0027c
        .dc.l   Grunt_Tmpl160_08e19c          | +00280  -> $08E19C
        .dc.w   0x0008,0xdc5a                                               | +00284
        .dc.l   Grunt_Tmpl162_08db50          | +00288  -> $08DB50
        .dc.w   0x0008,0xdc5a,0x0008,0xde70,0x0008,0xdefc,0x0008,0xdf0c     | +0028c
        .dc.w   0x0008,0xe21c                                               | +0029c
        .dc.l   Proj_Tmpl168_08b9a2           | +002a0  -> $08B9A2
        .dc.l   Proj_Tmpl169_08b9aa           | +002a4  -> $08B9AA
        .dc.l   Proj_Tmpl170_08b9b2           | +002a8  -> $08B9B2
        .dc.l   Gunner_Boot_059c2e            | +002ac  -> $059C2E
        .dc.l   Pow_Entry_03fe5a              | +002b0  -> $03FE5A
        .dc.l   Pow_Tied_03fcd8               | +002b4  -> $03FCD8
        .dc.l   Mob_Tmpl174_Init_0989bc       | +002b8  -> $0989BC
        .dc.l   Mob_Tmpl175_Init_098ad4       | +002bc  -> $098AD4
        .dc.l   Mob_Tmpl176_Init_098bc6       | +002c0  -> $098BC6
        .dc.w   0x0009,0x8bd0                                               | +002c4
        .dc.l   Mob_Tmpl178_Init_098d8c       | +002c8  -> $098D8C
        .dc.w   0x0009,0x8d96                                               | +002cc
        .dc.l   Grunt_Tmpl180_08e2ac          | +002d0  -> $08E2AC
        .dc.l   Swinger_Tmpl181_08e30c        | +002d4  -> $08E30C
        .dc.l   Bobber_Tmpl182_08e4e6         | +002d8  -> $08E4E6
        .dc.l   Leaper_Tmpl183_08e5ce         | +002dc  -> $08E5CE
        .dc.l   Runner_Tmpl184_08e6e0         | +002e0  -> $08E6E0
        .dc.l   Nest_Tmpl185_08e746           | +002e4  -> $08E746
        .dc.l   Swarmer_Tmpl186_08ecda        | +002e8  -> $08ECDA
        .dc.l   Nest2_Tmpl187_08ea74          | +002ec  -> $08EA74
        .dc.l   Static_Tmpl188_08ed82         | +002f0  -> $08ED82
        .dc.l   CamProp_Tmpl189_08edc6        | +002f4  -> $08EDC6
        .dc.l   CamProp_Tmpl190_08edd6        | +002f8  -> $08EDD6
        .dc.l   Lob_Tmpl191_08ee16            | +002fc  -> $08EE16
        .dc.w   0x0003,0xfc38                                               | +00300
        .dc.l   Gunner2_Init_05a25a           | +00304  -> $05A25A
        .dc.l   Walker_Init_05a72e            | +00308  -> $05A72E
        .dc.l   Mob_Tmpl195_Init_098f40       | +0030c  -> $098F40
        .dc.l   Mob_Tmpl196_Init_098fa2       | +00310  -> $098FA2
        .dc.w   0x0009,0x8f4e,0x0009,0x8fb0,0x0009,0x8f5c,0x0009,0x8fbe     | +00314
        .dc.l   Mob_Tmpl201_Init_0990bc       | +00324  -> $0990BC
        .dc.l   Mob_Tmpl202_Init_09911e       | +00328  -> $09911E
        .dc.w   0x0009,0x90ca,0x0009,0x912c,0x0009,0x90d8,0x0009,0x913a     | +0032c
        .dc.l   Mob_Tmpl207_Init_0991da       | +0033c  -> $0991DA
        .dc.l   Mob_Tmpl208_Init_0992d8       | +00340  -> $0992D8
        .dc.l   Mob_Tmpl209_Init_099346       | +00344  -> $099346
        .dc.l   Mob_Tmpl210_Init_099446       | +00348  -> $099446
        .dc.l   Mob_Tmpl211_Init_0994a2       | +0034c  -> $0994A2
        .dc.w   0x0009,0x92e6,0x0009,0x9354,0x0009,0x9454,0x0009,0x94b0     | +00350
        .dc.w   0x0009,0x92f4,0x0009,0x9362,0x0009,0x9462,0x0009,0x94be     | +00360
        .dc.l   Mob_Tmpl220_Init_0994fe       | +00370  -> $0994FE
        .dc.l   Mob_Tmpl221_Init_0995cc       | +00374  -> $0995CC
        .dc.l   Mob_Tmpl222_Init_0996b6       | +00378  -> $0996B6
        .dc.w   0x0009,0x96d0,0x0009,0x96ea,0x0009,0x9704,0x0009,0x971e     | +0037c
        .dc.l   Continue_Text_DrawCredits_091630| +0038c  -> $091630
        .dc.l   Enemy46_Boot_046140           | +00390  -> $046140
        .dc.l   Enemy46_Boot_046140           | +00394  -> $046140
        .dc.l   Drop_SpawnRandom_046322       | +00398  -> $046322
        .dc.l   Drop_Spawn_TmplF2_0463ee      | +0039c  -> $0463EE
        .dc.l   Drop_Spawn_Tmpl1B_046404      | +003a0  -> $046404
        .dc.l   Drop_Spawn_Tmpl1B_B_04641a    | +003a4  -> $04641A
        .dc.l   Drop_Spawn_Tmpl6D_046430      | +003a8  -> $046430
        .dc.l   Drop_Spawn_Tmpl2C_046446      | +003ac  -> $046446
        .dc.l   Drop_Spawn_Tmpl2B_04645c      | +003b0  -> $04645C
        .dc.w   0x0004,0x6474,0x0004,0x647c                                 | +003b4
        .dc.l   Drop_Spawn_Tmpl3A_12F_046484  | +003bc  -> $046484
        .dc.l   Drop_SpawnFromTable_0464a6    | +003c0  -> $0464A6
        .dc.w   0x0004,0x64b2                                               | +003c4
        .dc.l   Drop_Spawn_Tmpl4F_0463da      | +003c8  -> $0463DA
        .dc.l   Drop_SpawnThrown_046534       | +003cc  -> $046534
        .dc.l   Cut_Dropper_08ccea            | +003d0  -> $08CCEA
        .dc.l   Cut_Item_08ce1e               | +003d4  -> $08CE1E
        .dc.l   Cut_SetMode2_08c7e2           | +003d8  -> $08C7E2
        .dc.l   ClearGlobalFlag_08c7f2        | +003dc  -> $08C7F2
        .dc.l   Cut_SetVariant1_08c800        | +003e0  -> $08C800
        .dc.l   Charger_Init_04290C           | +003e4  -> $04290C
        .dc.l   AutoDemo_RecordBest_07926c    | +003e8  -> $07926C
        .dc.l   AutoDemo_Tmpl_Driver_079298   | +003ec  -> $079298
        .dc.l   Heading16Sprite_Handler_0433BE| +003f0  -> $0433BE
        .dc.l   TaskHandler_083be2            | +003f4  -> $083BE2
        .dc.l   Proj_Drop_V0_08b34c           | +003f8  -> $08B34C
        .dc.l   Proj_Drop_V1_08b366           | +003fc  -> $08B366
        .dc.l   Proj_Drop_V2_08b380           | +00400  -> $08B380
        .dc.l   Proj_Drop_V3_08b39a           | +00404  -> $08B39A
        .dc.l   S5_Bunker_088ff4              | +00408  -> $088FF4
        .dc.l   Prop_Roof_04f2c2              | +0040c  -> $04F2C2
        .dc.l   Prop_Boat_04f7ca              | +00410  -> $04F7CA
        .dc.l   Prop_SignA_04f70e             | +00414  -> $04F70E
        .dc.l   Prop_SignB_04f76c             | +00418  -> $04F76C
        .dc.l   M4_CamFloor_Init_0859d4       | +0041c  -> $0859D4
        .dc.l   M4_Water_Spawn2_0857f2        | +00420  -> $0857F2
        .dc.l   M4_Debris_Init_0858d0         | +00424  -> $0858D0
        .dc.l   M4_Rail_SpawnRow_08577a       | +00428  -> $08577A
        .dc.l   TaskHandler_084f26            | +0042c  -> $084F26
        .dc.l   M4_Carrier_Init_085190        | +00430  -> $085190
        .dc.l   Heli_InitTmpl_087b26          | +00434  -> $087B26
        .dc.l   Proj_Thrown_08b258            | +00438  -> $08B258
        .dc.l   Prop_TrapFlame_053a42         | +0043c  -> $053A42
        .dc.l   Obstacle_Tmpl110_060764       | +00440  -> $060764
        .dc.l   Obstacle_Tmpl111_060a0e       | +00444  -> $060A0E
        .dc.w   0x0004,0xa802                                               | +00448
        .dc.l   TaskHandler_084db0            | +0044c  -> $084DB0
        .dc.l   ItemProp_Tmpl114_06063a       | +00450  -> $06063A
        .dc.l   ItemProp_Tmpl115_06065c       | +00454  -> $06065C
        .dc.l   ItemProp_Tmpl116_06067e       | +00458  -> $06067E
        .dc.l   ItemProp_Tmpl117_0606a0       | +0045c  -> $0606A0
        .dc.l   ItemProp_Tmpl118_0606c2       | +00460  -> $0606C2
        .dc.w   0x0004,0x2d42                                               | +00464
        .dc.l   Skirmisher_Init_042D3C        | +00468  -> $042D3C
        .dc.w   0x0009,0xa31a                                               | +0046c
        .dc.l   Item_Tmpl284_Ammo4_09a918     | +00470  -> $09A918
        .dc.l   Item_Tmpl285_Ammo3_09a976     | +00474  -> $09A976
        .dc.l   Item_Tmpl286_Ammo1_09a9d4     | +00478  -> $09A9D4
        .dc.l   Item_Tmpl287_Ammo2_09aa32     | +0047c  -> $09AA32
        .dc.l   Item_Tmpl288_AmmoSeq_09aa90   | +00480  -> $09AA90
        .dc.l   Item_Tmpl289_Weapon_09abc2    | +00484  -> $09ABC2
        .dc.l   Item_Tmpl290_WeaponSwap_09ac52| +00488  -> $09AC52
        .dc.l   Item_Tmpl291_Bombs_09ad54     | +0048c  -> $09AD54
        .dc.l   Slug_SpawnDropB_02aea4        | +00490  -> $02AEA4
        .dc.l   Item_Tmpl293_Pow_09b3c6       | +00494  -> $09B3C6
        .dc.l   Item_Tmpl294_Food_09aec4      | +00498  -> $09AEC4
        .dc.w   0x0009,0xaed8,0x0009,0xb11e,0x0009,0xaeec,0x0009,0xaf02     | +0049c
        .dc.w   0x0009,0xaf16,0x0009,0xaf3e,0x0009,0xaf2a,0x0009,0xb1ca     | +004ac
        .dc.w   0x0009,0xaf52,0x0009,0xaf68,0x0009,0xaf7c,0x0009,0xaf90     | +004bc
        .dc.w   0x0009,0xafa4,0x0009,0xafb8,0x0009,0xafcc,0x0009,0xafe0     | +004cc
        .dc.w   0x0009,0xaff4,0x0009,0xb034,0x0009,0xb048,0x0009,0xb008     | +004dc
        .dc.w   0x0009,0xb01e                                               | +004ec
        .dc.l   Item_Tmpl316_ComboTimer_09b82c| +004f0  -> $09B82C
        .dc.l   Item_Tmpl317_Static_09bb56    | +004f4  -> $09BB56
        .dc.l   Miniboss2_Attach_045DE4       | +004f8  -> $045DE4
        .dc.w   0x0006,0x05b6                                               | +004fc
        .dc.l   Cut_SetVariant2_08c810        | +00500  -> $08C810
        .dc.w   0x0028,0xdb5a,0x0028,0xdb6a,0x0003,0x8f14                   | +00504
        .dc.l   DuckTrigger_SpawnPairLeft_038f48| +00510  -> $038F48
        .dc.l   ClearGlobalFlag_029598        | +00514  -> $029598
        .dc.l   SetGlobalFlagFF_029588        | +00518  -> $029588
        .dc.l   Continue_StoreCount_Exit_0469e2| +0051c  -> $0469E2
        .dc.l   Allen_Init_0508d6             | +00520  -> $0508D6
