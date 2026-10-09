"""
Metal Slug 1 — Tabla de símbolos absolutos.
=============================================
Dirección CPU (absoluta) -> nombre del símbolo tal como aparece en el
código C decompilado. Cada entrada resuelve una referencia extern del
código C hacia una función/etiqueta cuya localización conocemos.

Al enlazar en modo de matching unitario, cada símbolo se resuelve con
`--defsym=Name=0xADDR`, de forma que las instrucciones `lea pc+disp,a0`,
`jsr abs.l`, `bsr.w`, etc. se codifiquen contra la dirección real de
ROM y podamos comparar bit-a-bit con el binario original.

ARCHIVO AUTO-MANTENIDO — no editar manualmente entradas ThunkTarget_*,
TaskHandler_*, StateTable_*; se regeneran desde los escáneres.
"""

SYMBOLS = {
    # ---- Símbolos semánticos identificados manualmente ---------------
    0x000008F2: "VBlankCallbackDefault",
    # 0x00028D8E: promovido a símbolo interno de asm/script_dispatch.s
    # como Script_DispatchOpcode (Wave T#1). Ya no necesita --defsym.

    # ---- Wave T: targets llamados desde ASM (por nombre estable) ------
    0x00043F5E: "Sub_00043F5E",  # blitter de fila (PC-rel desde $43fac)
    # 0x00000506 promovido a Task_AllocFail_000506 en registry (Wave JJJJJ).
    # 0x00077C98 promovido a Spawner_Handler_077c98 en registry (Wave YYYY).
    # 0x000277C4 promovido a Entity_MoveAndCollide_F_0277c4 en registry (Wave CCCCC).
    # 0x000273FC promovido a Entity_SaveRegs_0273fc en registry (Wave BBBBB).
    0x000027444: "Sub_000027444",  # probe/collision compartido por T#11 y T#13
    # 0x0002773C promovido a Entity_MoveAndCollide_E_02773c en registry (Wave CCCCC).
    0x00027D32: "Entity_RestoreTransformSetC_027d32",  # brazo hermano (bcs.w) de T#7
    0x00027CD0: "Entity_RestoreTransformSetC_027cd0",  # brazo hermano (bcs.w) de T#9
    0x00027C0C: "Entity_RestoreTransformSetC_027c0c",  # brazo hermano (bcs.w) de T#11
    0x00027C6E: "Entity_RestoreTransformSetC_027c6e",  # brazo hermano (bcs.w) de T#13
    0x00027D94: "Entity_RestoreTransformSetC_027d94",  # brazo hermano (bcs.w) de T#15

    # ---- Wave U: backends comunes del cluster InputMask -----------------
    # Nota: los backends antes marcados como InputEvtBackend_* se han
    # promovido a InputMask_* (semantica identificada). Los thunks Wave U
    # los referencian con estos nombres nuevos.
    0x0005CFA8: "InputMask_CheckChannelAvail_05cfa8",  # backend #1 (18 thunks)
    0x0005CFF8: "InputMask_TestChannelBit_05cff8",     # backend #3 (10 thunks)
    0x00005D674: "Sub_00005D674",  # probe real llamado por $5CFC8 (jsr pc)
    0x00005CC08: "Sub_00005CC08",  # tabla contexto default del backend $5CFC8 (lea pc)
    0x000020E2: "FUN_000020e2",
    0x0000212E: "FUN_0000212e",
    # 0x00028108 promovido a Entity_ApplyFadeShade_028108 en registry
    #      (Wave QQ#2, 9 callers via .equ .Lposthook/.Lactor_process +
    #      1 jmp directo en EntitySetField38AndUpdate).
    0x0005022A: "StateMachineRun",
    0x00099AFC: "FUN_00099afc",
    0x00C004C2: "BIOS_FIX_CLEAR",

    # ---- Wave Y: targets externos referenciados por asm 68000 puro ----
    # 0x000329EE promovido a OpcodeOffsetTable_0329EE en registry (Wave AAAA).
    # 0x0009B51E promovido a Score_Popup_Value_09b51e en registry (Wave RRR).
    # 0x0004CB44 promovido a SpriteSetPtrTbl6_04cb44 en registry (Wave NNNN).
    # Templates usados por Entity_Build3ChainCircular (Y#10)
    # 0x0003010C promovido a Chain3_TplC_03010c en registry (Wave ZZZ).
    # 0x00030068 promovido a Chain3_TplA_030068 en registry (Wave ZZZ).
    # 0x000300BA promovido a Chain3_TplB_0300ba en registry (Wave ZZZ).
    # Templates usados por Entity_Build4FromTemplates (Y#11)

    # ---- Wave Z: externos referenciados por asm 68000 puro ----
    # 0x0005B1B2 promovido a Sprite_DispatchSplashHook_05b1b2 en registry (Wave DDDD).
    # 0x0005AA96 promovido a SpriteDispatchJT_05AA96 en registry (Wave DDDD).
    0x00044182: "Sub_00044182",              # colisión llamada por Entity_Probe_02785C
    # 0x00027036 promovido a Entity_MoveAndCollide_B_027036 en registry (Wave BBBBB).
    # 0x00026B56 promovido a Entity_MoveAndCollide_A_026b56 en registry (Wave BBBBB).
    # 0x00047872 promovido a Font_TileWithPal_047872 en registry (Wave DDDDD).
    0x00044022: "Sub_00044022",              # blit setup llamado por Helper_05026C
    # 0x000523EE promovido a Template_0523EE en registry (Wave HHHHH).
    # 0x000524AA promovido a Template_0524AA en registry (Wave HHHHH).

    # ---- Wave Z batch 2: externos referenciados por asm 68000 puro ----
    # 0x0005E3A2 promovido a Player_GetEntity_05e3a2 en registry (Wave SSSS).
    # 0x0005E618 promovido a Target_IsAhead_05e618 en registry (Wave SSSS).
    # 0x00028A96 promovido a Entity_HitboxCollide_028A96 en registry (Wave SS#2).
    # 0x00051862 promovido a Nibbles_Pack8_051862 en registry (Wave QQQQ).
    # 0x00051828 promovido a Nibbles_Unpack4_051828 en registry (Wave QQQQ).
    # 0x0005188C promovido a PlayerState_FlagTable_05188c en registry (Wave QQQQ).
    # 0x000272A8 promovido a Entity_MoveAndCollide_C_0272a8 en registry (Wave BBBBB).
    # 0x0006DD5C promovido a Frag_Scatter_06dd5c en registry (Wave VVVV).
    # 0x0006DF32 promovido a FireBurst_Tmpl_06df32 en registry (Wave VVVV).
    # 0x0006E2BC: se usa PcThunkTarget_06e2bc (ya expuesto abajo, linea ~931).
    #             El nombre canonico historico se conserva; Sub_0006E2BC eliminado.
    # 0x00047822 promovido a Font_BigGlyphToTile_047822 en registry (Wave DDDDD).
    # 0x000477D4 promovido a Font_SmallGlyphToTile_0477d4 en registry (Wave DDDDD).
    0x00046AC6: "Sub_00046AC6",              # jsr abs.l inicial de Init_JsrThenTailCall (Z2 #12)
    0x00000FE0: "Sub_00000FE0",              # bra.w tail-call de Init_JsrThenTailCall
    # 0x00046A48 promovido a TimeUp_Banner_Task_046a48 en registry (Wave DDDDD).
    0x00106F28: "GlobalFlag_106F28",         # flag global chequeada por Handler_ConditionalHitCounter
    # 0x00001C34 promovido a Timer_WaitFlag21_001c34 en registry (Wave JJJJJ).

    # ---- Entry points BIOS y objetivos internos (Wave P) --------------
    0x00000868: "Sys_HW_Reset",
    0x00024E76: "Player_Start_Inner",
    0x00024FB6: "Demo_Start_Inner",
    0x00024E38: "TitleModeInit",
    0x00000656: "FUN_00000656",
    0x0000085E: "SoftReset_085E",
    0x0000080C: "UserMode0_080C",
    0x00000832: "UserMode1_0832",
    0x00000836: "UserMode2_0836",
    0x00000840: "UserMode3_0840",
    0x0000097A: "GameFrame",
    0x00001E5E: "FUN_00001E5E",
    0x00000960: "LspcModeCheck_0960",
    0x00013600: "FUN_00013600",
    0x00000518: "FUN_00000518",
    0x00000546: "Task_UnlinkAlive_0546",
    0x00000588: "Task_UnlinkDead_0588",
    0x000005B6: "FUN_000005B6",
    0x000005DA: "Task_ChangeHandler_05DA",
    0x000005FE: "ThunkTarget_0005fe",
    0x00000626: "Task_ChangeAndRun_0626",
    0x000006E2: "FUN_000006E2",
    0x000006CA: "FUN_000006CA",
    # 0x00000400 promovido a Task_IdleRts_000400 en registry (Wave JJJJJ).


    # ---- Tablas de StateDispatchStub (AUTO-GEN) ----------------------
    0x002985F8: "StateTable_2985f8",
    0x00298634: "StateTable_298634",
    0x00298670: "StateTable_298670",
    0x002986AC: "StateTable_2986ac",
    0x002986E8: "StateTable_2986e8",
    0x0029A552: "StateTable_29a552",
    0x0029A566: "StateTable_29a566",
    0x0029A57A: "StateTable_29a57a",
    0x0029A58E: "StateTable_29a58e",
    0x0029A5A2: "StateTable_29a5a2",
    0x0029A5B6: "StateTable_29a5b6",
    0x0029A5CA: "StateTable_29a5ca",
    0x0029A5DE: "StateTable_29a5de",
    0x0029A5F2: "StateTable_29a5f2",
    0x0029A61A: "StateTable_29a61a",
    0x0029A62E: "StateTable_29a62e",
    0x0029A642: "StateTable_29a642",
    0x0029A656: "StateTable_29a656",
    0x0029A66A: "StateTable_29a66a",
    0x0029A67E: "StateTable_29a67e",
    0x0029A692: "StateTable_29a692",
    0x0029A6A6: "StateTable_29a6a6",
    0x0029A6BA: "StateTable_29a6ba",
    0x0029A6CE: "StateTable_29a6ce",
    0x0029A6E2: "StateTable_29a6e2",
    0x0029A6F6: "StateTable_29a6f6",
    0x0029A70A: "StateTable_29a70a",
    0x0029A71E: "StateTable_29a71e",
    0x0029A732: "StateTable_29a732",
    0x0029A746: "StateTable_29a746",
    0x0029A75A: "StateTable_29a75a",
    0x0029A76E: "StateTable_29a76e",
    0x0029A782: "StateTable_29a782",
    0x0029A796: "StateTable_29a796",
    0x0029A7AA: "StateTable_29a7aa",
    0x0029A7BE: "StateTable_29a7be",
    0x0029A7D2: "StateTable_29a7d2",
    0x0029A7E6: "StateTable_29a7e6",
    0x0029A7FA: "StateTable_29a7fa",
    0x0029A80E: "StateTable_29a80e",
    0x0029A822: "StateTable_29a822",
    0x0029A836: "StateTable_29a836",
    0x0029A84A: "StateTable_29a84a",
    0x0029A85E: "StateTable_29a85e",
    0x0029A872: "StateTable_29a872",
    0x0029A886: "StateTable_29a886",
    0x0029A89A: "StateTable_29a89a",
    0x0029A8AE: "StateTable_29a8ae",
    0x0029A8C2: "StateTable_29a8c2",
    0x0029A8D6: "StateTable_29a8d6",
    0x0029A8EA: "StateTable_29a8ea",
    0x0029A8FE: "StateTable_29a8fe",
    0x0029A912: "StateTable_29a912",
    0x0029A926: "StateTable_29a926",
    0x0029A93A: "StateTable_29a93a",
    0x0029A94E: "StateTable_29a94e",
    0x0029A962: "StateTable_29a962",
    0x0029A976: "StateTable_29a976",
    0x0029A98A: "StateTable_29a98a",
    0x0029A99E: "StateTable_29a99e",
    0x0029A9B2: "StateTable_29a9b2",
    0x0029A9C6: "StateTable_29a9c6",
    0x0029A9DA: "StateTable_29a9da",
    0x0029A9EE: "StateTable_29a9ee",
    0x0029AA02: "StateTable_29aa02",
    0x0029AA16: "StateTable_29aa16",
    0x0029AA2A: "StateTable_29aa2a",
    0x0029AA3E: "StateTable_29aa3e",
    0x0029AA52: "StateTable_29aa52",
    0x0029AA66: "StateTable_29aa66",
    0x0029AA7A: "StateTable_29aa7a",
    0x0029AA8E: "StateTable_29aa8e",
    0x0029AAA2: "StateTable_29aaa2",
    0x0029AAB6: "StateTable_29aab6",
    0x0029AACA: "StateTable_29aaca",
    0x0029AADE: "StateTable_29aade",
    0x0029AAF2: "StateTable_29aaf2",
    0x0029AB06: "StateTable_29ab06",
    0x0029AB1A: "StateTable_29ab1a",
    0x0029AB2E: "StateTable_29ab2e",
    0x0029AB42: "StateTable_29ab42",
    0x0029AB56: "StateTable_29ab56",
    0x0029AB6A: "StateTable_29ab6a",
    0x0029AB7E: "StateTable_29ab7e",
    0x0029AB92: "StateTable_29ab92",
    0x0029ABA6: "StateTable_29aba6",
    0x0029ABBA: "StateTable_29abba",
    0x0029ABCE: "StateTable_29abce",
    0x0029ABE2: "StateTable_29abe2",
    0x0029ABF6: "StateTable_29abf6",
    0x0029AC0A: "StateTable_29ac0a",
    0x0029AC1E: "StateTable_29ac1e",
    0x0029AC32: "StateTable_29ac32",
    0x0029AC46: "StateTable_29ac46",
    0x0029AC5A: "StateTable_29ac5a",
    0x0029AC6E: "StateTable_29ac6e",
    0x0029AC82: "StateTable_29ac82",
    0x0029AC96: "StateTable_29ac96",
    0x0029ACAA: "StateTable_29acaa",
    0x0029ACBE: "StateTable_29acbe",
    0x0029ACD2: "StateTable_29acd2",
    0x0029ACE6: "StateTable_29ace6",
    0x0029ACFA: "StateTable_29acfa",
    0x0029AD0E: "StateTable_29ad0e",
    0x0029AD22: "StateTable_29ad22",
    0x0029AD36: "StateTable_29ad36",
    0x0029AD4A: "StateTable_29ad4a",
    0x0029AD5E: "StateTable_29ad5e",
    0x0029AD72: "StateTable_29ad72",
    0x0029AD86: "StateTable_29ad86",
    0x0029AD9A: "StateTable_29ad9a",
    0x0029ADAE: "StateTable_29adae",
    0x0029ADC2: "StateTable_29adc2",
    0x0029ADD6: "StateTable_29add6",
    0x0029ADEA: "StateTable_29adea",
    0x0029ADFE: "StateTable_29adfe",
    0x0029AE12: "StateTable_29ae12",
    0x0029AE26: "StateTable_29ae26",
    0x0029AE3A: "StateTable_29ae3a",
    0x0029AE4E: "StateTable_29ae4e",
    0x0029AE62: "StateTable_29ae62",
    0x0029AE76: "StateTable_29ae76",
    0x0029AE8A: "StateTable_29ae8a",
    0x0029AE9E: "StateTable_29ae9e",
    0x0029AEB2: "StateTable_29aeb2",
    0x0029AEC6: "StateTable_29aec6",
    0x0029AEDA: "StateTable_29aeda",
    0x0029AEEE: "StateTable_29aeee",
    0x0029AF02: "StateTable_29af02",
    0x0029AF16: "StateTable_29af16",
    0x0029AF2A: "StateTable_29af2a",
    0x0029AF3E: "StateTable_29af3e",
    0x0029AF52: "StateTable_29af52",
    0x0029AF66: "StateTable_29af66",
    0x0029AF7A: "StateTable_29af7a",
    0x0029AF8E: "StateTable_29af8e",
    0x0029AFA2: "StateTable_29afa2",
    0x0029AFB6: "StateTable_29afb6",
    0x0029AFCA: "StateTable_29afca",
    0x0029AFDE: "StateTable_29afde",
    0x0029AFF2: "StateTable_29aff2",
    0x0029B006: "StateTable_29b006",
    0x0029B01A: "StateTable_29b01a",
    0x0029B02E: "StateTable_29b02e",
    0x0029B042: "StateTable_29b042",
    0x0029B056: "StateTable_29b056",
    0x0029B06A: "StateTable_29b06a",
    0x0029B07E: "StateTable_29b07e",
    0x0029B092: "StateTable_29b092",
    0x0029B0A6: "StateTable_29b0a6",
    0x0029B0BA: "StateTable_29b0ba",
    0x0029B0CE: "StateTable_29b0ce",
    0x0029B0E2: "StateTable_29b0e2",
    0x0029B0F6: "StateTable_29b0f6",
    0x0029B10A: "StateTable_29b10a",
    0x0029B11E: "StateTable_29b11e",
    0x0029B132: "StateTable_29b132",
    0x0029B146: "StateTable_29b146",
    0x0029B15A: "StateTable_29b15a",
    0x0029B16E: "StateTable_29b16e",
    0x0029B182: "StateTable_29b182",
    0x0029B196: "StateTable_29b196",
    0x0029B1AA: "StateTable_29b1aa",
    0x0029B1BE: "StateTable_29b1be",
    0x0029B1D2: "StateTable_29b1d2",
    0x0029B1E6: "StateTable_29b1e6",
    0x0029B1FA: "StateTable_29b1fa",
    0x0029B20E: "StateTable_29b20e",
    0x0029B222: "StateTable_29b222",
    0x0029B236: "StateTable_29b236",
    0x0029B24A: "StateTable_29b24a",
    0x0029B25E: "StateTable_29b25e",
    0x0029B272: "StateTable_29b272",
    0x0029B286: "StateTable_29b286",
    0x002E9C50: "StateTable_2e9c50",
    0x002E9C64: "StateTable_2e9c64",
    0x002E9C78: "StateTable_2e9c78",
    0x002E9C8C: "StateTable_2e9c8c",
    0x002E9CA0: "StateTable_2e9ca0",
    0x002E9CB4: "StateTable_2e9cb4",
    0x002E9F20: "StateTable_2e9f20",
    0x002EA7A4: "StateTable_2ea7a4",
    0x002EA7B8: "StateTable_2ea7b8",
    0x002EA7CC: "StateTable_2ea7cc",
    0x002EA7E0: "StateTable_2ea7e0",
    0x002EA7F4: "StateTable_2ea7f4",
    0x002EA808: "StateTable_2ea808",
    0x002EA81C: "StateTable_2ea81c",
    0x002EA830: "StateTable_2ea830",
    0x002EA844: "StateTable_2ea844",
    0x002ED1FC: "StateTable_2ed1fc",
    0x002ED210: "StateTable_2ed210",
    0x002ED224: "StateTable_2ed224",
    0x002ED238: "StateTable_2ed238",
    0x002ED24C: "StateTable_2ed24c",
    0x002ED260: "StateTable_2ed260",
    0x002ED274: "StateTable_2ed274",
    0x002ED288: "StateTable_2ed288",
    0x002ED29C: "StateTable_2ed29c",
    0x002ED2B0: "StateTable_2ed2b0",
    0x002ED2C4: "StateTable_2ed2c4",
    0x002ED2D8: "StateTable_2ed2d8",
    0x002ED2EC: "StateTable_2ed2ec",
    0x002ED300: "StateTable_2ed300",
    0x002ED314: "StateTable_2ed314",
    0x002ED328: "StateTable_2ed328",
    0x002ED33C: "StateTable_2ed33c",
    0x002ED350: "StateTable_2ed350",
    0x002ED364: "StateTable_2ed364",
    0x002ED378: "StateTable_2ed378",
    0x002ED38C: "StateTable_2ed38c",
    0x002ED3A0: "StateTable_2ed3a0",
    0x002ED3B4: "StateTable_2ed3b4",
    0x002ED3C8: "StateTable_2ed3c8",
    0x002ED3DC: "StateTable_2ed3dc",
    0x002ED3F0: "StateTable_2ed3f0",
    0x002ED404: "StateTable_2ed404",
    0x002ED418: "StateTable_2ed418",
    0x002ED42C: "StateTable_2ed42c",
    0x002ED440: "StateTable_2ed440",
    0x002ED454: "StateTable_2ed454",
    0x002ED468: "StateTable_2ed468",
    0x002ED47C: "StateTable_2ed47c",
    0x002ED490: "StateTable_2ed490",
    0x002ED4A4: "StateTable_2ed4a4",
    0x002ED4B8: "StateTable_2ed4b8",
    0x002ED4CC: "StateTable_2ed4cc",
    0x002ED4E0: "StateTable_2ed4e0",
    0x002ED4F4: "StateTable_2ed4f4",
    0x002ED508: "StateTable_2ed508",
    0x002ED51C: "StateTable_2ed51c",
    0x002ED530: "StateTable_2ed530",
    0x002ED544: "StateTable_2ed544",
    0x002ED558: "StateTable_2ed558",
    0x002ED56C: "StateTable_2ed56c",
    0x002ED580: "StateTable_2ed580",
    0x002ED594: "StateTable_2ed594",
    0x002ED5A8: "StateTable_2ed5a8",
    0x002ED5BC: "StateTable_2ed5bc",
    0x002ED5D0: "StateTable_2ed5d0",
    0x002ED5E4: "StateTable_2ed5e4",
    0x002ED5F8: "StateTable_2ed5f8",
    0x002ED60C: "StateTable_2ed60c",
    0x002ED620: "StateTable_2ed620",
    0x002ED634: "StateTable_2ed634",
    0x002ED648: "StateTable_2ed648",
    0x002ED65C: "StateTable_2ed65c",
    0x002ED670: "StateTable_2ed670",
    0x002ED684: "StateTable_2ed684",
    0x002ED698: "StateTable_2ed698",
    0x002ED6AC: "StateTable_2ed6ac",
    0x002ED6C0: "StateTable_2ed6c0",
    0x002ED6D4: "StateTable_2ed6d4",
    0x002ED6E8: "StateTable_2ed6e8",
    0x002ED6FC: "StateTable_2ed6fc",
    0x002ED710: "StateTable_2ed710",
    0x002ED724: "StateTable_2ed724",
    0x002EDAE4: "StateTable_2edae4",
    0x002EDB20: "StateTable_2edb20",
    0x002EDB70: "StateTable_2edb70",
    0x002EDBAC: "StateTable_2edbac",
    0x002EDBFC: "StateTable_2edbfc",
    0x002EDC10: "StateTable_2edc10",
    0x002EDC24: "StateTable_2edc24",
    0x002EDC38: "StateTable_2edc38",
    0x002EDC4C: "StateTable_2edc4c",
    0x002EDC60: "StateTable_2edc60",
    0x002EDC74: "StateTable_2edc74",
    0x002EDC88: "StateTable_2edc88",
    0x002EDC9C: "StateTable_2edc9c",

    # ---- Handlers de SetTaskHandler (AUTO-GEN) -----------------------
    0x00000B90: "TaskHandler_000b90",
    0x00000EF0: "TaskHandler_000ef0",
    0x00000F1A: "TaskHandler_000f1a",
    # 0x00001B4C promovido a Rank_DelayStart_001b4c en registry (Wave JJJJJ).
    # 0x00001B70 promovido a Rank_DelayTick_001b70 en registry (Wave JJJJJ).
    # 0x00001B80 promovido a Rank_DelayGate_001b80 en registry (Wave JJJJJ).
    # 0x000257EC promovido a HUD_State_InsertCoin_0257ec en registry (Wave BBBBB).
    # 0x00025882 promovido a HUD_State_Continue_025882 en registry (Wave BBBBB).
    # 0x00025AD8 promovido a HUD_State_Respawn_025ad8 en registry (Wave BBBBB).
    # 0x00025B34 promovido a HUD_State_GameOverEntry_025b34 en registry (Wave BBBBB).
    0x00025D5C: "TaskHandler_025d5c",
    # 0x00025D64 promovido a HUD_State_GameOverFinal_025d64 en registry (Wave BBBBB).
    # 0x0002B05E promovido a Slug_DropDescend_02b05e en registry (Wave CCCC).
    # 0x0002B264 promovido a Slug_DropBossDescend_02b264 en registry (Wave CCCC).
    # 0x0002D02E promovido a Slug_Fall_02d02e en registry (Wave CCCC).
    # 0x0002DA38 promovido a Slug_DeathExplode_02da38 en registry (Wave CCCC).
    0x0002FF86: "TaskHandler_02ff86",
    0x00030BF6: "TaskHandler_030bf6",
    0x00030D74: "TaskHandler_030d74",
    0x000318AC: "TaskHandler_0318ac",
    # 0x000318D4 promovido a VehicleLaunch_ReleaseParent_0318d4 en registry (Wave AAAA).
    0x000321BC: "TaskHandler_0321bc",
    # 0x00036D64 promovido a Player_Knockback_036d64 en registry (Wave VVV).
    # 0x00037B8E promovido a Player_DeathPit_037b8e en registry (Wave VVV).
    # 0x00037C1A promovido a TaskHandler_037c1a en registry (Wave VVV).
    # 0x00038CEE promovido a TaskHandler_038cee en registry (Wave WWW).
    # 0x00038E4A promovido a TaskHandler_038e4a en registry (Wave WWW).
    # 0x000391AA promovido a TaskHandler_0391aa en registry (Wave WWW).
    # 0x0003DC16 promovido a Results_Countdown_03dc16 en registry (Wave RRRR).
    # 0x0003DC2C promovido a Results_FadeInit_03dc2c en registry (Wave RRRR).
    # 0x0003DC74 promovido a Results_SetupPlayers_03dc74 en registry (Wave RRRR).
    # 0x0003DEA8 promovido a Results_WaitDone_03dea8 en registry (Wave RRRR).
    # 0x0003DEBE promovido a Results_FadeOut_03debe en registry (Wave RRRR).
    # 0x0003DEE2 promovido a Results_Teardown_03dee2 en registry (Wave RRRR).
    # 0x0003DF32 promovido a Results_ToBanner_03df32 en registry (Wave RRRR).
    0x0003DF54: "TaskHandler_03df54",
    # 0x0003E084 promovido a Results_ColPhaseScore_03e084 en registry (Wave RRRR).
    # 0x0003E4E6 promovido a Results_ColFinish_03e4e6 en registry (Wave RRRR).
    0x0003E50C: "TaskHandler_03e50c",
    # 0x0003EAA2 promovido a Results_BannerAFinal_03eaa2 en registry (Wave RRRR).
    0x0003EB82: "TaskHandler_03eb82",
    # 0x0003EBF8 promovido a Results_BannerBDraw_03ebf8 en registry (Wave RRRR).
    0x0003EC16: "TaskHandler_03ec16",
    # 0x0003EC8C promovido a Results_BannerCDraw_03ec8c en registry (Wave RRRR).
    0x0003ECAA: "TaskHandler_03ecaa",
    # 0x0003FCC0 promovido a Pow_FreedWait_03fcc0 en registry (Wave RRRR).
    # 0x0003FDD0 promovido a Pow_WaitRider_03fdd0 en registry (Wave RRRR).
    # 0x0003FE66 promovido a Pow_Idle_03fe66 en registry (Wave RRRR).
    # 0x0004049C promovido a SquadLeader_Enter_04049c en registry (Wave RRRR).
    # 0x00040D18 promovido a SquadLeader_DeathDone_040d18 en registry (Wave RRRR).
    # 0x00040E54 promovido a SquadLeader_Order88_040e54 en registry (Wave RRRR).
    0x00040EF2: "TaskHandler_040ef2",
    0x0004155A: "TaskHandler_04155a",
    0x0004157E: "TaskHandler_04157e",
    0x0004181C: "TaskHandler_04181c",
    0x00042740: "TaskHandler_042740",
    0x00042A44: "TaskHandler_042a44",
    0x00042A56: "TaskHandler_042a56",
    0x00042A6E: "TaskHandler_042a6e",
    0x00042ACC: "TaskHandler_042acc",
    0x00044C1A: "TaskHandler_044c1a",
    0x00044DF2: "TaskHandler_044df2",
    0x00044F8A: "TaskHandler_044f8a",
    0x00044F9A: "TaskHandler_044f9a",
    0x00045F2C: "TaskHandler_045f2c",
    0x000463BA: "TaskHandler_0463ba",
    0x000465DE: "TaskHandler_0465de",
    # 0x00046664 promovido a Fade_WhiteFlash_Done_046664 en registry (Wave DDDDD).
    # 0x000466B4 promovido a SceneC_Load_Spawn2_0466b4 en registry (Wave DDDDD).
    # 0x000466DA promovido a SceneC_Load_Finish_0466da en registry (Wave DDDDD).
    # 0x0004703A promovido a MissionNumBanner_WaitDismiss_04703a en registry (Wave DDDDD).
    # 0x00047050 promovido a MissionNumBanner_ScrollOut_047050 en registry (Wave DDDDD).
    # 0x00047146 promovido a MissionStart_Clear_047146 en registry (Wave DDDDD).
    # 0x0004718A promovido a MissionStart_Redraw_04718a en registry (Wave DDDDD).
    # 0x00047278 promovido a MissionComplete_Pause_047278 en registry (Wave DDDDD).
    # 0x0004728E promovido a MissionComplete_Clear_04728e en registry (Wave DDDDD).
    # 0x000472D2 promovido a MissionComplete_Redraw_0472d2 en registry (Wave DDDDD).
    # 0x0004731C promovido a MissionComplete_ScrollOut_04731c en registry (Wave DDDDD).
    # 0x00047362 promovido a MissionComplete_Finish_047362 en registry (Wave DDDDD).
    0x00048B1E: "TaskHandler_048b1e",
    0x00048B26: "TaskHandler_048b26",
    # 0x00048DDC promovido a PowRope_BrokenA_048ddc en registry (Wave JJJJ).
    # 0x00048DEC promovido a PowRope_BrokenB_048dec en registry (Wave JJJJ).
    # 0x0004968A promovido a PowHang_Struggle_04968a en registry (Wave LLLL).
    # 0x0004A014 promovido a HumanDeath_EntryKind0_04a014 en registry (Wave KKKK).
    # 0x0004A024 promovido a HumanDeath_EntryKind1_04a024 en registry (Wave KKKK).
    # 0x0004A18C promovido a HumanDeath_CorpseA_04a18c en registry (Wave KKKK).
    0x0004AC32: "TaskHandler_04ac32",
    # 0x0004BC48 promovido a GunPlatform_SpawnCrewAndIdle_04bc48 en registry (Wave NNNN).
    # 0x0004C578 promovido a GunPlatform_MarkDead_04c578 en registry (Wave NNNN).
    # 0x0004C58C promovido a GunPlatform_FreeClearBit1_04c58c en registry (Wave NNNN).
    # 0x0004C606 promovido a GunPlatform_DestroyedWithWreck_04c606 en registry (Wave NNNN).
    # 0x0004C68A promovido a GunPlatform_Wreck_04c68a en registry (Wave NNNN).
    0x0004C934: "TaskHandler_04c934",
    # 0x0004DC88 promovido a Prop_TowerTopWreckLoop_04dc88 en registry (Wave OOOO).
    # 0x0004DE32 promovido a Prop_TowerBaseWreckLoop_04de32 en registry (Wave OOOO).
    # 0x0004EB94 promovido a Prop_FortressWaitScroll_04eb94 en registry (Wave PPPP).
    # 0x0004F2A4 promovido a Prop_SlotPrioCheckRts_04f2a4 en registry (Wave PPPP).
    # 0x00050976 promovido a Allen_Spawn_050976 en registry (Wave QQQQ).
    # 0x00051452 promovido a Allen_GrenadeExplodeTick_051452 en registry (Wave QQQQ).
    0x0005147E: "TaskHandler_05147e",
    # 0x00052514 promovido a PalFade_Out_Step_052514 en registry (Wave HHHHH).
    0x000526AA: "TaskHandler_0526aa",
    0x00053C5C: "TaskHandler_053c5c",
    # 0x00053C64 promovido a Prop_BurnFollowVictim_053c64 en registry (Wave HHHH).
    0x00056058: "TaskHandler_056058",
    # 0x00056204 promovido a Bounce_Explode_056204 en registry (Wave FFFFF).
    0x00056596: "TaskHandler_056596",
    # 0x00057F4E promovido a Soldier_GrabThrownA_057f4e en registry (Wave EEEE).
    # 0x00058412 promovido a Soldier_Hurt_058412 en registry (Wave EEEE).
    # 0x00058B1E promovido a Soldier_ThrowGrenadeA_058b1e en registry (Wave EEEE).
    # 0x00058C8E promovido a Soldier_ThrowGrenadeB_058c8e en registry (Wave EEEE).
    # 0x00058CCE promovido a Soldier_ThrowGrenadeB_Loop_058cce en registry (Wave EEEE).
    # 0x0005943A promovido a Ending_ShowAll_05943a en registry (Wave EEEEE).
    # 0x0005947A promovido a Ending_ShowOver_05947a en registry (Wave EEEEE).
    # 0x000594BA promovido a Ending_Exit_0594ba en registry (Wave EEEEE).
    # 0x00059722 promovido a Result_PrintContinueLabel_059722 en registry (Wave EEEEE).
    # 0x00059756 promovido a Result_RollContinues_059756 en registry (Wave EEEEE).
    # 0x000597B0 promovido a Result_PrintPrisonerLabel_0597b0 en registry (Wave EEEEE).
    # 0x0005980A promovido a Result_RollPrisoners_05980a en registry (Wave EEEEE).
    # 0x00059864 promovido a Result_PrintScore_059864 en registry (Wave EEEEE).
    # 0x000598AE promovido a Result_HiScoreEntry_0598ae en registry (Wave EEEEE).
    # 0x0005994A promovido a Ending_Seq_ShowMission_05994a en registry (Wave EEEEE).
    # 0x0005996C promovido a Ending_Seq_Wait30_05996c en registry (Wave EEEEE).
    # 0x00059988 promovido a Ending_Seq_Wipe_059988 en registry (Wave EEEEE).
    # 0x000599AA promovido a Ending_Seq_Wait15_0599aa en registry (Wave EEEEE).
    # 0x000599C6 promovido a Ending_Seq_PanelP1_0599c6 en registry (Wave EEEEE).
    # 0x000599F2 promovido a Ending_Seq_PanelP2_0599f2 en registry (Wave EEEEE).
    # 0x00059A1A promovido a Ending_Seq_FadeA0_059a1a en registry (Wave EEEEE).
    # 0x00059A40 promovido a Ending_Seq_Fade40_059a40 en registry (Wave EEEEE).
    # 0x00059A70 promovido a Ending_Seq_Done_059a70 en registry (Wave EEEEE).
    # 0x00059B86 promovido a Ending_ShowPeaceForever_059b86 en registry (Wave EEEEE).
    # 0x00059BC6 promovido a Ending_PeaceExit_059bc6 en registry (Wave EEEEE).
    # 0x00059C42 promovido a Gunner_Search_059c42 en registry (Wave EEEEE).
    # 0x00059D62 promovido a Gunner_Child_Sync_059d62 en registry (Wave EEEEE).
    # 0x0005A28A promovido a Gunner2_Search_05a28a en registry (Wave EEEEE).
    # 0x0005A66E promovido a Gunner2_Child_Sync_05a66e en registry (Wave EEEEE).
    # 0x0005A764 promovido a Walker_Patrol_05a764 en registry (Wave EEEEE).
    0x0005CBEA: "TaskHandler_05cbea",
    # 0x0005F00A promovido a DebugColl_Idle_05f00a en registry (Wave SSSS).
    # 0x0005F0B0 promovido a DebugColl_ShowHighNibble_05f0b0 en registry (Wave SSSS).
    # 0x0005F482 promovido a TowerSoldier_Fire_05f482 en registry (Wave SSSS).
    # 0x0005FA56 promovido a HutOccupant_Idle_05fa56 en registry (Wave SSSS).
    # 0x0005FB88 promovido a HutOccupant_WaitParent_05fb88 en registry (Wave SSSS).
    # 0x0005FBE6 promovido a HutDoor_Idle_05fbe6 en registry (Wave SSSS).
    # 0x0005FCE6 promovido a HutDoor_Free_05fce6 en registry (Wave SSSS).
    # 0x000606EE promovido a ItemProp_Idle_0606ee en registry (Wave SSSS).
    # 0x00060BF6 promovido a Obstacle095_Idle_060bf6 en registry (Wave SSSS).
    # 0x00060C40 promovido a Obstacle_FlushMusicJmp77FD6_060c40 en registry (Wave SSSS).
    # 0x00060D3A promovido a Crate_Idle_060d3a en registry (Wave SSSS).
    0x00060DE8: "TaskHandler_060de8",
    # 0x000620A0 promovido a JmpScheduler_0620a0 en registry (Wave TTTT).
    0x000620A8: "TaskHandler_0620a8",
    0x00062F8C: "TaskHandler_062f8c",
    0x00062F9A: "TaskHandler_062f9a",
    # 0x00063942 promovido a JmpScheduler_063942 en registry (Wave TTTT).
    0x00063952: "TaskHandler_063952",
    0x00064222: "TaskHandler_064222",
    0x0006422A: "TaskHandler_06422a",
    # 0x00064D7A promovido a Hostage_Free_064d7a en registry (Wave TTTT).
    0x00064D8A: "TaskHandler_064d8a",
    0x0006515E: "TaskHandler_06515e",
    # 0x000667A4 promovido a Paratrooper_Spawner_Done_0667a4 en registry (Wave UUUU).
    0x00066A86: "TaskHandler_066a86",
    0x00066A8E: "TaskHandler_066a8e",
    0x00067B7C: "TaskHandler_067b7c",
    0x00067B84: "TaskHandler_067b84",
    # 0x00068310 promovido a S5Gate_WaitScroll_068310 en registry (Wave UUUU).
    # 0x00068346 promovido a S5Gate_Begin_068346 en registry (Wave UUUU).
    # 0x0006850E promovido a S5Airship_PartIdle_06850e en registry (Wave UUUU).
    # 0x00068684 promovido a S5Airship_PartState3_068684 en registry (Wave UUUU).
    # 0x000688F8 promovido a S5Airship_HatchFinal_0688f8 en registry (Wave UUUU).
    0x0006895C: "TaskHandler_06895c",
    0x0006986A: "TaskHandler_06986a",
    0x00069872: "TaskHandler_069872",
    # 0x0006A41C promovido a Bazooka_Corpse_06a41c en registry (Wave VVVV).
    0x0006A452: "TaskHandler_06a452",
    0x0006A468: "TaskHandler_06a468",
    # 0x0006A93E promovido a BazookaCrew_Die_06a93e en registry (Wave VVVV).
    0x0006ACAA: "TaskHandler_06acaa",
    # 0x0006C4C6 promovido a RocketVehicle_DetachLinks_06c4c6 en registry (Wave VVVV).
    0x0006C53E: "TaskHandler_06c53e",
    0x0006C554: "TaskHandler_06c554",
    0x0006DA7A: "TaskHandler_06da7a",
    0x0006DA90: "TaskHandler_06da90",
    # 0x0006E062 promovido a FireBurst_Fly_06e062 en registry (Wave WWWW).
    # 0x0006E80A promovido a Gunship_Free_06e80a en registry (Wave WWWW).
    0x0006E818: "TaskHandler_06e818",
    # 0x0006E88C promovido a Gunship_SpawnCrewArc_06e88c en registry (Wave WWWW).
    # 0x0006E932 promovido a Gunship_FallToGround_06e932 en registry (Wave WWWW).
    # 0x0006F340 promovido a M5Boss_WaitScroll_06f340 en registry (Wave WWWW).
    # 0x0006F38C promovido a M5Boss_PlayMusic10BA_06f38c en registry (Wave WWWW).
    # 0x0006F9CA promovido a M5Boss_Die_06f9ca en registry (Wave WWWW).
    # 0x0006FA60 promovido a M5Boss_Explode_06fa60 en registry (Wave WWWW).
    0x0006FB02: "TaskHandler_06fb02",
    # 0x000700A6 promovido a M5Boss_Free_0700a6 en registry (Wave WWWW).
    # 0x00070290 promovido a M5Boss_SparkEnd_070290 en registry (Wave WWWW).
    0x000704F2: "TaskHandler_0704f2",
    # 0x00070694 promovido a M5Boss_Grenade_070694 en registry (Wave WWWW).
    # 0x0007079E promovido a M5Boss_GrenadeRest_07079e en registry (Wave WWWW).
    0x000707C8: "TaskHandler_0707c8",
    # 0x000713A2 promovido a FinalBoss_SnapIdle_0713a2 en registry (Wave WWWW).
    # 0x000716B6 promovido a FinalBoss_ResetParts_0716b6 en registry (Wave WWWW).
    0x000716E2: "TaskHandler_0716e2",
    0x000716EA: "TaskHandler_0716ea",
    0x000716F2: "TaskHandler_0716f2",
    # 0x00071B9A promovido a FinalBoss_HeadDieB_071b9a en registry (Wave WWWW).
    0x000724D4: "TaskHandler_0724d4",
    # 0x00073454 promovido a M5Tank_Sink_073454 en registry (Wave XXXX).
    0x000734F4: "TaskHandler_0734f4",
    0x000734FC: "TaskHandler_0734fc",
    # 0x000735A6 promovido a M5Tank_PartExplode_0735a6 en registry (Wave XXXX).
    # 0x000738DA promovido a M5Tank_TurretWreck_0738da en registry (Wave XXXX).
    # 0x00073B62 promovido a M5Tank_CasingGround_073b62 en registry (Wave XXXX).
    # 0x00073F9A promovido a M5Tank_MuzzleE_073f9a en registry (Wave XXXX).
    # 0x000751A6 promovido a ScriptedProp_Run_0751a6 en registry (Wave XXXX).
    # 0x0007525C promovido a ScriptedProp_Dead_07525c en registry (Wave XXXX).
    # 0x00076056 promovido a ScriptedProp_Wall_Idle_076056 en registry (Wave YYYY).
    # 0x000760A8 promovido a ScriptedProp_Wall_WaitTimer_0760a8 en registry (Wave YYYY).
    # 0x000760C8 promovido a ScriptedProp_Wall_Alt_0760c8 en registry (Wave YYYY).
    # 0x00076290 promovido a ScriptedProp_TowerB_Aim_076290 en registry (Wave YYYY).
    # 0x0007646E promovido a ScriptedProp_Child_Collapse_07646e en registry (Wave YYYY).
    # 0x0007692C promovido a ScriptedProp_Window_Idle_07692c en registry (Wave YYYY).
    # 0x00076A90 promovido a ScriptedProp_RoofA_Debris_076a90 en registry (Wave YYYY).
    # 0x00076B3A promovido a ScriptedProp_RoofB_Debris_076b3a en registry (Wave YYYY).
    # 0x00076BD0 promovido a ScriptedProp_Gate_Idle_076bd0 en registry (Wave YYYY).
    # 0x0007726A promovido a MovingPlatform_Run_07726a en registry (Wave YYYY).
    0x00077A8E: "TaskHandler_077a8e",
    # 0x00079326 promovido a AutoDemo_Stop_079326 en registry (Wave YYYY).
    # 0x00079A8A promovido a Crew_Jump_Run_079a8a en registry (Wave YYYY).
    # 0x00079B42 promovido a Crew_Free_079b42 en registry (Wave YYYY).
    # 0x00079C3C promovido a Crew_Hatch_Flee_079c3c en registry (Wave YYYY).
    # 0x00079C68 promovido a Crew_Hatch_Free_079c68 en registry (Wave YYYY).
    # 0x00079CAA promovido a Crew_JumpB_Run_079caa en registry (Wave YYYY).
    # 0x00079D76 promovido a Crew_FreeB_079d76 en registry (Wave YYYY).
    # 0x00079E68 promovido a Crew_HatchB_Flee_079e68 en registry (Wave YYYY).
    # 0x00079E94 promovido a Crew_HatchB_Free_079e94 en registry (Wave YYYY).
    # 0x00079F6A promovido a Crew_Gunner_Run_079f6a en registry (Wave YYYY).
    # 0x0007A00A promovido a Crew_Hostage_Idle_07a00a en registry (Wave ZZZZ).
    # 0x0007A1C0 promovido a Crew_Captor_Idle_07a1c0 en registry (Wave ZZZZ).
    0x0007A3E6: "TaskHandler_07a3e6",
    # 0x0007A3EE promovido a Crew_Captor_MarkParentDead_07a3ee en registry (Wave ZZZZ).
    0x0007AA78: "TaskHandler_07aa78",
    0x0007AA94: "TaskHandler_07aa94",
    0x0007ABF4: "TaskHandler_07abf4",
    0x0007AC4A: "TaskHandler_07ac4a",
    # 0x0007BACE promovido a M2Boss_WaitScroll_07bace en registry (Wave ZZZZ).
    # 0x0007BB1A promovido a M2Boss_IntroAnim_07bb1a en registry (Wave ZZZZ).
    # 0x0007C106 promovido a M2Boss_Turret_Recoil_07c106 en registry (Wave ZZZZ).
    # 0x0007C374 promovido a M2Boss_Turret_DeathSwing_07c374 en registry (Wave ZZZZ).
    # 0x0007C424 promovido a M2Boss_Turret_DeathFall_07c424 en registry (Wave ZZZZ).
    # 0x0007C644 promovido a M2Boss_Turret_FreeWithParent_07c644 en registry (Wave ZZZZ).
    0x0007C65C: "TaskHandler_07c65c",
    # 0x0007CF5C promovido a Crab_SpawnClaws_07cf5c en registry (Wave ZZZZ).
    # 0x0007D898 promovido a Crab_Leg_Detach_07d898 en registry (Wave ZZZZ).
    # 0x0007DBA2 promovido a Crab_MarkDetached_07dba2 en registry (Wave ZZZZ).
    # 0x0007DBA8 promovido a Crab_Free_07dba8 en registry (Wave ZZZZ).
    # 0x0007DCB6 promovido a Crab_Patrol_Walk_07dcb6 en registry (Wave ZZZZ).
    # 0x0007E078 promovido a Carrier_InitWithHatchB_07e078 en registry (Wave ZZZZ).
    # 0x0007E08C promovido a Carrier_Init_07e08c en registry (Wave ZZZZ).
    # 0x0007E674 promovido a Carrier_Cockpit_Run_07e674 en registry (Wave ZZZZ).
    # 0x0007E880 promovido a Carrier_Mark_Run_07e880 en registry (Wave ZZZZ).
    # 0x0007EBE6 promovido a Carrier_Spawner_Run_07ebe6 en registry (Wave ZZZZ).
    # 0x0007F022 promovido a Carrier_Trooper_Aim_07f022 en registry (Wave ZZZZ).
    # 0x0007F186 promovido a Carrier_Trooper_Fire_07f186 en registry (Wave ZZZZ).
    # 0x0007F282 promovido a Carrier_Rider_Ride_07f282 en registry (Wave ZZZZ).
    # 0x0007F2CA promovido a Carrier_Rider_Jump_07f2ca en registry (Wave ZZZZ).
    # 0x0007F87C promovido a Carrier_Gunner_PoseRun_07f87c en registry (Wave ZZZZ).
    0x0007FDB6: "TaskHandler_07fdb6",
    0x0007FDBC: "TaskHandler_07fdbc",
    0x00080382: "TaskHandler_080382",
    0x000803D2: "TaskHandler_0803d2",
    0x000803E8: "TaskHandler_0803e8",
    0x00080454: "TaskHandler_080454",
    0x00080508: "TaskHandler_080508",
    0x000805EE: "TaskHandler_0805ee",
    0x00081214: "TaskHandler_081214",
    0x00082456: "TaskHandler_082456",
    0x00082464: "TaskHandler_082464",
    # 0x00085134 promovido a M4_PlatformSpawn_085134 en registry (Wave JJJ).
    # 0x00085484 promovido a M4_Turret_Death_085484 en registry (Wave JJJ).
    # 0x00085A08 promovido a M4_CamFloor_Step_085a08 en registry (Wave JJJ).
    # 0x000865BE promovido a Boss_Shadow_Clear_0865be en registry (Wave KKK).
    # 0x00086854 promovido a Fort_Idle_086854 en registry (Wave KKK).
    # 0x00089398 promovido a S5_Depot_Idle_089398 en registry (Wave LLL).
    # 0x00089504 promovido a Airship_Landed_089504 en registry (Wave LLL).
    # 0x000895CC promovido a Airship_Hover_0895cc en registry (Wave LLL).
    # 0x000898D4 promovido a Turret8_Aim_0898d4 en registry (Wave LLL).
    # 0x00089960 promovido a Turret8_Track_089960 en registry (Wave LLL).
    # 0x00089A04 promovido a Turret8_Cooldown_089a04 en registry (Wave LLL).
    # 0x0008A31C promovido a S5_TowerA_Stage2_08a31c en registry (Wave LLL).
    # 0x0008A44C promovido a S5_TowerA_Stage3_08a44c en registry (Wave LLL).
    # 0x0008A516 promovido a S5_TowerA_Stage4_08a516 en registry (Wave LLL).
    # 0x0008A5C8 promovido a S5_TowerA_Destroy_08a5c8 en registry (Wave LLL).
    # 0x0008A9B0 promovido a S5_TowerB_Stage2_08a9b0 en registry (Wave LLL).
    # 0x0008AAF2 promovido a S5_TowerB_Stage3_08aaf2 en registry (Wave LLL).
    # 0x0008ABD4 promovido a S5_TowerB_Stage4_08abd4 en registry (Wave LLL).
    # 0x0008AC92 promovido a S5_TowerB_Destroy_08ac92 en registry (Wave LLL).
    # 0x0008AE38 promovido a Wreck_FlagClear_08ae38 en registry (Wave LLL).
    # 0x0008AF68 promovido a TowerPort_Stage2_08af68 en registry (Wave LLL).
    # 0x0008B03C promovido a TowerPort_Idle_08b03c en registry (Wave LLL).
    # 0x0008B10A promovido a Wreck_SparkBurst_08b10a en registry (Wave LLL).
    0x0008BB84: "TaskHandler_08bb84",
    # 0x0008C678 promovido a Icon_Anchor_Run_08c678 en registry (Wave MMM).
    # 0x0008C8FA promovido a Cut_Watcher_Play_08c8fa en registry (Wave MMM).
    # 0x0008CF22 promovido a SceneB_Stage2_08cf22 en registry (Wave MMM).
    # 0x0008CF6C promovido a SceneB_Stage3_08cf6c en registry (Wave MMM).
    # 0x0008CFB6 promovido a SceneB_Stage4_08cfb6 en registry (Wave MMM).
    # 0x0008D000 promovido a SceneB_Stage5_08d000 en registry (Wave MMM).
    # 0x0008D04A promovido a SceneB_Stage6_08d04a en registry (Wave MMM).
    # 0x0008D094 promovido a SceneB_Tail_08d094 en registry (Wave MMM).
    # 0x0008D41C promovido a Capsule_Descend_08d41c en registry (Wave NNN).
    # 0x0008D450 promovido a Capsule_WaitGround_08d450 en registry (Wave NNN).
    0x0008D472: "TaskHandler_08d472",
    # 0x0008D4C6 promovido a Capsule_Flash_08d4c6 en registry (Wave NNN).
    # 0x0008D4FE promovido a Capsule_FadeIn_08d4fe en registry (Wave NNN).
    # 0x0008D55C promovido a Capsule_GlowDown_08d55c en registry (Wave NNN).
    # 0x0008D580 promovido a Capsule_Glow2_08d580 en registry (Wave NNN).
    # 0x0008D5A8 promovido a Capsule_GlowUp_08d5a8 en registry (Wave NNN).
    # 0x0008D62E promovido a Capsule_Fall_08d62e en registry (Wave NNN).
    # 0x0008D72A promovido a MissionEnd_SpawnDropper_08d72a en registry (Wave NNN).
    # 0x0008D774 promovido a Capsule_DimWaitScroll_08d774 en registry (Wave NNN).
    # 0x0008D7BE promovido a Capsule_FallToGround_08d7be en registry (Wave NNN).
    # 0x0008D8F4 promovido a MissionEnd_ScrollUp_08d8f4 en registry (Wave NNN).
    # 0x0008DB7A promovido a Grunt_HopDown_08db7a en registry (Wave NNN).
    # 0x0008DBE2 promovido a Grunt_HopUp_08dbe2 en registry (Wave NNN).
    # 0x0008DF72 promovido a Grunt_Run_08df72 en registry (Wave NNN).
    # 0x0008DFB8 promovido a Grunt2_Rand_Stand_08dfb8 en registry (Wave NNN).
    # 0x0008E288 promovido a Sentry_Wait_08e288 en registry (Wave NNN).
    # 0x0008E2F0 promovido a Grunt_Tmpl180_Run_08e2f0 en registry (Wave NNN).
    # 0x0008E4FE promovido a Bobber_Descend_08e4fe en registry (Wave OOO).
    # 0x0008E566 promovido a Bobber_Ascend_08e566 en registry (Wave OOO).
    # 0x0008E622 promovido a Leaper_Jump_08e622 en registry (Wave OOO).
    # 0x0008E716 promovido a Runner_Run_08e716 en registry (Wave OOO).
    # 0x0008EA16 promovido a Nest_FlyOff_Run_08ea16 en registry (Wave OOO).
    # 0x0008EB02 promovido a Nest2_HitWait_08eb02 en registry (Wave OOO).
    # 0x0008F96A promovido a GameOver_Spawn_08f96a en registry (Wave PPP).
    # 0x0008FCCA promovido a GameOver_Wait_08fcca en registry (Wave PPP).
    # 0x0008FD2E promovido a GameOver_Wait2_08fd2e en registry (Wave PPP).
    # 0x0008FD68 promovido a GameOver_Final_08fd68 en registry (Wave PPP).
    # 0x0008FDAA promovido a GameOver_WaitCredit_08fdaa en registry (Wave PPP).
    # 0x0008FDF8 promovido a GameOver_Continue_08fdf8 en registry (Wave PPP).
    # 0x0008FE32 promovido a GameOver_ContinuePath_08fe32 en registry (Wave PPP).
    # 0x0008FE9C promovido a GameOver_ContinueHold_08fe9c en registry (Wave PPP).
    # 0x0008FEB6 promovido a GameOver_ContinueEnd_08feb6 en registry (Wave PPP).
    0x0008FECA: "TaskHandler_08feca",
    # 0x00090098 promovido a GO_Sprite_FollowParent_090098 en registry (Wave PPP).
    0x00090E7E: "TaskHandler_090e7e",
    # 0x00091338 promovido a Continue_Spawn_091338 en registry (Wave PPP).
    0x000913AA: "TaskHandler_0913aa",
    # 0x00091514 promovido a Continue_Text_Init_091514 en registry (Wave PPP).
    # 0x0009152C promovido a Continue_Text_Clear_09152c en registry (Wave PPP).
    # 0x00091558 promovido a Continue_Text_Blink_091558 en registry (Wave PPP).
    0x000916C0: "TaskHandler_0916c0",
    # 0x00097852 promovido a HiScore_WaitLogo_097852 en registry (Wave QQQ).
    # 0x0009788C promovido a HiScore_HeaderDelay_09788c en registry (Wave QQQ).
    # 0x000978AC promovido a HiScore_DrawRowsStep_0978ac en registry (Wave QQQ).
    # 0x000978FA promovido a HiScore_DrawAllRows_0978fa en registry (Wave QQQ).
    # 0x0009792E promovido a HiScore_WaitExit_09792e en registry (Wave QQQ).
    0x0009794A: "TaskHandler_09794a",
    # 0x0009806A promovido a NameEntry_Blink_09806a en registry (Wave QQQ).
    # 0x00098308 promovido a MemCard_LoadDialog_Run_098308 en registry (Wave QQQ).
    # 0x000983F6 promovido a MemCard_Dialog_ExitDelay_0983f6 en registry (Wave QQQ).
    # 0x00098482 promovido a MemCard_SaveDialog_Run_098482 en registry (Wave QQQ).
    0x00098836: "TaskHandler_098836",
    # 0x00098886 promovido a LogoScene_Piece_Run_098886 en registry (Wave QQQ).
    # 0x0009890C promovido a LogoScene_Center_Run_09890c en registry (Wave QQQ).
    # 0x000989E0 promovido a Mob_Walk_0989e0 en registry (Wave QQQ).
    # 0x00098AFE promovido a Mob_Tmpl175_Idle_098afe en registry (Wave QQQ).
    # 0x00098C00 promovido a Mob_Tmpl176_Body_098c00 en registry (Wave QQQ).
    # 0x00098DC6 promovido a Mob_Tmpl178_Body_098dc6 en registry (Wave QQQ).
    # 0x00099004 promovido a Mob_Stand_099004 en registry (Wave QQQ).
    # 0x00099180 promovido a Mob_Appear_099180 en registry (Wave QQQ).
    # 0x0009921A promovido a Mob_Sit_09921a en registry (Wave QQQ).
    # 0x000993A2 promovido a Mob_Tmpl209_Body_0993a2 en registry (Wave QQQ).
    # 0x0009953E promovido a Mob_RunLeft_09953e en registry (Wave QQQ).
    # 0x00099610 promovido a Mob_RunLeft2_099610 en registry (Wave QQQ).
    # 0x0009976A promovido a Mob_PhysLoop_09976a en registry (Wave QQQ).
    0x00099794: "TaskHandler_099794",
    # 0x00099A64 promovido a DebugCursor_Run_099a64 en registry (Wave QQQ).
    # 0x0009A280 promovido a Gun_Child_Init_09a280 en registry (Wave RRR).
    # 0x0009A2B8 promovido a Gun_Child_Sync_09a2b8 en registry (Wave RRR).
    0x0009B47C: "TaskHandler_09b47c",

    # ---- Targets de JsrAbsThunk (AUTO-GEN) ---------------------------
    0x000004AE: "ThunkTarget_0004ae",
    0x000006FE: "ThunkTarget_0006fe",
    0x00000772: "ThunkTarget_000772",
    0x00000FFE: "ThunkTarget_000ffe",
    0x0000236E: "ThunkTarget_00236e",
    0x00002C26: "ThunkTarget_002c26",
    0x00002C30: "ThunkTarget_002c30",
    0x000138FE: "ThunkTarget_0138fe",
    0x0002783A: "ThunkTarget_02783a",
    0x0002785C: "ThunkTarget_02785c",
    0x0002788C: "ThunkTarget_02788c",
    0x00027A92: "ThunkTarget_027a92",
    0x00027AFC: "ThunkTarget_027afc",
    0x00027C8C: "ThunkTarget_027c8c",
    0x00027CEE: "ThunkTarget_027cee",
    0x00028292: "ThunkTarget_028292",
    0x000283CA: "ThunkTarget_0283ca",
    0x000283D8: "ThunkTarget_0283d8",
    0x00028998: "ThunkTarget_028998",
    0x00028D70: "ThunkTarget_028d70",
    0x0003060A: "ThunkTarget_03060a",
    0x00032AFA: "ThunkTarget_032afa",
    0x00032B36: "ThunkTarget_032b36",
    0x00032CBA: "ThunkTarget_032cba",
    0x00032D00: "ThunkTarget_032d00",
    0x000436DE: "ThunkTarget_0436de",
    0x00043FAC: "ThunkTarget_043fac",
    0x00047482: "ThunkTarget_047482",
    0x000477FC: "ThunkTarget_0477fc",
    0x0004784C: "ThunkTarget_04784c",
    0x00047888: "ThunkTarget_047888",
    0x00049FD0: "ThunkTarget_049fd0",
    0x0005026C: "ThunkTarget_05026c",
    0x0005170C: "ThunkTarget_05170c",
    0x000517FE: "ThunkTarget_0517fe",
    # 0x0005180C promovido a Clear8Bytes_05180c en registry (Wave QQQQ).
    0x00051914: "ThunkTarget_051914",
    0x000519BE: "ThunkTarget_0519be",
    # 0x00051DE2 promovido a CellCommit_MMIO_051DE2 en registry (Wave LL#1).
    # El alias ThunkTarget_051de2 se define ahora como .globl dentro de
    # asm/collision_cell_apply_051bxx.s para que bsr.w ThunkTarget_051de2 en
    # collision_probes_051cxx.s (KK#2) siga resolviendose sin edicion.
    # 0x00051DE2: "ThunkTarget_051de2",
    # 0x00051F30 promovido a TransformCommit_MMIO_051F30 en registry (Wave KK#1).
    # NOTA: el alias ThunkTarget_051f30 se mantiene abajo como referencia externa
    # para que los thunks Wave I existentes (jsr_abs_thunks.c) sigan resolviendo.
    0x0005239E: "ThunkTarget_05239e",
    0x000523B2: "ThunkTarget_0523b2",
    0x0005A9D6: "ThunkTarget_05a9d6",
    0x0005A9E2: "ThunkTarget_05a9e2",
    0x0005CA2A: "ThunkTarget_05ca2a",
    # 0x0005CCC8 promovido a Entity_CopyField6D_05ccc8 en registry (Wave GGGGG).
    0x0005CDFC: "ThunkTarget_05cdfc",
    0x0005D00E: "ThunkTarget_05d00e",
    0x0005D6C2: "ThunkTarget_05d6c2",
    0x0005DA56: "ThunkTarget_05da56",
    0x0005DA9C: "ThunkTarget_05da9c",
    0x0005DAD8: "ThunkTarget_05dad8",
    0x0005DB1A: "ThunkTarget_05db1a",
    0x0005DC34: "ThunkTarget_05dc34",
    0x0005DD02: "ThunkTarget_05dd02",
    0x0005DD2A: "ThunkTarget_05dd2a",
    0x0005E4B2: "ThunkTarget_05e4b2",
    0x0005E5A8: "ThunkTarget_05e5a8",
    0x0005E9E4: "ThunkTarget_05e9e4",
    0x0006E224: "ThunkTarget_06e224",
    0x0006E412: "ThunkTarget_06e412",
    0x00077C7E: "ThunkTarget_077c7e",
    0x000799DE: "ThunkTarget_0799de",
    0x000818AA: "ThunkTarget_0818aa",
    0x0008B558: "ThunkTarget_08b558",
    0x0008F308: "ThunkTarget_08f308",
    0x00096A80: "ThunkTarget_096a80",
    0x000981FC: "ThunkTarget_0981fc",
    0x00099812: "ThunkTarget_099812",
    0x0009A7AA: "ThunkTarget_09a7aa",
    0x0009B9F6: "ThunkTarget_09b9f6",


    # ---- Targets de Waves J/K (AUTO-GEN) -----------------------------
    # 0x00001AF8 promovido a Attract_StartIfP2Flag_001af8 en registry (Wave JJJJJ).
    0x0001399C: "PcThunkTarget_01399c",
    # 0x00025E74 promovido a HUD_SetStartMask_025e74 en registry (Wave BBBBB).
    # 0x000281C8 promovido a Entity_ApplyVelLatch_0281c8 en registry (Wave CCCCC).
    # 0x0002870A promovido a Hitbox_SideOfImpact_02870a en registry (Wave CCCCC).
    # 0x00028758 promovido a Hitbox_CheckBit0_028758 en registry (Wave CCCCC).
    # 0x0002A46C promovido a Slug_ClearFlags8D_02a46c en registry (Wave BBBB).
    # 0x0002AB86 promovido a Slug_InputFireByLayout_02ab86 en registry (Wave BBBB).
    # 0x0002AC4C promovido a Slug_TestField100609_02ac4c en registry (Wave BBBB).
    # 0x0002AC80 promovido a Slug_ConsumeField89_02ac80 en registry (Wave BBBB).
    # 0x0002FADA promovido a Slug_ResetDamageIdx_02fada en registry (Wave ZZZ).
    0x00032EA4: "PcThunkTarget_032ea4",
    0x00032EBA: "PcThunkTarget_032eba",
    0x00032F3C: "PcThunkTarget_032f3c",
    0x00032F88: "PcThunkTarget_032f88",
    # 0x000334A2 promovido a Probe_Bit3At100001_0334A2 en registry (Wave NN#1).
    # El alias antiguo se retira: los callers que hacian bsr.w PcThunkTarget_0334a2
    # ahora resuelven al simbolo canonico definido en el .text de la nueva Wave.
    # 0x000334A2: "PcThunkTarget_0334a2",
    0x00033522: "PcThunkTarget_033522",
    # 0x00036DCA promovido a Player_Knockback_AirCtrl_036dca en registry (Wave VVV).
    # 0x00039416 promovido a PcThunkTarget_039416 en registry (Wave WWW).
    # 0x0003E7A6 promovido a Results_RosterDrawDispatch_03e7a6 en registry (Wave RRRR).
    # 0x0003E84C promovido a Results_RosterDrawB_03e84c en registry (Wave RRRR).
    # 0x0003EE48 promovido a Pow_CountIfPending_03ee48 en registry (Wave RRRR).
    # 0x00041C1A: promovido a Squad_ComputeTargetPos_041C1A (Wave TT)
    # 0x00041DDC: promovido a Squad_BobYWide_041DDC (Wave TT)
    # 0x00041E02: promovido a Squad_BobYNarrow_041E02 (Wave TT)
    # 0x00041FF6: promovido a Squad_PollSharedState_041FF6 (Wave TT)
    # 0x00042040: promovido a Squad_WriteBackState_042040 (Wave TT)
    # ---- Handlers del subsistema escuadron (Wave TT, pc-rel) ---------
    # 0x00040F00: promovido a SquadMember_Handler_040F00 en registry (Wave UU)
    # 0x00040F82: promovido a SquadMember_OnStateChange_040F82 en registry (Wave UU)
    # 0x000415C6: promovido a PairChild_HandlerA_0415C6 en registry (Wave UU)
    # 0x00041626: promovido a PairChild_HandlerB_041626 en registry (Wave UU)
    # 0x000419CC: promovido a TrioChild_HandlerA_0419CC en registry (Wave UU)
    # 0x000419FC: promovido a TrioChild_HandlerB_0419FC en registry (Wave UU)
    # ---- Wave UU: rts INTERNOS de islas SetTaskHandler/JsrAbsThunk ----
    # (idioma "branch a mitad de isla": la cola per-frame sale con bcc/beq
    #  al rts final de la isla adyacente en vez de duplicar un rts propio)
    0x00041326: "SetHandlerRts_041326",  # rts de SetTaskHandler_041320 (cola comun de miembro)
    0x00041488: "SetHandlerRts_041488",  # rts de SetTaskHandler_041482 (SwoopPhysics)
    0x00041558: "JsrAbsRts_041558",      # rts de JsrAbsThunk_041552 (FlipTouchdown)
    0x0004157C: "SetHandlerRts_04157c",  # rts de SetTaskHandler_041576 (TouchdownIdle)
    0x00041624: "SetHandlerRts_041624",  # rts de SetTaskHandler_04161e (PairChild A)
    0x00041760: "SetHandlerRts_041760",  # rts de SetTaskHandler_04175a (PairChild B)
    0x0004184E: "SetHandlerRts_04184e",  # rts de SetTaskHandler_041848 (DropRun)
    0x00041918: "SetHandlerRts_041918",  # rts de SetTaskHandler_041912 (ZigzagFall)
    0x000419BC: "SetHandlerRts_0419bc",  # rts de SetTaskHandler_0419b6 (GlideAttack)
    0x00041AB2: "SetHandlerRts_041ab2",  # rts de SetTaskHandler_041aac (TrioChild B)
    0x00041C18: "SetHandlerRts_041c18",  # rts de SetTaskHandler_041c12 (FinalPose)

    # ---- Wave VV: entradas mid-island (rts dentro de islas C ya matcheadas)
    0x0004247A: "SetTaskWRts_04247a",    # rts de SetTaskW_042476 (EngageAndTimers)
    0x000424A8: "SetHandlerRts_0424a8",  # rts de SetTaskHandler_0424a2 (MeleeGate)
    0x0004290A: "JsrAbsRts_04290a",      # rts de JsrAbsThunk_042904 (Charger_TrackTarget)
    # 0x0004698C promovido a Continue_IsStartP2_04698c en registry (Wave DDDDD).
    # 0x0004707E promovido a MissionNum_TileByMission_04707e en registry (Wave DDDDD).
    # 0x0004FAF8 promovido a Prop_BunkerBlitState10_04faf8 en registry (Wave QQQQ).
    # 0x00053DCA promovido a Prop_SyncSpriteWithParent_053dca en registry (Wave HHHH).
    # 0x00055148 promovido a NeonSign_TilesOffA_055148 en registry (Wave MMMM).
    # 0x00055214 promovido a NeonSign_TilesOnB_055214 en registry (Wave MMMM).
    # 0x00056E1E promovido a Soldier_DespawnIfOffscreen_056e1e en registry (Wave FFFF).
    # 0x00057226 promovido a Soldier_SpawnVariants_057226 en registry (Wave FFFF).
    # 0x0005CDA8 promovido a InputEvtThunk_05cda8 en registry (Wave GGGGG).
    0x0005CEF8: "JmpTarget_05cef8",
    0x0005CF04: "JmpTarget_05cf04",
    0x0005CF6C: "PcThunkTarget_05cf6c",
    0x0005DBC2: "PcThunkTarget_05dbc2",
    # 0x0005DD5C promovido a Entity_CheckOnScreenBox_05dd5c en registry (Wave GGGGG).
    # 0x0005E018 promovido a Atan2_Angle256_05e018 en registry (Wave SSSS).
    # 0x0005E530 promovido a Rng_PickWordFromTable_05e530 en registry (Wave SSSS).
    # 0x00063336 promovido a Camper_ScrollGate_063336 en registry (Wave TTTT).
    # 0x000634F6 promovido a Camper_SpawnPairB_0634f6 en registry (Wave TTTT).
    # 0x00065C94 promovido a EntityGroup_SpawnLinkedFromTemplateList_065C94 en registry (Wave RR#5).
    # 0x0006896A promovido a Camera0_RelinkAndWrapScroll_06896A en registry (Wave RR#3).
    # 0x00068AB8 promovido a S5Airship_PastRightEdge_068ab8 en registry (Wave UUUU).
    # 0x0006D13C promovido a RocketVehicle_LinkBlocked_06d13c en registry (Wave VVVV).
    # 0x0006E2BC promovido a Entity_CopyAnimFromLeader_06E2BC en registry (Wave SS#1).
    # 0x00070AB0 promovido a M5Boss_AnimStep_070ab0 en registry (Wave WWWW).
    # 0x00072A94 promovido a FinalBoss_PickBeamPatternB_072a94 en registry (Wave XXXX).
    # 0x00072C98 promovido a Entity_CheckActiveBoxOverlap_072C98 en registry (Wave RR#1).
    # 0x00074166 promovido a M5Tank_TargetNear_074166 en registry (Wave XXXX).
    # 0x000745E2 promovido a MiniScript_Step_0745e2 en registry (Wave XXXX).
    # 0x000798AC promovido a Entity_CheckBoxOverlapWithSelector_0798AC en registry (Wave RR#2).
    # 0x00088438 promovido a Entity_HitboxPulseTable_088438 en registry (Wave KKK).
    # 0x0008846A promovido a Entity_HitboxPulseSaved_08846a en registry (Wave KKK).
    # 0x0008B82C promovido a Airship_DropSoldier_08b82c en registry (Wave LLL).
    # 0x0008D804 promovido a Capsule_CheckMissionEnd_08d804 en registry (Wave NNN).
    # 0x0008EA50 promovido a Nest_DieIfParentGone_08ea50 en registry (Wave OOO).
    # 0x0008EFB0 promovido a Phys_GroundKill_08efb0 en registry (Wave OOO).
    # 0x00097A60 promovido a HiScore_TryEnter_P1_097a60 en registry (Wave QQQ).
    # 0x00097A72 promovido a HiScore_TryEnter_P2_097a72 en registry (Wave QQQ).
    # 0x00097C5C promovido a HiScore_DrawNameChars_097c5c en registry (Wave QQQ).
    # 0x00099DE4 promovido a OptionsMenu_AdjDifficulty_099de4 en registry (Wave QQQ).
    # 0x00099E14 promovido a OptionsMenu_AdjLives_099e14 en registry (Wave QQQ).
    # 0x00099E9C promovido a OptionsMenu_Adj2PMode_099e9c en registry (Wave QQQ).
    # 0x00099EE4 promovido a OptionSelect_DrawCursor_099ee4 en registry (Wave QQQ).
    # 0x00099F3A promovido a FixGlyph16_DrawCursorA_099F3A en registry (Wave SS#7).
    # 0x00099FD2 promovido a FixGlyphRun_Draw2F61F0_099FD2 en registry (Wave SS#9).
    # 0x00099FF2 promovido a FixGlyph16_DrawDigit72EF_099FF2 en registry (Wave SS#10).
    # 0x0009A03C promovido a FixGlyph16_DrawDigit72F3_09A03C en registry (Wave SS#11).
    # 0x0009A086 promovido a FixGlyphRun_DrawPad2P_09A086 en registry (Wave SS#12).
    0x0009A0BA: "JsrAbsRts_09a0ba",          # rts INTERNO de JsrAbsThunk_09a0b4 (Wave I);
                                             # destino del bne.w de salida temprana de
                                             # FixGlyphRun_DrawPad2P_09A086 (Wave SS#12).
    # 0x000283EC promovido a Hitbox_RunList_0283ec en registry (Wave CCCCC).
    # ---- Wave W: destinos externos de Entity_AllocSpriteSlot_00236E ----
    0x000029A6: "Rts_shared_29A6",     # rts compartido (usado por 3 branches del validador)
    0x000029A8: "Entity_ProbeSpriteSlot_29A8", # sub-prologo compartido llamado con jsr $29a8(pc)
    # 0x0005D71C promovido a HEX_TABLE_5D71C en registry (Wave GGGGG).
    0x00009A7CC: "Sub_00009A7CC",     # movement probe llamado por Entity_ProbeMoveX_09A7AA (retorna Carry)
    0x0005A9E6: "Sprite_Blit_5A9E6",  # backend estandar del cluster Sprite_Dispatch_05CA2A (W#13)
    # 0x0000076A promovido a Entity_AllocFail_00076a en registry (Wave JJJJJ).
    0x0005D8F2: "Sub_00005D8F2",     # helper "prep VRAM/params" llamado por Debug_DrawHUDVars_096A80 (X#1) entre andi.l y jsr a W#3
    0x0005D904: "Sub_BinToDecimalDecoder_05D904",  # tail-call desde Decimal_Clamp99999999_05D8F2 (X#2): bin-to-BCD 8-nibble decoder
    # 0x0005D944 promovido a Trap15_DivByZero_05D944 en registry (Wave GGGGG).
    # 0x00002BC4 promovido a PalSlot_Release_002bc4 en registry (Wave JJJJJ).
    0x00005E4CA: "Parent_GetPrioPos_05e4ca",      # helper local (RNG?), llamado por Entity_ReserveAndSetPos_05E4B2 (W#10)
    # ---- Wave V (continuacion): destinos externos de los helpers 049FD0 / 0799DE ---
    # 0x00049FBA promovido a HumanDeath_HitCheckUnlessCutscene_049fba en registry (Wave LLLL).
    # 0x00027EBA promovido a SpritePubEffect_027EBA en registry (Wave NN#1).
    # Los callers via jsr $27EBA.l se resuelven al simbolo canonico del .text.
    # 0x00027EBA: "Sub_00027EBA",         # probe global llamado por Entity_ProbeAndInstallHandler_049FD0
    # 0x0004A034 promovido a HumanDeath_EntryKind2_04a034 en registry (Wave KKKK).
    # 0x000799A4 promovido a Rank_SubIndex_0799a4 en registry (Wave YYYY).
    # 0x00079A0E promovido a Tbl_DecodeShort_079A0E en registry (Wave YYYY).
    0x0028D876: "JmpTarget_28d876",

    # ---- Wave EE batch 1: labels/thunks internos del cluster $001260..$001AB4
    0x00001260: "Init_ModeToggle_001260",
    0x0000188A: "Attract_WaitStateBackbone_00188A",             # cabecera "wait state" comun (target de bra.w)
    0x000018DA: "Init_EntitySpawn_0018DA",
    0x00001922: "Dispatcher_ModeTable_001922",
    0x00001940: "Label_001940",             # submodo A continuation
    0x0000199A: "Label_00199A",             # submodo B continuation
    # 0x00001C88 promovido a Hud_Delay1200_ClearDirty_001c88 en registry (Wave JJJJJ).
    # 0x00001CD4 promovido a TaskList_ChangeAndRunEight_001CD4 en registry (Wave SS#6).
    # 0x00001DCC promovido a Task_InstallMissionSlots_001dcc en registry (Wave JJJJJ).
    # 0x00001DB8 promovido a Hud_DrawCreditsAndOverlay_001db8 en registry (Wave JJJJJ).
    # 0x00001E0A promovido a Set106ECC_CD_001e0a en registry (Wave JJJJJ).
    0x00024FEC: "Sub_00024FEC",             # callee jsr abs.l x3 en $1260
    0x0002A24A: "Sub_0002A24A",             # callee jsr abs.l en $18DA
    0x00000FC6: "Sub_00000FC6",             # tail target (bra.w) desde $1AA6

    # ---- Wave MM batch 1: externals del scheduler bootstrap $000E8E
    #      Todos son destinos de jsr/bsr desde SchedulerBootstrap_Boot_000E8E
    #      y AttractHandler_10002C. $52712 NO se anade aqui: fue promovido
    #      a simbolo canonico Pubcleaner_10A2Cx_052712 en Wave LL#1.
    #      $24FEC y $46AC6 ya existen (arriba/abajo).
    # 0x00001D3C promovido a Task_ResetAllSlots_001d3c en registry (Wave JJJJJ).
    # 0x00001DA4 promovido a Task_InstallBootSlots_001da4 en registry (Wave JJJJJ).
    # 0x00001E1C promovido a Scratch_AllocZero_001e1c en registry (Wave JJJJJ).
    0x0005CACE: "Sub_0005CACE",             # SchedulerBootstrap_Boot -> jsr abs.l $5CACE
    # 0x0005E998 promovido a Rng_Seed_05e998 en registry (Wave SSSS).
    # 0x00098720 promovido a LogoScene_Tpl_098720 en registry (Wave QQQ).

    # ---- Wave MM batch 1: entradas de la super-tabla dispatch $000B92
    #      referenciadas por 6x lea.l XXX(pc), a0 en SchedulerBootstrap_Boot.
    #      GAS no calcula PC-rel a partir de literales numericos ni de
    #      simbolos definidos via `.set XXX, 0xNNN` (los trata como
    #      constantes literales); necesitamos externals resueltos por el
    #      linker via --defsym para que emita el displacement correcto.
    #      Se registrara BootDispatchTable_000B92 (760 B, .long array) en
    #      Wave MM batch 2.
    # ---- Wave MM batch 2: los 6 externals BootTblEntry_XXX de MM#1 quedaron
    #      promovidos a labels internos globales de BootDispatchTable_000B92
    #      (definida en asm/boot_dispatch_table_000b92.s con 6 .globl
    #      BootTblEntry_XXX apuntando a los offsets exactos). Los
    #      `lea.l BootTblEntry_XXX(pc), a0` de scheduler_bootstrap_000exxx.s
    #      siguen resolviendose sin cambio via reubicacion R_68K_PC16 que
    #      el linker aplica contra la seccion .text.BootDispatchTable_000B92.
    # 0x00000B92: "BootTblEntry_B92",   # T1 head - ahora label interno
    # 0x00000BBA: "BootTblEntry_BBA",   # T1[10]  - ahora label interno
    # 0x00000BCE: "BootTblEntry_BCE",   # T2[1]   - ahora label interno
    # 0x00000BDA: "BootTblEntry_BDA",   # T2[4]   - ahora label interno
    # 0x00000BE6: "BootTblEntry_BE6",   # T2[7]   - ahora label interno
    # 0x00000E6E: "BootTblEntry_E6E",   # T6[12]  - ahora label interno

    # ---- Wave MM batch 3: externals de los 8 handlers de la super-tabla.
    #      5 task templates en la region de datos $9xxxx (apuntados por
    #      lea.l XXX.l, a1 seguido de jsr ThunkTarget_0004ae = Task_Alloc):
    0x00091330: "TaskTpl_091330",           # AttractHandler_00109C
    # 0x000913AC promovido a Continue_Tpl_0913ac en registry (Wave PPP).
    # 0x00099B06 promovido a OptionSelect2_Tpl_099b06 en registry (Wave QQQ).
    # 0x000977D6 promovido a HiScore_Tpl_Frame_0977d6 en registry (Wave QQQ).
    # 0x000977EA promovido a HiScore_Tpl_Loader_0977ea en registry (Wave QQQ).
    #      2 probes CCR-C en la zona $5D0xxx (invocados por bcs.w desde
    #      AttractPhase2_Probes5D0_00122E):
    0x0005D09A: "Sub_0005D09A",             # probe #1 CCR-C
    0x0005D0AC: "Sub_0005D0AC",             # probe #2 CCR-C

    # ---- Wave NN batch 1: externals del Top-1 del scan corregido
    #      (PlayerRoute_PublishState_033522, 18 callers reales).
    #      Los 2 handlers-siguientes ($033572 y $033578) son publicados en
    #      (a6) desde el publicador PlayerRoute y viven inmediatamente
    #      despues; se registraran como dispatcher en Wave NN batch 2.
    # PlayerHandlerA_033572 y PlayerHandlerB_033578 promovidos a labels
    # internos globales de PlayerStateDispatch_033572 (Wave NN#2, definidos
    # en asm/player_dispatch_0335xx.s con .globl). Los `lea.l XXX(pc), a1`
    # de player_route_publish_033xxx.s (NN#1) siguen resolviendose sin cambio.
    # 0x00033572: "PlayerHandlerA_033572",   # ahora label interno
    # 0x00033578: "PlayerHandlerB_033578",   # ahora label interno
    #      Callees de SpritePubEffect_027EBA (helpers de probe y effect):
    # 0x00027DB2 promovido a CollMap_LookupTile_027db2 en registry (Wave CCCCC).
    # 0x00027E28 promovido a CollMap_TestSolidOrPlatform_027e28 en registry (Wave CCCCC).
    # 0x0009993C promovido a Trail_FindNearest_09993c en registry (Wave QQQ).
    0x00278BA8: "Data_00278BA8",            # array de configs de effect (data)
    #      Etiqueta fin-de-funcion para Probe_Bit3At100001_0334A2: el beq.w
    #      inicial salta al primer byte JUSTO DESPUES del rts (idioma
    #      "salida por el borde"). El linker resuelve a $0334C6.

    # ---- Wave NN batch 2: externals del dispatcher + spawn constructor.
    #      LUT de 2 punteros en $3349A (P1/P2) referenciada por lea pc-rel:
    0x0003349A: "PlayerStateLUT_03349A",     # LUT de 2 ptrs (tabla P1/P2)
    #      Callees pc-rel del spawn constructor:
    # 0x00032A02 promovido a PlayerEntity_InitAuxState_032A02 en registry
    #      (Wave QQ#1).
    0x00032FF2: "Sub_00032FF2",             # post-init hook 1 (pc-rel)
    # 0x0005E98A promovido a Entity_MarkFlag2TimerMax_05e98a en registry (Wave SSSS).
    # 0x0008F6D2 promovido a PlayerSlot_MaskF0F0_08f6d2 en registry (Wave PPP).
    # 0x000517AA promovido a Player_SetIndexFromParent_0517aa en registry (Wave QQQQ).
    0x00032AA8: "Sub_00032AA8",             # post-init hook 3 (pc-rel)
    #      Callees abs.l del spawn constructor:
    # 0x000394A8 promovido a PlayerArm_Spawn_0394a8 en registry (Wave WWW).
    0x00027BC8: "Sub_00027BC8",

    # ---- Wave NN batch 3: externals de helpers del cluster player +
    #      maquina de animacion $03705A.
    0x00032AC8: "Sub_00032AC8",             # helper local (pc-rel desde $033656)
    0x00033742: "Sub_00033742",             # bra.w target desde $0336B0
    #      Data pointers para PlayerAnimState_03705A (LUT de anim frames):
    0x0279B04: "Data_00279B04",            # data ptr para state $26 route A
    0x0279B0E: "Data_00279B0E",            # data ptr para state $25 route A
    0x0279B18: "Data_00279B18",            # data ptr para state $26 route B
    0x0279B22: "Data_00279B22",            # data ptr para state $25 route B

    # ---- Wave FF batch 1: cluster attract handlers restantes
    0x00001744: "Attract_InitBIOS_001744",
    0x000017C8: "Attract_InitTaskAdd_3DBC8_0017C8",
    0x000017E6: "Attract_InitShow27_TaskAdd_0017E6",
    0x00001812: "Attract_SetTimers2_And_Gate21_001812",
    0x0000182C: "Attract_TailChain_1CD4_1DA4_00182C",
    0x00001838: "Attract_SoftReset_10FDAF_001838",
    0x00001846: "Attract_DoubleCheck_400_Publish_001846",
    0x00001AB6: "Attract_PostStart_Cleanup_001AB6",
    # 0x00052712 promovido a Pubcleaner_10A2Cx_052712 en registry (Wave LL#1).
    # El alias ThunkTarget_052712 se define ahora como .globl dentro de
    # asm/pubcleaner_10a2cx_052712.s para que jsr $52712.l en
    # attract_cluster_batch_ff.s siga resolviendose sin edicion.
    # 0x00052712: "ThunkTarget_052712",
    # 0x00046682 promovido a SceneC_Load_Task_046682 en registry (Wave DDDDD).
    # 0x00059B6A promovido a Ending_PeaceWait_059b6a en registry (Wave EEEEE).
    # 0x00002B58 promovido a PalSlot_LoadListHi_002b58 en registry (Wave JJJJJ).
    # 0x000009B4 promovido a ScriptSlotPairTable_0009B4 en registry (Wave SS#4).
    # 0x0000050E promovido a Task_AllocAndMarkBusy_00050e en registry (Wave JJJJJ).
                                             # bset #0,+0x12); callee x12 de
                                             # TaskSlots_BootInstall_000A7C (Wave SS#5).
    0x00002352: "InputGuardCall219c",
    # 0x00001C44 promovido a Banner_DelayThenStart_001c44 en registry (Wave JJJJJ).
    # 0x0003DBC8 promovido a Results_Entry_03dbc8 en registry (Wave RRRR).
    # 0x00046608 promovido a Fade_WhiteFlash_Task_046608 en registry (Wave DDDDD).
    0x00000F76: "PcThunkTarget_000F76",
    # 0x0005D288 promovido a InputEvt_ToggleChain_05d288 en registry (Wave GGGGG).

    # ---- Wave FF batch 2: helper geometrico
    0x000437DA: "Sub_000437DA",

    # ---- Wave GG batch 1: cluster attract state handlers $096xxx
    0x000967FE: "Attract_State0_Handler_0967FE",
    0x00096840: "Attract_State1_Handler_096840",
    0x00096882: "Attract_State2_Handler_096882",
    0x000968C4: "Attract_State3_Handler_0968C4",
    0x00096906: "Attract_State4_Handler_096906",
    0x00096948: "Attract_State5_Handler_096948",
    0x0009698A: "Attract_State7_Handler_09698A",
    # 0x00043568 promovido a SceneLoader_Main_043568 en registry (Wave HH#1).
    0x000967C0: "Attract_Sub_setup_967C0",
    0x000969C2: "Attract_Sub_969C2",
    0x00096B24: "Attract_Sub_96B24",

    # ---- Wave HH batch 1: externals llamados por SceneLoader_Main_043568.
    #      Todos con nombre provisional hasta identificar semantica exacta.
    #      NOTA: Camera_ResetSmoothing_0434EA se referencia por bsr.w PC-rel
    #      corto, no necesita defsym externo (esta en el mismo linker script
    #      via registry). Idem Buffer_ClearBlock1024L_043EDA (W#CC1).
    # 0x0001390E / 0x00013952 / 0x00013982 promovidos al registry como
    # Scratch_Alloc_01390E / Spawn_TypeB_013952 / Spawn_TypeA_013982
    # en Wave JJ#2 (asm/sprite_allocator_0139xx.s).
    #
    # $5A88A es el reset del subsistema de sprites invocado en la cabecera
    # de Scratch_Alloc antes de reparticionar los dos pools.
    # 0x0005A88A NO se promueve al registry: cae DENTRO de
    # VRAM_FixLayerAutoclear_05A824 (Wave DD). Se mantiene como alias externo
    # porque Scratch_Alloc_01390E (JJ#2) hace jsr abs.l al punto de entrada
    # interno $05A88A, no al inicio de la funcion contenedora.
    0x0005A88A: "Fn_0005A88A",
    # 0x00051ABE promovido a Entity_AllocAndInit_051ABE en registry (Wave HHHHH).
    # 0x0007707C promovido a Platform_ListsInit_07707c en registry (Wave YYYY).
    # 0x0008F158 promovido a Rings_InitAll_08f158 en registry (Wave OOO).
    # 0x0003EE3A promovido a Subsystem_ScoresInit_03EE3A en registry (Wave RRRR).
    # 0x000997B8 promovido a Trail_RingReset_0997b8 en registry (Wave QQQ).
    0x0004CB5C: "Subsystem_MiscInit_04CB5C",
    0x00043D6C: "Reset4CameraLongs_043D6C",

    # ---- Wave HH batch 2: externals llamados por sub-helpers attract $096xxx.
    #      $5E2D8 es el kernel de calculo player->pair invocado 2x desde
    #      SelectPositive. $5DCCE es el blit-sprite del culler. $5CD18/$5D11C
    #      son probes del debug trigger. Task_AllocFromFreeList ($4AE) esta ya
    #      registrado en registry.py como funcion matcheada; se referencia por
    #      su nombre canonico sin defsym.
    # 0x00005E2D8 promovido a SlotExtractCoords_05E2D8 en registry (Wave II#1).

    # ---- Wave II batch 2: external del bucle desenrollado de camara.
    #      $043DAA es CameraApplyOne (aplica transform a un sistema de
    #      camara); CameraApplyAll4_043D86 lo invoca 3x por bsr.w y la 4a
    #      vez por fall-through, reutilizando su rts.
    # 0x00043DAA promovido a CameraApplyOne_043DAA en registry (Wave JJ#1).

    # ---- Wave JJ batch 1: externals del cluster de aplicacion de camara.
    #      $51B80 publica la transformacion escalada, $51F30 la confirma
    #      (ya expuesto como ThunkTarget_051f30). $51C08/$51C82/$51CF6 son
    #      los tres probes con retorno CCR-C de los hooks A/B/C. $43E8C es
    #      el procesador al que los tres hooks hacen tail-jump sin retorno.
    # 0x00051B80 promovido a Integrator_XY_051B80 en registry (Wave KK#1).

    # ---- Wave KK batch 1: externals del cluster camara/sprites.
    #      $1F4A ejecuta un handler inline (call by continuation) que
    #      TransformCommit_MMIO_051F30 le pasa via a0. $51F94 es el propio
    #      handler inline, adyacente a TransformCommit; se cerrara en
    #      Wave KK batch 2 junto con los 3 probes grandes de camara.
    # 0x00001F4A promovido a Deferred_Push_001f4a en registry (Wave JJJJJ).
    # 0x00051F94 promovido a TileMap_HandlerInline_051F94 en registry (Wave KK#2).

    # ---- Wave KK batch 2: externals de los 3 probes CCR de camara.
    #      $51D84 es el probe basico (interseccion rect/rect) con retorno
    #      CCR-C set = colision. $51BA8 es apply_basico, invocado dentro
    #      del bucle de aplicacion de los tres probes.
    # 0x00051BA8 promovido a CellApply_BidirScan_051BA8 en registry (Wave LL#1).
    # El alias Fn_00051BA8 se define ahora como .globl dentro de
    # asm/collision_cell_apply_051bxx.s para que bsr.w Fn_00051BA8 en
    # collision_probes_051cxx.s (KK#2) siga resolviendose sin edicion.
    # 0x00051BA8: "Fn_00051BA8",
    # 0x00051D84 promovido a CellMap_ClipRectToWindow_051d84 en registry (Wave HHHHH).
    0x00051C08: "Fn_00051C08",
    0x00051C82: "Fn_00051C82",
    0x00051CF6: "Fn_00051CF6",
    0x00043E8C: "Fn_00043E8C",
    0x00005DCCE: "Fn_0005DCCE",
    0x00005CD18: "Fn_00005CD18",
    0x00005D11C: "Fn_00005D11C",

    # ---- Wave HH batch 3: externals para PlayerCtx_Reset.
    #      Fn_00025012 es la funcion vecina de PlayerCtx_ResetTwoBlocks a
    #      la que se hace `bcs.w` cuando player_count < 2 (fall-through
    #      al pipeline multi-jugador).
    #      NOTA: $5DA9C (backend fill-tilemap del Fix Layer, llamado 4x
    #      desde FixLayer_QuadBatch) ya esta expuesto arriba como
    #      ThunkTarget_05da9c y se reutiliza ese alias para no colisionar
    #      con los thunks Wave I que tambien lo referencian.
    0x00025012: "Fn_00025012",

    # ---- Wave GG batch 2: cluster anim state machine $08Cxxx
    0x0008C008: "Anim_State_F1_08C008",
    0x0008C15E: "Anim_State_F2_08C15E",
    0x0008C1AC: "Anim_State_F3_08C1AC",
    0x0008C1EA: "Anim_State_F4_08C1EA",
    0x0008C23A: "Anim_State_F5_08C23A",
    0x0008C296: "Anim_State_F6_08C296",
    # 0x000022C8 promovido a Sound_Push06_0022c8 en registry (Wave JJJJJ).
    0x00028CD4: "Sub_00028CD4",
    # 0x00002308 promovido a Sound_Push0A_0B_002308 en registry (Wave JJJJJ).
    # 0x0008BC74 promovido a Anim_ScriptStep_08bc74 en registry (Wave MMM).
    # 0x0008C2B8 promovido a Icon_Base_08c2b8 en registry (Wave MMM).
    # 0x0008C322 promovido a Icon_Slot1_08c322 en registry (Wave MMM).
    # 0x0008C37E promovido a Icon_Slot2_08c37e en registry (Wave MMM).
    # 0x0008C3DA promovido a Icon_Slot3_08c3da en registry (Wave MMM).
    # 0x0008C436 promovido a Icon_Slot4_08c436 en registry (Wave MMM).
    # 0x0008C5B2 promovido a Icon_Anchor_Init_08c5b2 en registry (Wave MMM).

    # ---- Wave XX: entradas mid-island (rts internos de islas C matcheadas
    #      en $0442xx..$04580x, referenciadas por bcc/bcs de los clusters
    #      de la VM de mision / boss / punteria).
    0x00044458: "LeaA1Plus4Rts_044458",   # rts de LeaA1Plus4_044454 (MissionOp07)
    0x00044AFC: "SetHandlerRts_044afc",   # rts de SetTaskHandler_044af6 (Boss_Active)
    0x00044D2E: "SetHandlerRts_044d2e",   # rts de SetTaskHandler_044d28 (Boss_Descend)
    0x00044DF0: "SetHandlerRts_044df0",   # rts de SetTaskHandler_044dea (BossShot_Fly)
    0x00044EEC: "SetHandlerRts_044eec",   # rts de SetTaskHandler_044ee6 (Miniboss_Ride)
    0x00044F88: "SetHandlerRts_044f88",   # rts de SetTaskHandler_044f82 (Miniboss_Hop)
    0x0004580A: "SetTaskBRts_04580a",     # rts de SetTaskB_045806 (Ent_AnimFrame)

    # ---- Wave XX: externals de la VM de mision / spawner / punteria.
    0x00028134: "Fn_00028134",            # setup fisica proyectil (BossShot/Miniboss)
    # 0x00028C20 promovido a Hitbox_OverlapPointA4_028c20 en registry (Wave CCCCC).
    0x00027D50: "Fn_00027D50",            # tick de vuelo del proyectil del boss
    # 0x000280C6 promovido a Entity_TileUnderToShade_0280c6 en registry (Wave CCCCC).
    # 0x0002A1AA promovido a Slug_InitBoss_02a1aa en registry (Wave BBBB).
    # 0x0002AC6A promovido a Slug_TestField100609B_02ac6a en registry (Wave BBBB).
    # 0x00030C14 promovido a EnemyShot_Straight_030c14 en registry (Wave AAAA).
    # 0x00030C70 promovido a EnemyShot_Bounce_030c70 en registry (Wave AAAA).
    # 0x000308C2 promovido a PlayerGrenade_Spawn_0308c2 en registry (Wave AAAA).
    # 0x0005DCA4 promovido a Entity_NegIfFacing_05dca4 en registry (Wave GGGGG).
    # 0x0005DD56 promovido a Entity_SetOffscreenFlag_05dd56 en registry (Wave GGGGG).
    # 0x0005E452 promovido a Parent_IsLiveHandler_05e452 en registry (Wave SSSS).
    # 0x0005E912 promovido a Hud_WriteTimerCounters_05e912 en registry (Wave SSSS).
    # 0x00077F6A promovido a Explosion_Fire_077f6a en registry (Wave YYYY).
    # 0x00079298 promovido a AutoDemo_Tmpl_Driver_079298 en registry (Wave YYYY).
    0x0008C85C: "Fn_0008C85C",            # init subsistema paralelo (MissionDriver_Init)
    # 0x0008C864 promovido a Cut_Watcher_Init_08c864 en registry (Wave MMM).
    # 0x0008F6F2 promovido a PlayerSlot_SetLowNibble_08f6f2 en registry (Wave PPP).
    # 0x0008F714 promovido a PlayerSlot_TestMaskCur_08f714 en registry (Wave PPP).
    # --- Wave YY: defsyms mid-isla + refs a huecos futuros ($04580C..$046258) ---
    0x00045DD2: "Jsr5B6Rts_045dd2",       # jsr $5B6 dentro de isla $45DC6 (cola Boss2Shot)
    0x00045F2A: "SetHandlerRts_045f2a",   # rts tras set-handler en isla $45F24
    0x0004611E: "SetXN_04611e",           # ori.b #$11,ccr; rts en isla $46118 (retorno con flags)
    0x0004613A: "SetXN_04613a",           # ori.b #$11,ccr; rts en isla $46134
    0x0004625E: "SetHandlerRts_04625e",   # rts tras set-handler en isla $46258
    # 0x00046260 promovido a Enemy46_PhaseC_046260 en registry (Wave DDDDD).
    # 0x000463C2 promovido a Drop_ProbeAndNudgeY_0463c2 en registry (Wave DDDDD).
    # --- Wave ZZ: defsyms mid-isla del banner de mision ($07A970..$07BA28) ---
    0x0007A9EE: "SetHandlerRts_07a9ee",  # rts de SetTaskHandler_07a9e8
    0x0007AA76: "SetHandlerRts_07aa76",  # rts de SetTaskHandler_07aa70
    0x0007AA92: "SetHandlerRts_07aa92",  # rts de SetTaskHandler_07aa8c
    0x0007ABF2: "SetHandlerRts_07abf2",  # rts de SetTaskHandler_07abec
    0x0007AC48: "SetHandlerRts_07ac48",  # rts de SetTaskHandler_07ac42

    # --- Wave CCC: modulo Squad Deploy ($07FBD2..$08072E) -----------------
    # Handlers en huecos futuros (aun sin matchear) referenciados por asm:
    # 0x0007FB28 promovido a Carrier_Gunner_Idle_07fb28 en registry (Wave ZZZZ).
    # 0x0007F22A promovido a Carrier_Rider_Init_07f22a en registry (Wave ZZZZ).
    # RTS internos (+6) de islas C ya matcheadas (targets de bcc.w):
    0x0007FC18: "SetHandlerRts_07fc18",  # rts de SetTaskHandler_07fc12
    0x0007FD46: "SetHandlerRts_07fd46",  # rts de SetTaskHandler_07fd40
    0x0007FDA6: "SetHandlerRts_07fda6",  # rts de SetTaskHandler_07fda0
    0x00080088: "JsrAbsRts_080088",      # rts de JsrAbsThunk_080082
    0x0008010E: "SetHandlerRts_08010e",  # rts de SetTaskHandler_080108
    0x000801EC: "SetHandlerRts_0801ec",  # rts de SetTaskHandler_0801e6
    0x0008022C: "SetHandlerRts_08022c",  # rts de SetTaskHandler_080226
    0x000802DE: "SetHandlerRts_0802de",  # rts de SetTaskHandler_0802d8
    0x000803D0: "SetHandlerRts_0803d0",  # rts de SetTaskHandler_0803ca
    0x00080506: "SetHandlerRts_080506",  # rts de SetTaskHandler_080500
    0x0008053E: "SetHandlerRts_08053e",  # rts de SetTaskHandler_080538
    0x0008067C: "SetHandlerRts_08067c",  # rts de SetTaskHandler_080676
    0x00080734: "SetHandlerRts_080734",  # rts de SetTaskHandler_08072e

    # --- Wave DDD: death handlers del Squad Deploy ($080736..$08180E) ------
    # RTS internos (+6) de islas C ya matcheadas (targets de bcc.w):
    0x0008089E: "SetHandlerRts_08089e",  # rts de SetTaskHandler_080898
    0x00080BD4: "SetHandlerRts_080bd4",  # rts de SetTaskHandler_080bce
    0x00080CDA: "SetHandlerRts_080cda",  # rts de SetTaskHandler_080cd4
    0x00080D90: "SetHandlerRts_080d90",  # rts de SetTaskHandler_080d8a
    0x00080E46: "SetHandlerRts_080e46",  # rts de SetTaskHandler_080e40
    0x00080EF8: "SetHandlerRts_080ef8",  # rts de SetTaskHandler_080ef2
    0x0008134A: "SetHandlerRts_08134a",  # rts de SetTaskHandler_081344
    0x00081638: "SetHandlerRts_081638",  # rts de SetTaskHandler_081632
    0x0008166C: "SetHandlerRts_08166c",  # rts de SetTaskHandler_081666

    # --- Wave EEE: escuadron paracaidista/APC ($081816..$082834) -----------
    # RTS internos (+6) de islas C ya matcheadas (targets de bcc.w):
    0x00081CAA: "ClrRamWordRts_081caa",  # rts de ClrRamWord_081ca4
    0x00081CD4: "SetHandlerRts_081cd4",  # rts de SetTaskHandler_081cce
    0x00081D62: "SetHandlerRts_081d62",  # rts de SetTaskHandler_081d5c
    0x00081FEE: "SetHandlerRts_081fee",  # rts de SetTaskHandler_081fe8
    0x00082050: "SetHandlerRts_082050",  # rts de SetTaskHandler_08204a
    0x00082454: "SetHandlerRts_082454",  # rts de SetTaskHandler_08244e
    0x00082512: "SetHandlerRts_082512",  # rts de SetTaskHandler_08250c
    0x00082560: "SetHandlerRts_082560",  # rts de SetTaskHandler_08255a
    0x00082624: "SetHandlerRts_082624",  # rts de SetTaskHandler_08261e
    0x0008267A: "SetHandlerRts_08267a",  # rts de SetTaskHandler_082674
    0x0008283A: "SetHandlerRts_08283a",  # rts de SetTaskHandler_082834
    # (Los 23 refs forward $8283C..$831DA se promovieron a simbolos reales
    #  en asm/para_squad_helpers_082cxx.s durante Wave FFF.)

    # --- Wave FFF: helpers y handlers de escape ($08283C..$08325A) ----------
    # RTS internos (+6) de islas C ya matcheadas (targets de bcc.w):
    0x0008292A: "SetHandlerRts_08292a",  # rts de SetTaskHandler_082924
    0x000829AA: "SetHandlerRts_0829aa",  # rts de SetTaskHandler_0829a4
    0x00082A64: "SetHandlerRts_082a64",  # rts de SetTaskHandler_082a5e
    0x00082B44: "SetHandlerRts_082b44",  # rts de SetTaskHandler_082b3e
    0x00082BB6: "SetHandlerRts_082bb6",  # rts de SetTaskHandler_082bb0
    0x00082C06: "SetHandlerRts_082c06",  # rts de SetTaskHandler_082c00

    # --- Wave GGG: modulo de miniboss con secuencia de estados ($083262..$083BDA)
    # RTS internos de islas C ya matcheadas (targets de bcc.w):
    0x00083B90: "Jsr5B6Rts_083b90",      # rts de Jsr5B6ThenJmpScheduler_083b84 (+12)
    0x00083BE0: "JsrAbsRts_083be0",      # rts de JsrAbsThunk_083bda (+6)
    # Refs forward a huecos futuros (aun sin matchear):
    # 0x00085FB0 promovido a Boss_RandomDropSpawn_085fb0 en registry (Wave JJJ).
    # 0x00086050 promovido a Boss_SineBob_086050 en registry (Wave JJJ).
    # 0x00086076 promovido a Boss_SpawnGuardList_086076 en registry (Wave JJJ).
    # 0x000863BE promovido a Entity_TickIfState2_0863be en registry (Wave JJJ).

    # --- Wave HHH: fases finales del miniboss y transiciones de oleada ($083BE2..$084828)
    # RTS internos de islas C ya matcheadas (targets de bcc/blt/bcs.w):
    0x0008440E: "SetHandlerRts_08440e",  # rts de SetTaskHandler_084408 (+6)
    0x000844BE: "SetHandlerRts_0844be",  # rts de SetTaskHandler_0844b8 (+6)
    0x0008450A: "SetHandlerRts_08450a",  # rts de SetTaskHandler_084504 (+6)
    0x00084834: "Jsr5B6Rts_084834",      # rts de Jsr5B6ThenJmpScheduler_084828 (+12)
    # Refs forward a huecos futuros (aun sin matchear):
    # 0x000860E4 promovido a Boss_Spawn45Children_0860e4 en registry (Wave JJJ).
    # 0x0008610C promovido a Boss_SpawnStepList_08610c en registry (Wave JJJ).
    # 0x00086196 promovido a Boss_SpawnTenEscorts_086196 en registry (Wave JJJ).
    # 0x00086300 promovido a Boss_Spawn4Finale_086300 en registry (Wave JJJ).
    # 0x00086328 promovido a Boss_SpawnRow8_086328 en registry (Wave JJJ).
    # 0x00086364 promovido a Boss_SpawnRow9_086364 en registry (Wave JJJ).
    0x000863E4: "Sub_000863E4",          # helper (jsr pc desde $83E08)
    0x000863F2: "Sub_000863F2",          # helper (jsr pc desde $83E1C)
    0x00086400: "Sub_00086400",          # helper (jsr pc desde $84254)
    0x0008640E: "Sub_0008640E",          # helper (jsr pc desde $84348)
    0x0008641C: "Sub_0008641C",          # helper (jsr pc desde $8434C)
    # 0x000864B6 promovido a Boss_BlitTable_A_0864b6 en registry (Wave JJJ).
    # 0x000864D0 promovido a Boss_BlitTable_B_0864d0 en registry (Wave JJJ).
    # 0x000864EA promovido a Boss_BlitTable_C_0864ea en registry (Wave JJJ).
    # 0x0008651E promovido a Boss_BlitTable_E_08651e en registry (Wave JJJ).
    # 0x00086538 promovido a Boss_BlitTable_F_086538 en registry (Wave JJJ).

    # --- Wave III: escuadron de rescate y ciclo de vuelo ($084836..$08512C)
    # RTS internos de islas C ya matcheadas (targets de bcc/bne/bgt.w):
    0x00084898: "SetHandlerRts_084898",  # rts de SetTaskHandler_084892 (+6)
    0x000848DC: "SetHandlerRts_0848dc",  # rts de SetTaskHandler_0848d6 (+6)
    0x00084B22: "SetHandlerRts_084b22",  # rts de SetTaskHandler_084b1c (+6)
    0x00084B98: "SetHandlerRts_084b98",  # rts de SetTaskHandler_084b92 (+6)
    0x00084BD0: "SetHandlerRts_084bd0",  # rts de SetTaskHandler_084bca (+6)
    0x00084C24: "SetHandlerRts_084c24",  # rts de SetTaskHandler_084c1e (+6)
    0x00084C5C: "Jsr5B6Rts_084c5c",      # rts de Jsr5B6ThenJmpScheduler_084c50 (+12)
    # Refs forward a huecos futuros (aun sin matchear):
    # 0x00085EE8 promovido a Flight_WobbleArm_085ee8 en registry (Wave JJJ).
    # 0x00085F08 promovido a Flight_WobbleStep_085f08 en registry (Wave JJJ).
    # 0x00085F44 promovido a Flight_HitboxParams_085f44 en registry (Wave JJJ).
    # 0x00085F60 promovido a Flight_AltitudeCheck_085f60 en registry (Wave JJJ).
    # 0x0008601C promovido a Boss_PhaseJingle_08601c en registry (Wave JJJ).
    # 0x000863A0 promovido a Entity_BobY1_0863a0 en registry (Wave JJJ).
    # 0x00086504 promovido a Boss_BlitTable_D_086504 en registry (Wave JJJ).

    # --- Wave JJJ: Mision 4 - transporte/torreta/agua + helpers boss ($08512C..$0865BE)
    # RTS internos de islas C ya matcheadas (targets de bcc/bra.w colgantes):
    0x00085482: "SetHandlerRts_085482",  # rts de SetTaskHandler_08547c (+6)
    0x00085606: "SetTaskWRts_085606",    # rts de SetTaskW_085602 (+4)
    0x000856A8: "SetTaskWRts_0856a8",    # rts de SetTaskW_0856a4 (+4)
    0x00085ACC: "SetHandlerRts_085acc",  # rts de SetTaskHandler_085ac6 (+6)
    0x00085D02: "JsrAbsRts_085d02",      # rts de JsrAbsThunk_085cfc (+6)
    0x0008604E: "JsrAbsRts_08604e",      # rts de JsrAbsThunk_086048 (+6)
    0x000863D4: "JsrAbsRts_0863d4",      # rts de JsrAbsThunk_0863ce (+6)

    # --- Wave KKK: fortaleza escena 4 ($0865BE..$088A56)
    # RTS interno de isla C ya matcheada (target de bne.w en Heli_RotorAnim_0883ec):
    0x00088436: "SetTaskWRts_088436",    # rts de SetTaskW_088432 (+4)
    # Hueco futuro referenciado por bne.w desde Fort_BlitWreck_088a28:
    # 0x00088A64 promovido a Fort_BlitWreck_V1_088a64 en registry (Wave LLL).

    # --- Wave LLL: region $088A56..$08BA00
    # RTS internos de islas C ya matcheadas (targets de bcc/bra.w colgantes):
    0x00089502: "SetHandlerRts_089502",  # rts de SetTaskHandler_0894fc (+6)
    0x000895CA: "SetHandlerRts_0895ca",  # rts de SetTaskHandler_0895c4 (+6)
    0x00089686: "Jsr5B6Rts_089686",  # rts de Jsr5B6ThenJmpScheduler_08967a (+12)
    0x000896DC: "JsrAbsRts_0896dc",  # rts de JsrAbsThunk_0896d6 (+6)
    0x0008989A: "JsrAbsRts_08989a",  # rts de JsrAbsThunk_089894 (+6)
    0x0008995E: "SetHandlerRts_08995e",  # rts de SetTaskHandler_089958 (+6)
    0x00089A02: "SetHandlerRts_089a02",  # rts de SetTaskHandler_0899fc (+6)
    0x00089A2E: "SetHandlerRts_089a2e",  # rts de SetTaskHandler_089a28 (+6)
    0x0008A31A: "SetHandlerRts_08a31a",  # rts de SetTaskHandler_08a314 (+6)
    0x0008A44A: "SetHandlerRts_08a44a",  # rts de SetTaskHandler_08a444 (+6)
    0x0008A9AE: "SetHandlerRts_08a9ae",  # rts de SetTaskHandler_08a9a8 (+6)
    0x0008AAF0: "SetHandlerRts_08aaf0",  # rts de SetTaskHandler_08aaea (+6)
    0x0008AF66: "SetHandlerRts_08af66",  # rts de SetTaskHandler_08af60 (+6)
    0x0008B03A: "JmpSchedRts_08b03a",  # rts de JmpToScheduler_08b034 (+6)
    0x0008B108: "SetHandlerRts_08b108",  # rts de SetTaskHandler_08b102 (+6)
    0x0008B716: "JsrPcRts_08b716",  # rts de JsrPcThunk_08b712 (+4)
    # Huecos futuros referenciados por pc-rel desde esta region:
    # 0x0008BA0C promovido a Proj_Bounce_V1_08ba0c en registry (Wave MMM).
    # 0x0008BA52 promovido a Proj_Bounce_V2_08ba52 en registry (Wave MMM).
    # 0x0008BB34 promovido a Proj_Tmpl_InitHitboxProbe_08bb34 en registry (Wave MMM).
    # 0x0008BB5E promovido a Proj_Tmpl_InitHitbox_08bb5e en registry (Wave MMM).
    # --- Wave MMM: RTS internos de islas C
    0x0008BB28: "SetHandlerRts_08bb28",  # rts de SetTaskHandler_08bb22 (+6)
    0x0008BCE4: "SetXNMid_08bce4",  # rts de SetXN_08bce0 (+4)
    0x0008BE26: "SetXNMid_08be26",  # rts de SetXN_08be22 (+4)
    0x0008BF94: "SetXNMid_08bf94",  # rts de SetXN_08bf90 (+4)
    0x0008C006: "SetXNMid_08c006",  # rts de SetXN_08c002 (+4)
    0x0008C72E: "Jsr5B6Rts_08c72e",  # rts de Jsr5B6ThenJmpScheduler_08c722 (+12)
    0x0008C938: "SetHandlerRts_08c938",  # rts de SetTaskHandler_08c932 (+6)
    0x0008CF20: "SetHandlerRts_08cf20",  # rts de SetTaskHandler_08cf1a (+6)
    0x0008CF6A: "SetHandlerRts_08cf6a",  # rts de SetTaskHandler_08cf64 (+6)
    0x0008CFB4: "SetHandlerRts_08cfb4",  # rts de SetTaskHandler_08cfae (+6)
    0x0008CFFE: "SetHandlerRts_08cffe",  # rts de SetTaskHandler_08cff8 (+6)
    0x0008D048: "SetHandlerRts_08d048",  # rts de SetTaskHandler_08d042 (+6)
    0x0008D092: "SetHandlerRts_08d092",  # rts de SetTaskHandler_08d08c (+6)
    # --- Wave MMM: refs forward a huecos futuros
    # 0x0008D24C promovido a Screen_InBoundsY_Latched_08d24c en registry (Wave NNN).
    # 0x0008D2D4 promovido a Pos_IntegrateY88_08d2d4 en registry (Wave NNN).
    # 0x0008D3B4 promovido a Capsule_Fly_08d3b4 en registry (Wave NNN).
    # --- Wave NNN: RTS internos de islas C
    0x0008D34C: "SetTaskWRts_08d34c",  # rts de SetTaskW_08d348 (+4)
    0x0008D41A: "SetHandlerRts_08d41a",  # rts de SetTaskHandler_08d414 (+6)
    0x0008D44E: "SetHandlerRts_08d44e",  # rts de SetTaskHandler_08d448 (+6)
    0x0008D4FC: "SetHandlerRts_08d4fc",  # rts de SetTaskHandler_08d4f6 (+6)
    0x0008D536: "SetHandlerRts_08d536",  # rts de SetTaskHandler_08d530 (+6)
    0x0008D55A: "SetHandlerRts_08d55a",  # rts de SetTaskHandler_08d554 (+6)
    0x0008D62C: "SetHandlerRts_08d62c",  # rts de SetTaskHandler_08d626 (+6)
    0x0008D66E: "SetHandlerRts_08d66e",  # rts de SetTaskHandler_08d668 (+6)
    0x0008D728: "SetHandlerRts_08d728",  # rts de SetTaskHandler_08d722 (+6)
    0x0008D7BC: "SetHandlerRts_08d7bc",  # rts de SetTaskHandler_08d7b6 (+6)
    0x0008D840: "SetHandlerRts_08d840",  # rts de SetTaskHandler_08d83a (+6)
    0x0008D8F2: "SetHandlerRts_08d8f2",  # rts de SetTaskHandler_08d8ec (+6)
    0x0008E19A: "JsrAbsRts_08e19a",  # rts de JsrAbsThunk_08e194 (+6)
    0x0008E286: "SetHandlerRts_08e286",  # rts de SetTaskHandler_08e280 (+6)
    0x0008E2EE: "SetHandlerRts_08e2ee",  # rts de SetTaskHandler_08e2e8 (+6)
    # --- Wave NNN: refs forward a huecos futuros
    # 0x0008EDC6 promovido a CamProp_Tmpl189_08edc6 en registry (Wave OOO).
    # 0x0008EFCE promovido a Phys_PlayerNearX_08efce en registry (Wave OOO).
    # 0x0008F002 promovido a Phys_FacingFromParam_08f002 en registry (Wave OOO).
    # 0x0008F010 promovido a Phys_VelXFromParam_08f010 en registry (Wave OOO).
    # 0x0008F02C promovido a Phys_ScrollTarget_08f02c en registry (Wave OOO).
    # 0x0008F040 promovido a Phys_ScrollReached_08f040 en registry (Wave OOO).
    # 0x0008F070 promovido a Snd_ByParam9A_Base_08f070 en registry (Wave OOO).
    # 0x0008F084 promovido a Snd_ByParam9A_A_08f084 en registry (Wave OOO).
    # 0x0008F0D0 promovido a Snd_ByParam9A_B_08f0d0 en registry (Wave OOO).
    # 0x0008F108 promovido a Prio_Set8018_08f108 en registry (Wave OOO).
    # --- Wave OOO: RTS internos de islas C
    0x0008EA72: "SetHandlerRts_08ea72",  # rts de SetTaskHandler_08ea6c (+6)
    0x0008EFCC: "Jsr5B6Rts_08efcc",  # rts de Jsr5B6ThenJmpScheduler_08efc0 (+12)
    0x0008F0CE: "JsrAbsRts_08f0ce",  # rts de JsrAbsThunk_08f0c8 (+6)
    0x0008F106: "JsrAbsRts_08f106",  # rts de JsrAbsThunk_08f100 (+6)
    # --- Wave PPP: RTS internos de islas C
    0x0008F794: "SetCMid_08f794",  # rts de SetC_08f790 (+4)
    0x0008F882: "SetCMid_08f882",  # rts de SetC_08f87e (+4)
    0x0008FCA8: "SetHandlerRts_08fca8",  # rts de SetTaskHandler_08fca2 (+6)
    0x0008FD2C: "SetHandlerRts_08fd2c",  # rts de SetTaskHandler_08fd26 (+6)
    0x0008FD66: "SetHandlerRts_08fd66",  # rts de SetTaskHandler_08fd60 (+6)
    0x0008FDA8: "SetHandlerRts_08fda8",  # rts de SetTaskHandler_08fda2 (+6)
    0x0008FDF6: "SetHandlerRts_08fdf6",  # rts de SetTaskHandler_08fdf0 (+6)
    0x0008FE30: "SetHandlerRts_08fe30",  # rts de SetTaskHandler_08fe2a (+6)
    0x0008FE9A: "SetHandlerRts_08fe9a",  # rts de SetTaskHandler_08fe94 (+6)
    0x0008FEB4: "SetHandlerRts_08feb4",  # rts de SetTaskHandler_08feae (+6)
    0x000900E2: "JsrAbsRts_0900e2",  # rts de JsrAbsThunk_0900dc (+6)
    0x00090C10: "JsrAbsRts_090c10",  # rts de JsrAbsThunk_090c0a (+6)
    0x00090E7C: "SetHandlerRts_090e7c",  # rts de SetTaskHandler_090e76 (+6)
    0x00090EFE: "JsrAbsRts_090efe",  # rts de JsrAbsThunk_090ef8 (+6)
    0x00091094: "SetHandlerRts_091094",  # rts de SetTaskHandler_09108e (+6)
    0x00091556: "SetHandlerRts_091556",  # rts de SetTaskHandler_091550 (+6)
    0x00091576: "SetHandlerRts_091576",  # rts de SetTaskHandler_091570 (+6)
    # --- Wave QQQ: RTS internos de islas C
    0x000978AA: "SetHandlerRts_0978aa",  # rts de SetTaskHandler_0978a4 (+6)
    0x00097CC2: "JsrAbsRts_097cc2",  # rts de JsrAbsThunk_097cbc (+6)
    0x00098988: "JsrAbsRts_098988",  # rts de JsrAbsThunk_098982 (+6)
    0x00098A86: "SetHandlerRts_098a86",  # rts de SetTaskHandler_098a80 (+6)
    0x00098AD2: "SetHandlerRts_098ad2",  # rts de SetTaskHandler_098acc (+6)
    0x00098B64: "SetHandlerRts_098b64",  # rts de SetTaskHandler_098b5e (+6)
    0x00098BB0: "SetHandlerRts_098bb0",  # rts de SetTaskHandler_098baa (+6)
    0x00098D20: "SetHandlerRts_098d20",  # rts de SetTaskHandler_098d1a (+6)
    0x00098D8A: "SetHandlerRts_098d8a",  # rts de SetTaskHandler_098d84 (+6)
    0x00098E4C: "SetHandlerRts_098e4c",  # rts de SetTaskHandler_098e46 (+6)
    0x00098E98: "SetHandlerRts_098e98",  # rts de SetTaskHandler_098e92 (+6)
    0x00098F3E: "SetHandlerRts_098f3e",  # rts de SetTaskHandler_098f38 (+6)
    0x0009906E: "SetHandlerRts_09906e",  # rts de SetTaskHandler_099068 (+6)
    0x000990BA: "SetHandlerRts_0990ba",  # rts de SetTaskHandler_0990b4 (+6)
    0x000991D8: "SetHandlerRts_0991d8",  # rts de SetTaskHandler_0991d2 (+6)
    0x00099286: "SetHandlerRts_099286",  # rts de SetTaskHandler_099280 (+6)
    0x000992D6: "SetHandlerRts_0992d6",  # rts de SetTaskHandler_0992d0 (+6)
    0x000993F8: "SetHandlerRts_0993f8",  # rts de SetTaskHandler_0993f2 (+6)
    0x00099444: "SetHandlerRts_099444",  # rts de SetTaskHandler_09943e (+6)
    0x0009958E: "SetHandlerRts_09958e",  # rts de SetTaskHandler_099588 (+6)
    0x000995CA: "SetHandlerRts_0995ca",  # rts de SetTaskHandler_0995c4 (+6)
    0x0009966E: "SetHandlerRts_09966e",  # rts de SetTaskHandler_099668 (+6)
    0x000996B4: "SetHandlerRts_0996b4",  # rts de SetTaskHandler_0996ae (+6)
    0x00099792: "SetHandlerRts_099792",  # rts de SetTaskHandler_09978c (+6)
    0x00099E12: "JsrPcRts_099e12",  # rts de JsrPcThunk_099e0e (+4)
    0x00099E56: "JsrPcRts_099e56",  # rts de JsrPcThunk_099e52 (+4)
    0x00099E9A: "JsrPcRts_099e9a",  # rts de JsrPcThunk_099e96 (+4)
    0x00099ED0: "JsrPcRts_099ed0",  # rts de JsrPcThunk_099ecc (+4)
    # --- Wave RRR: RTS internos de islas C
    0x0009A0FA: "Jsr5B6Rts_09a0fa",  # rts de Jsr5B6ThenJmpScheduler_09a0ee (+12)
    0x0009A2B6: "SetHandlerRts_09a2b6",  # rts de SetTaskHandler_09a2b0 (+6)
    0x0009A2FE: "SetHandlerRts_09a2fe",  # rts de SetTaskHandler_09a2f8 (+6)
    0x0009A8BC: "JsrAbsRts_09a8bc",  # rts de JsrAbsThunk_09a8b6 (+6)
    0x0009B668: "SetHandlerRts_09b668",  # rts de SetTaskHandler_09b662 (+6)
    0x0009B758: "SetHandlerRts_09b758",  # rts de SetTaskHandler_09b752 (+6)
    0x0009B7BA: "SetHandlerRts_09b7ba",  # rts de SetTaskHandler_09b7b4 (+6)
    0x0009B7F6: "SetHandlerRts_09b7f6",  # rts de SetTaskHandler_09b7f0 (+6)
    0x0009B82A: "SetHandlerRts_09b82a",  # rts de SetTaskHandler_09b824 (+6)
    0x0009C25C: "Jsr5B6Rts_09c25c",  # rts de Jsr5B6ThenJmpScheduler_09c250 (+12)
    # --- Wave TTT: RTS internos de islas C
    0x00032C7C: "JsrAbsRts_032c7c",  # rts de JsrAbsThunk_032c76 (+6)
    0x00032E1E: "ClearXNMid_032e1e",  # rts de ClearXN_032e1a (+4)
    0x00033374: "JsrAbsRts_033374",  # rts de JsrAbsThunk_03336e (+6)
    0x000342C2: "SetHandlerRts_0342c2",  # rts de SetTaskHandler_0342bc (+6)
    # --- Wave TTT: refs forward a huecos futuros
    # 0x000324BC promovido a Player_GroundTblA_0324bc en registry (Wave AAAA).
    # 0x000324C6 promovido a Player_GroundTblB_0324c6 en registry (Wave AAAA).
    # 0x000325E4 promovido a Player_AttackTblA_0325e4 en registry (Wave AAAA).
    # 0x000326E0 promovido a Player_HitboxStand_0326e0 en registry (Wave AAAA).
    # 0x000329D4 promovido a Player_WeaponAmmoTbl_0329d4 en registry (Wave AAAA).
    # 0x000329E8 promovido a Player_WeaponFlagTbl_0329e8 en registry (Wave AAAA).
    # 0x000342C4 promovido a Player_ShootStand_0342c4 en registry (Wave UUU).
    # 0x0003437E promovido a Player_ShootStandUp_03437e en registry (Wave UUU).
    # 0x000345B8 promovido a Player_ReenterByInput_0345b8 en registry (Wave UUU).
    # 0x00034704 promovido a Player_Stand_034704 en registry (Wave UUU).
    # 0x00034B38 promovido a Player_WalkRight_034b38 en registry (Wave UUU).
    # 0x00034D32 promovido a Player_WalkLeft_034d32 en registry (Wave UUU).
    # 0x00035ABA promovido a Player_TurnRight_035aba en registry (Wave UUU).
    # 0x00035BF8 promovido a Player_TurnLeft_035bf8 en registry (Wave UUU).
    # 0x00035D34 promovido a Player_Melee_035d34 en registry (Wave UUU).
    # 0x000360BC promovido a Player_ThrowGrenade_Stand_0360bc en registry (Wave UUU).
    # 0x00036914 promovido a Player_JumpStart_036914 en registry (Wave VVV).
    # 0x00036C8C promovido a Player_SpawnFreeFall_036c8c en registry (Wave VVV).
    # 0x00037018 promovido a Player_KnockbackHold_037018 en registry (Wave VVV).
    # 0x00037C74 promovido a Player_CrouchEnter_037c74 en registry (Wave VVV).
    # 0x0003827A promovido a Player_CrouchIdle_03827a en registry (Wave VVV).
    # 0x0003873C promovido a Player_CrouchShoot_03873c en registry (Wave VVV).
    # 0x00038BE4 promovido a DroppedWeapon_Spawn_038be4 en registry (Wave WWW).
    # 0x00038CF6 promovido a Parachute_Spawn_038cf6 en registry (Wave WWW).
    # --- Wave UUU: refs forward a huecos futuros
    # 0x00032638 promovido a Player_AttackTblB_032638 en registry (Wave AAAA).
    # 0x00032788 promovido a Player_HitboxMelee_032788 en registry (Wave AAAA).
    # 0x000328D8 promovido a Player_HitboxDeath_0328d8 en registry (Wave AAAA).
    # 0x000366FE promovido a Player_RideSlug_Pose2_0366fe en registry (Wave VVV).
    # 0x00036796 promovido a Player_SlugJumpOff_036796 en registry (Wave VVV).
    # --- Wave VVV: refs forward a huecos futuros
    # 0x000324D0 promovido a Player_VelYTbl_0324d0 en registry (Wave AAAA).
    # 0x000324D8 promovido a Player_VelYTblB_0324d8 en registry (Wave AAAA).
    # 0x000324E8 promovido a Player_VelXTbl_0324e8 en registry (Wave AAAA).
    # 0x00032734 promovido a Player_HitboxCrouch_032734 en registry (Wave AAAA).
    # 0x00032830 promovido a Player_HitboxAir_032830 en registry (Wave AAAA).
    # 0x00032884 promovido a Player_HitboxKnockback_032884 en registry (Wave AAAA).
    # 0x0003292C promovido a Player_HitboxSlug_03292c en registry (Wave AAAA).
    # 0x000388F0 promovido a Player_CrouchThrowGrenade_0388f0 en registry (Wave WWW).
    # 0x00038A28 promovido a Player_CrouchMelee_038a28 en registry (Wave WWW).
    # 0x00038AE6 promovido a Player_CrouchReload_038ae6 en registry (Wave WWW).
    # 0x00039148 promovido a PlayerDeathFx_Splash_039148 en registry (Wave WWW).
    # 0x000391EE promovido a PlayerDeathFx_Ripple_0391ee en registry (Wave WWW).
    # 0x00039214 promovido a PlayerDeathFx_Alt_039214 en registry (Wave WWW).
    # --- Wave WWW: refs forward a huecos futuros
    # 0x000327DC promovido a Player_HitboxGrenade_0327dc en registry (Wave AAAA).
    # --- Wave YYY: RTS internos de islas C
    0x0003DA9E: "JsrAbsRts_03da9e",  # rts de JsrAbsThunk_03da98 (+6)
    # --- Wave YYY: refs forward a huecos futuros
    # 0x0003DAA8 promovido a SlugCannon_ArmOffsetCurve_03daa8 en registry (Wave RRRR).
    # --- Wave ZZZ: RTS internos de islas C
    0x000301EE: "JsrAbsRts_0301ee",  # rts de JsrAbsThunk_0301e8 (+6)
    0x00030390: "JsrAbsRts_030390",  # rts de JsrAbsThunk_03038a (+6)
    0x00030608: "JsrAbsRts_030608",  # rts de JsrAbsThunk_030602 (+6)
    # --- Wave ZZZ: refs forward a huecos futuros
    # 0x000295A6 promovido a Slug_AngleToSpriteIdx_0295a6 en registry (Wave BBBB).
    # 0x00029790 promovido a Slug_HitboxA_029790 en registry (Wave BBBB).
    # 0x00029834 promovido a Slug_HitboxB_029834 en registry (Wave BBBB).
    # 0x000298D8 promovido a Slug_HitboxC_0298d8 en registry (Wave BBBB).
    # 0x0002999E promovido a Slug_HitboxCb_02999e en registry (Wave BBBB).
    # 0x00029A14 promovido a Slug_HitboxCbC_029a14 en registry (Wave BBBB).
    # 0x00029A68 promovido a Slug_AttackTbl00_029a68 en registry (Wave BBBB).
    # 0x0002A024 promovido a Slug_AttackPtrTbl_02a024 en registry (Wave BBBB).
    # 0x0002A060 promovido a Slug_StateByAnglePtrTbl_02a060 en registry (Wave BBBB).
    # 0x0002A328 promovido a Slug_CallGroundProbeA_02a328 en registry (Wave BBBB).
    # 0x0002A34E promovido a Slug_CallGroundProbeB_02a34e en registry (Wave BBBB).
    # 0x0002A478 promovido a Slug_UpdateAirFlag_02a478 en registry (Wave BBBB).
    # 0x0002A4EC promovido a Slug_TerrainIsSlope_02a4ec en registry (Wave BBBB).
    # 0x0002A4F0 promovido a Slug_UpdateAngleIsSlope_02a4f0 en registry (Wave BBBB).
    # 0x0002A59A promovido a Slug_CheckFreeThenC_02a59a en registry (Wave BBBB).
    # 0x0002A664 promovido a Slug_TryStartDestroyed_02a664 en registry (Wave BBBB).
    # 0x0002A690 promovido a Slug_TryStartDestroyedB_02a690 en registry (Wave BBBB).
    # 0x0002A752 promovido a Slug_PhysicsA_02a752 en registry (Wave BBBB).
    # 0x0002A760 promovido a Slug_PhysicsB_02a760 en registry (Wave BBBB).
    # 0x0002A766 promovido a Slug_PhysicsC_02a766 en registry (Wave BBBB).
    # 0x0002A7D8 promovido a Slug_PhysicsE_02a7d8 en registry (Wave BBBB).
    # 0x0002A824 promovido a Slug_PhysicsF_02a824 en registry (Wave BBBB).
    # 0x0002A878 promovido a Slug_PhysicsAir_02a878 en registry (Wave BBBB).
    # 0x0002A8C0 promovido a Slug_GroundContact_02a8c0 en registry (Wave BBBB).
    # 0x0002A958 promovido a Slug_TerrainSlope_02a958 en registry (Wave BBBB).
    # 0x0002A9A0 promovido a Slug_SlopeToAnimIdx_02a9a0 en registry (Wave BBBB).
    # 0x0002AA0E promovido a Slug_UpdateAnimKeepIdx_02aa0e en registry (Wave BBBB).
    # 0x0002AA24 promovido a Slug_UpdateAnimAndChassis_02aa24 en registry (Wave BBBB).
    # 0x0002AAC0 promovido a Slug_CanFire_02aac0 en registry (Wave BBBB).
    # 0x0002AAF0 promovido a Slug_InputDirByLayoutA_02aaf0 en registry (Wave BBBB).
    # 0x0002AB3C promovido a Slug_InputDirByLayoutB_02ab3c en registry (Wave BBBB).
    # 0x0002ACB8 promovido a Slug_CheckPlayersNear_02acb8 en registry (Wave BBBB).
    # 0x0002B38C promovido a Slug_IdleFlat_02b38c en registry (Wave CCCC).
    # 0x0002B4D2 promovido a Slug_IdleSlope_02b4d2 en registry (Wave CCCC).
    # 0x0002B7DA promovido a Slug_SlopeMount_02b7da en registry (Wave CCCC).
    # 0x0002B8CE promovido a Slug_AccelRightMusicTbl_02b8ce en registry (Wave CCCC).
    # 0x0002BA34 promovido a Slug_AccelLeftMusicTbl_02ba34 en registry (Wave CCCC).
    # 0x0002BB9A promovido a Slug_DestroyedMusicTbl_02bb9a en registry (Wave CCCC).
    # 0x0002BBA4 promovido a Slug_DestroyedSlide_02bba4 en registry (Wave CCCC).
    # 0x0002BBF2 promovido a Slug_DestroyedSlideInit_02bbf2 en registry (Wave CCCC).
    # 0x0002BF64 promovido a Slug_CruiseRightB_02bf64 en registry (Wave CCCC).
    # 0x0002C07A promovido a Slug_CruiseLeftB_02c07a en registry (Wave CCCC).
    # 0x0002C24A promovido a Slug_FireFlat_02c24a en registry (Wave CCCC).
    # 0x0002C95C promovido a Slug_JumpCrouch_Loop_02c95c en registry (Wave CCCC).
    # 0x0002CFFA promovido a Slug_FallStart_02cffa en registry (Wave CCCC).
    # 0x0002DC5C promovido a Slug_DeathFade_02dc5c en registry (Wave CCCC).
    # 0x0002DCBC promovido a Slug_DeathFinishJmp_02dcbc en registry (Wave CCCC).
    # --- Wave AAAA: RTS internos de islas C
    0x00030BB4: "SetHandlerRts_030bb4",  # rts de SetTaskHandler_030bae (+6)
    0x00030D02: "SetHandlerRts_030d02",  # rts de SetTaskHandler_030cfc (+6)
    0x00030D5A: "SetHandlerRts_030d5a",  # rts de SetTaskHandler_030d54 (+6)
    0x00031688: "SetHandlerRts_031688",  # rts de SetTaskHandler_031682 (+6)
    0x000317D0: "SetHandlerRts_0317d0",  # rts de SetTaskHandler_0317ca (+6)
    0x000319CE: "SetHandlerRts_0319ce",  # rts de SetTaskHandler_0319c8 (+6)
    0x00031AB8: "Jsr5B6Rts_031ab8",  # rts de Jsr5B6ThenJmpScheduler_031aac (+12)
    0x00031BCE: "Jsr5B6Rts_031bce",  # rts de Jsr5B6ThenJmpScheduler_031bc2 (+12)
    0x00031C1E: "Jsr5B6Rts_031c1e",  # rts de Jsr5B6ThenJmpScheduler_031c12 (+12)
    0x00031C68: "Jsr5B6Rts_031c68",  # rts de Jsr5B6ThenJmpScheduler_031c5c (+12)
    0x00031DC0: "JsrAbsRts_031dc0",  # rts de JsrAbsThunk_031dba (+6)
    0x0003206E: "SetHandlerRts_03206e",  # rts de SetTaskHandler_032068 (+6)
    0x000320D2: "SetHandlerRts_0320d2",  # rts de SetTaskHandler_0320cc (+6)
    0x000321BA: "SetHandlerRts_0321ba",  # rts de SetTaskHandler_0321b4 (+6)
    # --- Wave BBBB: RTS internos de islas C
    0x0002A274: "ClearXNMid_02a274",  # rts de ClearXN_02a270 (+4)
    0x0002A28C: "SetXNMid_02a28c",  # rts de SetXN_02a288 (+4)
    0x0002A2F6: "SetXNMid_02a2f6",  # rts de SetXN_02a2f2 (+4)
    0x0002A662: "JsrAbsRts_02a662",  # rts de JsrAbsThunk_02a65c (+6)
    0x0002ABB8: "JsrAbsRts_02abb8",  # rts de JsrAbsThunk_02abb2 (+6)
    0x0002ABEE: "SetXNMid_02abee",  # rts de SetXN_02abea (+4)
    0x0002AC0C: "SetXNMid_02ac0c",  # rts de SetXN_02ac08 (+4)
    0x0002ACA0: "ClearXNMid_02aca0",  # rts de ClearXN_02ac9c (+4)
    # --- Wave BBBB: refs forward a huecos futuros
    # 0x0002DCC0 promovido a Slug_DeathStart_02dcc0 en registry (Wave CCCC).
    # --- Wave CCCC: RTS internos de islas C
    0x0002B262: "SetHandlerRts_02b262",  # rts de SetTaskHandler_02b25c (+6)
    0x0002D734: "SetHandlerRts_02d734",  # rts de SetTaskHandler_02d72e (+6)
    # --- Wave DDDD: refs forward a huecos futuros
    # 0x0005A8BA promovido a FadeLut_16x16_05a8ba en registry (Wave EEEEE).
    # --- Wave EEEE: refs forward a huecos futuros
    # 0x00056ACC promovido a Soldier_PhysicsStep_056acc en registry (Wave FFFF).
    # 0x00056B92 promovido a Soldier_Think_056b92 en registry (Wave FFFF).
    # 0x00056E36 promovido a Soldier_LeaveTimerExpired_056e36 en registry (Wave FFFF).
    # 0x00056F64 promovido a Soldier_TestGrabBreak_056f64 en registry (Wave FFFF).
    # 0x00056F8A promovido a Soldier_SetVelXByFacing_056f8a en registry (Wave FFFF).
    # 0x00056FA0 promovido a Soldier_TestSurrender_056fa0 en registry (Wave FFFF).
    # 0x00056FEC promovido a Soldier_ProbeWalkEdge_056fec en registry (Wave FFFF).
    # 0x0005740E promovido a Soldier_PickFallAnim_05740e en registry (Wave FFFF).
    # 0x00057494 promovido a Soldier_AttackTblMelee_057494 en registry (Wave FFFF).
    # 0x000574E8 promovido a Soldier_TestMeleeRange_0574e8 en registry (Wave FFFF).
    # 0x00057558 promovido a Soldier_WalkStart_057558 en registry (Wave FFFF).
    # 0x00057AE4 promovido a Soldier_HitCheckTail_057ae4 en registry (Wave FFFF).
    # 0x00057CA8 promovido a Soldier_GrabHoldFlag_057ca8 en registry (Wave FFFF).
    # --- Wave FFFF: RTS internos de islas C
    0x00057042: "SetHandlerRts_057042",  # rts de SetTaskHandler_05703c (+6)
    0x00057556: "SetHandlerRts_057556",  # rts de SetTaskHandler_057550 (+6)
    0x00057D02: "SetHandlerRts_057d02",  # rts de SetTaskHandler_057cfc (+6)
    # --- Wave GGGG: RTS internos de islas C
    0x000539EE: "Jsr5B6Rts_0539ee",  # rts de Jsr5B6ThenJmpScheduler_0539e2 (+12)
    # --- Wave GGGG: refs forward a huecos futuros
    # 0x000539F0 promovido a Prop_IndestructibleChild_0539f0 en registry (Wave HHHH).
    # 0x00053D80 promovido a Prop_PickSpriteByVictimDir_053d80 en registry (Wave HHHH).
    # 0x00053E0C promovido a Prop_PlayBreakMusicByPhase_053e0c en registry (Wave HHHH).
    # 0x00053E78 promovido a Prop_RunDebrisScriptByPhase_053e78 en registry (Wave HHHH).
    # 0x00053E9C promovido a Prop_PickRandomItemPtr_053e9c en registry (Wave HHHH).
    # 0x00053EBA promovido a Prop_RunDebrisScriptByPrio_053eba en registry (Wave HHHH).
    # 0x00053EE2 promovido a Prop_GateDebrisA_053ee2 en registry (Wave HHHH).
    # 0x00053F08 promovido a Prop_GateDebrisB_053f08 en registry (Wave HHHH).
    # 0x00053F2E promovido a Prop_GateDebrisC_053f2e en registry (Wave HHHH).
    # 0x00053F54 promovido a Prop_GateDebrisD_053f54 en registry (Wave HHHH).
    # --- Wave HHHH: RTS internos de islas C
    0x00053A40: "JsrPcRts_053a40",  # rts de JsrPcThunk_053a3c (+4)
    0x00053DC8: "JsrAbsRts_053dc8",  # rts de JsrAbsThunk_053dc2 (+6)
    0x00053E76: "JsrAbsRts_053e76",  # rts de JsrAbsThunk_053e70 (+6)
    # --- Wave IIII: RTS internos de islas C
    0x00048A42: "SetHandlerRts_048a42",  # rts de SetTaskHandler_048a3c (+6)
    # --- Wave IIII: refs forward a huecos futuros
    # 0x00048A44 promovido a Pow_FreeStateTail_048a44 en registry (Wave JJJJ).
    # 0x00048A90 promovido a Pow_TiedStateTail_048a90 en registry (Wave JJJJ).
    # 0x00048CA4 promovido a PowFx_HitBurst_048ca4 en registry (Wave JJJJ).
    # 0x00048D0E promovido a PowRope_Spawn_048d0e en registry (Wave JJJJ).
    # 0x00048EA6 promovido a Pow_FreeInit_048ea6 en registry (Wave JJJJ).
    # 0x00048F04 promovido a Pow_SetRunVelAndSprite_048f04 en registry (Wave JJJJ).
    # 0x00048F2E promovido a Pow_ScrollProbeOrFall_048f2e en registry (Wave JJJJ).
    # 0x00048F54 promovido a Pow_TiedSwingStep_048f54 en registry (Wave JJJJ).
    # 0x00048FB0 promovido a Pow_ScrollAndProbe_048fb0 en registry (Wave JJJJ).
    # 0x00048FCC promovido a Pow_PickIdleSpriteIdx_048fcc en registry (Wave JJJJ).
    # 0x00049010 promovido a Pow_CanBeRescued_049010 en registry (Wave JJJJ).
    # 0x00049054 promovido a Pow_TiedTurnTowardTarget_049054 en registry (Wave JJJJ).
    # 0x000490FA promovido a Pow_TargetInReach_0490fa en registry (Wave JJJJ).
    # 0x00049172 promovido a Pow_TargetFarX_049172 en registry (Wave JJJJ).
    # 0x00049196 promovido a Pow_ShouldRunAway_049196 en registry (Wave JJJJ).
    # 0x000491DE promovido a Pow_ShouldWait_0491de en registry (Wave JJJJ).
    # 0x0004921E promovido a Pow_TargetNearX_04921e en registry (Wave JJJJ).
    # 0x00049256 promovido a Pow_TurnTimerAndCheck_049256 en registry (Wave JJJJ).
    # 0x0004926A promovido a Pow_ShouldTurn_04926a en registry (Wave JJJJ).
    # 0x000492A4 promovido a Pow_TargetWithin30_0492a4 en registry (Wave JJJJ).
    # 0x000492F8 promovido a Pow_AtScreenEdge_0492f8 en registry (Wave JJJJ).
    # 0x0004932C promovido a Pow_HitReceivedCheck_04932c en registry (Wave JJJJ).
    # 0x00049346 promovido a Pow_BlockedTimer_049346 en registry (Wave JJJJ).
    # 0x0004936E promovido a Pow_RetargetIfLost_04936e en registry (Wave JJJJ).
    # 0x0004939C promovido a Pow_TargetYNear_04939c en registry (Wave JJJJ).
    # 0x000493E4 promovido a Pow_SpawnFxFromTurnAngle_0493e4 en registry (Wave JJJJ).
    # 0x0004940E promovido a Pow_SpawnFxByDir_04940e en registry (Wave JJJJ).
    # --- Wave JJJJ: RTS internos de islas C
    0x00048A8E: "SetHandlerRts_048a8e",  # rts de SetTaskHandler_048a88 (+6)
    0x00048B1C: "SetHandlerRts_048b1c",  # rts de SetTaskHandler_048b16 (+6)
    0x00048B9E: "SetHandlerRts_048b9e",  # rts de SetTaskHandler_048b98 (+6)
    0x00048CA2: "SetHandlerRts_048ca2",  # rts de SetTaskHandler_048c9c (+6)
    0x00048D0C: "SetHandlerRts_048d0c",  # rts de SetTaskHandler_048d06 (+6)
    0x00048E2A: "SetHandlerRts_048e2a",  # rts de SetTaskHandler_048e24 (+6)
    0x00048EA4: "SetHandlerRts_048ea4",  # rts de SetTaskHandler_048e9e (+6)
    # --- Wave KKKK: RTS internos de islas C
    0x0004A012: "SetHandlerRts_04a012",  # rts de SetTaskHandler_04a00c (+6)
    0x0004AC30: "SetHandlerRts_04ac30",  # rts de SetTaskHandler_04ac2a (+6)
    # --- Wave KKKK: refs forward a huecos futuros
    # 0x00049FAA promovido a HumanDeath_StateTblPtrs_049faa en registry (Wave LLLL).
    # --- Wave LLLL: RTS internos de islas C
    0x00049B04: "JsrAbsRts_049b04",  # rts de JsrAbsThunk_049afe (+6)
    0x00049B56: "SetHandlerRts_049b56",  # rts de SetTaskHandler_049b50 (+6)
    # --- Wave MMMM: RTS internos de islas C
    0x00055146: "JsrPcRts_055146",  # rts de JsrPcThunk_055142 (+4)
    # --- Wave NNNN: RTS internos de islas C
    0x0004BD5C: "SetHandlerRts_04bd5c",  # rts de SetTaskHandler_04bd56 (+6)
    0x0004BE02: "SetHandlerRts_04be02",  # rts de SetTaskHandler_04bdfc (+6)
    0x0004BE70: "SetHandlerRts_04be70",  # rts de SetTaskHandler_04be6a (+6)
    0x0004BEDE: "SetHandlerRts_04bede",  # rts de SetTaskHandler_04bed8 (+6)
    0x0004BF56: "SetHandlerRts_04bf56",  # rts de SetTaskHandler_04bf50 (+6)
    0x0004BFEE: "SetHandlerRts_04bfee",  # rts de SetTaskHandler_04bfe8 (+6)
    0x0004C086: "SetHandlerRts_04c086",  # rts de SetTaskHandler_04c080 (+6)
    0x0004C126: "SetHandlerRts_04c126",  # rts de SetTaskHandler_04c120 (+6)
    0x0004C1AE: "SetHandlerRts_04c1ae",  # rts de SetTaskHandler_04c1a8 (+6)
    0x0004C274: "SetHandlerRts_04c274",  # rts de SetTaskHandler_04c26e (+6)
    0x0004C2E2: "SetHandlerRts_04c2e2",  # rts de SetTaskHandler_04c2dc (+6)
    0x0004C448: "SetHandlerRts_04c448",  # rts de SetTaskHandler_04c442 (+6)
    0x0004C576: "SetHandlerRts_04c576",  # rts de SetTaskHandler_04c570 (+6)
    0x0004C604: "SetHandlerRts_04c604",  # rts de SetTaskHandler_04c5fe (+6)
    0x0004C688: "SetHandlerRts_04c688",  # rts de SetTaskHandler_04c682 (+6)
    0x0004C6D2: "SetHandlerRts_04c6d2",  # rts de SetTaskHandler_04c6cc (+6)
    0x0004C830: "SetHandlerRts_04c830",  # rts de SetTaskHandler_04c82a (+6)
    0x0004C90E: "SetHandlerRts_04c90e",  # rts de SetTaskHandler_04c908 (+6)
    0x0004C956: "SetHandlerRts_04c956",  # rts de SetTaskHandler_04c950 (+6)
    0x0004CBB6: "JsrAbsRts_04cbb6",  # rts de JsrAbsThunk_04cbb0 (+6)
    # --- Wave OOOO: RTS internos de islas C
    0x0004DAD0: "SetHandlerRts_04dad0",  # rts de SetTaskHandler_04daca (+6)
    0x0004DC48: "SetHandlerRts_04dc48",  # rts de SetTaskHandler_04dc42 (+6)
    0x0004DDEA: "SetHandlerRts_04ddea",  # rts de SetTaskHandler_04dde4 (+6)
    0x0004DF2E: "JsrPcRts_04df2e",  # rts de JsrPcThunk_04df2a (+4)
    0x0004DF96: "JsrPcRts_04df96",  # rts de JsrPcThunk_04df92 (+4)
    # --- Wave OOOO: refs forward a huecos futuros
    # 0x0004E580 promovido a Prop_BarrierActive_04e580 en registry (Wave PPPP).
    # 0x0004ED90 promovido a Prop_BarrierPost_04ed90 en registry (Wave PPPP).
    # 0x0004F2C2 promovido a Prop_Roof_04f2c2 en registry (Wave PPPP).
    # 0x0004FA70 promovido a Prop_OffscreenLeftCheck_04fa70 en registry (Wave QQQQ).
    # 0x0004FA8A promovido a Prop_TowerBlitState10_04fa8a en registry (Wave QQQQ).
    # 0x0004FB3C promovido a Prop_NestBlitByFacing_04fb3c en registry (Wave QQQQ).
    # --- Wave PPPP: RTS internos de islas C
    0x0004EBBA: "JsrAbsRts_04ebba",  # rts de JsrAbsThunk_04ebb4 (+6)
    0x0004F3AC: "SetHandlerRts_04f3ac",  # rts de SetTaskHandler_04f3a6 (+6)
    # --- Wave PPPP: refs forward a huecos futuros
    # 0x0004FB8A promovido a Barrier_SpawnPiece01_04fb8a en registry (Wave QQQQ).
    # 0x0004FBE0 promovido a Barrier_SpawnPiece02_04fbe0 en registry (Wave QQQQ).
    # 0x0004FC36 promovido a Barrier_SpawnPiece04_04fc36 en registry (Wave QQQQ).
    # 0x0004FC8C promovido a Barrier_SpawnPiece08_04fc8c en registry (Wave QQQQ).
    # 0x0004FCD8 promovido a Barrier_SpawnPieceLeft_04fcd8 en registry (Wave QQQQ).
    # 0x0004FD2C promovido a Barrier_SpawnPieceRight_04fd2c en registry (Wave QQQQ).
    # 0x0004FD8A promovido a Gatehouse_SpawnPiece01_04fd8a en registry (Wave QQQQ).
    # 0x0004FDDE promovido a Gatehouse_SpawnPiece02_04fdde en registry (Wave QQQQ).
    # 0x0004FE32 promovido a Gatehouse_SpawnPiece04_04fe32 en registry (Wave QQQQ).
    # 0x0004FE86 promovido a Gatehouse_SpawnPiece08_04fe86 en registry (Wave QQQQ).
    # 0x0004FEDA promovido a Gatehouse_SpawnPiece10_04feda en registry (Wave QQQQ).
    # 0x0004FF2E promovido a Gatehouse_SpawnPiece20_04ff2e en registry (Wave QQQQ).
    # 0x0004FF82 promovido a Gatehouse_SpawnPiece40_04ff82 en registry (Wave QQQQ).
    # 0x0004FFD6 promovido a Gatehouse_SpawnPiece80_04ffd6 en registry (Wave QQQQ).
    # 0x0005002A promovido a Fortress_SpawnPiece01_05002a en registry (Wave QQQQ).
    # 0x0005007E promovido a Fortress_SpawnPiece02_05007e en registry (Wave QQQQ).
    # 0x000500D2 promovido a Fortress_SpawnPiece04_0500d2 en registry (Wave QQQQ).
    # 0x00050126 promovido a Fortress_SpawnPiece08_050126 en registry (Wave QQQQ).
    # 0x0005017A promovido a Fortress_SpawnPiece10_05017a en registry (Wave QQQQ).
    # 0x000501D8 promovido a Gatehouse_BlitDamaged_0501d8 en registry (Wave QQQQ).
    # --- Wave QQQQ: RTS internos de islas C
    0x0004FAF6: "JsrPcRts_04faf6",  # rts de JsrPcThunk_04faf2 (+4)
    0x0004FB72: "JsrPcRts_04fb72",  # rts de JsrPcThunk_04fb6e (+4)
    0x00051390: "SetHandlerRts_051390",  # rts de SetTaskHandler_05138a (+6)
    0x00051694: "JsrAbsRts_051694",  # rts de JsrAbsThunk_05168e (+6)
    # --- Wave RRRR: RTS internos de islas C
    0x0003DC2A: "SetHandlerRts_03dc2a",  # rts de SetTaskHandler_03dc24 (+6)
    0x0003DC72: "SetHandlerRts_03dc72",  # rts de SetTaskHandler_03dc6c (+6)
    0x0003DEBC: "SetHandlerRts_03debc",  # rts de SetTaskHandler_03deb6 (+6)
    0x0003DF30: "SetHandlerRts_03df30",  # rts de SetTaskHandler_03df2a (+6)
    0x0003DF52: "SetHandlerRts_03df52",  # rts de SetTaskHandler_03df4c (+6)
    0x0003E6D8: "JsrPcRts_03e6d8",  # rts de JsrPcThunk_03e6d4 (+4)
    0x0003E7BE: "JsrPcRts_03e7be",  # rts de JsrPcThunk_03e7ba (+4)
    0x0003FF12: "JsrAbsRts_03ff12",  # rts de JsrAbsThunk_03ff0c (+6)
    0x0004049A: "SetHandlerRts_04049a",  # rts de SetTaskHandler_040494 (+6)
    0x00040D16: "SetHandlerRts_040d16",  # rts de SetTaskHandler_040d10 (+6)
    0x00040E52: "SetHandlerRts_040e52",  # rts de SetTaskHandler_040e4c (+6)
    0x00040EB8: "SetHandlerRts_040eb8",  # rts de SetTaskHandler_040eb2 (+6)
    # --- Wave SSSS: RTS internos de islas C
    0x0005E74A: "JsrAbsRts_05e74a",  # rts de JsrAbsThunk_05e744 (+6)
    0x0005E764: "JsrAbsRts_05e764",  # rts de JsrAbsThunk_05e75e (+6)
    0x0005E8F4: "JsrAbsRts_05e8f4",  # rts de JsrAbsThunk_05e8ee (+6)
    0x0005F044: "SetHandlerRts_05f044",  # rts de SetTaskHandler_05f03e (+6)
    0x0005F0AE: "SetHandlerRts_05f0ae",  # rts de SetTaskHandler_05f0a8 (+6)
    0x0005F118: "SetHandlerRts_05f118",  # rts de SetTaskHandler_05f112 (+6)
    0x0005F3F0: "SetHandlerRts_05f3f0",  # rts de SetTaskHandler_05f3ea (+6)
    0x0005FAA4: "SetHandlerRts_05faa4",  # rts de SetTaskHandler_05fa9e (+6)
    0x0005FB22: "SetHandlerRts_05fb22",  # rts de SetTaskHandler_05fb1c (+6)
    0x0005FB86: "SetHandlerRts_05fb86",  # rts de SetTaskHandler_05fb80 (+6)
    0x0005FC24: "SetHandlerRts_05fc24",  # rts de SetTaskHandler_05fc1e (+6)
    0x0005FC76: "SetHandlerRts_05fc76",  # rts de SetTaskHandler_05fc70 (+6)
    0x0005FCE4: "SetHandlerRts_05fce4",  # rts de SetTaskHandler_05fcde (+6)
    0x00060D9E: "SetHandlerRts_060d9e",  # rts de SetTaskHandler_060d98 (+6)
    0x00060DE6: "SetHandlerRts_060de6",  # rts de SetTaskHandler_060de0 (+6)
    0x000614E4: "JsrAbsRts_0614e4",  # rts de JsrAbsThunk_0614de (+6)
    0x00061F9C: "SetHandlerRts_061f9c",  # rts de SetTaskHandler_061f96 (+6)
    0x00062006: "SetHandlerRts_062006",  # rts de SetTaskHandler_062000 (+6)
    # --- Wave SSSS: refs forward a huecos futuros
    # 0x0000FFD0 promovido a Data_00ffd0 en registry (Wave KKKKK).
    # 0x0005DE18 promovido a AtanLog_Table_05de18 en registry (Wave GGGGG).
    # 0x0005DF18 promovido a AtanExp_Table_05df18 en registry (Wave GGGGG).
    # 0x00062008 promovido a LateProp_HitThenDie_062008 en registry (Wave TTTT).
    # 0x00062014 promovido a LateProp_TakeHit_062014 en registry (Wave TTTT).
    # 0x00062046 promovido a LateProp_HPCheck_062046 en registry (Wave TTTT).
    # 0x00062084 promovido a LateProp_TakeHitB_062084 en registry (Wave TTTT).
    # 0x000626B8 promovido a Facing_SignDelta_0626b8 en registry (Wave TTTT).
    # 0x000626D8 promovido a Facing_Matches77_0626d8 en registry (Wave TTTT).
    # 0x000626F0 promovido a LateProp_XPastThreshold_0626f0 en registry (Wave TTTT).
    # 0x00062710 promovido a LateProp_StepX_062710 en registry (Wave TTTT).
    # 0x00062732 promovido a LateProp_ClampX_062732 en registry (Wave TTTT).
    # 0x00062758 promovido a LateProp_StepAnim_062758 en registry (Wave TTTT).
    # --- Wave TTTT: RTS internos de islas C
    0x00062082: "SetHandlerRts_062082",  # rts de SetTaskHandler_06207c (+6)
    0x00062128: "SetHandlerRts_062128",  # rts de SetTaskHandler_062122 (+6)
    0x00062324: "SetHandlerRts_062324",  # rts de SetTaskHandler_06231e (+6)
    0x00062358: "SetHandlerRts_062358",  # rts de SetTaskHandler_062352 (+6)
    0x0006241A: "SetHandlerRts_06241a",  # rts de SetTaskHandler_062414 (+6)
    0x000624D4: "SetHandlerRts_0624d4",  # rts de SetTaskHandler_0624ce (+6)
    0x00062534: "SetHandlerRts_062534",  # rts de SetTaskHandler_06252e (+6)
    0x00062638: "SetHandlerRts_062638",  # rts de SetTaskHandler_062632 (+6)
    0x00062682: "SetHandlerRts_062682",  # rts de SetTaskHandler_06267c (+6)
    0x000627C8: "JsrAbsRts_0627c8",  # rts de JsrAbsThunk_0627c2 (+6)
    0x00062A06: "SetHandlerRts_062a06",  # rts de SetTaskHandler_062a00 (+6)
    0x00062DCA: "SetHandlerRts_062dca",  # rts de SetTaskHandler_062dc4 (+6)
    0x00062E42: "SetHandlerRts_062e42",  # rts de SetTaskHandler_062e3c (+6)
    0x00062E88: "SetHandlerRts_062e88",  # rts de SetTaskHandler_062e82 (+6)
    0x00062EDE: "SetHandlerRts_062ede",  # rts de SetTaskHandler_062ed8 (+6)
    0x00062F8A: "SetHandlerRts_062f8a",  # rts de SetTaskHandler_062f84 (+6)
    0x00063046: "SetHandlerRts_063046",  # rts de SetTaskHandler_063040 (+6)
    0x000630BA: "SetHandlerRts_0630ba",  # rts de SetTaskHandler_0630b4 (+6)
    0x00063104: "SetHandlerRts_063104",  # rts de SetTaskHandler_0630fe (+6)
    0x000631CE: "SetHandlerRts_0631ce",  # rts de SetTaskHandler_0631c8 (+6)
    0x00063224: "SetHandlerRts_063224",  # rts de SetTaskHandler_06321e (+6)
    0x00063318: "SetHandlerRts_063318",  # rts de SetTaskHandler_063312 (+6)
    0x00063888: "SetHandlerRts_063888",  # rts de SetTaskHandler_063882 (+6)
    0x00063902: "SetHandlerRts_063902",  # rts de SetTaskHandler_0638fc (+6)
    0x00063B36: "SetHandlerRts_063b36",  # rts de SetTaskHandler_063b30 (+6)
    0x00063BB4: "SetHandlerRts_063bb4",  # rts de SetTaskHandler_063bae (+6)
    0x00063C0A: "SetHandlerRts_063c0a",  # rts de SetTaskHandler_063c04 (+6)
    0x00064220: "SetHandlerRts_064220",  # rts de SetTaskHandler_06421a (+6)
    0x00064378: "SetHandlerRts_064378",  # rts de SetTaskHandler_064372 (+6)
    0x000643D8: "SetHandlerRts_0643d8",  # rts de SetTaskHandler_0643d2 (+6)
    0x000646FE: "SetHandlerRts_0646fe",  # rts de SetTaskHandler_0646f8 (+6)
    0x00064A7E: "SetHandlerRts_064a7e",  # rts de SetTaskHandler_064a78 (+6)
    0x00064AD6: "SetHandlerRts_064ad6",  # rts de SetTaskHandler_064ad0 (+6)
    0x00064B16: "SetHandlerRts_064b16",  # rts de SetTaskHandler_064b10 (+6)
    0x00064CEC: "SetHandlerRts_064cec",  # rts de SetTaskHandler_064ce6 (+6)
    0x00064D78: "SetHandlerRts_064d78",  # rts de SetTaskHandler_064d72 (+6)
    0x0006504C: "SetHandlerRts_06504c",  # rts de SetTaskHandler_065046 (+6)
    0x000650B6: "SetHandlerRts_0650b6",  # rts de SetTaskHandler_0650b0 (+6)
    0x000650FC: "SetHandlerRts_0650fc",  # rts de SetTaskHandler_0650f6 (+6)
    0x0006515C: "SetHandlerRts_06515c",  # rts de SetTaskHandler_065156 (+6)
    0x0006527A: "SetHandlerRts_06527a",  # rts de SetTaskHandler_065274 (+6)
    0x000652F2: "SetHandlerRts_0652f2",  # rts de SetTaskHandler_0652ec (+6)
    0x00065378: "SetHandlerRts_065378",  # rts de SetTaskHandler_065372 (+6)
    0x000653BC: "SetHandlerRts_0653bc",  # rts de SetTaskHandler_0653b6 (+6)
    0x00065412: "SetHandlerRts_065412",  # rts de SetTaskHandler_06540c (+6)
    0x00065468: "SetHandlerRts_065468",  # rts de SetTaskHandler_065462 (+6)
    0x000655F2: "SetHandlerRts_0655f2",  # rts de SetTaskHandler_0655ec (+6)
    0x00065666: "SetHandlerRts_065666",  # rts de SetTaskHandler_065660 (+6)
    0x0006576A: "SetHandlerRts_06576a",  # rts de SetTaskHandler_065764 (+6)
    0x00065AA2: "SetHandlerRts_065aa2",  # rts de SetTaskHandler_065a9c (+6)
    0x00065AF2: "SetHandlerRts_065af2",  # rts de SetTaskHandler_065aec (+6)
    # --- Wave TTTT: refs forward a huecos futuros
    # 0x0006600E promovido a FloatBarrel_Submerge_06600e en registry (Wave UUUU).
    # 0x00066622 promovido a FloatBarrel_FaceTarget_066622 en registry (Wave UUUU).
    # 0x00066644 promovido a FloatBarrel_TargetInRange_066644 en registry (Wave UUUU).
    # --- Wave UUUU: RTS internos de islas C
    0x000665E6: "SetTaskWRts_0665e6",  # rts de SetTaskW_0665e2 (+4)
    0x0006679A: "SetHandlerRts_06679a",  # rts de SetTaskHandler_066794 (+6)
    0x00066A84: "SetHandlerRts_066a84",  # rts de SetTaskHandler_066a7e (+6)
    0x00066B6A: "SetHandlerRts_066b6a",  # rts de SetTaskHandler_066b64 (+6)
    0x00066BC8: "SetHandlerRts_066bc8",  # rts de SetTaskHandler_066bc2 (+6)
    0x000671DA: "SetHandlerRts_0671da",  # rts de SetTaskHandler_0671d4 (+6)
    0x00067654: "SetHandlerRts_067654",  # rts de SetTaskHandler_06764e (+6)
    0x0006777E: "SetHandlerRts_06777e",  # rts de SetTaskHandler_067778 (+6)
    0x00067B7A: "SetHandlerRts_067b7a",  # rts de SetTaskHandler_067b74 (+6)
    0x00067C7A: "SetHandlerRts_067c7a",  # rts de SetTaskHandler_067c74 (+6)
    0x00067CE6: "SetHandlerRts_067ce6",  # rts de SetTaskHandler_067ce0 (+6)
    0x00067E18: "SetHandlerRts_067e18",  # rts de SetTaskHandler_067e12 (+6)
    0x00067E78: "SetHandlerRts_067e78",  # rts de SetTaskHandler_067e72 (+6)
    0x00068204: "JsrAbsRts_068204",  # rts de JsrAbsThunk_0681fe (+6)
    0x0006830E: "SetHandlerRts_06830e",  # rts de SetTaskHandler_068308 (+6)
    0x00068344: "SetHandlerRts_068344",  # rts de SetTaskHandler_06833e (+6)
    0x000685D6: "SetHandlerRts_0685d6",  # rts de SetTaskHandler_0685d0 (+6)
    0x00068620: "SetHandlerRts_068620",  # rts de SetTaskHandler_06861a (+6)
    0x00068682: "SetHandlerRts_068682",  # rts de SetTaskHandler_06867c (+6)
    0x000686CC: "SetHandlerRts_0686cc",  # rts de SetTaskHandler_0686c6 (+6)
    0x00068948: "SetHandlerRts_068948",  # rts de SetTaskHandler_068942 (+6)
    0x0006895A: "SetHandlerRts_06895a",  # rts de SetTaskHandler_068954 (+6)
    0x000697CC: "SetHandlerRts_0697cc",  # rts de SetTaskHandler_0697c6 (+6)
    0x00069868: "SetHandlerRts_069868",  # rts de SetTaskHandler_069862 (+6)
    0x00069922: "JsrAbsRts_069922",  # rts de JsrAbsThunk_06991c (+6)
    0x000699B0: "SetHandlerRts_0699b0",  # rts de SetTaskHandler_0699aa (+6)
    0x00069AE6: "SetHandlerRts_069ae6",  # rts de SetTaskHandler_069ae0 (+6)
    0x00069BA8: "SetHandlerRts_069ba8",  # rts de SetTaskHandler_069ba2 (+6)
    0x00069CCE: "SetHandlerRts_069cce",  # rts de SetTaskHandler_069cc8 (+6)
    0x00069EA8: "SetXNMid_069ea8",  # rts de SetXN_069ea4 (+4)
    0x00069EF6: "ClrRamWordRts_069ef6",  # rts de ClrRamWord_069ef0 (+6)
    # --- Wave VVVV: RTS internos de islas C
    0x0006A042: "JsrAbsRts_06a042",  # rts de JsrAbsThunk_06a03c (+6)
    0x0006A3D4: "SetHandlerRts_06a3d4",  # rts de SetTaskHandler_06a3ce (+6)
    0x0006A41A: "SetHandlerRts_06a41a",  # rts de SetTaskHandler_06a414 (+6)
    0x0006A450: "SetHandlerRts_06a450",  # rts de SetTaskHandler_06a44a (+6)
    0x0006A7BA: "SetHandlerRts_06a7ba",  # rts de SetTaskHandler_06a7b4 (+6)
    0x0006A96A: "SetHandlerRts_06a96a",  # rts de SetTaskHandler_06a964 (+6)
    0x0006AA02: "SetHandlerRts_06aa02",  # rts de SetTaskHandler_06a9fc (+6)
    0x0006ABE0: "SetHandlerRts_06abe0",  # rts de SetTaskHandler_06abda (+6)
    0x0006AC5A: "SetHandlerRts_06ac5a",  # rts de SetTaskHandler_06ac54 (+6)
    0x0006ACA8: "SetHandlerRts_06aca8",  # rts de SetTaskHandler_06aca2 (+6)
    0x0006AF66: "JsrAbsRts_06af66",  # rts de JsrAbsThunk_06af60 (+6)
    0x0006AFF4: "SetHandlerRts_06aff4",  # rts de SetTaskHandler_06afee (+6)
    0x0006B012: "SetHandlerRts_06b012",  # rts de SetTaskHandler_06b00c (+6)
    0x0006B1D2: "SetHandlerRts_06b1d2",  # rts de SetTaskHandler_06b1cc (+6)
    0x0006B278: "JsrAbsRts_06b278",  # rts de JsrAbsThunk_06b272 (+6)
    0x0006C466: "SetHandlerRts_06c466",  # rts de SetTaskHandler_06c460 (+6)
    0x0006C4C4: "SetHandlerRts_06c4c4",  # rts de SetTaskHandler_06c4be (+6)
    0x0006C53C: "SetHandlerRts_06c53c",  # rts de SetTaskHandler_06c536 (+6)
    0x0006CE46: "SetHandlerRts_06ce46",  # rts de SetTaskHandler_06ce40 (+6)
    0x0006CE64: "SetHandlerRts_06ce64",  # rts de SetTaskHandler_06ce5e (+6)
    0x0006D04C: "SetHandlerRts_06d04c",  # rts de SetTaskHandler_06d046 (+6)
    0x0006D9EA: "SetHandlerRts_06d9ea",  # rts de SetTaskHandler_06d9e4 (+6)
    0x0006DA4A: "SetHandlerRts_06da4a",  # rts de SetTaskHandler_06da44 (+6)
    0x0006DBD2: "SetHandlerRts_06dbd2",  # rts de SetTaskHandler_06dbcc (+6)
    0x0006DC5C: "SetHandlerRts_06dc5c",  # rts de SetTaskHandler_06dc56 (+6)
    0x0006DC8E: "SetHandlerRts_06dc8e",  # rts de SetTaskHandler_06dc88 (+6)
    0x0006DCDE: "SetHandlerRts_06dcde",  # rts de SetTaskHandler_06dcd8 (+6)
    0x0006DF04: "SetHandlerRts_06df04",  # rts de SetTaskHandler_06defe (+6)
    # --- Wave VVVV: refs forward a huecos futuros
    # 0x0006E15E promovido a FireBurst_FreeIfOffWorld_06e15e en registry (Wave WWWW).
    # 0x0006E176 promovido a Walker_PickBurstVel_06e176 en registry (Wave WWWW).
    # 0x0006E20C promovido a Facing_NegIfLeft_06e20c en registry (Wave WWWW).
    # 0x0006E2FE promovido a FireBurst_TickHit_06e2fe en registry (Wave WWWW).
    # 0x0006E31E promovido a Walker_PlayCry_06e31e en registry (Wave WWWW).
    # 0x0006E34A promovido a Entity_CopyParentAnimTimer_06e34a en registry (Wave WWWW).
    # 0x0006E356 promovido a Frag_PlaySnd157To159_06e356 en registry (Wave WWWW).
    # 0x0006E394 promovido a Walker_ClearFlag10E39A_06e394 en registry (Wave WWWW).
    # 0x0006E484 promovido a Walker_SetSpriteByFlags7E7F_06e484 en registry (Wave WWWW).
    # --- Wave WWWW: RTS internos de islas C
    0x0006E15C: "SetHandlerRts_06e15c",  # rts de SetTaskHandler_06e156 (+6)
    0x0006E174: "SetHandlerRts_06e174",  # rts de SetTaskHandler_06e16e (+6)
    0x0006E3A2: "ClrRamWordRts_06e3a2",  # rts de ClrRamWord_06e39c (+6)
    0x0006E4A6: "JsrAbsRts_06e4a6",  # rts de JsrAbsThunk_06e4a0 (+6)
    0x0006E66A: "SetHandlerRts_06e66a",  # rts de SetTaskHandler_06e664 (+6)
    0x0006E750: "SetHandlerRts_06e750",  # rts de SetTaskHandler_06e74a (+6)
    0x0006E88A: "SetHandlerRts_06e88a",  # rts de SetTaskHandler_06e884 (+6)
    0x0006E930: "SetHandlerRts_06e930",  # rts de SetTaskHandler_06e92a (+6)
    0x0006E964: "SetHandlerRts_06e964",  # rts de SetTaskHandler_06e95e (+6)
    0x0006EA94: "SetHandlerRts_06ea94",  # rts de SetTaskHandler_06ea8e (+6)
    0x0006EB82: "SetHandlerRts_06eb82",  # rts de SetTaskHandler_06eb7c (+6)
    0x0006ED40: "SetHandlerRts_06ed40",  # rts de SetTaskHandler_06ed3a (+6)
    0x0006EE80: "SetHandlerRts_06ee80",  # rts de SetTaskHandler_06ee7a (+6)
    0x0006F1F4: "SetHandlerRts_06f1f4",  # rts de SetTaskHandler_06f1ee (+6)
    0x0006F33E: "SetHandlerRts_06f33e",  # rts de SetTaskHandler_06f338 (+6)
    0x0006F38A: "SetHandlerRts_06f38a",  # rts de SetTaskHandler_06f384 (+6)
    0x0006FA5E: "SetHandlerRts_06fa5e",  # rts de SetTaskHandler_06fa58 (+6)
    0x0006FAF8: "SetHandlerRts_06faf8",  # rts de SetTaskHandler_06faf2 (+6)
    0x0006FB9E: "SetHandlerRts_06fb9e",  # rts de SetTaskHandler_06fb98 (+6)
    0x0006FC4E: "SetHandlerRts_06fc4e",  # rts de SetTaskHandler_06fc48 (+6)
    0x0006FEEE: "SetHandlerRts_06feee",  # rts de SetTaskHandler_06fee8 (+6)
    0x0007010C: "SetHandlerRts_07010c",  # rts de SetTaskHandler_070106 (+6)
    0x0007016C: "SetHandlerRts_07016c",  # rts de SetTaskHandler_070166 (+6)
    0x0007020C: "SetHandlerRts_07020c",  # rts de SetTaskHandler_070206 (+6)
    0x0007028E: "SetHandlerRts_07028e",  # rts de SetTaskHandler_070288 (+6)
    0x000702A6: "SetHandlerRts_0702a6",  # rts de SetTaskHandler_0702a0 (+6)
    0x000702E2: "SetHandlerRts_0702e2",  # rts de SetTaskHandler_0702dc (+6)
    0x000704F0: "SetHandlerRts_0704f0",  # rts de SetTaskHandler_0704ea (+6)
    0x00070692: "SetHandlerRts_070692",  # rts de SetTaskHandler_07068c (+6)
    0x0007073A: "SetHandlerRts_07073a",  # rts de SetTaskHandler_070734 (+6)
    0x0007079C: "SetHandlerRts_07079c",  # rts de SetTaskHandler_070796 (+6)
    0x000707C6: "SetHandlerRts_0707c6",  # rts de SetTaskHandler_0707c0 (+6)
    0x00070814: "SetHandlerRts_070814",  # rts de SetTaskHandler_07080e (+6)
    0x0007085A: "SetHandlerRts_07085a",  # rts de SetTaskHandler_070854 (+6)
    0x000708B0: "SetHandlerRts_0708b0",  # rts de SetTaskHandler_0708aa (+6)
    0x000708F2: "SetHandlerRts_0708f2",  # rts de SetTaskHandler_0708ec (+6)
    0x00070920: "SetHandlerRts_070920",  # rts de SetTaskHandler_07091a (+6)
    0x00070954: "SetHandlerRts_070954",  # rts de SetTaskHandler_07094e (+6)
    0x00070A30: "SetHandlerRts_070a30",  # rts de SetTaskHandler_070a2a (+6)
    0x000713A0: "SetHandlerRts_0713a0",  # rts de SetTaskHandler_07139a (+6)
    0x000715F8: "SetHandlerRts_0715f8",  # rts de SetTaskHandler_0715f2 (+6)
    0x000716B4: "SetHandlerRts_0716b4",  # rts de SetTaskHandler_0716ae (+6)
    0x00071A4C: "SetHandlerRts_071a4c",  # rts de SetTaskHandler_071a46 (+6)
    0x00071B02: "SetHandlerRts_071b02",  # rts de SetTaskHandler_071afc (+6)
    0x00071B54: "SetHandlerRts_071b54",  # rts de SetTaskHandler_071b4e (+6)
    0x00071B98: "SetHandlerRts_071b98",  # rts de SetTaskHandler_071b92 (+6)
    0x00071C82: "SetHandlerRts_071c82",  # rts de SetTaskHandler_071c7c (+6)
    0x00071DB8: "SetHandlerRts_071db8",  # rts de SetTaskHandler_071db2 (+6)
    0x00071E4E: "SetHandlerRts_071e4e",  # rts de SetTaskHandler_071e48 (+6)
    0x00071EB4: "SetHandlerRts_071eb4",  # rts de SetTaskHandler_071eae (+6)
    0x00071F70: "SetHandlerRts_071f70",  # rts de SetTaskHandler_071f6a (+6)
    # --- Wave WWWW: refs forward a huecos futuros
    # 0x00071FFC promovido a FinalBoss_FlameTail_FreeIfOffWorld_071ffc en registry (Wave XXXX).
    # 0x0007229C promovido a FinalBoss_Spark_07229c en registry (Wave XXXX).
    # 0x0007231E promovido a FinalBoss_ShiftX_Explode_07231e en registry (Wave XXXX).
    # 0x0007233A promovido a FinalBoss_FlushAndExplodeB_07233a en registry (Wave XXXX).
    # 0x00072364 promovido a FinalBoss_ExplodeC_072364 en registry (Wave XXXX).
    # 0x000723D2 promovido a FinalBoss_LimbPart_Init_0723d2 en registry (Wave XXXX).
    # 0x000726D4 promovido a FinalBoss_InScreenByVel_0726d4 en registry (Wave XXXX).
    # 0x00072750 promovido a FinalBoss_EdgeFlagByVel_072750 en registry (Wave XXXX).
    # 0x00072782 promovido a Facing_NegIfLeft_072782 en registry (Wave XXXX).
    # 0x0007279A promovido a FinalBoss_SyncStateToParentAndTick_07279a en registry (Wave XXXX).
    # 0x000727C4 promovido a FinalBoss_SyncStateToParent_0727c4 en registry (Wave XXXX).
    # 0x000727EA promovido a FinalBoss_TickAttackTimers_0727ea en registry (Wave XXXX).
    # 0x00072B5A promovido a FinalBoss_HeadTargetTimer_072b5a en registry (Wave XXXX).
    # 0x00072B96 promovido a FinalBoss_PlayPartCry_072b96 en registry (Wave XXXX).
    # 0x00072BCE promovido a FinalBoss_CopyParentPosPrio_072bce en registry (Wave XXXX).
    # 0x00072BDE promovido a FinalBoss_OffsetByLimbTable_072bde en registry (Wave XXXX).
    # 0x00072C08 promovido a FinalBoss_FetchPartSprite_072c08 en registry (Wave XXXX).
    # 0x00072C44 promovido a FinalBoss_HitTestPlayers_072c44 en registry (Wave XXXX).
    # 0x00072DB8 promovido a FinalBoss_SpawnSmokeColumn_072db8 en registry (Wave XXXX).
    # --- Wave XXXX: RTS internos de islas C
    0x00072018: "SetHandlerRts_072018",  # rts de SetTaskHandler_072012 (+6)
    0x000720A4: "SetHandlerRts_0720a4",  # rts de SetTaskHandler_07209e (+6)
    0x00072142: "SetHandlerRts_072142",  # rts de SetTaskHandler_07213c (+6)
    0x000721E8: "SetHandlerRts_0721e8",  # rts de SetTaskHandler_0721e2 (+6)
    0x00072276: "SetHandlerRts_072276",  # rts de SetTaskHandler_072270 (+6)
    0x00072304: "SetHandlerRts_072304",  # rts de SetTaskHandler_0722fe (+6)
    0x000723D0: "SetHandlerRts_0723d0",  # rts de SetTaskHandler_0723ca (+6)
    0x00072434: "SetHandlerRts_072434",  # rts de SetTaskHandler_07242e (+6)
    0x00072488: "SetHandlerRts_072488",  # rts de SetTaskHandler_072482 (+6)
    0x000724D2: "SetHandlerRts_0724d2",  # rts de SetTaskHandler_0724cc (+6)
    0x000726D2: "SetHandlerRts_0726d2",  # rts de SetTaskHandler_0726cc (+6)
    0x0007282A: "JsrPcRts_07282a",  # rts de JsrPcThunk_072826 (+4)
    0x00072C96: "JsrPcRts_072c96",  # rts de JsrPcThunk_072c92 (+4)
    0x00073452: "SetHandlerRts_073452",  # rts de SetTaskHandler_07344c (+6)
    0x000735A4: "SetHandlerRts_0735a4",  # rts de SetTaskHandler_07359e (+6)
    0x00073686: "SetHandlerRts_073686",  # rts de SetTaskHandler_073680 (+6)
    0x0007375A: "SetHandlerRts_07375a",  # rts de SetTaskHandler_073754 (+6)
    0x00073802: "SetHandlerRts_073802",  # rts de SetTaskHandler_0737fc (+6)
    0x00073950: "SetHandlerRts_073950",  # rts de SetTaskHandler_07394a (+6)
    0x00073A30: "SetHandlerRts_073a30",  # rts de SetTaskHandler_073a2a (+6)
    0x00073B60: "SetHandlerRts_073b60",  # rts de SetTaskHandler_073b5a (+6)
    0x00073B90: "SetHandlerRts_073b90",  # rts de SetTaskHandler_073b8a (+6)
    0x00073C26: "SetHandlerRts_073c26",  # rts de SetTaskHandler_073c20 (+6)
    0x00073E5A: "SetHandlerRts_073e5a",  # rts de SetTaskHandler_073e54 (+6)
    0x00073ECE: "SetHandlerRts_073ece",  # rts de SetTaskHandler_073ec8 (+6)
    0x00073FC8: "SetHandlerRts_073fc8",  # rts de SetTaskHandler_073fc2 (+6)
    0x0007410C: "SetHandlerRts_07410c",  # rts de SetTaskHandler_074106 (+6)
    0x0007422A: "JsrAbsRts_07422a",  # rts de JsrAbsThunk_074224 (+6)
    0x0007525A: "SetHandlerRts_07525a",  # rts de SetTaskHandler_075254 (+6)
    0x00075FFA: "SetTaskWRts_075ffa",  # rts de SetTaskW_075ff6 (+4)
    # --- Wave XXXX: refs forward a huecos futuros
    # 0x00076012 promovido a ScriptedProp_SpawnFlagAndInit_076012 en registry (Wave YYYY).
    # 0x0007690A promovido a ScriptedProp_Window_Init_07690a en registry (Wave YYYY).
    # 0x00076A2A promovido a ScriptedProp_RoofA_Run_076a2a en registry (Wave YYYY).
    # 0x00076ADE promovido a ScriptedProp_RoofB_Run_076ade en registry (Wave YYYY).
    # 0x00076BE8 promovido a ScriptedProp_Base_Run_076be8 en registry (Wave YYYY).
    # 0x00076E10 promovido a Frag_Launch_076e10 en registry (Wave YYYY).
    # 0x00077FD6 promovido a Explosion_FlashSndB_077fd6 en registry (Wave YYYY).
    # --- Wave YYYY: RTS internos de islas C
    0x00076010: "SetHandlerRts_076010",  # rts de SetTaskHandler_07600a (+6)
    0x000760A6: "SetHandlerRts_0760a6",  # rts de SetTaskHandler_0760a0 (+6)
    0x000760C6: "SetHandlerRts_0760c6",  # rts de SetTaskHandler_0760c0 (+6)
    0x000760F2: "SetHandlerRts_0760f2",  # rts de SetTaskHandler_0760ec (+6)
    0x00076A8E: "SetHandlerRts_076a8e",  # rts de SetTaskHandler_076a88 (+6)
    0x00076ADC: "Jsr5B6Rts_076adc",  # rts de Jsr5B6ThenJmpScheduler_076ad0 (+12)
    0x00076B38: "SetHandlerRts_076b38",  # rts de SetTaskHandler_076b32 (+6)
    0x000778A0: "SetHandlerRts_0778a0",  # rts de SetTaskHandler_07789a (+6)
    0x000778FA: "SetHandlerRts_0778fa",  # rts de SetTaskHandler_0778f4 (+6)
    0x00077A14: "SetHandlerRts_077a14",  # rts de SetTaskHandler_077a0e (+6)
    0x00077A8C: "SetHandlerRts_077a8c",  # rts de SetTaskHandler_077a86 (+6)
    0x00077B66: "SetHandlerRts_077b66",  # rts de SetTaskHandler_077b60 (+6)
    0x00079442: "SetHandlerRts_079442",  # rts de SetTaskHandler_07943c (+6)
    0x000798AA: "JsrPcRts_0798aa",  # rts de JsrPcThunk_0798a6 (+4)
    0x00079AE2: "SetHandlerRts_079ae2",  # rts de SetTaskHandler_079adc (+6)
    0x00079B20: "SetHandlerRts_079b20",  # rts de SetTaskHandler_079b1a (+6)
    0x00079C10: "SetHandlerRts_079c10",  # rts de SetTaskHandler_079c0a (+6)
    0x00079C3A: "SetHandlerRts_079c3a",  # rts de SetTaskHandler_079c34 (+6)
    0x00079D02: "SetHandlerRts_079d02",  # rts de SetTaskHandler_079cfc (+6)
    0x00079D54: "SetHandlerRts_079d54",  # rts de SetTaskHandler_079d4e (+6)
    0x00079E28: "SetHandlerRts_079e28",  # rts de SetTaskHandler_079e22 (+6)
    0x00079E66: "SetHandlerRts_079e66",  # rts de SetTaskHandler_079e60 (+6)
    0x00079FE6: "SetHandlerRts_079fe6",  # rts de SetTaskHandler_079fe0 (+6)
    # --- Wave YYYY: refs forward a huecos futuros
    # 0x0007A19E promovido a Crew_Hostage_Init_07a19e en registry (Wave ZZZZ).
    # --- Wave ZZZZ: RTS internos de islas C
    0x0007A0AE: "SetHandlerRts_07a0ae",  # rts de SetTaskHandler_07a0a8 (+6)
    0x0007A130: "SetHandlerRts_07a130",  # rts de SetTaskHandler_07a12a (+6)
    0x0007A19C: "SetHandlerRts_07a19c",  # rts de SetTaskHandler_07a196 (+6)
    0x0007A27A: "SetHandlerRts_07a27a",  # rts de SetTaskHandler_07a274 (+6)
    0x0007A304: "SetHandlerRts_07a304",  # rts de SetTaskHandler_07a2fe (+6)
    0x0007A3E4: "SetHandlerRts_07a3e4",  # rts de SetTaskHandler_07a3de (+6)
    0x0007BB18: "SetHandlerRts_07bb18",  # rts de SetTaskHandler_07bb12 (+6)
    0x0007BC8C: "SetHandlerRts_07bc8c",  # rts de SetTaskHandler_07bc86 (+6)
    0x0007BD20: "SetHandlerRts_07bd20",  # rts de SetTaskHandler_07bd1a (+6)
    0x0007BDCA: "SetHandlerRts_07bdca",  # rts de SetTaskHandler_07bdc4 (+6)
    0x0007BE7A: "SetHandlerRts_07be7a",  # rts de SetTaskHandler_07be74 (+6)
    0x0007BEF2: "SetHandlerRts_07bef2",  # rts de SetTaskHandler_07beec (+6)
    0x0007C06E: "SetHandlerRts_07c06e",  # rts de SetTaskHandler_07c068 (+6)
    0x0007C1B4: "SetHandlerRts_07c1b4",  # rts de SetTaskHandler_07c1ae (+6)
    0x0007C372: "SetHandlerRts_07c372",  # rts de SetTaskHandler_07c36c (+6)
    0x0007C422: "SetHandlerRts_07c422",  # rts de SetTaskHandler_07c41c (+6)
    0x0007C478: "SetHandlerRts_07c478",  # rts de SetTaskHandler_07c472 (+6)
    0x0007C510: "SetHandlerRts_07c510",  # rts de SetTaskHandler_07c50a (+6)
    0x0007C752: "SetHandlerRts_07c752",  # rts de SetTaskHandler_07c74c (+6)
    0x0007C8B0: "SetHandlerRts_07c8b0",  # rts de SetTaskHandler_07c8aa (+6)
    0x0007C916: "SetHandlerRts_07c916",  # rts de SetTaskHandler_07c910 (+6)
    0x0007CA5E: "SetHandlerRts_07ca5e",  # rts de SetTaskHandler_07ca58 (+6)
    0x0007CEDE: "SdsRts_07cede",  # rts de SDS_07ced8 (+6)
    0x0007D054: "SetHandlerRts_07d054",  # rts de SetTaskHandler_07d04e (+6)
    0x0007D176: "SetHandlerRts_07d176",  # rts de SetTaskHandler_07d170 (+6)
    0x0007D22C: "SetHandlerRts_07d22c",  # rts de SetTaskHandler_07d226 (+6)
    0x0007D330: "SetHandlerRts_07d330",  # rts de SetTaskHandler_07d32a (+6)
    0x0007D3AE: "SetHandlerRts_07d3ae",  # rts de SetTaskHandler_07d3a8 (+6)
    0x0007D4A8: "SetHandlerRts_07d4a8",  # rts de SetTaskHandler_07d4a2 (+6)
    0x0007D5BE: "SetHandlerRts_07d5be",  # rts de SetTaskHandler_07d5b8 (+6)
    0x0007D636: "SetHandlerRts_07d636",  # rts de SetTaskHandler_07d630 (+6)
    0x0007D6A4: "SetHandlerRts_07d6a4",  # rts de SetTaskHandler_07d69e (+6)
    0x0007D76C: "SetHandlerRts_07d76c",  # rts de SetTaskHandler_07d766 (+6)
    0x0007D896: "SetHandlerRts_07d896",  # rts de SetTaskHandler_07d890 (+6)
    0x0007DA44: "SetHandlerRts_07da44",  # rts de SetTaskHandler_07da3e (+6)
    0x0007DDDE: "SetHandlerRts_07ddde",  # rts de SetTaskHandler_07ddd8 (+6)
    0x0007DE8C: "SetHandlerRts_07de8c",  # rts de SetTaskHandler_07de86 (+6)
    0x0007DF6A: "SetHandlerRts_07df6a",  # rts de SetTaskHandler_07df64 (+6)
    0x0007E262: "SetHandlerRts_07e262",  # rts de SetTaskHandler_07e25c (+6)
    0x0007E298: "SetHandlerRts_07e298",  # rts de SetTaskHandler_07e292 (+6)
    0x0007E2D0: "SetHandlerRts_07e2d0",  # rts de SetTaskHandler_07e2ca (+6)
    0x0007E2F0: "SetHandlerRts_07e2f0",  # rts de SetTaskHandler_07e2ea (+6)
    0x0007E40A: "SetHandlerRts_07e40a",  # rts de SetTaskHandler_07e404 (+6)
    0x0007E502: "SetHandlerRts_07e502",  # rts de SetTaskHandler_07e4fc (+6)
    0x0007E600: "SetHandlerRts_07e600",  # rts de SetTaskHandler_07e5fa (+6)
    0x0007E72C: "SetHandlerRts_07e72c",  # rts de SetTaskHandler_07e726 (+6)
    0x0007E852: "SetHandlerRts_07e852",  # rts de SetTaskHandler_07e84c (+6)
    0x0007E984: "SetHandlerRts_07e984",  # rts de SetTaskHandler_07e97e (+6)
    0x0007E9D8: "SetHandlerRts_07e9d8",  # rts de SetTaskHandler_07e9d2 (+6)
    0x0007EA2C: "SetHandlerRts_07ea2c",  # rts de SetTaskHandler_07ea26 (+6)
    0x0007EA7E: "SetHandlerRts_07ea7e",  # rts de SetTaskHandler_07ea78 (+6)
    0x0007EAC0: "SetHandlerRts_07eac0",  # rts de SetTaskHandler_07eaba (+6)
    0x0007EB28: "SetHandlerRts_07eb28",  # rts de SetTaskHandler_07eb22 (+6)
    0x0007EB74: "SetHandlerRts_07eb74",  # rts de SetTaskHandler_07eb6e (+6)
    0x0007ECCE: "SetHandlerRts_07ecce",  # rts de SetTaskHandler_07ecc8 (+6)
    0x0007ECFE: "SetHandlerRts_07ecfe",  # rts de SetTaskHandler_07ecf8 (+6)
    0x0007EDF2: "SetHandlerRts_07edf2",  # rts de SetTaskHandler_07edec (+6)
    0x0007EE3A: "SetHandlerRts_07ee3a",  # rts de SetTaskHandler_07ee34 (+6)
    0x0007EE78: "SetHandlerRts_07ee78",  # rts de SetTaskHandler_07ee72 (+6)
    0x0007EF10: "SetHandlerRts_07ef10",  # rts de SetTaskHandler_07ef0a (+6)
    0x0007EF48: "JsrAbsRts_07ef48",  # rts de JsrAbsThunk_07ef42 (+6)
    0x0007EF86: "SetHandlerRts_07ef86",  # rts de SetTaskHandler_07ef80 (+6)
    0x0007EFCA: "SetHandlerRts_07efca",  # rts de SetTaskHandler_07efc4 (+6)
    0x0007F05E: "SetHandlerRts_07f05e",  # rts de SetTaskHandler_07f058 (+6)
    0x0007F0FA: "SetHandlerRts_07f0fa",  # rts de SetTaskHandler_07f0f4 (+6)
    0x0007F146: "SetHandlerRts_07f146",  # rts de SetTaskHandler_07f140 (+6)
    0x0007F184: "SetHandlerRts_07f184",  # rts de SetTaskHandler_07f17e (+6)
    0x0007F228: "SetHandlerRts_07f228",  # rts de SetTaskHandler_07f222 (+6)
    0x0007F2C8: "SetHandlerRts_07f2c8",  # rts de SetTaskHandler_07f2c2 (+6)
    0x0007F3E6: "SetHandlerRts_07f3e6",  # rts de SetTaskHandler_07f3e0 (+6)
    0x0007F462: "SetHandlerRts_07f462",  # rts de SetTaskHandler_07f45c (+6)
    0x0007F4C4: "SetHandlerRts_07f4c4",  # rts de SetTaskHandler_07f4be (+6)
    0x0007F530: "SetHandlerRts_07f530",  # rts de SetTaskHandler_07f52a (+6)
    0x0007F5A2: "SetHandlerRts_07f5a2",  # rts de SetTaskHandler_07f59c (+6)
    0x0007F5E8: "SetHandlerRts_07f5e8",  # rts de SetTaskHandler_07f5e2 (+6)
    0x0007F63A: "SetHandlerRts_07f63a",  # rts de SetTaskHandler_07f634 (+6)
    0x0007F66A: "JsrAbsRts_07f66a",  # rts de JsrAbsThunk_07f664 (+6)
    0x0007F766: "SetHandlerRts_07f766",  # rts de SetTaskHandler_07f760 (+6)
    0x0007F7EE: "SetHandlerRts_07f7ee",  # rts de SetTaskHandler_07f7e8 (+6)
    0x0007F848: "SetHandlerRts_07f848",  # rts de SetTaskHandler_07f842 (+6)
    0x0007F8DA: "SetHandlerRts_07f8da",  # rts de SetTaskHandler_07f8d4 (+6)
    0x0007F986: "SetHandlerRts_07f986",  # rts de SetTaskHandler_07f980 (+6)
    0x0007F9CE: "SetHandlerRts_07f9ce",  # rts de SetTaskHandler_07f9c8 (+6)
    0x0007FA1E: "SetHandlerRts_07fa1e",  # rts de SetTaskHandler_07fa18 (+6)
    0x0007FA90: "SetHandlerRts_07fa90",  # rts de SetTaskHandler_07fa8a (+6)
    0x0007FACA: "SetHandlerRts_07faca",  # rts de SetTaskHandler_07fac4 (+6)
    0x0007FB26: "SetHandlerRts_07fb26",  # rts de SetTaskHandler_07fb20 (+6)
    0x0007FB6C: "SetHandlerRts_07fb6c",  # rts de SetTaskHandler_07fb66 (+6)
    0x0007FBD0: "SetHandlerRts_07fbd0",  # rts de SetTaskHandler_07fbca (+6)
    # --- Wave AAAAA: datos de escena $916C8..$967B4 y listas attract $96BBC..$97730
    0x000916C8: "SceneDescTable_0916C8",
    0x00091748: "SceneEntities_091748",
    0x00091782: "SceneScript_091782",
    0x00091EEC: "SceneTrig_091EEC",
    0x00091F0C: "SceneTrig_091F0C",
    0x00091F5A: "SceneTrig_091F5A",
    0x00091F8C: "SceneTrig_091F8C",
    0x00091FBE: "SceneTrig_091FBE",
    0x00091FC8: "SceneTrig_091FC8",
    0x00091FCE: "SceneTrig_091FCE",
    0x00091FEC: "SceneTrig_091FEC",
    0x00091FF6: "SceneTrig_091FF6",
    0x00091FFA: "SceneEntities_091FFA",
    0x00092034: "SceneScript_092034",
    0x000924AE: "SceneTrig_0924AE",
    0x000924D0: "SceneTrig_0924D0",
    0x000924F2: "SceneTrig_0924F2",
    0x00092510: "SceneTrig_092510",
    0x00092534: "SceneEntities_092534",
    0x0009256E: "SceneScript_09256E",
    0x00092ADC: "SceneTrig_092ADC",
    0x00092AF2: "SceneTrig_092AF2",
    0x00092B10: "SceneTrig_092B10",
    0x00092B98: "SceneEntities_092B98",
    0x00092BD2: "SceneScript_092BD2",
    0x000931C4: "SceneTrig_0931C4",
    0x000931D6: "SceneTrig_0931D6",
    0x000931F0: "SceneTrig_0931F0",
    0x0009327E: "SceneTrig_09327E",
    0x0009329C: "SceneTrig_09329C",
    0x000932B2: "SceneTrig_0932B2",
    0x000932BE: "SceneEntities_0932BE",
    0x000932F8: "SceneScript_0932F8",
    0x00093542: "SceneEntities_093542",
    0x0009357C: "SceneScript_09357C",
    0x00093BE6: "SceneTrig_093BE6",
    0x00093BEC: "SceneTrig_093BEC",
    0x00093C0E: "SceneTrig_093C0E",
    0x00093C78: "SceneTrig_093C78",
    0x00093CA0: "SceneEntities_093CA0",
    0x00093CDA: "SceneScript_093CDA",
    0x00093F20: "SceneEntities_093F20",
    0x00093F30: "SceneScript_093F30",
    0x0009411C: "SceneEntities_09411C",
    0x0009412C: "SceneScript_09412C",
    0x00094312: "SceneEntities_094312",
    0x00094322: "SceneScript_094322",
    0x000944D8: "SceneEntities_0944D8",
    0x00094512: "SceneScript_094512",
    0x00095656: "SceneEntities_095656",
    0x00095690: "SceneScript_095690",
    0x00096554: "SceneEntities_096554",
    0x00096580: "SceneScript_096580",
    0x00096772: "SceneEntities_096772",
    0x00096782: "SceneScript_096782",
    0x000967A4: "ChildRank_CmpByte10_0967A4",
    0x00096BBC: "AttractSprites_List0_096BBC",
    0x00096CAE: "AttractSprites_List1_096CAE",
    0x00096FA8: "AttractSprites_List2_096FA8",
    0x0009718A: "AttractSprites_List3_09718A",
    0x000972CC: "AttractSprites_List4_0972CC",
    0x00097422: "AttractSprites_List5_097422",
    0x00097500: "AttractSprites_List6_097500",
    0x000975A2: "AttractSprites_List7_0975A2",
    0x00097720: "ChildRank_CmpByte10_097720",
    # --- Wave BBBBB: RTS internos de islas C
    0x00025920: "SetHandlerRts_025920",  # rts de SetTaskHandler_02591a (+6)
    0x0002599E: "SetHandlerRts_02599e",  # rts de SetTaskHandler_025998 (+6)
    0x00025CC4: "SetHandlerRts_025cc4",  # rts de SetTaskHandler_025cbe (+6)
    0x00025D5A: "SetHandlerRts_025d5a",  # rts de SetTaskHandler_025d54 (+6)
    0x00025DD0: "JsrPcRts_025dd0",  # rts de JsrPcThunk_025dcc (+4)
    0x00025E46: "SetHandlerRts_025e46",  # rts de SetTaskHandler_025e40 (+6)
    0x000266CA: "NopCCRMid_0266ca",  # rts de NopCCR_0266c6 (+4)
    # --- Wave BBBBB: refs forward a huecos futuros
    # 0x00027E7E promovido a CollMap_TestSolidBitC_027e7e en registry (Wave CCCCC).
    # 0x00027E9C promovido a CollMap_TestPlatformBitC_027e9c en registry (Wave CCCCC).
    # 0x0002800E promovido a Entity_ScaleDyForGravity_02800e en registry (Wave CCCCC).
    # 0x00028074 promovido a Entity_ClampXToScreenEdge_028074 en registry (Wave CCCCC).
    # --- Wave CCCCC: RTS internos de islas C
    0x0002831C: "ClearXNMid_02831c",  # rts de ClearXN_028318 (+4)
    0x00028362: "ClearXNMid_028362",  # rts de ClearXN_02835e (+4)
    0x000283A8: "ClearXNMid_0283a8",  # rts de ClearXN_0283a4 (+4)
    0x000294AE: "JsrPcRts_0294ae",  # rts de JsrPcThunk_0294aa (+4)
    # --- Wave DDDDD: RTS internos de islas C
    0x000463B8: "SetHandlerRts_0463b8",  # rts de SetTaskHandler_0463b2 (+6)
    0x000465DC: "SetHandlerRts_0465dc",  # rts de SetTaskHandler_0465d6 (+6)
    0x00046662: "SetHandlerRts_046662",  # rts de SetTaskHandler_04665c (+6)
    0x000466D8: "SetHandlerRts_0466d8",  # rts de SetTaskHandler_0466d2 (+6)
    0x000469E0: "JsrPcRts_0469e0",  # rts de JsrPcThunk_0469dc (+4)
    0x00047038: "SetHandlerRts_047038",  # rts de SetTaskHandler_047032 (+6)
    0x0004704E: "SetHandlerRts_04704e",  # rts de SetTaskHandler_047048 (+6)
    0x00047122: "JsrPcRts_047122",  # rts de JsrPcThunk_04711e (+4)
    0x00047144: "SetHandlerRts_047144",  # rts de SetTaskHandler_04713e (+6)
    0x00047188: "SetHandlerRts_047188",  # rts de SetTaskHandler_047182 (+6)
    0x000471D8: "SetHandlerRts_0471d8",  # rts de SetTaskHandler_0471d2 (+6)
    0x00047276: "SetHandlerRts_047276",  # rts de SetTaskHandler_047270 (+6)
    0x0004728C: "SetHandlerRts_04728c",  # rts de SetTaskHandler_047286 (+6)
    0x000472D0: "SetHandlerRts_0472d0",  # rts de SetTaskHandler_0472ca (+6)
    0x0004731A: "SetHandlerRts_04731a",  # rts de SetTaskHandler_047314 (+6)
    0x00047360: "SetHandlerRts_047360",  # rts de SetTaskHandler_04735a (+6)
    # --- Wave EEEEE: RTS internos de islas C
    0x00059438: "SetHandlerRts_059438",  # rts de SetTaskHandler_059432 (+6)
    0x00059478: "SetHandlerRts_059478",  # rts de SetTaskHandler_059472 (+6)
    0x000594B8: "SetHandlerRts_0594b8",  # rts de SetTaskHandler_0594b2 (+6)
    0x000595AC: "Jsr5B6Rts_0595ac",  # rts de Jsr5B6ThenJmpScheduler_0595a0 (+12)
    0x00059664: "JsrAbsRts_059664",  # rts de JsrAbsThunk_05965e (+6)
    0x00059720: "SetHandlerRts_059720",  # rts de SetTaskHandler_05971a (+6)
    0x00059754: "SetHandlerRts_059754",  # rts de SetTaskHandler_05974e (+6)
    0x000597AE: "SetHandlerRts_0597ae",  # rts de SetTaskHandler_0597a8 (+6)
    0x00059808: "SetHandlerRts_059808",  # rts de SetTaskHandler_059802 (+6)
    0x00059862: "SetHandlerRts_059862",  # rts de SetTaskHandler_05985c (+6)
    0x000598AC: "SetHandlerRts_0598ac",  # rts de SetTaskHandler_0598a6 (+6)
    0x000598FA: "Jsr5B6Rts_0598fa",  # rts de Jsr5B6ThenJmpScheduler_0598ee (+12)
    0x00059948: "SetHandlerRts_059948",  # rts de SetTaskHandler_059942 (+6)
    0x0005996A: "SetHandlerRts_05996a",  # rts de SetTaskHandler_059964 (+6)
    0x00059986: "SetHandlerRts_059986",  # rts de SetTaskHandler_059980 (+6)
    0x000599A8: "SetHandlerRts_0599a8",  # rts de SetTaskHandler_0599a2 (+6)
    0x000599C4: "SetHandlerRts_0599c4",  # rts de SetTaskHandler_0599be (+6)
    0x000599F0: "SetHandlerRts_0599f0",  # rts de SetTaskHandler_0599ea (+6)
    0x00059A18: "SetHandlerRts_059a18",  # rts de SetTaskHandler_059a12 (+6)
    0x00059A3E: "SetHandlerRts_059a3e",  # rts de SetTaskHandler_059a38 (+6)
    0x00059A6E: "SetHandlerRts_059a6e",  # rts de SetTaskHandler_059a68 (+6)
    0x00059B58: "JsrAbsRts_059b58",  # rts de JsrAbsThunk_059b52 (+6)
    0x00059B84: "SetHandlerRts_059b84",  # rts de SetTaskHandler_059b7e (+6)
    0x00059BC4: "SetHandlerRts_059bc4",  # rts de SetTaskHandler_059bbe (+6)
    0x00059C2C: "Jsr5B6Rts_059c2c",  # rts de Jsr5B6ThenJmpScheduler_059c20 (+12)
    0x0005A258: "Jsr5B6Rts_05a258",  # rts de Jsr5B6ThenJmpScheduler_05a24c (+12)
    # --- Wave FFFFF: RTS internos de islas C
    0x00055FC8: "SetHandlerRts_055fc8",  # rts de SetTaskHandler_055fc2 (+6)
    0x00056202: "SetHandlerRts_056202",  # rts de SetTaskHandler_0561fc (+6)
    0x00056278: "Jsr5B6Rts_056278",  # rts de Jsr5B6ThenJmpScheduler_05626c (+12)
    0x00056594: "SetHandlerRts_056594",  # rts de SetTaskHandler_05658e (+6)
    # --- Wave IIIII: refs forward a huecos futuros
    0x00012F30: "Sub_00012F30",  # hueco futuro (ref pc-rel desde esta region)
    # --- Wave JJJJJ: RTS internos de islas C
    0x00001B1A: "SetHandlerRts_001b1a",  # rts de SetTaskHandler_001b14 (+6)
    0x00001B7E: "SetHandlerRts_001b7e",  # rts de SetTaskHandler_001b78 (+6)
    0x00001C42: "SetHandlerRts_001c42",  # rts de SetTaskHandler_001c3c (+6)
    0x00001E08: "JsrAbsRts_001e08",  # rts de JsrAbsThunk_001e02 (+6)
    # --- Wave JJJJJ: refs forward a huecos futuros
    # 0x00002F30 promovido a Data_002f30 en registry (Wave KKKKK).
}
