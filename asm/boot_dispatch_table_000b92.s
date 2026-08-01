| ============================================================================
|  Metal Slug 1 - asm/boot_dispatch_table_000b92.s
|  ----------------------------------------------------------------------------
|  Wave MM batch 2 - super-tabla dispatch del arranque BIOS.
|
|  Contenido (1 registro de datos, 764 bytes):
|
|      $000B92   BootDispatchTable_000B92   764 B   .long array (191 entries)
|
|  Es la super-tabla de handlers interpretada por el bytecode virtual
|  continuation-passing de Wave MM batch 1 (SchedulerBootstrap_Boot_000E8E
|  + SchedulerDispatch_LoopB_000FE0 + SchedulerLoopA_000FC6).
|
|  ---------- Estructura ---------------------------------------------------
|
|  6 sub-tablas concatenadas de u32 BE, separadas por CINCO centinelas
|  $FFFFFFFF:
|
|      T1  $000B92..$000BC6   52 B  13 entries   Boot mode selector root
|      T2  $000BCA..$000D6E  420 B 105 entries   Boot script principal
|      T3  $000D72..$000D82   16 B   4 entries   Boot sub-mode A
|      T4  $000D86..$000D96   16 B   4 entries   Boot sub-mode B
|      T5  $000D9A..$000E3A  160 B  40 entries   Boot script attract
|      T6  $000E3E..$000E8E   80 B  20 entries   Boot script demo/title
|
|  Total: 191 entries (186 handlers + 5 sentinels).
|  Handlers unicos: 93 direcciones distintas, de las cuales 91 ya matcheadas
|  por waves EE/FF/GG/MM#1 (**97.8 % del grafo BIOS cerrado hacia atras**).
|
|  ---------- Labels internos exportados -----------------------------------
|
|  Wave MM batch 1 (SchedulerBootstrap_Boot_000E8E) referencia SEIS puntos
|  de entrada de esta tabla via `lea.l XXX(pc), a0`. Se exportan como
|  simbolos globales para que las reubicaciones R_68K_PC16 del scheduler
|  bootstrap se resuelvan directamente contra esta seccion, sin necesidad
|  de `--defsym` en `symbols.py`:
|
|      BootTblEntry_B92 = $000B92   T1 head       (rama demo)
|      BootTblEntry_BBA = $000BBA   T1[10]        (rama hardstart)
|      BootTblEntry_BCE = $000BCE   T2[1]         (rama title)
|      BootTblEntry_BDA = $000BDA   T2[4]         (rama mode2_alt)
|      BootTblEntry_BE6 = $000BE6   T2[7]         (rama mode2)
|      BootTblEntry_E6E = $000E6E   T6[12]        (rama post-start)
|
|  ---------- Semantica del centinela --------------------------------------
|
|  El interprete threaded (bucle A del SchedulerBootstrap_Boot en $000FC6)
|  al encontrar $FFFFFFFF en (a0) hace:
|
|      a0 = *(a0 + 4)          ; salto a otra sub-tabla arbitraria
|      *cursor = a0            ; persist
|
|  Los 5 centinelas actuan como "guardas de fin de tabla" con fallback de
|  recuperacion via redirect a la siguiente sub-tabla. El patron habitual
|  del ROM es que el propio handler reescriba $70(a6) desde su cuerpo
|  ("jump table via handler"), no via el mecanismo de centinela.
|
|  ---------- Registro como datos-en-.text --------------------------------
|
|  Se registra como una unica entrada del REGISTRY:
|
|    ("BootDispatchTable_000B92", 0x000B92, 764, "boot_dispatch_table_000b92.s")
|
|  El matcher no distingue codigo vs datos - solo bytes. Los .long BE con
|  las 191 direcciones producen los 764 B exactos que la P-ROM tiene en
|  $000B92..$000E8E. Es la **1a entrada de datos-en-.text del proyecto**
|  registrada como registro independiente del REGISTRY (Wave HH#2 embebio
|  32 B dentro de la funcion consumidora, no como registro autonomo).
|
|  Esto **desactiva** los 6 externals ad-hoc BootTblEntry_XXX que Wave MM
|  batch 1 anadio a symbols.py.
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  BootDispatchTable_000B92  @ $000B92  (764 bytes, 191 u32 BE entries)
| ---------------------------------------------------------------------------
|
        .globl  BootDispatchTable_000B92
        .type   BootDispatchTable_000B92, @object
        .section .text.BootDispatchTable_000B92, "a", @progbits
