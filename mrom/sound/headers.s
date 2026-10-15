/* headers.s - music / SFX pointer tables (indexed by 68k command - 20h) and the
 * headers they point to (Seq_StartMusic_4FA3 / Seq_StartSfx_507B) */

MusicTable:			/* file 0x0815f; BankPtrTable_0080[1]; 68k cmd 20h+id with type 00h */
	PTR24 Music_MissionComplete_Hdr		/*   0 */
	PTR24 Music_Mission1_Hdr		/*   1 */
	PTR24 Music_Mission2_Hdr		/*   2 */
	PTR24 Music_Mission3_Hdr		/*   3 */
	PTR24 Music_Mission4_Hdr		/*   4 */
	PTR24 Music_Mission5_Hdr		/*   5 */
	PTR24 Music_Mission6_Hdr		/*   6 */
	PTR24 Music_AttractDemo_Hdr		/*   7 */
	PTR24 Music_Boss_Hdr		/*   8 */
	PTR24 Music_SceneB_Hdr		/*   9 */
	PTR24 Music_SceneC_Hdr		/*  10 */
	PTR24 Music_Ending_Hdr		/*  11 */
	PTR24 Music_Title_Hdr		/*  12 */

Music_MissionComplete_Hdr:			/* file 0x08193; music 0, 68k id 0020; used by MissionComplete_Task_04720c, BannerComplete_Boot_07A9F0 */
	MUSIC_HDR 0x76
	TRACK Music_MissionComplete_Trk0, 0
	TRACK Music_MissionComplete_Trk1, 0
	TRACK Music_MissionComplete_Trk2, 0
	TRACK Music_MissionComplete_Trk3, 0
	TRACK Music_MissionComplete_Trk4, 0
	TRACK Music_MissionComplete_Trk5, 0
	TRACK Music_MissionComplete_Trk6, 0
	TRACK Music_MissionComplete_Trk7, 0
	TRACK Music_MissionComplete_Trk8, 0
	TRACK Music_MissionComplete_Trk9, 255
	TRACK Music_MissionComplete_Trk10, 0
	TRACKS_END

Music_Mission1_Hdr:			/* file 0x081cf; music 1, 68k id 0021; used by MissionMusicTable_28DF00[0], MissionMusicTable_28DF00[6], MissionMusicTable_28DF00[7] */
	MUSIC_HDR 0x7e
	TRACK Music_Mission1_Trk0, 0
	TRACK Music_Mission1_Trk1, 0
	TRACK Music_Mission1_Trk2, 0
	TRACK Music_Mission1_Trk3, 0
	TRACK Music_Mission1_Trk4, 255
	TRACK Music_Mission1_Trk5, 0
	TRACK Music_Mission1_Trk6, 0
	TRACK Music_Mission1_Trk7, 0
	TRACK Music_Mission1_Trk8, 255
	TRACK Music_Mission1_Trk9, 255
	TRACKS_END

Music_Mission2_Hdr:			/* file 0x08206; music 2, 68k id 0022; used by MissionMusicTable_28DF00[1] */
	MUSIC_HDR 0x82
	TRACK Music_Mission2_Trk0, 0
	TRACK Music_Mission2_Trk1, 0
	TRACK Music_Mission2_Trk2, 0
	TRACK Music_Mission2_Trk3, 0
	TRACK Music_Mission2_Trk4, 255
	TRACK Music_Mission2_Trk5, 0
	TRACK Music_Mission2_Trk6, 0
	TRACK Music_Mission2_Trk7, 0
	TRACK Music_Mission2_Trk8, 255
	TRACK Music_Mission2_Trk9, 255
	TRACK Music_Mission2_Trk10, 0
	TRACK Music_Mission2_Trk11, 255
	TRACK Music_Mission2_Trk12, 255
	TRACKS_END

Music_Mission3_Hdr:			/* file 0x0824c; music 3, 68k id 0023; used by M2Boss_Turret_DeathFall_07c424, Airship_Depart_0895fe, MissionMusicTable_28DF00[2] */
	MUSIC_HDR 0x49
	TRACK Music_Mission3_Trk0, 0
	TRACK Music_Mission3_Trk1, 0
	TRACK Music_Mission3_Trk2, 0
	TRACK Music_Mission3_Trk3, 0
	TRACK Music_Mission3_Trk4, 0
	TRACK Music_Mission3_Trk5, 255
	TRACK Music_Mission3_Trk6, 0
	TRACK Music_Mission3_Trk7, 0
	TRACK Music_Mission3_Trk8, 255
	TRACK Music_Mission3_Trk9, 255
	TRACKS_END

Music_Mission4_Hdr:			/* file 0x08283; music 4, 68k id 0024; used by MissionMusicTable_28DF00[3] */
	MUSIC_HDR 0x81
	TRACK Music_Mission4_Trk0, 0
	TRACK Music_Mission4_Trk1, 0
	TRACK Music_Mission4_Trk2, 0
	TRACK Music_Mission4_Trk3, 0
	TRACK Music_Mission4_Trk4, 0
	TRACK Music_Mission4_Trk5, 0
	TRACK Music_Mission4_Trk6, 0
	TRACK Music_Mission4_Trk7, 0
	TRACK Music_Mission4_Trk8, 0
	TRACK Music_Mission4_Trk9, 0
	TRACKS_END

Music_Mission5_Hdr:			/* file 0x082ba; music 5, 68k id 0025; used by MissionMusicTable_28DF00[4] */
	MUSIC_HDR 0x80
	TRACK Music_Mission5_Trk0, 0
	TRACK Music_Mission5_Trk1, 0
	TRACK Music_Mission5_Trk2, 0
	TRACK Music_Mission5_Trk3, 0
	TRACK Music_Mission5_Trk4, 0
	TRACK Music_Mission5_Trk5, 0
	TRACK Music_Mission5_Trk6, 0
	TRACK Music_Mission5_Trk7, 0
	TRACK Music_Mission5_Trk8, 0
	TRACKS_END

Music_Mission6_Hdr:			/* file 0x082ec; music 6, 68k id 0026; used by MissionMusicTable_28DF00[5] */
	MUSIC_HDR 0x82
	TRACK Music_Mission6_Trk0, 0
	TRACK Music_Mission6_Trk1, 0
	TRACK Music_Mission6_Trk2, 0
	TRACK Music_Mission6_Trk3, 0
	TRACK Music_Mission6_Trk4, 0
	TRACK Music_Mission6_Trk5, 0
	TRACK Music_Mission6_Trk6, 0
	TRACK Music_Mission6_Trk7, 0
	TRACK Music_Mission6_Trk8, 0
	TRACK Music_Mission6_Trk9, 0
	TRACK Music_Mission6_Trk10, 0
	TRACKS_END

Music_AttractDemo_Hdr:			/* file 0x08328; music 7, 68k id 0027; used by Attract_InitShow27_TaskAdd_0017E6 */
	MUSIC_HDR 0x6c
	TRACK Music_AttractDemo_Trk0, 0
	TRACK Music_AttractDemo_Trk1, 0
	TRACK Music_AttractDemo_Trk2, 0
	TRACK Music_AttractDemo_Trk3, 0
	TRACK Music_AttractDemo_Trk4, 0
	TRACK Music_AttractDemo_Trk5, 0
	TRACK Music_AttractDemo_Trk6, 0
	TRACK Music_AttractDemo_Trk7, 0
	TRACK Music_AttractDemo_Trk8, 0
	TRACKS_END

Music_Boss_Hdr:			/* file 0x0835a; music 8, 68k id 0028; used by FinalBoss_Tmpl72_071068, M5Tank_Tmpl76_072fc8, ScriptedProp_Run_0751a6, M2Boss_IntroAnim_07bb1a (+1) */
	MUSIC_HDR 0x86
	TRACK Music_Boss_Trk0, 0
	TRACK Music_Boss_Trk1, 0
	TRACK Music_Boss_Trk2, 0
	TRACK Music_Boss_Trk3, 0
	TRACK Music_Boss_Trk4, 0
	TRACK Music_Boss_Trk5, 0
	TRACK Music_Boss_Trk6, 0
	TRACK Music_Boss_Trk7, 0
	TRACK Music_Boss_Trk8, 0
	TRACK Music_Boss_Trk9, 0
	TRACK Music_Boss_Trk10, 0
	TRACKS_END

Music_SceneB_Hdr:			/* file 0x08396; music 9, 68k id 0029; used by SceneB_Init_08ce64 */
	MUSIC_HDR 0x7a
	TRACK Music_SceneB_Trk0, 0
	TRACK Music_SceneB_Trk1, 0
	TRACK Music_SceneB_Trk2, 0
	TRACK Music_SceneB_Trk3, 0
	TRACK Music_SceneB_Trk4, 0
	TRACK Music_SceneB_Trk5, 0
	TRACK Music_SceneB_Trk6, 0
	TRACK Music_SceneB_Trk7, 0
	TRACK Music_SceneB_Trk8, 0
	TRACK Music_SceneB_Trk9, 0
	TRACK Music_SceneB_Trk10, 0
	TRACKS_END

