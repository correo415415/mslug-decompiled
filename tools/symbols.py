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
    0x00000506: "Task_AllocFail_0506",  # rama 'free-list vacia' de Task_AllocFromFreeList
    0x00077C98: "Handler_077c98",  # handler literal instalado por Entity_InstallHandlerAndCopyXf
    0x000277C4: "Sub_000277C4",  # probe/collision llamado por Entity_ProbeTransformFreeCcr (T#7)
    0x000273FC: "Sub_000273FC",  # probe/collision llamado por Entity_ProbeTransformFreeCcr_027c8c (T#9)
    0x000027444: "Sub_000027444",  # probe/collision compartido por T#11 y T#13
    0x0002773C: "Sub_00002773C",  # probe/collision llamado por T#15
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
    0x00028108: "FUN_00028108",
    0x0005022A: "StateMachineRun",
    0x00099AFC: "FUN_00099afc",
    0x00C004C2: "BIOS_FIX_CLEAR",

    # ---- Wave Y: targets externos referenciados por asm 68000 puro ----
    0x000329EE: "OpcodeOffsetTable_0329EE",  # tabla de 16 word-offsets usada por Entity_DispatchOpcodeNibble (Y#6)
    0x0009B51E: "ScriptTemplate_09B51E",     # template de script/entity usado por Entity_AllocByPlayerSlot (Y#7)
    0x0004CB44: "PtrTable6_04CB44",          # tabla de 6 long-ptr usada por Table_LoadPtrByIdxClamp6 (Y#5)
    # Templates usados por Entity_Build3ChainCircular (Y#10)
    0x0003010C: "Template_03010C",
    0x00030068: "Template_030068",
    0x000300BA: "Template_0300BA",
    # Templates usados por Entity_Build4FromTemplates (Y#11)
    0x0008121C: "Template_08121C",
    0x0008123C: "Template_08123C",
    0x00081260: "Template_081260",
    0x00081284: "Template_081284",

    # ---- Wave Z: externos referenciados por asm 68000 puro ----
    0x0005B1B2: "Sub_0005B1B2",              # jsr pc-rel desde Sprite_Dispatch dual entry
    0x0005AA96: "SpriteDispatchJT_05AA96",   # jump-table target del dispatch dual
    0x00044182: "Sub_00044182",              # colisión llamada por Entity_Probe_02785C
    0x00027036: "Sub_00027036",              # colisión pc-rel de Entity_Probe_02788C
    0x00026B56: "Sub_00026B56",              # colisión pc-rel de Entity_Probe_027A92
    0x00047872: "Sub_00047872",              # callback pc-rel del bucle en Helper_047888
    0x00044022: "Sub_00044022",              # blit setup llamado por Helper_05026C
    0x000523EE: "Template_0523EE",           # template del spawner Helper_05239E
    0x000524AA: "Template_0524AA",           # template del spawner Helper_0523B2

    # ---- Wave Z batch 2: externos referenciados por asm 68000 puro ----
    0x0005E3A2: "Sub_0005E3A2",              # probe llamado por ProbeTwoAttemptsCcr (Z2 #1)
    0x0005E618: "Sub_0005E618",              # confirm llamado por ProbeTwoAttemptsCcr (Z2 #1)
    0x00028A96: "Sub_00028A96",              # helper por-slot de Player_Dispatch3Slots (Z2 #2)
    0x00051862: "Sub_00051862",              # bsr desde Player_StateDispatch (Z2 #3)
    0x00051828: "Sub_00051828",              # bsr post-jt desde Player_StateDispatch
    0x0005188C: "StateJumpTable_05188C",     # jump-table PC-rel de Player_StateDispatch
    0x000272A8: "Sub_000272A8",              # colision PC-rel de Entity_ProbeRevertCcr_027AFC (Z2 #5)
    0x0006DD5C: "Template_06DD5C",           # template del spawner-16x Entity_SpawnLoop16 (Z2 #6)
    0x0006DF32: "Template_06DF32",           # template del spawner Entity_SpawnAndTag (Z2 #7)
    # 0x0006E2BC: se usa PcThunkTarget_06e2bc (ya expuesto abajo, linea ~931).
    #             El nombre canonico historico se conserva; Sub_0006E2BC eliminado.
    0x00047822: "Sub_00047822",              # callback del list-apply $04784C (Z2 #8)
    0x000477D4: "Sub_000477D4",              # callback del list-apply $0477FC (Z2 #9)
    0x00046AC6: "Sub_00046AC6",              # jsr abs.l inicial de Init_JsrThenTailCall (Z2 #12)
    0x00000FE0: "Sub_00000FE0",              # bra.w tail-call de Init_JsrThenTailCall
    0x00046A48: "Template_046A48",           # template del spawner Handler_TimerAndReplace (Z2 #13)
    0x00106F28: "GlobalFlag_106F28",         # flag global chequeada por Handler_ConditionalHitCounter
    0x00001C34: "Sub_00001C34",              # handler continuacion instalado por Handler_TimerAndReplace

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
    0x00000400: "RtsStub_0400",


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
    0x00001B4C: "TaskHandler_001b4c",
    0x00001B70: "TaskHandler_001b70",
    0x00001B80: "TaskHandler_001b80",
    0x000257EC: "TaskHandler_0257ec",
    0x00025882: "TaskHandler_025882",
    0x00025AD8: "TaskHandler_025ad8",
    0x00025B34: "TaskHandler_025b34",
    0x00025D5C: "TaskHandler_025d5c",
    0x00025D64: "TaskHandler_025d64",
    0x0002B05E: "TaskHandler_02b05e",
    0x0002B264: "TaskHandler_02b264",
    0x0002D02E: "TaskHandler_02d02e",
    0x0002DA38: "TaskHandler_02da38",
    0x0002FF86: "TaskHandler_02ff86",
    0x00030BF6: "TaskHandler_030bf6",
    0x00030D74: "TaskHandler_030d74",
    0x000318AC: "TaskHandler_0318ac",
    0x000318D4: "TaskHandler_0318d4",
    0x000321BC: "TaskHandler_0321bc",
    0x00036D64: "TaskHandler_036d64",
    0x00037B8E: "TaskHandler_037b8e",
    0x00037C1A: "TaskHandler_037c1a",
    0x00038CEE: "TaskHandler_038cee",
    0x00038E4A: "TaskHandler_038e4a",
    0x000391AA: "TaskHandler_0391aa",
    0x0003DC16: "TaskHandler_03dc16",
    0x0003DC2C: "TaskHandler_03dc2c",
    0x0003DC74: "TaskHandler_03dc74",
    0x0003DEA8: "TaskHandler_03dea8",
    0x0003DEBE: "TaskHandler_03debe",
    0x0003DEE2: "TaskHandler_03dee2",
    0x0003DF32: "TaskHandler_03df32",
    0x0003DF54: "TaskHandler_03df54",
    0x0003E084: "TaskHandler_03e084",
    0x0003E4E6: "TaskHandler_03e4e6",
    0x0003E50C: "TaskHandler_03e50c",
    0x0003EAA2: "TaskHandler_03eaa2",
    0x0003EB82: "TaskHandler_03eb82",
    0x0003EBF8: "TaskHandler_03ebf8",
    0x0003EC16: "TaskHandler_03ec16",
    0x0003EC8C: "TaskHandler_03ec8c",
    0x0003ECAA: "TaskHandler_03ecaa",
    0x0003FCC0: "TaskHandler_03fcc0",
    0x0003FDD0: "TaskHandler_03fdd0",
    0x0003FE66: "TaskHandler_03fe66",
    0x0004049C: "TaskHandler_04049c",
    0x00040D18: "TaskHandler_040d18",
    0x00040E54: "TaskHandler_040e54",
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
    0x00046664: "TaskHandler_046664",
    0x000466B4: "TaskHandler_0466b4",
    0x000466DA: "TaskHandler_0466da",
    0x0004703A: "TaskHandler_04703a",
    0x00047050: "TaskHandler_047050",
    0x00047146: "TaskHandler_047146",
    0x0004718A: "TaskHandler_04718a",
    0x00047278: "TaskHandler_047278",
    0x0004728E: "TaskHandler_04728e",
    0x000472D2: "TaskHandler_0472d2",
    0x0004731C: "TaskHandler_04731c",
    0x00047362: "TaskHandler_047362",
    0x00048B1E: "TaskHandler_048b1e",
    0x00048B26: "TaskHandler_048b26",
    0x00048DDC: "TaskHandler_048ddc",
    0x00048DEC: "TaskHandler_048dec",
    0x0004968A: "TaskHandler_04968a",
    0x0004A014: "Handler_0004A014",     # handler PC-rel instalado (canal B)
    0x0004A024: "TaskHandler_04a024",
    0x0004A18C: "TaskHandler_04a18c",
    0x0004AC32: "TaskHandler_04ac32",
    0x0004BC48: "TaskHandler_04bc48",
    0x0004C578: "TaskHandler_04c578",
    0x0004C58C: "TaskHandler_04c58c",
    0x0004C606: "TaskHandler_04c606",
    0x0004C68A: "TaskHandler_04c68a",
    0x0004C934: "TaskHandler_04c934",
    0x0004DC88: "TaskHandler_04dc88",
    0x0004DE32: "TaskHandler_04de32",
    0x0004EB94: "TaskHandler_04eb94",
    0x0004F2A4: "TaskHandler_04f2a4",
    0x00050976: "TaskHandler_050976",
    0x00051452: "TaskHandler_051452",
    0x0005147E: "TaskHandler_05147e",
    0x00052514: "TaskHandler_052514",
    0x000526AA: "TaskHandler_0526aa",
    0x00053C5C: "TaskHandler_053c5c",
    0x00053C64: "TaskHandler_053c64",
    0x00056058: "TaskHandler_056058",
    0x00056204: "TaskHandler_056204",
    0x00056596: "TaskHandler_056596",
    0x00057F4E: "TaskHandler_057f4e",
    0x00058412: "TaskHandler_058412",
    0x00058B1E: "TaskHandler_058b1e",
    0x00058C8E: "TaskHandler_058c8e",
    0x00058CCE: "TaskHandler_058cce",
    0x0005943A: "TaskHandler_05943a",
    0x0005947A: "TaskHandler_05947a",
    0x000594BA: "TaskHandler_0594ba",
    0x00059722: "TaskHandler_059722",
    0x00059756: "TaskHandler_059756",
    0x000597B0: "TaskHandler_0597b0",
    0x0005980A: "TaskHandler_05980a",
    0x00059864: "TaskHandler_059864",
    0x000598AE: "TaskHandler_0598ae",
    0x0005994A: "TaskHandler_05994a",
    0x0005996C: "TaskHandler_05996c",
    0x00059988: "TaskHandler_059988",
    0x000599AA: "TaskHandler_0599aa",
    0x000599C6: "TaskHandler_0599c6",
    0x000599F2: "TaskHandler_0599f2",
    0x00059A1A: "TaskHandler_059a1a",
    0x00059A40: "TaskHandler_059a40",
    0x00059A70: "TaskHandler_059a70",
    0x00059B86: "TaskHandler_059b86",
    0x00059BC6: "TaskHandler_059bc6",
    0x00059C42: "TaskHandler_059c42",
    0x00059D62: "TaskHandler_059d62",
    0x0005A28A: "TaskHandler_05a28a",
    0x0005A66E: "TaskHandler_05a66e",
    0x0005A764: "TaskHandler_05a764",
    0x0005CBEA: "TaskHandler_05cbea",
    0x0005F00A: "TaskHandler_05f00a",
    0x0005F0B0: "TaskHandler_05f0b0",
    0x0005F482: "TaskHandler_05f482",
    0x0005FA56: "TaskHandler_05fa56",
    0x0005FB88: "TaskHandler_05fb88",
    0x0005FBE6: "TaskHandler_05fbe6",
    0x0005FCE6: "TaskHandler_05fce6",
    0x000606EE: "TaskHandler_0606ee",
    0x00060BF6: "TaskHandler_060bf6",
    0x00060C40: "TaskHandler_060c40",
    0x00060D3A: "TaskHandler_060d3a",
    0x00060DE8: "TaskHandler_060de8",
    0x000620A0: "TaskHandler_0620a0",
    0x000620A8: "TaskHandler_0620a8",
    0x00062F8C: "TaskHandler_062f8c",
    0x00062F9A: "TaskHandler_062f9a",
    0x00063942: "TaskHandler_063942",
    0x00063952: "TaskHandler_063952",
    0x00064222: "TaskHandler_064222",
    0x0006422A: "TaskHandler_06422a",
    0x00064D7A: "TaskHandler_064d7a",
    0x00064D8A: "TaskHandler_064d8a",
    0x0006515E: "TaskHandler_06515e",
    0x000667A4: "TaskHandler_0667a4",
    0x00066A86: "TaskHandler_066a86",
    0x00066A8E: "TaskHandler_066a8e",
    0x00067B7C: "TaskHandler_067b7c",
    0x00067B84: "TaskHandler_067b84",
    0x00068310: "TaskHandler_068310",
    0x00068346: "TaskHandler_068346",
    0x0006850E: "TaskHandler_06850e",
    0x00068684: "TaskHandler_068684",
    0x000688F8: "TaskHandler_0688f8",
    0x0006895C: "TaskHandler_06895c",
    0x0006986A: "TaskHandler_06986a",
    0x00069872: "TaskHandler_069872",
    0x0006A41C: "TaskHandler_06a41c",
    0x0006A452: "TaskHandler_06a452",
    0x0006A468: "TaskHandler_06a468",
    0x0006A93E: "TaskHandler_06a93e",
    0x0006ACAA: "TaskHandler_06acaa",
    0x0006C4C6: "TaskHandler_06c4c6",
    0x0006C53E: "TaskHandler_06c53e",
    0x0006C554: "TaskHandler_06c554",
    0x0006DA7A: "TaskHandler_06da7a",
    0x0006DA90: "TaskHandler_06da90",
    0x0006E062: "TaskHandler_06e062",
    0x0006E80A: "TaskHandler_06e80a",
    0x0006E818: "TaskHandler_06e818",
    0x0006E88C: "TaskHandler_06e88c",
    0x0006E932: "TaskHandler_06e932",
    0x0006F340: "TaskHandler_06f340",
    0x0006F38C: "TaskHandler_06f38c",
    0x0006F9CA: "TaskHandler_06f9ca",
    0x0006FA60: "TaskHandler_06fa60",
    0x0006FB02: "TaskHandler_06fb02",
    0x000700A6: "TaskHandler_0700a6",
    0x00070290: "TaskHandler_070290",
    0x000704F2: "TaskHandler_0704f2",
    0x00070694: "TaskHandler_070694",
    0x0007079E: "TaskHandler_07079e",
    0x000707C8: "TaskHandler_0707c8",
    0x000713A2: "TaskHandler_0713a2",
    0x000716B6: "TaskHandler_0716b6",
    0x000716E2: "TaskHandler_0716e2",
    0x000716EA: "TaskHandler_0716ea",
    0x000716F2: "TaskHandler_0716f2",
    0x00071B9A: "TaskHandler_071b9a",
    0x000724D4: "TaskHandler_0724d4",
    0x00073454: "TaskHandler_073454",
    0x000734F4: "TaskHandler_0734f4",
    0x000734FC: "TaskHandler_0734fc",
    0x000735A6: "TaskHandler_0735a6",
    0x000738DA: "TaskHandler_0738da",
    0x00073B62: "TaskHandler_073b62",
    0x00073F9A: "TaskHandler_073f9a",
    0x000751A6: "TaskHandler_0751a6",
    0x0007525C: "TaskHandler_07525c",
    0x00076056: "TaskHandler_076056",
    0x000760A8: "TaskHandler_0760a8",
    0x000760C8: "TaskHandler_0760c8",
    0x00076290: "TaskHandler_076290",
    0x0007646E: "TaskHandler_07646e",
    0x0007692C: "TaskHandler_07692c",
    0x00076A90: "TaskHandler_076a90",
    0x00076B3A: "TaskHandler_076b3a",
    0x00076BD0: "TaskHandler_076bd0",
    0x0007726A: "TaskHandler_07726a",
    0x00077A8E: "TaskHandler_077a8e",
    0x00079326: "TaskHandler_079326",
    0x00079A8A: "TaskHandler_079a8a",
    0x00079B42: "TaskHandler_079b42",
    0x00079C3C: "TaskHandler_079c3c",
    0x00079C68: "TaskHandler_079c68",
    0x00079CAA: "TaskHandler_079caa",
    0x00079D76: "TaskHandler_079d76",
    0x00079E68: "TaskHandler_079e68",
    0x00079E94: "TaskHandler_079e94",
    0x00079F6A: "TaskHandler_079f6a",
    0x0007A00A: "TaskHandler_07a00a",
    0x0007A1C0: "TaskHandler_07a1c0",
    0x0007A3E6: "TaskHandler_07a3e6",
    0x0007A3EE: "TaskHandler_07a3ee",
    0x0007AA78: "TaskHandler_07aa78",
    0x0007AA94: "TaskHandler_07aa94",
    0x0007ABF4: "TaskHandler_07abf4",
    0x0007AC4A: "TaskHandler_07ac4a",
    0x0007BACE: "TaskHandler_07bace",
    0x0007BB1A: "TaskHandler_07bb1a",
    0x0007C106: "TaskHandler_07c106",
    0x0007C374: "TaskHandler_07c374",
    0x0007C424: "TaskHandler_07c424",
    0x0007C644: "TaskHandler_07c644",
    0x0007C65C: "TaskHandler_07c65c",
    0x0007CF5C: "TaskHandler_07cf5c",
    0x0007D898: "TaskHandler_07d898",
    0x0007DBA2: "TaskHandler_07dba2",
    0x0007DBA8: "TaskHandler_07dba8",
    0x0007DCB6: "TaskHandler_07dcb6",
    0x0007E078: "TaskHandler_07e078",
    0x0007E08C: "TaskHandler_07e08c",
    0x0007E674: "TaskHandler_07e674",
    0x0007E880: "TaskHandler_07e880",
    0x0007EBE6: "TaskHandler_07ebe6",
    0x0007F022: "TaskHandler_07f022",
    0x0007F186: "TaskHandler_07f186",
    0x0007F282: "TaskHandler_07f282",
    0x0007F2CA: "TaskHandler_07f2ca",
    0x0007F87C: "TaskHandler_07f87c",
    0x0007FDB6: "TaskHandler_07fdb6",
    0x0007FDBC: "TaskHandler_07fdbc",
    0x00080382: "TaskHandler_080382",
    0x000803D2: "TaskHandler_0803d2",
    0x000803E8: "TaskHandler_0803e8",
    0x00080454: "TaskHandler_080454",
    0x00080508: "TaskHandler_080508",
    0x000805EE: "TaskHandler_0805ee",
    0x00081018: "TaskHandler_081018",
    0x000811CC: "TaskHandler_0811cc",
    0x000811E4: "TaskHandler_0811e4",
    0x00081214: "TaskHandler_081214",
    0x000812BE: "TaskHandler_0812be",
    0x00081BEE: "TaskHandler_081bee",
    0x00081D64: "TaskHandler_081d64",
    0x00081FAC: "TaskHandler_081fac",
    0x00081FF0: "TaskHandler_081ff0",
    0x00082388: "TaskHandler_082388",
    0x00082456: "TaskHandler_082456",
    0x00082464: "TaskHandler_082464",
    0x00084410: "TaskHandler_084410",
    0x000844C0: "TaskHandler_0844c0",
    0x0008450C: "TaskHandler_08450c",
    0x0008489A: "TaskHandler_08489a",
    0x000848DE: "TaskHandler_0848de",
    0x00084B24: "TaskHandler_084b24",
    0x00084B9A: "TaskHandler_084b9a",
    0x00084BD2: "TaskHandler_084bd2",
    0x00084C26: "TaskHandler_084c26",
    0x00084F5E: "TaskHandler_084f5e",
    0x00084FCA: "TaskHandler_084fca",
    0x00085134: "TaskHandler_085134",
    0x00085484: "TaskHandler_085484",
    0x00085A08: "TaskHandler_085a08",
    0x000865BE: "TaskHandler_0865be",
    0x00086854: "TaskHandler_086854",
    0x00089398: "TaskHandler_089398",
    0x00089504: "TaskHandler_089504",
    0x000895CC: "TaskHandler_0895cc",
    0x000898D4: "TaskHandler_0898d4",
    0x00089960: "TaskHandler_089960",
    0x00089A04: "TaskHandler_089a04",
    0x0008A31C: "TaskHandler_08a31c",
    0x0008A44C: "TaskHandler_08a44c",
    0x0008A516: "TaskHandler_08a516",
    0x0008A5C8: "TaskHandler_08a5c8",
    0x0008A9B0: "TaskHandler_08a9b0",
    0x0008AAF2: "TaskHandler_08aaf2",
    0x0008ABD4: "TaskHandler_08abd4",
    0x0008AC92: "TaskHandler_08ac92",
    0x0008AE38: "TaskHandler_08ae38",
    0x0008AF68: "TaskHandler_08af68",
    0x0008B03C: "TaskHandler_08b03c",
    0x0008B10A: "TaskHandler_08b10a",
    0x0008BB84: "TaskHandler_08bb84",
    0x0008C678: "TaskHandler_08c678",
    0x0008C8FA: "TaskHandler_08c8fa",
    0x0008CF22: "TaskHandler_08cf22",
    0x0008CF6C: "TaskHandler_08cf6c",
    0x0008CFB6: "TaskHandler_08cfb6",
    0x0008D000: "TaskHandler_08d000",
    0x0008D04A: "TaskHandler_08d04a",
    0x0008D094: "TaskHandler_08d094",
    0x0008D41C: "TaskHandler_08d41c",
    0x0008D450: "TaskHandler_08d450",
    0x0008D472: "TaskHandler_08d472",
    0x0008D4C6: "TaskHandler_08d4c6",
    0x0008D4FE: "TaskHandler_08d4fe",
    0x0008D55C: "TaskHandler_08d55c",
    0x0008D580: "TaskHandler_08d580",
    0x0008D5A8: "TaskHandler_08d5a8",
    0x0008D62E: "TaskHandler_08d62e",
    0x0008D72A: "TaskHandler_08d72a",
    0x0008D774: "TaskHandler_08d774",
    0x0008D7BE: "TaskHandler_08d7be",
    0x0008D8F4: "TaskHandler_08d8f4",
    0x0008DB7A: "TaskHandler_08db7a",
    0x0008DBE2: "TaskHandler_08dbe2",
    0x0008DF72: "TaskHandler_08df72",
    0x0008DFB8: "TaskHandler_08dfb8",
    0x0008E288: "TaskHandler_08e288",
    0x0008E2F0: "TaskHandler_08e2f0",
    0x0008E4FE: "TaskHandler_08e4fe",
    0x0008E566: "TaskHandler_08e566",
    0x0008E622: "TaskHandler_08e622",
    0x0008E716: "TaskHandler_08e716",
    0x0008EA16: "TaskHandler_08ea16",
    0x0008EB02: "TaskHandler_08eb02",
    0x0008F96A: "TaskHandler_08f96a",
    0x0008FCCA: "TaskHandler_08fcca",
    0x0008FD2E: "TaskHandler_08fd2e",
    0x0008FD68: "TaskHandler_08fd68",
    0x0008FDAA: "TaskHandler_08fdaa",
    0x0008FDF8: "TaskHandler_08fdf8",
    0x0008FE32: "TaskHandler_08fe32",
    0x0008FE9C: "TaskHandler_08fe9c",
    0x0008FEB6: "TaskHandler_08feb6",
    0x0008FECA: "TaskHandler_08feca",
    0x00090098: "TaskHandler_090098",
    0x00090E7E: "TaskHandler_090e7e",
    0x00091338: "TaskHandler_091338",
    0x000913AA: "TaskHandler_0913aa",
    0x00091514: "TaskHandler_091514",
    0x0009152C: "TaskHandler_09152c",
    0x00091558: "TaskHandler_091558",
    0x000916C0: "TaskHandler_0916c0",
    0x00097852: "TaskHandler_097852",
    0x0009788C: "TaskHandler_09788c",
    0x000978AC: "TaskHandler_0978ac",
    0x000978FA: "TaskHandler_0978fa",
    0x0009792E: "TaskHandler_09792e",
    0x0009794A: "TaskHandler_09794a",
    0x0009806A: "TaskHandler_09806a",
    0x00098308: "TaskHandler_098308",
    0x000983F6: "TaskHandler_0983f6",
    0x00098482: "TaskHandler_098482",
    0x00098836: "TaskHandler_098836",
    0x00098886: "TaskHandler_098886",
    0x0009890C: "TaskHandler_09890c",
    0x000989E0: "TaskHandler_0989e0",
    0x00098AFE: "TaskHandler_098afe",
    0x00098C00: "TaskHandler_098c00",
    0x00098DC6: "TaskHandler_098dc6",
    0x00099004: "TaskHandler_099004",
    0x00099180: "TaskHandler_099180",
    0x0009921A: "TaskHandler_09921a",
    0x000993A2: "TaskHandler_0993a2",
    0x0009953E: "TaskHandler_09953e",
    0x00099610: "TaskHandler_099610",
    0x0009976A: "TaskHandler_09976a",
    0x00099794: "TaskHandler_099794",
    0x00099A64: "TaskHandler_099a64",
    0x0009A280: "TaskHandler_09a280",
    0x0009A2B8: "TaskHandler_09a2b8",
    0x0009B47C: "TaskHandler_09b47c",
    0x0018D74E: "TaskHandler_18d74e",

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
    0x0005180C: "ThunkTarget_05180c",
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
    0x0005CCC8: "ThunkTarget_05ccc8",
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
    0x00001AF8: "PcThunkTarget_001af8",
    0x0001399C: "PcThunkTarget_01399c",
    0x00025E74: "PcThunkTarget_025e74",
    0x000281C8: "PcThunkTarget_0281c8",
    0x0002870A: "JmpTarget_02870a",
    0x00028758: "JmpTarget_028758",
    0x0002A46C: "PcThunkTarget_02a46c",
    0x0002AB86: "PcThunkTarget_02ab86",
    0x0002AC4C: "PcThunkTarget_02ac4c",
    0x0002AC80: "PcThunkTarget_02ac80",
    0x0002FADA: "PcThunkTarget_02fada",
    0x00032EA4: "PcThunkTarget_032ea4",
    0x00032EBA: "PcThunkTarget_032eba",
    0x00032F3C: "PcThunkTarget_032f3c",
    0x00032F88: "PcThunkTarget_032f88",
    # 0x000334A2 promovido a Probe_Bit3At100001_0334A2 en registry (Wave NN#1).
    # El alias antiguo se retira: los callers que hacian bsr.w PcThunkTarget_0334a2
    # ahora resuelven al simbolo canonico definido en el .text de la nueva Wave.
    # 0x000334A2: "PcThunkTarget_0334a2",
    0x00033522: "PcThunkTarget_033522",
    0x00036DCA: "PcThunkTarget_036dca",
    0x00039416: "PcThunkTarget_039416",
    0x0003E7A6: "PcThunkTarget_03e7a6",
    0x0003E84C: "PcThunkTarget_03e84c",
    0x0003EE48: "JmpTarget_03ee48",
    0x00041C1A: "PcThunkTarget_041c1a",
    0x00041DDC: "PcThunkTarget_041ddc",
    0x00041E02: "PcThunkTarget_041e02",
    0x00041FF6: "PcThunkTarget_041ff6",
    0x00042040: "PcThunkTarget_042040",
    0x0004698C: "PcThunkTarget_04698c",
    0x0004707E: "PcThunkTarget_04707e",
    0x0004FAF8: "PcThunkTarget_04faf8",
    0x00053DCA: "PcThunkTarget_053dca",
    0x00055148: "PcThunkTarget_055148",
    0x00055214: "PcThunkTarget_055214",
    0x00056E1E: "PcThunkTarget_056e1e",
    0x00057226: "JmpTarget_057226",
    0x0005CDA8: "JmpTarget_05cda8",
    0x0005CEF8: "JmpTarget_05cef8",
    0x0005CF04: "JmpTarget_05cf04",
    0x0005CF6C: "PcThunkTarget_05cf6c",
    0x0005DBC2: "PcThunkTarget_05dbc2",
    0x0005DD5C: "PcThunkTarget_05dd5c",
    0x0005E018: "PcThunkTarget_05e018",
    0x0005E530: "PcThunkTarget_05e530",
    0x00063336: "PcThunkTarget_063336",
    0x000634F6: "PcThunkTarget_0634f6",
    0x00065C94: "PcThunkTarget_065c94",
    0x0006896A: "PcThunkTarget_06896a",
    0x00068AB8: "PcThunkTarget_068ab8",
    0x0006D13C: "PcThunkTarget_06d13c",
    0x0006E2BC: "PcThunkTarget_06e2bc",
    0x00070AB0: "PcThunkTarget_070ab0",
    0x00072A94: "PcThunkTarget_072a94",
    0x00072C98: "PcThunkTarget_072c98",
    0x00074166: "PcThunkTarget_074166",
    0x000745E2: "PcThunkTarget_0745e2",
    0x000798AC: "PcThunkTarget_0798ac",
    0x00088438: "PcThunkTarget_088438",
    0x0008846A: "PcThunkTarget_08846a",
    0x0008B82C: "PcThunkTarget_08b82c",
    0x0008D804: "PcThunkTarget_08d804",
    0x0008EA50: "PcThunkTarget_08ea50",
    0x0008EFB0: "PcThunkTarget_08efb0",
    0x00097A60: "PcThunkTarget_097a60",
    0x00097A72: "PcThunkTarget_097a72",
    0x00097C5C: "PcThunkTarget_097c5c",
    0x00099DE4: "PcThunkTarget_099de4",
    0x00099E14: "PcThunkTarget_099e14",
    0x00099E9C: "PcThunkTarget_099e9c",
    0x00099EE4: "PcThunkTarget_099ee4",
    0x00099F3A: "PcThunkTarget_099f3a",
    0x00099FD2: "PcThunkTarget_099fd2",
    0x00099FF2: "PcThunkTarget_099ff2",
    0x0009A03C: "PcThunkTarget_09a03c",
    0x0009A086: "PcThunkTarget_09a086",
    0x000283EC: "Sub_0002_83EC",  # destino del bne.w fall-through de Entity_ProbeSlot4c_0283D8 (Wave V#2)
    # ---- Wave W: destinos externos de Entity_AllocSpriteSlot_00236E ----
    0x000029A6: "Rts_shared_29A6",     # rts compartido (usado por 3 branches del validador)
    0x000029A8: "Entity_ProbeSpriteSlot_29A8", # sub-prologo compartido llamado con jsr $29a8(pc)
    0x0005D71C: "HEX_TABLE_5D71C",     # tabla ASCII "0123456789ABCDEF" compartida por el cluster hex-formatter (W#3, W#4, W#5)
    0x00009A7CC: "Sub_00009A7CC",     # movement probe llamado por Entity_ProbeMoveX_09A7AA (retorna Carry)
    0x0005A9E6: "Sprite_Blit_5A9E6",  # backend estandar del cluster Sprite_Dispatch_05CA2A (W#13)
    0x0000076A: "EmptyEntity_Init_00076A",  # dummy entity trampoline al que salta el brazo empty de Entity_AllocFromFreeList_0006FE (W#16)
    0x0005D8F2: "Sub_00005D8F2",     # helper "prep VRAM/params" llamado por Debug_DrawHUDVars_096A80 (X#1) entre andi.l y jsr a W#3
    0x0005D904: "Sub_BinToDecimalDecoder_05D904",  # tail-call desde Decimal_Clamp99999999_05D8F2 (X#2): bin-to-BCD 8-nibble decoder
    0x0005D944: "Trap15_DivByZero_05D944",  # brazo d1==0 de Sub_LongDivide_05D920 (X#4): TRAP #15 halt sistema
    0x00002BC4: "Sub_00002BC4",        # release slot idx, llamado por Entity_FlushSlotHistory_013600 (W#9)
    0x00005E4CA: "Sub_00005E4CA",      # helper local (RNG?), llamado por Entity_ReserveAndSetPos_05E4B2 (W#10)
    # ---- Wave V (continuacion): destinos externos de los helpers 049FD0 / 0799DE ---
    0x00049FBA: "Sub_00049FBA",         # probe local llamado por Entity_ProbeAndInstallHandler_049FD0
    # 0x00027EBA promovido a SpritePubEffect_027EBA en registry (Wave NN#1).
    # Los callers via jsr $27EBA.l se resuelven al simbolo canonico del .text.
    # 0x00027EBA: "Sub_00027EBA",         # probe global llamado por Entity_ProbeAndInstallHandler_049FD0
    0x0004A034: "Handler_0004A034",     # handler PC-rel instalado (canal A)
    0x000799A4: "Sub_0007_99A4",        # subindice usado por Tbl_Decode2D_0799DE
    0x00079A0E: "Tbl_DecodeShort_079A0E", # rama "tabla corta" (magic==2)
    0x0028D876: "JmpTarget_28d876",

    # ---- Wave EE batch 1: labels/thunks internos del cluster $001260..$001AB4
    0x00001260: "Init_ModeToggle_001260",
    0x0000188A: "Attract_WaitStateBackbone_00188A",             # cabecera "wait state" comun (target de bra.w)
    0x000018DA: "Init_EntitySpawn_0018DA",
    0x00001922: "Dispatcher_ModeTable_001922",
    0x00001940: "Label_001940",             # submodo A continuation
    0x0000199A: "Label_00199A",             # submodo B continuation
    0x00001C88: "PcThunkTarget_001C88",     # ver: destino de lea pc+d,a1 desde $001286
    0x00001CD4: "PcThunkTarget_001CD4",     # destino jsr pc+d desde $1286 y $12E6
    0x00001DCC: "PcThunkTarget_001DCC",     # destino jsr pc+d desde $18b4
    0x00001DB8: "Sub_00001DB8",             # callee bsr.w desde $12CA y $18DE
    0x00001E0A: "Sub_00001E0A",             # callee bsr.w desde $17E6 y $1A9E
    0x00024FEC: "Sub_00024FEC",             # callee jsr abs.l x3 en $1260
    0x0002A24A: "Sub_0002A24A",             # callee jsr abs.l en $18DA
    0x00000FC6: "Sub_00000FC6",             # tail target (bra.w) desde $1AA6

    # ---- Wave MM batch 1: externals del scheduler bootstrap $000E8E
    #      Todos son destinos de jsr/bsr desde SchedulerBootstrap_Boot_000E8E
    #      y AttractHandler_10002C. $52712 NO se anade aqui: fue promovido
    #      a simbolo canonico Pubcleaner_10A2Cx_052712 en Wave LL#1.
    #      $24FEC y $46AC6 ya existen (arriba/abajo).
    0x00001D3C: "PcThunkTarget_001D3C",     # SchedTail_JsrD3C_001026 -> bsr.w $1D3C
    0x00001DA4: "Sub_00001DA4",             # SchedulerBootstrap_Boot -> bsr.w $1DA4
    0x00001E1C: "PcThunkTarget_001E1C",     # SchedulerBootstrap_Boot -> jsr pc+d $1E1C
    0x0005CACE: "Sub_0005CACE",             # SchedulerBootstrap_Boot -> jsr abs.l $5CACE
    0x0005E998: "Sub_0005E998",             # SchedulerBootstrap_Boot -> jsr abs.l $5E998 (x2)
    0x00098720: "TaskTpl_098720",           # AttractHandler_10002C -> lea abs.l $98720, a1

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
    0x000913AC: "TaskTpl_0913AC",           # AttractHandler_2Task_0010F2 (task 1)
    0x00099B06: "TaskTpl_099B06",           # AttractHandler_2Task_0010F2 (task 2)
    0x000977D6: "TaskTpl_0977D6",           # AttractHandler_Frame_001172
    0x000977EA: "TaskTpl_0977EA",           # AttractHandler_Loader_0011EA
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
    0x00027DB2: "Sub_00027DB2",             # helper coord/probe (pc-rel)
    0x00027E28: "Sub_00027E28",             # probe rect/rect CCR-C (pc-rel)
    0x0009993C: "Sub_0009993C",             # publica effect_id en d6
    0x00278BA8: "Data_00278BA8",            # array de configs de effect (data)
    #      Etiqueta fin-de-funcion para Probe_Bit3At100001_0334A2: el beq.w
    #      inicial salta al primer byte JUSTO DESPUES del rts (idioma
    #      "salida por el borde"). El linker resuelve a $0334C6.
    0x000334C6: "Probe_Bit3At100001_End",

    # ---- Wave NN batch 2: externals del dispatcher + spawn constructor.
    #      LUT de 2 punteros en $3349A (P1/P2) referenciada por lea pc-rel:
    0x0003349A: "PlayerStateLUT_03349A",     # LUT de 2 ptrs (tabla P1/P2)
    #      Callees pc-rel del spawn constructor:
    0x00032A02: "Sub_00032A02",             # helper local (pc-rel)
    0x00032FF2: "Sub_00032FF2",             # post-init hook 1 (pc-rel)
    0x00032AA8: "Sub_00032AA8",             # post-init hook 3 (pc-rel)
    #      Callees abs.l del spawn constructor:
    0x000394A8: "TaskTpl_0394A8",           # task template para player spawn
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
    0x00046682: "TaskHandler_00046682",
    0x00059B6A: "TaskHandler_00059B6A",
    0x00002B58: "Sub_00002B58",
    0x000009B4: "Sub_000009B4",
    0x00002352: "InputGuardCall219c",
    0x00001C44: "TaskHandler_001C44",
    0x0003DBC8: "TaskHandler_0003DBC8",
    0x00046608: "TaskHandler_00046608",
    0x00000F76: "PcThunkTarget_000F76",
    0x0005D288: "Sub_0005D288",

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
    0x00051ABE: "Entity_AllocAndInit_051ABE",
    0x0007707C: "Subsystem_HudInit_07707C",
    0x0008F158: "Subsystem_AudioSceneInit_08F158",
    0x0003EE3A: "Subsystem_ScoresInit_03EE3A",
    0x000997B8: "Subsystem_AttractHookInit_997B8",
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
    0x00001F4A: "Fn_00001F4A",
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
    0x00051D84: "Fn_00051D84",
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
    0x000022C8: "Sub_000022C8",
    0x00028CD4: "Sub_00028CD4",
    0x00002308: "Sub_00002308",
    0x0008BC74: "Sub_0008BC74",
    0x0008C2B8: "Sub_0008C2B8",
    0x0008C322: "Sub_0008C322",
    0x0008C37E: "Sub_0008C37E",
    0x0008C3DA: "Sub_0008C3DA",
    0x0008C436: "Sub_0008C436",
    0x0008C5B2: "Sub_0008C5B2",
}