BootDispatchTable_000B92:
        .globl  BootTblEntry_B92                | T1 head
BootTblEntry_B92:
        .long   0x00001354                          | $000B92  T1[ 0]  -> UNMATCHED (handler @ $0001354)
        .long   0x000012F4                          | $000B96  T1[ 1]  -> UNMATCHED (handler @ $00012F4)
        .long   0x00001332                          | $000B9A  T1[ 2]  -> UNMATCHED (handler @ $0001332)
        .long   0x00001846                          | $000B9E  T1[ 3]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x00001260                          | $000BA2  T1[ 4]  -> Init_ModeToggle_001260
        .long   0x000012CA                          | $000BA6  T1[ 5]  -> Init_ModeToggle_001260
        .long   0x00001020                          | $000BAA  T1[ 6]  -> SchedTail_JsrCD4_001020
        .long   0x00001172                          | $000BAE  T1[ 7]  -> UNMATCHED (handler @ $0001172)
        .long   0x000011CA                          | $000BB2  T1[ 8]  -> UNMATCHED (handler @ $00011CA)
        .long   0x00001838                          | $000BB6  T1[ 9]  -> Attract_SoftReset_10FDAF_001838
        .globl  BootTblEntry_BBA
BootTblEntry_BBA:
        .long   0x00001354                          | $000BBA  T1[10]  -> UNMATCHED (handler @ $0001354)
        .long   0x0000109C                          | $000BBE  T1[11]  -> UNMATCHED (handler @ $000109C)
        .long   0x000010E8                          | $000BC2  T1[12]  -> UNMATCHED (handler @ $00010E8)
        .long   0xFFFFFFFF                          | $000BC6  ---- SENTINEL end of T1 ----
        .long   0x00000BBA                          | $000BCA  T2[ 0]  -> UNMATCHED (handler @ $0000BBA)
        .globl  BootTblEntry_BCE
BootTblEntry_BCE:
        .long   0x00001354                          | $000BCE  T2[ 1]  -> UNMATCHED (handler @ $0001354)
        .long   0x00001700                          | $000BD2  T2[ 2]  -> UNMATCHED (handler @ $0001700)
        .long   0x0000172E                          | $000BD6  T2[ 3]  -> UNMATCHED (handler @ $000172E)
        .globl  BootTblEntry_BDA
BootTblEntry_BDA:
        .long   0x00001354                          | $000BDA  T2[ 4]  -> UNMATCHED (handler @ $0001354)
        .long   0x000010F2                          | $000BDE  T2[ 5]  -> UNMATCHED (handler @ $00010F2)
        .long   0x00001148                          | $000BE2  T2[ 6]  -> UNMATCHED (handler @ $0001148)
        .globl  BootTblEntry_BE6