Music_SceneC_Hdr:			/* file 0x083d2; music 10, 68k id 002a; used by SceneC_Init_08d0a8 */
	MUSIC_HDR 0x7a
	TRACK Music_SceneC_Trk0, 0
	TRACK Music_SceneC_Trk1, 0
	TRACK Music_SceneC_Trk2, 0
	TRACK Music_SceneC_Trk3, 0
	TRACK Music_SceneC_Trk4, 0
	TRACK Music_SceneC_Trk5, 0
	TRACK Music_SceneC_Trk6, 0
	TRACK Music_SceneC_Trk7, 0
	TRACK Music_SceneC_Trk8, 0
	TRACK Music_SceneC_Trk9, 0
	TRACK Music_SceneC_Trk10, 0
	TRACK Music_SceneC_Trk11, 0
	TRACK Music_SceneC_Trk12, 0
	TRACK Music_SceneC_Trk13, 0
	TRACKS_END

Music_Ending_Hdr:			/* file 0x0841d; music 11, 68k id 002b; used by Ending_SpawnOrbitFx_0598fc, Anim_State_F1_08C008 */
	MUSIC_HDR 0x76
	TRACK Music_Ending_Trk0, 0
	TRACK Music_Ending_Trk1, 0
	TRACK Music_Ending_Trk2, 0
	TRACK Music_Ending_Trk3, 0
	TRACK Music_Ending_Trk4, 0
	TRACKS_END

Music_Title_Hdr:			/* file 0x0843b; music 12, 68k id 002c; used by LogoScene_Tpl_098720 */
	MUSIC_HDR 0x32
	TRACK Music_Title_Trk0, 0
	TRACK Music_Title_Trk1, 0
	TRACK Music_Title_Trk2, 0
	TRACK Music_Title_Trk3, 0
	TRACK Music_Title_Trk4, 0
	TRACK Music_Title_Trk5, 0
	TRACK Music_Title_Trk6, 0
	TRACK Music_Title_Trk7, 0
	TRACK Music_Title_Trk8, 0
	TRACK Music_Title_Trk9, 0
	TRACKS_END

SfxTableA:			/* file 0x08472; BankPtrTable_0080[2]; type 10h (12h..1Fh alias it) */
	PTR24 SfxA_000_CamperDie_Hdr		/*   0 */
	PTR24 SfxA_001_DeathExplosion_Hdr		/*   1 */
	PTR24 SfxA_002_EnemyDeathCry_Hdr		/*   2 */
	PTR24 SfxA_003_StructureDestroyed_Hdr		/*   3 */
	PTR24 SfxA_004_LargePropBreak_Hdr		/*   4 */
	PTR24 SfxA_005_RocketHit_Hdr		/*   5 */
	PTR24 SfxAHdr_006		/*   6 */
	PTR24 SfxA_007_GrenadeExplosion_Hdr		/*   7 */
	PTR24 SfxA_008_StructureCollapse_Hdr		/*   8 */
	PTR24 SfxA_009_SignBreak_Hdr		/*   9 */
	PTR24 SfxAHdr_010		/*  10 */
	PTR24 SfxA_011_ArmorDamaged_Hdr		/*  11 */
	PTR24 SfxAHdr_012		/*  12 */
	PTR24 SfxA_013_TurretDestroyed_Hdr		/*  13 */
	PTR24 SfxA_014_SmallPropBreak_Hdr		/*  14 */
	PTR24 SfxA_015_PropDamaged_Hdr		/*  15 */
	PTR24 SfxA_016_PropWreck_Hdr		/*  16 */
	PTR24 0		/*  17 */
	PTR24 SfxA_018_BigExplosion_Hdr		/*  18 */
	PTR24 SfxAHdr_019		/*  19 */
	PTR24 SfxA_020_SlugExplode_Hdr		/*  20 */
	PTR24 SfxAHdr_021		/*  21 */
	PTR24 SfxAHdr_022		/*  22 */
	PTR24 SfxAHdr_023		/*  23 */
	PTR24 SfxAHdr_024		/*  24 */
	PTR24 SfxA_025_CrateBurst_Hdr		/*  25 */
	PTR24 0		/*  26 */
	PTR24 0		/*  27 */
	PTR24 0		/*  28 */
	PTR24 0		/*  29 */
	PTR24 SfxA_030_BarrelPop_Hdr		/*  30 */
	PTR24 SfxA_031_Touchdown_Hdr		/*  31 */
	PTR24 SfxAHdr_032		/*  32 */
	PTR24 SfxA_033_FinalBossHeadDie_Hdr		/*  33 */
	PTR24 SfxA_034_BurningScream_Hdr		/*  34 */
	PTR24 SfxA_035_ParatrooperDie_Hdr		/*  35 */
	PTR24 SfxAHdr_036		/*  36 */
	PTR24 0		/*  37 */
	PTR24 0		/*  38 */
	PTR24 0		/*  39 */
	PTR24 SfxAHdr_040		/*  40 */
	PTR24 0		/*  41 */
	PTR24 SfxAHdr_042		/*  42 */
	PTR24 SfxAHdr_043		/*  43 */
	PTR24 SfxAHdr_044		/*  44 */
	PTR24 SfxAHdr_045		/*  45 */
	PTR24 SfxAHdr_046		/*  46 */
	PTR24 SfxAHdr_047		/*  47 */
	PTR24 0		/*  48 */
	PTR24 SfxAHdr_049		/*  49 */
	PTR24 SfxA_050_WaterSplash_Hdr		/*  50 */
	PTR24 SfxA_051_PlayerDeath_Hdr		/*  51 */
	PTR24 SfxAHdr_052		/*  52 */
	PTR24 SfxAHdr_053		/*  53 */
	PTR24 SfxAHdr_054		/*  54 */
	PTR24 0		/*  55 */
	PTR24 0		/*  56 */
	PTR24 0		/*  57 */
	PTR24 0		/*  58 */
	PTR24 0		/*  59 */
	PTR24 SfxA_060_HostageThrown_Hdr		/*  60 */
	PTR24 SfxA_061_HostageAppear_Hdr		/*  61 */
	PTR24 SfxA_062_HostageRun_Hdr		/*  62 */
	PTR24 0		/*  63 */
	PTR24 SfxA_064_HostagePose_Hdr		/*  64 */
	PTR24 SfxA_065_CrabLegDetach_Hdr		/*  65 */
	PTR24 SfxAHdr_066		/*  66 */
	PTR24 0		/*  67 */
	PTR24 SfxA_068_ShellToss_Hdr		/*  68 */
	PTR24 SfxAHdr_069		/*  69 */
	PTR24 SfxAHdr_070		/*  70 */
	PTR24 SfxAHdr_071		/*  71 */
	PTR24 0		/*  72 */
	PTR24 SfxAHdr_073		/*  73 */
	PTR24 SfxA_074_BossExplode_Hdr		/*  74 */
	PTR24 0		/*  75 */
	PTR24 0		/*  76 */
	PTR24 0		/*  77 */
	PTR24 0		/*  78 */
	PTR24 0		/*  79 */
	PTR24 0		/*  80 */
	PTR24 SfxAHdr_081		/*  81 */
	PTR24 0		/*  82 */
	PTR24 SfxAHdr_083		/*  83 */
	PTR24 SfxAHdr_084		/*  84 */
	PTR24 SfxA_085_M5BossAttack_Hdr		/*  85 */
	PTR24 SfxA_086_ExplosiveProp_Hdr		/*  86 */
	PTR24 SfxAHdr_087		/*  87 */
	PTR24 0		/*  88 */
	PTR24 0		/*  89 */
	PTR24 0		/*  90 */
	PTR24 0		/*  91 */
	PTR24 0		/*  92 */
	PTR24 0		/*  93 */
	PTR24 0		/*  94 */
	PTR24 SfxAHdr_095		/*  95 */
	PTR24 SfxAHdr_096		/*  96 */
	PTR24 SfxAHdr_097		/*  97 */
	PTR24 0		/*  98 */
	PTR24 SfxA_099_SlugHPUp_Hdr		/*  99 */
	PTR24 SfxA_100_VehicleLaunch_Hdr		/* 100 */
	PTR24 SfxAHdr_101		/* 101 */
	PTR24 SfxA_102_FireBurst_Hdr		/* 102 */
	PTR24 SfxAHdr_103		/* 103 */
	PTR24 0		/* 104 */
	PTR24 SfxA_105_SlugDamage_Hdr		/* 105 */
	PTR24 SfxAHdr_106		/* 106 */
	PTR24 SfxAHdr_107		/* 107 */
	PTR24 SfxA_108_EnemyShot_Hdr		/* 108 */
	PTR24 SfxA_109_TurretShell_Hdr		/* 109 */
	PTR24 SfxA_110_DoorBreak_Hdr		/* 110 */
	PTR24 SfxAHdr_111		/* 111 */
	PTR24 SfxAHdr_112		/* 112 */
	PTR24 SfxAHdr_113		/* 113 */
	PTR24 SfxAHdr_114		/* 114 */
	PTR24 0		/* 115 */
	PTR24 0		/* 116 */
	PTR24 0		/* 117 */
	PTR24 0		/* 118 */
	PTR24 0		/* 119 */
	PTR24 SfxA_120_TrapFlame_Hdr		/* 120 */
	PTR24 SfxAHdr_121		/* 121 */
	PTR24 0		/* 122 */
	PTR24 SfxAHdr_123		/* 123 */
	PTR24 SfxAHdr_124		/* 124 */
	PTR24 SfxAHdr_125		/* 125 */
	PTR24 SfxA_126_TankEngine_Hdr		/* 126 */
	PTR24 SfxAHdr_127		/* 127 */
	PTR24 SfxAHdr_128		/* 128 */
	PTR24 SfxAHdr_129		/* 129 */
	PTR24 SfxAHdr_130		/* 130 */
	PTR24 SfxAHdr_131		/* 131 */
	PTR24 SfxAHdr_132		/* 132 */
	PTR24 SfxA_133_Mortar_Hdr		/* 133 */
	PTR24 SfxAHdr_134		/* 134 */
	PTR24 SfxAHdr_135		/* 135 */
	PTR24 SfxA_136_HatchOpen_Hdr		/* 136 */
	PTR24 SfxAHdr_137		/* 137 */
	PTR24 SfxAHdr_138		/* 138 */
	PTR24 SfxAHdr_139		/* 139 */
	PTR24 SfxAHdr_140		/* 140 */
	PTR24 SfxAHdr_141		/* 141 */
	PTR24 SfxA_142_HutDoor_Hdr		/* 142 */
	PTR24 SfxA_143_SlugAccelRight_Hdr		/* 143 */
	PTR24 SfxA_144_SlugAccelLeft_Hdr		/* 144 */
	PTR24 SfxAHdr_145		/* 145 */
	PTR24 SfxA_146_SlugDestroyedSlide_Hdr		/* 146 */
	PTR24 SfxAHdr_147		/* 147 */
	PTR24 SfxAHdr_148		/* 148 */
	PTR24 SfxAHdr_149		/* 149 */
	PTR24 SfxAHdr_150		/* 150 */
	PTR24 SfxAHdr_151		/* 151 */
	PTR24 SfxA_152_GunshipAttack_Hdr		/* 152 */
	PTR24 SfxAHdr_153		/* 153 */
	PTR24 SfxAHdr_154		/* 154 */
	PTR24 SfxA_155_WallBreak_Hdr		/* 155 */
	PTR24 0		/* 156 */
	PTR24 0		/* 157 */
	PTR24 0		/* 158 */
	PTR24 SfxAHdr_159		/* 159 */
	PTR24 SfxAHdr_160		/* 160 */
	PTR24 SfxAHdr_161		/* 161 */
	PTR24 SfxAHdr_162		/* 162 */
	PTR24 SfxAHdr_163		/* 163 */
	PTR24 SfxAHdr_164		/* 164 */
	PTR24 SfxAHdr_165		/* 165 */
	PTR24 SfxAHdr_166		/* 166 */
	PTR24 0		/* 167 */
	PTR24 0		/* 168 */
	PTR24 0		/* 169 */
	PTR24 0		/* 170 */
	PTR24 0		/* 171 */
	PTR24 0		/* 172 */
	PTR24 0		/* 173 */
	PTR24 0		/* 174 */
	PTR24 0		/* 175 */
	PTR24 0		/* 176 */
	PTR24 0		/* 177 */
	PTR24 0		/* 178 */
	PTR24 0		/* 179 */
	PTR24 SfxA_180_ContinueCountdown_Hdr		/* 180 */
	PTR24 SfxAHdr_181		/* 181 */
	PTR24 SfxA_182_ResultsPrize_Hdr		/* 182 */
	PTR24 SfxAHdr_183		/* 183 */
	PTR24 SfxA_184_ResultsTally_Hdr		/* 184 */
	PTR24 0		/* 185 */
	PTR24 0		/* 186 */
	PTR24 0		/* 187 */
	PTR24 0		/* 188 */
	PTR24 0		/* 189 */
	PTR24 SfxA_190_GameOverFinal_Hdr		/* 190 */
	PTR24 SfxA_191_GameOverBanner_Hdr		/* 191 */
	PTR24 SfxA_192_PauseSlide_Hdr		/* 192 */
	PTR24 SfxA_193_Pause_Hdr		/* 193 */
	PTR24 SfxAHdr_194		/* 194 */
	PTR24 SfxAHdr_195		/* 195 */
	PTR24 SfxAHdr_196		/* 196 */
	PTR24 0		/* 197 */
	PTR24 0		/* 198 */
	PTR24 0		/* 199 */
	PTR24 SfxA_200_AimTurret_Hdr		/* 200 */
	PTR24 SfxA_201_SlugDestroyed_Hdr		/* 201 */
	PTR24 SfxA_202_FragLaunch_Hdr		/* 202 */
	PTR24 0		/* 203 */
	PTR24 0		/* 204 */
	PTR24 0		/* 205 */
	PTR24 0		/* 206 */
	PTR24 0		/* 207 */
	PTR24 0		/* 208 */
	PTR24 0		/* 209 */
	PTR24 SfxA_210_VehicleCrash_Hdr		/* 210 */
	PTR24 SfxA_211_ShotgunFire_Hdr		/* 211 */
	PTR24 SfxA_212_FlameFire_Hdr		/* 212 */
	PTR24 SfxA_213_RocketFire_Hdr		/* 213 */
	PTR24 SfxA_214_HMGFire_Hdr		/* 214 */
	PTR24 SfxA_215_PistolFire_Hdr		/* 215 */
	PTR24 0		/* 216 */
	PTR24 0		/* 217 */
	PTR24 SfxAHdr_218		/* 218 */
	PTR24 SfxAHdr_219		/* 219 */
	PTR24 SfxAHdr_220		/* 220 */
	PTR24 SfxA_221_AmmoPickup_Hdr		/* 221 */
	PTR24 0		/* 222 */
	PTR24 SfxA_223_ShieldBreak_Hdr		/* 223 */

SfxTableB:			/* file 0x087f2; BankPtrTable_0080[3]; type 11h */
	PTR24 0		/*   0 */
	PTR24 Voice_FoodPickup_Hdr		/*   1 */
	PTR24 0		/*   2 */
	PTR24 Voice_PlayerDeath_Hdr		/*   3 */
	PTR24 Voice_Okay_Hdr		/*   4 */
	PTR24 0		/*   5 */
	PTR24 Voice_Bingo_Hdr		/*   6 */
	PTR24 Voice_LetsGo_Hdr		/*   7 */
	PTR24 Voice_HereWeGo_Hdr		/*   8 */
	PTR24 Voice_Yeah_Hdr		/*   9 */
	PTR24 Voice_YeeHaw_Hdr		/*  10 */
	PTR24 Voice_PowerUp_Hdr		/*  11 */
	PTR24 Voice_FlameShot_Hdr		/*  12 */
	PTR24 Voice_HeavyMachineGun_Hdr		/*  13 */
	PTR24 Voice_RocketLauncher_Hdr		/*  14 */
	PTR24 Voice_Shotgun_Hdr		/*  15 */
	PTR24 0		/*  16 */
	PTR24 Voice_Mission1Start_Hdr		/*  17 */
	PTR24 Voice_Mission2Start_Hdr		/*  18 */
	PTR24 Voice_Mission3Start_Hdr		/*  19 */
	PTR24 Voice_Mission4Start_Hdr		/*  20 */
	PTR24 Voice_Mission5Start_Hdr		/*  21 */
	PTR24 Voice_Mission6Start_Hdr		/*  22 */
	PTR24 Voice_MissionComplete_Hdr		/*  23 */
	PTR24 Voice_GameOver_Hdr		/*  24 */
	PTR24 0		/*  25 */
	PTR24 Voice_PowThankYou_Hdr		/*  26 */
	PTR24 0		/*  27 */
	PTR24 Voice_AllenComeOnBoy_Hdr		/*  28 */
	PTR24 Voice_AllenGoHomeToMommy_Hdr		/*  29 */
	PTR24 Voice_AllenMincemeat_Hdr		/*  30 */
	PTR24 Voice_AllenLaugh_Hdr		/*  31 */
	PTR24 Voice_AllenSeeYouInHell_Hdr		/*  32 */