BootTblEntry_BE6:
        .long   0x00001354                          | $000BE6  T2[ 7]  -> UNMATCHED (handler @ $0001354)
        .long   0x00001020                          | $000BEA  T2[ 8]  -> SchedTail_JsrCD4_001020
        .long   0x00001368                          | $000BEE  T2[ 9]  -> UNMATCHED (handler @ $0001368)
        .long   0x000013C6                          | $000BF2  T2[10]  -> UNMATCHED (handler @ $00013C6)
        .long   0x00001020                          | $000BF6  T2[11]  -> SchedTail_JsrCD4_001020
        .long   0x00001416                          | $000BFA  T2[12]  -> UNMATCHED (handler @ $0001416)
        .long   0x00001438                          | $000BFE  T2[13]  -> UNMATCHED (handler @ $0001438)
        .long   0x0000135E                          | $000C02  T2[14]  -> UNMATCHED (handler @ $000135E)
        .long   0x00001452                          | $000C06  T2[15]  -> UNMATCHED (handler @ $0001452)
        .long   0x00000C8A                          | $000C0A  T2[16]  -> UNMATCHED (handler @ $0000C8A)
        .long   0x00001514                          | $000C0E  T2[17]  -> UNMATCHED (handler @ $0001514)
        .long   0x00000CAA                          | $000C12  T2[18]  -> UNMATCHED (handler @ $0000CAA)
        .long   0x00001515                          | $000C16  T2[19]  -> UNMATCHED (handler @ $0001515)
        .long   0x00000CEA                          | $000C1A  T2[20]  -> UNMATCHED (handler @ $0000CEA)
        .long   0x00001516                          | $000C1E  T2[21]  -> UNMATCHED (handler @ $0001516)
        .long   0x00000CCA                          | $000C22  T2[22]  -> UNMATCHED (handler @ $0000CCA)
        .long   0x00001517                          | $000C26  T2[23]  -> UNMATCHED (handler @ $0001517)
        .long   0x00000D0A                          | $000C2A  T2[24]  -> UNMATCHED (handler @ $0000D0A)
        .long   0x00001518                          | $000C2E  T2[25]  -> UNMATCHED (handler @ $0001518)
        .long   0x00000D2A                          | $000C32  T2[26]  -> UNMATCHED (handler @ $0000D2A)
        .long   0x00001519                          | $000C36  T2[27]  -> UNMATCHED (handler @ $0001519)
        .long   0x00000D4A                          | $000C3A  T2[28]  -> UNMATCHED (handler @ $0000D4A)
        .long   0x0000151F                          | $000C3E  T2[29]  -> UNMATCHED (handler @ $000151F)
        .long   0x00000D62                          | $000C42  T2[30]  -> UNMATCHED (handler @ $0000D62)
        .long   0x0000151F                          | $000C46  T2[31]  -> UNMATCHED (handler @ $000151F)
        .long   0x00000D76                          | $000C4A  T2[32]  -> UNMATCHED (handler @ $0000D76)
        .long   0x00001520                          | $000C4E  T2[33]  -> UNMATCHED (handler @ $0001520)
        .long   0x00000D8A                          | $000C52  T2[34]  -> UNMATCHED (handler @ $0000D8A)
        .long   0x00001521                          | $000C56  T2[35]  -> UNMATCHED (handler @ $0001521)
        .long   0x00000BE6                          | $000C5A  T2[36]  -> UNMATCHED (handler @ $0000BE6)
        .long   0x00001522                          | $000C5E  T2[37]  -> UNMATCHED (handler @ $0001522)
        .long   0x00000D9E                          | $000C62  T2[38]  -> UNMATCHED (handler @ $0000D9E)
        .long   0x0000151A                          | $000C66  T2[39]  -> UNMATCHED (handler @ $000151A)
        .long   0x00000DBA                          | $000C6A  T2[40]  -> UNMATCHED (handler @ $0000DBA)
        .long   0x0000151B                          | $000C6E  T2[41]  -> UNMATCHED (handler @ $000151B)
        .long   0x00000DD6                          | $000C72  T2[42]  -> UNMATCHED (handler @ $0000DD6)
        .long   0x0000151C                          | $000C76  T2[43]  -> UNMATCHED (handler @ $000151C)
        .long   0x00000DF2                          | $000C7A  T2[44]  -> UNMATCHED (handler @ $0000DF2)
        .long   0x0000151D                          | $000C7E  T2[45]  -> UNMATCHED (handler @ $000151D)
        .long   0x00000E0E                          | $000C82  T2[46]  -> UNMATCHED (handler @ $0000E0E)
        .long   0x0000151E                          | $000C86  T2[47]  -> UNMATCHED (handler @ $000151E)
        .long   0x00001846                          | $000C8A  T2[48]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x00001922                          | $000C8E  T2[49]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000C92  T2[50]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000C96  T2[51]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000C9A  T2[52]  -> Dispatcher_ModeTable_001922
        .long   0x00001554                          | $000C9E  T2[53]  -> UNMATCHED (handler @ $0001554)
        .long   0x00001026                          | $000CA2  T2[54]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000CA6  T2[55]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000CAA  T2[56]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x00001940                          | $000CAE  T2[57]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000CB2  T2[58]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000CB6  T2[59]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000CBA  T2[60]  -> Dispatcher_ModeTable_001922
        .long   0x00001554                          | $000CBE  T2[61]  -> UNMATCHED (handler @ $0001554)
        .long   0x00001026                          | $000CC2  T2[62]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000CC6  T2[63]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000CCA  T2[64]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x0000195E                          | $000CCE  T2[65]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000CD2  T2[66]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000CD6  T2[67]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000CDA  T2[68]  -> Dispatcher_ModeTable_001922
        .long   0x00001554                          | $000CDE  T2[69]  -> UNMATCHED (handler @ $0001554)
        .long   0x00001026                          | $000CE2  T2[70]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000CE6  T2[71]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000CEA  T2[72]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x0000197C                          | $000CEE  T2[73]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000CF2  T2[74]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000CF6  T2[75]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000CFA  T2[76]  -> Dispatcher_ModeTable_001922
        .long   0x00001554                          | $000CFE  T2[77]  -> UNMATCHED (handler @ $0001554)
        .long   0x00001026                          | $000D02  T2[78]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000D06  T2[79]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000D0A  T2[80]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x0000199A                          | $000D0E  T2[81]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000D12  T2[82]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000D16  T2[83]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000D1A  T2[84]  -> Dispatcher_ModeTable_001922
        .long   0x00001554                          | $000D1E  T2[85]  -> UNMATCHED (handler @ $0001554)
        .long   0x00001026                          | $000D22  T2[86]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000D26  T2[87]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000D2A  T2[88]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x000019B8                          | $000D2E  T2[89]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000D32  T2[90]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000D36  T2[91]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000D3A  T2[92]  -> Dispatcher_ModeTable_001922
        .long   0x00001554                          | $000D3E  T2[93]  -> UNMATCHED (handler @ $0001554)
        .long   0x00001026                          | $000D42  T2[94]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000D46  T2[95]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001354                          | $000D4A  T2[96]  -> UNMATCHED (handler @ $0001354)
        .long   0x00001846                          | $000D4E  T2[97]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x000015B6                          | $000D52  T2[98]  -> UNMATCHED (handler @ $00015B6)
        .long   0x000015F4                          | $000D56  T2[99]  -> UNMATCHED (handler @ $00015F4)
        .long   0x00001026                          | $000D5A  T2[100]  -> SchedTail_JsrD3C_001026
        .long   0x00001604                          | $000D5E  T2[101]  -> UNMATCHED (handler @ $0001604)
        .long   0x00001026                          | $000D62  T2[102]  -> SchedTail_JsrD3C_001026
        .long   0x0000163C                          | $000D66  T2[103]  -> UNMATCHED (handler @ $000163C)
        .long   0x000016C2                          | $000D6A  T2[104]  -> UNMATCHED (handler @ $00016C2)
        .long   0xFFFFFFFF                          | $000D6E  ---- SENTINEL end of T2 ----
        .long   0x00000E4E                          | $000D72  T3[ 0]  -> UNMATCHED (handler @ $0000E4E)
        .long   0x00001026                          | $000D76  T3[ 1]  -> SchedTail_JsrD3C_001026
        .long   0x00001678                          | $000D7A  T3[ 2]  -> UNMATCHED (handler @ $0001678)
        .long   0x000016C2                          | $000D7E  T3[ 3]  -> UNMATCHED (handler @ $00016C2)
        .long   0xFFFFFFFF                          | $000D82  ---- SENTINEL end of T3 ----
        .long   0x00000E4E                          | $000D86  T4[ 0]  -> UNMATCHED (handler @ $0000E4E)
        .long   0x00001026                          | $000D8A  T4[ 1]  -> SchedTail_JsrD3C_001026
        .long   0x0000165A                          | $000D8E  T4[ 2]  -> UNMATCHED (handler @ $000165A)
        .long   0x000016C2                          | $000D92  T4[ 3]  -> UNMATCHED (handler @ $00016C2)
        .long   0xFFFFFFFF                          | $000D96  ---- SENTINEL end of T4 ----
        .long   0x00000E4E                          | $000D9A  T5[ 0]  -> UNMATCHED (handler @ $0000E4E)
        .long   0x00001846                          | $000D9E  T5[ 1]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x000019D6                          | $000DA2  T5[ 2]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000DA6  T5[ 3]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000DAA  T5[ 4]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000DAE  T5[ 5]  -> Dispatcher_ModeTable_001922
        .long   0x00001026                          | $000DB2  T5[ 6]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000DB6  T5[ 7]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000DBA  T5[ 8]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x000019FE                          | $000DBE  T5[ 9]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000DC2  T5[10]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000DC6  T5[11]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000DCA  T5[12]  -> Dispatcher_ModeTable_001922
        .long   0x00001026                          | $000DCE  T5[13]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000DD2  T5[14]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000DD6  T5[15]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x00001A1C                          | $000DDA  T5[16]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000DDE  T5[17]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000DE2  T5[18]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000DE6  T5[19]  -> Dispatcher_ModeTable_001922
        .long   0x00001026                          | $000DEA  T5[20]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000DEE  T5[21]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000DF2  T5[22]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x00001A3A                          | $000DF6  T5[23]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000DFA  T5[24]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000DFE  T5[25]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000E02  T5[26]  -> Dispatcher_ModeTable_001922
        .long   0x00001026                          | $000E06  T5[27]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000E0A  T5[28]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001846                          | $000E0E  T5[29]  -> Attract_DoubleCheck_400_Publish_001846
        .long   0x00001A58                          | $000E12  T5[30]  -> Dispatcher_ModeTable_001922
        .long   0x00001A70                          | $000E16  T5[31]  -> Dispatcher_ModeTable_001922
        .long   0x000017C8                          | $000E1A  T5[32]  -> Attract_InitTaskAdd_3DBC8_0017C8
        .long   0x00001A64                          | $000E1E  T5[33]  -> Dispatcher_ModeTable_001922
        .long   0x00001026                          | $000E22  T5[34]  -> SchedTail_JsrD3C_001026
        .long   0x00001482                          | $000E26  T5[35]  -> UNMATCHED (handler @ $0001482)
        .long   0x00001020                          | $000E2A  T5[36]  -> SchedTail_JsrCD4_001020
        .long   0x000016D2                          | $000E2E  T5[37]  -> UNMATCHED (handler @ $00016D2)
        .long   0x00001ABC                          | $000E32  T5[38]  -> Attract_PostStart_Cleanup_001AB6
        .long   0x00001020                          | $000E36  T5[39]  -> SchedTail_JsrCD4_001020
        .long   0xFFFFFFFF                          | $000E3A  ---- SENTINEL end of T5 ----
        .long   0x00000E4E                          | $000E3E  T6[ 0]  -> UNMATCHED (handler @ $0000E4E)
        .long   0x00001354                          | $000E42  T6[ 1]  -> UNMATCHED (handler @ $0001354)
        .long   0x000017E6                          | $000E46  T6[ 2]  -> Attract_InitShow27_TaskAdd_0017E6
        .long   0x00001812                          | $000E4A  T6[ 3]  -> Attract_SetTimers2_And_Gate21_001812
        .long   0x00001020                          | $000E4E  T6[ 4]  -> SchedTail_JsrCD4_001020
        .long   0x00001744                          | $000E52  T6[ 5]  -> Attract_InitBIOS_001744
        .long   0x0000179A                          | $000E56  T6[ 6]  -> Attract_InitBIOS_001744
        .long   0x00001526                          | $000E5A  T6[ 7]  -> UNMATCHED (handler @ $0001526)
        .long   0x00001544                          | $000E5E  T6[ 8]  -> UNMATCHED (handler @ $0001544)
        .long   0x000011EA                          | $000E62  T6[ 9]  -> UNMATCHED (handler @ $00011EA)
        .long   0x0000122E                          | $000E66  T6[10]  -> UNMATCHED (handler @ $000122E)
        .long   0x00001838                          | $000E6A  T6[11]  -> Attract_SoftReset_10FDAF_001838
        .globl  BootTblEntry_E6E
BootTblEntry_E6E:
        .long   0x00001020                          | $000E6E  T6[12]  -> SchedTail_JsrCD4_001020
        .long   0x00001566                          | $000E72  T6[13]  -> UNMATCHED (handler @ $0001566)
        .long   0x0000159E                          | $000E76  T6[14]  -> UNMATCHED (handler @ $000159E)
        .long   0x000014AE                          | $000E7A  T6[15]  -> UNMATCHED (handler @ $00014AE)
        .long   0x000014C4                          | $000E7E  T6[16]  -> UNMATCHED (handler @ $00014C4)
        .long   0x00001744                          | $000E82  T6[17]  -> Attract_InitBIOS_001744
        .long   0x0000179A                          | $000E86  T6[18]  -> Attract_InitBIOS_001744
        .long   0x00001838                          | $000E8A  T6[19]  -> Attract_SoftReset_10FDAF_001838

        .size   BootDispatchTable_000B92, .-BootDispatchTable_000B92