SfxA_000_CamperDie_Hdr:			/* file 0x08876; SFX A 0, 68k id 1020; used by Camper_Die_062d72 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_000_CamperDie_Trk0, 12
	TRACKS_END

SfxA_001_DeathExplosion_Hdr:			/* file 0x08888; SFX A 1, 68k id 1021; used by Camper_SpawnPairD_06355c, Hostage_Die_064a24, Bazooka_DeathExplodeA_06b9a4, Bazooka_DeathExplodeB_06b9da (+7) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_001_DeathExplosion_Trk0, 13
	TRACKS_END

SfxA_002_EnemyDeathCry_Hdr:			/* file 0x0889a; SFX A 2, 68k id 1022; used by PairChild_DeathCry_041762, SquadChild_GlideDeath_0419BE, TrioChild_OrbitDeath_041BB8, BossShot_Fly_044DA4 (+16) */
	SFX_HDR 0x109b, 0x10a3, 0x0000, 0x0000, 0x01
	TRACK SfxA_002_EnemyDeathCry_Trk0, 15
	TRACKS_END

SfxA_003_StructureDestroyed_Hdr:			/* file 0x088ac; SFX A 3, 68k id 1023; used by GunPlatform_Destroyed_04c5a4, TurretCar_Body_04cf2e, Prop_FortressWreck_04ed00, Prop_Tower_053306 (+15) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_003_StructureDestroyed_Trk0, 13
	TRACKS_END

SfxA_004_LargePropBreak_Hdr:			/* file 0x088be; SFX A 4, 68k id 1024; used by Prop_Large_052b8a */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_004_LargePropBreak_Trk0, 13
	TRACKS_END

SfxA_005_RocketHit_Hdr:			/* file 0x088d0; SFX A 5, 68k id 1025; used by Bazooka_RocketHit_06b1d4, RocketVehicle_RocketHit_06d04e, Crate_Opened_09c00a */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_005_RocketHit_Trk0, 15
	TRACKS_END

SfxAHdr_006:			/* file 0x088e2; SFX A 6, 68k id 1026; used by JsrAbsThunk_086034, Airship_Depart_0895fe, Wreck_SmokeRise_08b07c */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA006_Trk0, 12
	TRACKS_END

SfxA_007_GrenadeExplosion_Hdr:			/* file 0x088f4; SFX A 7, 68k id 1027; used by GunPlatform_HitCheck_04c6ea, Allen_GrenadeExplode_051392, Prop_Breakable2Stage_0527ba, Prop_MultiStage_054cf2 (+11) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_007_GrenadeExplosion_Trk0, 17
	TRACKS_END

SfxA_008_StructureCollapse_Hdr:			/* file 0x08906; SFX A 8, 68k id 1028; used by Prop_HutRoofFall_04da52, Prop_TowerBase_04dce6, Prop_TowerBaseWreck_04ddec, Prop_BridgeCollapse_04e05e (+6) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_008_StructureCollapse_Trk0, 12
	TRACKS_END

SfxA_009_SignBreak_Hdr:			/* file 0x08918; SFX A 9, 68k id 1029; used by Prop_SignBroken_052986, Prop_SignWreck_0529ca */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_009_SignBreak_Trk0, 14
	TRACKS_END

SfxAHdr_010:			/* file 0x0892a; SFX A 10, 68k id 102a; used by Prop_CompoundWall_054254, Prop_NeonSign_0545c0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA010_Trk0, 17
	TRACKS_END

SfxA_011_ArmorDamaged_Hdr:			/* file 0x0893c; SFX A 11, 68k id 102b; used by Prop_CompoundWallDamaged_054338, Prop_ScaledHP_054c0e, ArmoredCar_Damaged1_0878d8, ArmoredCar_Damaged2_087956 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_011_ArmorDamaged_Trk0, 13
	TRACKS_END

SfxAHdr_012:			/* file 0x0894e; SFX A 12, 68k id 102c; used by Prop_PlayBreakMusicByPhase_053e0c, Prop_Stall_0546f4, M5Tank_TurretWreck_0738da, Pillbox_Collapse_086ba4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA012_Trk0, 13
	TRACKS_END

SfxA_013_TurretDestroyed_Hdr:			/* file 0x08960; SFX A 13, 68k id 102d; used by Tank_Turret_069a32, M2Boss_Turret_DeathSwing_07c374, M4_Carrier_Fight_085270 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_013_TurretDestroyed_Trk0, 13
	TRACKS_END

SfxA_014_SmallPropBreak_Hdr:			/* file 0x08972; SFX A 14, 68k id 102e; used by Prop_Breakable_05495a, Prop_Small_054b1c, TaskHandler_083c92, TaskHandler_0846e0 (+3) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_014_SmallPropBreak_Trk0, 14
	TRACKS_END

SfxA_015_PropDamaged_Hdr:			/* file 0x08984; SFX A 15, 68k id 102f; used by Prop_BridgePillar_04e12c, Prop_NestActive_04e2b0, Prop_BarrierDamaged_04e68a, Prop_GatehouseDamaged_04e998 (+10) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_015_PropDamaged_Trk0, 12
	TRACKS_END

SfxA_016_PropWreck_Hdr:			/* file 0x08996; SFX A 16, 68k id 1030; used by Prop_TowerTopWreck_04dc4a, Prop_BunkerWreck_04df30, Prop_Bridge_04df98, Prop_NestWreck_04e32c (+11) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_016_PropWreck_Trk0, 12
	TRACKS_END

SfxA_018_BigExplosion_Hdr:			/* file 0x089a8; SFX A 18, 68k id 1032; used by SquadLeader_Death_040be6, S5Gate_Begin_068346, M5Tank_Phase3_0732e8, M2Boss_Turret_DeathSwing_07c374 (+2) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_018_BigExplosion_Trk0, 12
	TRACKS_END

SfxAHdr_019:			/* file 0x089ba; SFX A 19, 68k id 1033; used by SquadLeader_DeathDone_040d18, LateProp_RegisterB_062816, LateProp_SpawnPair_06282e, Mortar_SpawnDebris_063db2 (+12) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA019_Trk0, 12
	TRACKS_END

SfxA_020_SlugExplode_Hdr:			/* file 0x089cc; SFX A 20, 68k id 1034; used by Slug_ExplodeFx_02ff22 */
	SFX_HDR 0x10e9, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_020_SlugExplode_Trk0, 10
	TRACKS_END

SfxAHdr_021:			/* file 0x089de; SFX A 21, 68k id 1035; used by Prop_HouseVariants_052e20, Prop_PlayBreakMusicByPhase_053e0c, Prop_MultiStageCritical_054eea, S5_Depot_089290 (+6) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA021_Trk0, 13
	TRACKS_END

SfxAHdr_022:			/* file 0x089f0; SFX A 22, 68k id 1036 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA022_Trk0, 13
	TRACKS_END

SfxAHdr_023:			/* file 0x08a02; SFX A 23, 68k id 1037; used by TaskHandler_0834a4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA023_Trk0, 13
	TRACKS_END

SfxAHdr_024:			/* file 0x08a14; SFX A 24, 68k id 1038; used by ArmoredCar_Damaged2_087956 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA024_Trk0, 13
	TRACKS_END

SfxA_025_CrateBurst_Hdr:			/* file 0x08a26; SFX A 25, 68k id 1039; used by Crate_Burst_086db4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_025_CrateBurst_Trk0, 13
	TRACKS_END

SfxA_030_BarrelPop_Hdr:			/* file 0x08a38; SFX A 30, 68k id 103e; used by FloatBarrel_Pop_066228 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA_030_BarrelPop_Trk0, 32
	TRACKS_END

SfxA_031_Touchdown_Hdr:			/* file 0x08a4a; SFX A 31, 68k id 103f; used by SquadChild_FlipTouchdown_04148A */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA_031_Touchdown_Trk0, 33
	TRACKS_END

SfxAHdr_032:			/* file 0x08a5c; SFX A 32, 68k id 1040; used by TaskHandler_084bd2 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA032_Trk0, 32
	TRACKS_END

SfxA_033_FinalBossHeadDie_Hdr:			/* file 0x08a6e; SFX A 33, 68k id 1041; used by FinalBoss_HeadDie_071a90 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA_033_FinalBossHeadDie_Trk0, 32
	TRACKS_END

SfxA_034_BurningScream_Hdr:			/* file 0x08a80; SFX A 34, 68k id 1042; used by HumanDeath_Burning_04a9b0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA_034_BurningScream_Trk0, 32
	TRACKS_END

SfxA_035_ParatrooperDie_Hdr:			/* file 0x08a92; SFX A 35, 68k id 1043; used by Paratrooper_Die_0669ea */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA_035_ParatrooperDie_Trk0, 33
	TRACKS_END

SfxAHdr_036:			/* file 0x08aa4; SFX A 36, 68k id 1044 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA036_Trk0, 30
	TRACKS_END

SfxAHdr_040:			/* file 0x08ab6; SFX A 40, 68k id 1048 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA040_Trk0, 31
	TRACKS_END

SfxAHdr_042:			/* file 0x08ac8; SFX A 42, 68k id 104a */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA042_Trk0, 31
	TRACKS_END

SfxAHdr_043:			/* file 0x08ada; SFX A 43, 68k id 104b */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA043_Trk0, 31
	TRACKS_END

SfxAHdr_044:			/* file 0x08aec; SFX A 44, 68k id 104c */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA044_Trk0, 31
	TRACKS_END

SfxAHdr_045:			/* file 0x08afe; SFX A 45, 68k id 104d */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA045_Trk0, 31
	TRACKS_END

SfxAHdr_046:			/* file 0x08b10; SFX A 46, 68k id 104e */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA046_Trk0, 31
	TRACKS_END

SfxAHdr_047:			/* file 0x08b22; SFX A 47, 68k id 104f */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA047_Trk0, 30
	TRACKS_END

SfxAHdr_049:			/* file 0x08b34; SFX A 49, 68k id 1051; used by JsrAbsThunk_02a630, Wreck_SparkBurst_08b10a */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK SfxA049_Trk0, 51
	TRACKS_END

SfxA_050_WaterSplash_Hdr:			/* file 0x08b46; SFX A 50, 68k id 1052; used by JsrAbsThunk_02a5fe, FloatBarrel_Splash_0661d8, FloatBarrel_BobSettle_0662a4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK SfxA_050_WaterSplash_Trk0, 51
	TRACKS_END

SfxA_051_PlayerDeath_Hdr:			/* file 0x08b58; SFX A 51, 68k id 1053; used by Player_Death_0375d2, Player_Death_Alt_037684, TowerSoldier_Flee_05f5fa, Airship_PlayerTracker_0896de */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK SfxA_051_PlayerDeath_Trk0, 50
	TRACKS_END

SfxAHdr_052:			/* file 0x08b6a; SFX A 52, 68k id 1054; used by Prop_FragileBreaking_0548c8, TaskHandler_083800 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK SfxA052_Trk0, 30
	TRACKS_END

SfxAHdr_053:			/* file 0x08b7c; SFX A 53, 68k id 1055 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA053_Trk0, 31
	TRACKS_END

SfxAHdr_054:			/* file 0x08b8e; SFX A 54, 68k id 1056 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA054_Trk0, 31
	TRACKS_END

SfxA_060_HostageThrown_Hdr:			/* file 0x08ba0; SFX A 60, 68k id 105c; used by Hostage_Thrown_064bfc */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA_060_HostageThrown_Trk0, 15
	TRACKS_END

SfxA_061_HostageAppear_Hdr:			/* file 0x08bb2; SFX A 61, 68k id 105d; used by Hostage_Init_064640 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA_061_HostageAppear_Trk0, 16
	TRACKS_END

SfxA_062_HostageRun_Hdr:			/* file 0x08bc4; SFX A 62, 68k id 105e; used by Hostage_Stand_064938, Hostage_Run_064b18 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA_062_HostageRun_Trk0, 15
	TRACKS_END

SfxA_064_HostagePose_Hdr:			/* file 0x08bd6; SFX A 64, 68k id 1060; used by Hostage_Pose_0647c0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA_064_HostagePose_Trk0, 16
	TRACKS_END

SfxA_065_CrabLegDetach_Hdr:			/* file 0x08be8; SFX A 65, 68k id 1061; used by Crab_Leg_Detach_07d898 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x04
	TRACK SfxA_065_CrabLegDetach_Trk0, 29
	TRACKS_END

SfxAHdr_066:			/* file 0x08bfa; SFX A 66, 68k id 1062 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x04
	TRACK SfxA066_Trk0, 29
	TRACKS_END

SfxA_068_ShellToss_Hdr:			/* file 0x08c0c; SFX A 68, 68k id 1064; used by SquadPair_SpawnCore_0420FE, SquadPair_SpawnPlain_042188, PowItem_Toss_048ba0, Gunner2_Shell_05a55a (+5) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_068_ShellToss_Trk0, 14
	TRACKS_END

SfxAHdr_069:			/* file 0x08c1e; SFX A 69, 68k id 1065; used by Walker_SpawnSparkAndDebris_06e3a4, M2Boss_Shell_Spawn_07c918 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA069_Trk0, 14
	TRACKS_END

SfxAHdr_070:			/* file 0x08c30; SFX A 70, 68k id 1066; used by Cannon_SpawnPair_06449e, Cannon_SpawnPairB_0644e8, JsrAbsThunk_072d9c */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA070_Trk0, 14
	TRACKS_END

SfxAHdr_071:			/* file 0x08c42; SFX A 71, 68k id 1067; used by GunPlatform_SpawnExplosionMusic_04ca58, LateProp_SpawnDebris2_0627ca, Camper_SpawnGroup_06347e */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA071_Trk0, 14
	TRACKS_END

SfxAHdr_073:			/* file 0x08c54; SFX A 73, 68k id 1069 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x04
	TRACK SfxA073_Trk0, 28
	TRACKS_END

SfxA_074_BossExplode_Hdr:			/* file 0x08c66; SFX A 74, 68k id 106a; used by Boss_Descend_044C28, Boss2_Explode_045C20, Boss2_Death_045CEA */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x04
	TRACK SfxA_074_BossExplode_Trk0, 28
	TRACKS_END

SfxAHdr_081:			/* file 0x08c78; SFX A 81, 68k id 1071; used by Camper_Init_062a4e, Camper_Idle_062b84, FinalBoss_End_07111c, FinalBoss_Idle_0713ac (+3) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA081_Trk0, 11
	TRACKS_END

SfxAHdr_083:			/* file 0x08c8a; SFX A 83, 68k id 1073; used by M5Tank_Tmpl76_072fc8 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x05
	TRACK SfxA083_Trk0, 81
	TRACKS_END

SfxAHdr_084:			/* file 0x08c9c; SFX A 84, 68k id 1074; used by M5Boss_PickAttack_06f416, M2Boss_Turret_ParentPhase_07c070, TaskHandler_084f5e, TaskHandler_084fca */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x05
	TRACK SfxA084_Trk0, 80
	TRACKS_END

SfxA_085_M5BossAttack_Hdr:			/* file 0x08cae; SFX A 85, 68k id 1075; used by M5Boss_AttackA_06f59a, M5Boss_AttackAEnd_06f63c, M5Boss_AttackB_06f7f0, M5Boss_AttackBEnd_06f8c0 (+2) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x05
	TRACK SfxA_085_M5BossAttack_Trk0, 80
	TRACKS_END

SfxA_086_ExplosiveProp_Hdr:			/* file 0x08cc0; SFX A 86, 68k id 1076; used by Prop_Explosive_052cea */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x05
	TRACK SfxA_086_ExplosiveProp_Trk0, 81
	TRACKS_END

SfxAHdr_087:			/* file 0x08cd2; SFX A 87, 68k id 1077 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA087_Trk0, 14
	TRACKS_END

SfxAHdr_095:			/* file 0x08ce4; SFX A 95, 68k id 107f; used by Airship_Wait_0893ac */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA095_Trk0, 1
	TRACK SfxA095_Trk1, 1
	TRACKS_END

SfxAHdr_096:			/* file 0x08cfb; SFX A 96, 68k id 1080; used by Airship_Wait_0893ac */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA096_Trk0, 1
	TRACK SfxA096_Trk1, 1
	TRACKS_END

SfxAHdr_097:			/* file 0x08d12; SFX A 97, 68k id 1081; used by Airship_Steer_08b6a8 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA097_Trk0, 1
	TRACK SfxA097_Trk1, 1
	TRACKS_END

SfxA_099_SlugHPUp_Hdr:			/* file 0x08d29; SFX A 99, 68k id 1083; used by Slug_AddHP_02a2ba, Results_BannerAFinal_03eaa2 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA_099_SlugHPUp_Trk0, 1
	TRACK SfxA_099_SlugHPUp_Trk1, 1
	TRACKS_END

SfxA_100_VehicleLaunch_Hdr:			/* file 0x08d40; SFX A 100, 68k id 1084; used by VehicleLaunch_Init_0311c0, VehicleLaunch_InitDrop_0318e6 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_100_VehicleLaunch_Trk0, 100
	TRACKS_END

SfxAHdr_101:			/* file 0x08d52; SFX A 101, 68k id 1085 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA101_Trk0, 16
	TRACKS_END

SfxA_102_FireBurst_Hdr:			/* file 0x08d64; SFX A 102, 68k id 1086; used by FireBurst_Low_06dfe8, FireBurst_Arc_06e09a, Flag_Init_09c4d4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_102_FireBurst_Trk0, 14
	TRACKS_END

SfxAHdr_103:			/* file 0x08d76; SFX A 103, 68k id 1087; used by Player_PlayLifeMusic_032ed0, PlayerRoute_PublishState_033522, Player_Death_PlayMusic_03765a, Player_DeathPit_037b8e */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA103_Trk0, 29
	TRACKS_END

SfxA_105_SlugDamage_Hdr:			/* file 0x08d88; SFX A 105, 68k id 1089; used by Slug_DamageTick_02a6be */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_105_SlugDamage_Trk0, 13
	TRACKS_END

SfxAHdr_106:			/* file 0x08d9a; SFX A 106, 68k id 108a */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA106_Trk0, 101
	TRACKS_END

SfxAHdr_107:			/* file 0x08dac; SFX A 107, 68k id 108b; used by JsrAbsThunk_03da98 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA107_Trk0, 101
	TRACKS_END

SfxA_108_EnemyShot_Hdr:			/* file 0x08dbe; SFX A 108, 68k id 108c; used by Boss_PhaseFire_044AFE, Turret_ReadPlayerInput_04593A, Gunner_Shell_059da6, Turret8_FireBullet_08b626 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_108_EnemyShot_Trk0, 101
	TRACKS_END

SfxA_109_TurretShell_Hdr:			/* file 0x08dd0; SFX A 109, 68k id 108d; used by Prop_Boat_04f7ca, Roller_Task_056870, AimTurret_Shell_061344, AimTurret_HitCheck_061580 (+10) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_109_TurretShell_Trk0, 100
	TRACKS_END

SfxA_110_DoorBreak_Hdr:			/* file 0x08de2; SFX A 110, 68k id 108e; used by HutDoor_Break_05fc78, FireBurst_Tmpl_06df32 */
	SFX_HDR 0x10ae, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA_110_DoorBreak_Trk0, 100
	TRACKS_END

SfxAHdr_111:			/* file 0x08df4; SFX A 111, 68k id 108f; used by Slug_StopMusic108F_02a2f8 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA111_Trk0, 110
	TRACKS_END

SfxAHdr_112:			/* file 0x08e06; SFX A 112, 68k id 1090; used by JsrAbsThunk_02a5f2 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA112_Trk0, 110
	TRACKS_END

SfxAHdr_113:			/* file 0x08e18; SFX A 113, 68k id 1091; used by JsrAbsThunk_02a624 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA113_Trk0, 110
	TRACKS_END

SfxAHdr_114:			/* file 0x08e2a; SFX A 114, 68k id 1092; used by Slug_StopMusic1092_02a310 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x07
	TRACK SfxA114_Trk0, 110
	TRACKS_END

SfxA_120_TrapFlame_Hdr:			/* file 0x08e3c; SFX A 120, 68k id 1098; used by Prop_TrapFlame_053a42 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA_120_TrapFlame_Trk0, 120
	TRACKS_END

SfxAHdr_121:			/* file 0x08e4e; SFX A 121, 68k id 1099; used by TaskHandler_081db0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA121_Trk0, 120
	TRACK SfxA121_Trk1, 0
	TRACKS_END

SfxAHdr_123:			/* file 0x08e65; SFX A 123, 68k id 109b; used by SquadSpawn_GlideAttacker_04233E, AimTurret_Shell_061344 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA123_Trk0, 130
	TRACKS_END

SfxAHdr_124:			/* file 0x08e77; SFX A 124, 68k id 109c */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA124_Trk0, 120
	TRACK SfxA124_Trk1, 0
	TRACKS_END

SfxAHdr_125:			/* file 0x08e8e; SFX A 125, 68k id 109d; used by LateProp_Init_061aa2, LateProp_Idle_061b6a, LateProp_Die_061c54, LateProp_Spark_061e72 (+1) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA125_Trk0, 0
	TRACKS_END

SfxA_126_TankEngine_Hdr:			/* file 0x08ea0; SFX A 126, 68k id 109e; used by Tank_Drive_068ddc, Tank_Brake_06900a, Tank_Die_0694dc, Crab_Pause_07d178 (+3) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA_126_TankEngine_Trk0, 0
	TRACKS_END

SfxAHdr_127:			/* file 0x08eb2; SFX A 127, 68k id 109f; used by Obstacle_FlushMusicJmp77FD6_060c40, Crew_Captor_Death_07a338, TaskHandler_0828e0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x08
	TRACK SfxA127_Trk0, 150
	TRACKS_END

SfxAHdr_128:			/* file 0x08ec4; SFX A 128, 68k id 10a0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA128_Trk0, 140
	TRACKS_END

SfxAHdr_129:			/* file 0x08ed6; SFX A 129, 68k id 10a1 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA129_Trk0, 130
	TRACKS_END

SfxAHdr_130:			/* file 0x08ee8; SFX A 130, 68k id 10a2; used by SquadLeader_Enter_04049c, SquadLeader_Death_040be6 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA130_Trk0, 120
	TRACK SfxA130_Trk1, 0
	TRACKS_END

SfxAHdr_131:			/* file 0x08eff; SFX A 131, 68k id 10a3; used by TrioSpawner_PatternedPair_042206, TurretCar_DriverFlee_04d13a, M5Boss_Bomb_06fcfa */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0f
	TRACK SfxA131_Trk0, 130
	TRACK SfxA131_Trk1, 130
	TRACKS_END

SfxAHdr_132:			/* file 0x08f16; SFX A 132, 68k id 10a4; used by Scene3Prop_Init_065668, M5Boss_Fx2D82BA_0702a8, JsrAbsThunk_072f38, M5Tank_Rocket_Launch_073bac */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x10
	TRACK SfxA132_Trk0, 140
	TRACK SfxA132_Trk1, 140
	TRACKS_END

SfxA_133_Mortar_Hdr:			/* file 0x08f2d; SFX A 133, 68k id 10a5; used by Mortar_Tmpl3C_06361e, Mortar_Die_06388a */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA_133_Mortar_Trk0, 120
	TRACKS_END

SfxAHdr_134:			/* file 0x08f3f; SFX A 134, 68k id 10a6; used by TaskHandler_083be2, Prop_Breakable_A_08716a */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA134_Trk0, 170
	TRACKS_END

SfxAHdr_135:			/* file 0x08f51; SFX A 135, 68k id 10a7; used by M5Tank_Phase2_073290 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x11
	TRACK SfxA135_Trk0, 125
	TRACKS_END

SfxA_136_HatchOpen_Hdr:			/* file 0x08f63; SFX A 136, 68k id 10a8; used by S5Airship_HatchOpen_068804 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x11
	TRACK SfxA_136_HatchOpen_Trk0, 150
	TRACKS_END

SfxAHdr_137:			/* file 0x08f75; SFX A 137, 68k id 10a9; used by Prop_LampIdle_04d782, TaskHandler_0839e2, Prop_Signboard_0872fa */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x12
	TRACK SfxA137_Trk0, 170
	TRACKS_END

SfxAHdr_138:			/* file 0x08f87; SFX A 138, 68k id 10aa; used by LateProp_SpawnPairB_0628ac */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA138_Trk0, 130
	TRACKS_END

SfxAHdr_139:			/* file 0x08f99; SFX A 139, 68k id 10ab */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA139_Trk0, 130
	TRACKS_END

SfxAHdr_140:			/* file 0x08fab; SFX A 140, 68k id 10ac */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA140_Trk0, 130
	TRACKS_END

SfxAHdr_141:			/* file 0x08fbd; SFX A 141, 68k id 10ad */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA141_Trk0, 130
	TRACKS_END

SfxA_142_HutDoor_Hdr:			/* file 0x08fcf; SFX A 142, 68k id 10ae; used by HutOccupant_Out_05fb24, HutDoor_Break_05fc78, HutDoor_Free_05fce6 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x13
	TRACK SfxA_142_HutDoor_Trk0, 120
	TRACKS_END

SfxA_143_SlugAccelRight_Hdr:			/* file 0x08fe1; SFX A 143, 68k id 10af; used by Slug_Stall_02eca6, Slug_Destroyed_02fc70, Slug_AccelRightMusicTbl_02b8ce[0] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0b
	TRACK SfxA_143_SlugAccelRight_Trk0, 170
	TRACKS_END

SfxA_144_SlugAccelLeft_Hdr:			/* file 0x08ff3; SFX A 144, 68k id 10b0; used by Slug_Stall_02eca6, Slug_AccelLeftMusicTbl_02ba34[0] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0b
	TRACK SfxA_144_SlugAccelLeft_Trk0, 170
	TRACKS_END

SfxAHdr_145:			/* file 0x09005; SFX A 145, 68k id 10b1 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0b
	TRACK SfxA145_Trk0, 170
	TRACKS_END

SfxA_146_SlugDestroyedSlide_Hdr:			/* file 0x09017; SFX A 146, 68k id 10b2; used by Slug_DestroyedSlide_02bba4, Slug_DestroyedSlideInit_02bbf2, Slug_DestroyedMusicTbl_02bb9a[0], Slug_DestroyedMusicTbl_02bb9a[1] (+3) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0b
	TRACK SfxA_146_SlugDestroyedSlide_Trk0, 170
	TRACKS_END

SfxAHdr_147:			/* file 0x09029; SFX A 147, 68k id 10b3 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0c
	TRACK SfxA147_Trk0, 120
	TRACKS_END

SfxAHdr_148:			/* file 0x0903b; SFX A 148, 68k id 10b4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA148_Trk0, 0
	TRACKS_END

SfxAHdr_149:			/* file 0x0904d; SFX A 149, 68k id 10b5; used by M5Boss_Intro_Tmpl70_06f17e, Table_2D4A22[0] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0d
	TRACK SfxA149_Trk0, 121
	TRACKS_END

SfxAHdr_150:			/* file 0x0905f; SFX A 150, 68k id 10b6; used by Table_2D4A22[1] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0d
	TRACK SfxA150_Trk0, 121
	TRACKS_END

SfxAHdr_151:			/* file 0x09071; SFX A 151, 68k id 10b7; used by Table_2D4A22[2] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0d
	TRACK SfxA151_Trk0, 121
	TRACKS_END

SfxA_152_GunshipAttack_Hdr:			/* file 0x09083; SFX A 152, 68k id 10b8; used by Gunship_Attack_06e66c, Gunship_FlashCount5_06e820 */
	SFX_HDR 0x10c3, 0x10c4, 0x10c5, 0x0000, 0x0d
	TRACK SfxA_152_GunshipAttack_Trk0, 121
	TRACKS_END

SfxAHdr_153:			/* file 0x09095; SFX A 153, 68k id 10b9; used by M5Boss_Tmpl6F_06f1f6, Squad_ChildLeap_08022e */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0d
	TRACK SfxA153_Trk0, 121
	TRACKS_END

SfxAHdr_154:			/* file 0x090a7; SFX A 154, 68k id 10ba; used by M5Boss_PlayMusic10BA_06f38c, Squad_ChildInitTableA_080110, Squad_ChildInitFixed_0802e0, Squad_DeathPieces_080f32 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA154_Trk0, 0
	TRACKS_END

SfxA_155_WallBreak_Hdr:			/* file 0x090b9; SFX A 155, 68k id 10bb; used by Prop_Wall_052a26, Prop_WallStage2_052af4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA_155_WallBreak_Trk0, 0
	TRACKS_END

SfxAHdr_159:			/* file 0x090cb; SFX A 159, 68k id 10bf */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA159_Trk0, 120
	TRACKS_END

SfxAHdr_160:			/* file 0x090dd; SFX A 160, 68k id 10c0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA160_Trk0, 170
	TRACKS_END

SfxAHdr_161:			/* file 0x090ef; SFX A 161, 68k id 10c1 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA161_Trk0, 120
	TRACKS_END

SfxAHdr_162:			/* file 0x09101; SFX A 162, 68k id 10c2 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x00
	TRACK SfxA162_Trk0, 120
	TRACKS_END

SfxAHdr_163:			/* file 0x09113; SFX A 163, 68k id 10c3; used by M5Boss_Intro_Tmpl70_06f17e, Table_2D4A22[3] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x15
	TRACK SfxA163_Trk0, 121
	TRACKS_END

SfxAHdr_164:			/* file 0x09125; SFX A 164, 68k id 10c4; used by Table_2D4A22[4] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x15
	TRACK SfxA164_Trk0, 121
	TRACKS_END

SfxAHdr_165:			/* file 0x09137; SFX A 165, 68k id 10c5; used by Table_2D4A22[5] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x15
	TRACK SfxA165_Trk0, 121
	TRACKS_END

SfxAHdr_166:			/* file 0x09149; SFX A 166, 68k id 10c6; used by JsrAbsThunk_072e78, JsrAbsThunk_072ee0, M5Tank_MuzzleC_073ef8 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0f
	TRACK SfxA166_Trk0, 130
	TRACKS_END

SfxA_180_ContinueCountdown_Hdr:			/* file 0x0915b; SFX A 180, 68k id 10d4; used by ContinueBig_Init_0467f6, ContinueBig_Countdown_046878 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x14
	TRACK SfxA_180_ContinueCountdown_Trk0, 1
	TRACKS_END

SfxAHdr_181:			/* file 0x0916d; SFX A 181, 68k id 10d5; used by Cut_Watcher_Play_08c8fa */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x14
	TRACK SfxA181_Trk0, 1
	TRACK SfxA181_Trk1, 1
	TRACKS_END

SfxA_182_ResultsPrize_Hdr:			/* file 0x09184; SFX A 182, 68k id 10d6; used by Results_PrizeLand_03e9fa */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x14
	TRACK SfxA_182_ResultsPrize_Trk0, 1
	TRACKS_END

SfxAHdr_183:			/* file 0x09196; SFX A 183, 68k id 10d7; used by Handler_TimerAndReplace_001BCA */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x14
	TRACK SfxA183_Trk0, 1
	TRACK SfxA183_Trk1, 1
	TRACKS_END

SfxA_184_ResultsTally_Hdr:			/* file 0x091ad; SFX A 184, 68k id 10d8; used by Results_ColPhaseTotal_03e208, Results_ColWinner_03e2fe, JsrAbsThunk_05965e */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x14
	TRACK SfxA_184_ResultsTally_Trk0, 1
	TRACKS_END

SfxA_190_GameOverFinal_Hdr:			/* file 0x091bf; SFX A 190, 68k id 10de; used by GameOver_Final_08fd68 */
	SFX_HDR 0x10df, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_190_GameOverFinal_Trk0, 1
	TRACK SfxA_190_GameOverFinal_Trk1, 1
	TRACKS_END

SfxA_191_GameOverBanner_Hdr:			/* file 0x091d6; SFX A 191, 68k id 10df; used by GameOver_Spawn_08f96a, GameOver_Wait_08fcca, GameOver_Wait2_08fd2e, GO_Banner_0905c2 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_191_GameOverBanner_Trk0, 1
	TRACK SfxA_191_GameOverBanner_Trk1, 1
	TRACKS_END

SfxA_192_PauseSlide_Hdr:			/* file 0x091ed; SFX A 192, 68k id 10e0; used by AttractHandler_2Task_0010F2, Pause_Poll_013c3c, Pause_Active_013cae, GO_Sprite_Slide_08ff88 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA_192_PauseSlide_Trk0, 0
	TRACK SfxA_192_PauseSlide_Trk1, 0
	TRACK SfxA_192_PauseSlide_Trk2, 0
	TRACKS_END

SfxA_193_Pause_Hdr:			/* file 0x09209; SFX A 193, 68k id 10e1; used by Pause_Active_013cae, GO_Sprite_Mid_08fff6 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_193_Pause_Trk0, 1
	TRACKS_END

SfxAHdr_194:			/* file 0x0921b; SFX A 194, 68k id 10e2 */
	SFX_HDR 0x10df, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA194_Trk0, 1
	TRACKS_END

SfxAHdr_195:			/* file 0x0922d; SFX A 195, 68k id 10e3 */
	SFX_HDR 0x10df, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA195_Trk0, 1
	TRACK SfxA195_Trk1, 1
	TRACKS_END

SfxAHdr_196:			/* file 0x09244; SFX A 196, 68k id 10e4 */
	SFX_HDR 0x10df, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA196_Trk0, 1
	TRACKS_END

SfxA_200_AimTurret_Hdr:			/* file 0x09256; SFX A 200, 68k id 10e8; used by AimTurret_Tmpl08E_060e62, AimTurret_Die_06100c, AimTurret_DieB_0610f4, AimTurret_Dormant_061204 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA_200_AimTurret_Trk0, 120
	TRACKS_END

SfxA_201_SlugDestroyed_Hdr:			/* file 0x09268; SFX A 201, 68k id 10e9; used by Slug_DestroyedMusicAndBubble_02a638, Slug_DestroyedSlideInit_02bbf2, Slug_Knocked_02f0dc, Slug_Destroyed_02fc70 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x14
	TRACK SfxA_201_SlugDestroyed_Trk0, 1
	TRACKS_END

SfxA_202_FragLaunch_Hdr:			/* file 0x0927a; SFX A 202, 68k id 10ea; used by M5Tank_SpawnSmokeAndFrag_074416, Frag_Launch_076e10 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x0a
	TRACK SfxA_202_FragLaunch_Trk0, 130
	TRACKS_END

SfxA_210_VehicleCrash_Hdr:			/* file 0x0928c; SFX A 210, 68k id 10f2; used by VehicleLaunch_Crash_0317d2 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_210_VehicleCrash_Trk0, 99
	TRACKS_END

SfxA_211_ShotgunFire_Hdr:			/* file 0x0929e; SFX A 211, 68k id 10f3; used by PlayerFire_Shotgun_Spawn_03c9c4 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_211_ShotgunFire_Trk0, 101
	TRACKS_END

SfxA_212_FlameFire_Hdr:			/* file 0x092b0; SFX A 212, 68k id 10f4; used by PlayerFire_Flame_Spawn_03ccd2, Allen_SpawnBulletLevel_050822, Allen_SpawnBulletDiag_05087c, Gun_MuzzlePos_09a1b6 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_212_FlameFire_Trk0, 101
	TRACKS_END

SfxA_213_RocketFire_Hdr:			/* file 0x092c2; SFX A 213, 68k id 10f5; used by PlayerFire_Rocket_Spawn_03cb16 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_213_RocketFire_Trk0, 101
	TRACKS_END

SfxA_214_HMGFire_Hdr:			/* file 0x092d4; SFX A 214, 68k id 10f6; used by PlayerFire_HMG_Spawn_03c872 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_214_HMGFire_Trk0, 101
	TRACKS_END

SfxA_215_PistolFire_Hdr:			/* file 0x092e6; SFX A 215, 68k id 10f7; used by PlayerFire_Pistol_Spawn_03c710 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA_215_PistolFire_Trk0, 101
	TRACKS_END

SfxAHdr_218:			/* file 0x092f8; SFX A 218, 68k id 10fa; used by JsrAbsThunk_05e8ee */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA218_Trk0, 14
	TRACKS_END

SfxAHdr_219:			/* file 0x0930a; SFX A 219, 68k id 10fb; used by Player_Death_Debug_037af2, AnimCb_AttackProbeMusic_03942e */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA219_Trk0, 100
	TRACKS_END

SfxAHdr_220:			/* file 0x0931c; SFX A 220, 68k id 10fc */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x1e
	TRACK SfxA220_Trk0, 102
	TRACKS_END

SfxA_221_AmmoPickup_Hdr:			/* file 0x0932e; SFX A 221, 68k id 10fd; used by Item_GiveAmmoKind_09a854 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK SfxA_221_AmmoPickup_Trk0, 10
	TRACKS_END

SfxA_223_ShieldBreak_Hdr:			/* file 0x09340; SFX A 223, 68k id 10ff; used by Shield_Break_068206, Sub_00083064 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x01
	TRACK SfxA_223_ShieldBreak_Trk0, 14
	TRACKS_END

Voice_FoodPickup_Hdr:			/* file 0x09352; SFX B 1, 68k id 1121; used by Item_Food_Pickup_09b920 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x15
	TRACK Voice_FoodPickup_Trk0, 1
	TRACK Voice_FoodPickup_Trk1, 0
	TRACKS_END

Voice_PlayerDeath_Hdr:			/* file 0x09369; SFX B 3, 68k id 1123; used by Player_PlayLifeMusic_032ed0, PlayerRoute_PublishState_033522, Player_Death_PlayMusic_03765a, Player_DeathPit_037b8e */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x02
	TRACK Voice_PlayerDeath_Trk0, 10
	TRACKS_END

Voice_Okay_Hdr:			/* file 0x0937b; SFX B 4, 68k id 1124; used by Item_Tmpl289_Weapon_09abc2, Item_Tmpl290_WeaponSwap_09ac52, Item_Tmpl291_Bombs_09ad54 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_Okay_Trk0, 8
	TRACKS_END

Voice_Bingo_Hdr:			/* file 0x0938d; SFX B 6, 68k id 1126; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_Bingo_Trk0, 8
	TRACKS_END

Voice_LetsGo_Hdr:			/* file 0x0939f; SFX B 7, 68k id 1127; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_LetsGo_Trk0, 8
	TRACKS_END

Voice_HereWeGo_Hdr:			/* file 0x093b1; SFX B 8, 68k id 1128; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_HereWeGo_Trk0, 8
	TRACKS_END

Voice_Yeah_Hdr:			/* file 0x093c3; SFX B 9, 68k id 1129; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_Yeah_Trk0, 8
	TRACKS_END

Voice_YeeHaw_Hdr:			/* file 0x093d5; SFX B 10, 68k id 112a; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_YeeHaw_Trk0, 8
	TRACKS_END

Voice_PowerUp_Hdr:			/* file 0x093e7; SFX B 11, 68k id 112b; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_PowerUp_Trk0, 8
	TRACKS_END

Voice_FlameShot_Hdr:			/* file 0x093f9; SFX B 12, 68k id 112c; used by Player_WeaponMusicTable_032d28[2] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_FlameShot_Trk0, 5
	TRACKS_END

Voice_HeavyMachineGun_Hdr:			/* file 0x0940b; SFX B 13, 68k id 112d; used by Player_WeaponMusicTable_032d28[4] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_HeavyMachineGun_Trk0, 5
	TRACKS_END

Voice_RocketLauncher_Hdr:			/* file 0x0941d; SFX B 14, 68k id 112e; used by Player_WeaponMusicTable_032d28[3] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_RocketLauncher_Trk0, 5
	TRACKS_END

Voice_Shotgun_Hdr:			/* file 0x0942f; SFX B 15, 68k id 112f; used by Player_WeaponMusicTable_032d28[1] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x06
	TRACK Voice_Shotgun_Trk0, 5
	TRACKS_END

Voice_Mission1Start_Hdr:			/* file 0x09441; SFX B 17, 68k id 1131; used by MissionStartVoiceTable_28DF10[0] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_Mission1Start_Trk0, 1
	TRACKS_END

Voice_Mission2Start_Hdr:			/* file 0x09453; SFX B 18, 68k id 1132; used by MissionStartVoiceTable_28DF10[1] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_Mission2Start_Trk0, 1
	TRACKS_END

Voice_Mission3Start_Hdr:			/* file 0x09465; SFX B 19, 68k id 1133; used by MissionStartVoiceTable_28DF10[2] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_Mission3Start_Trk0, 1
	TRACKS_END

Voice_Mission4Start_Hdr:			/* file 0x09477; SFX B 20, 68k id 1134; used by MissionStartVoiceTable_28DF10[3] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_Mission4Start_Trk0, 1
	TRACKS_END

Voice_Mission5Start_Hdr:			/* file 0x09489; SFX B 21, 68k id 1135; used by MissionStartVoiceTable_28DF10[4] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_Mission5Start_Trk0, 1
	TRACKS_END

Voice_Mission6Start_Hdr:			/* file 0x0949b; SFX B 22, 68k id 1136; used by MissionStartVoiceTable_28DF10[5], MissionStartVoiceTable_28DF10[6], MissionStartVoiceTable_28DF10[7] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_Mission6Start_Trk0, 1
	TRACKS_END

Voice_MissionComplete_Hdr:			/* file 0x094ad; SFX B 23, 68k id 1137; used by BannerComplete_Boot_07A9F0 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_MissionComplete_Trk0, 1
	TRACKS_END

Voice_GameOver_Hdr:			/* file 0x094bf; SFX B 24, 68k id 1138; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_GameOver_Trk0, 10
	TRACKS_END

Voice_PowThankYou_Hdr:			/* file 0x094d1; SFX B 26, 68k id 113a; used by Pow_Rescued_040268 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_PowThankYou_Trk0, 9
	TRACKS_END

Voice_AllenComeOnBoy_Hdr:			/* file 0x094e3; SFX B 28, 68k id 113c; used by Allen_TauntVoiceTbl_0509d6[0], Allen_TauntVoiceTbl_0509d6[3] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_AllenComeOnBoy_Trk0, 10
	TRACKS_END

Voice_AllenGoHomeToMommy_Hdr:			/* file 0x094f5; SFX B 29, 68k id 113d; used by Allen_TauntVoiceTbl_0509d6[1] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_AllenGoHomeToMommy_Trk0, 10
	TRACKS_END

Voice_AllenMincemeat_Hdr:			/* file 0x09507; SFX B 30, 68k id 113e; used by Allen_TauntVoiceTbl_0509d6[2] */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_AllenMincemeat_Trk0, 10
	TRACKS_END

Voice_AllenLaugh_Hdr:			/* file 0x09519; SFX B 31, 68k id 113f; used by (no constant user found) */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_AllenLaugh_Trk0, 10
	TRACKS_END

Voice_AllenSeeYouInHell_Hdr:			/* file 0x0952b; SFX B 32, 68k id 1140; used by Allen_Death_050e40 */
	SFX_HDR 0x0000, 0x0000, 0x0000, 0x0000, 0x03
	TRACK Voice_AllenSeeYouInHell_Trk0, 10
	TRACKS_END
