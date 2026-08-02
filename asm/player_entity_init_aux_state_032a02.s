| ============================================================================
|  Metal Slug 1 - asm/player_entity_init_aux_state_032a02.s
|  ----------------------------------------------------------------------------
|  Wave QQ#1 - helper local de PlayerEntitySpawn.
|
|  PlayerEntity_InitAuxState_032A02  @ $032A02  (158 bytes, 2 callers)
|
|  ---------- Mapa de callers -----------------------------------------------
|
|      PlayerEntitySpawn_0335A6      (Wave NN#2) -> jsr $32A02(pc)
|      PlayerEntitySpawn_Alt_03364A  (Wave NN#2) -> jsr $32A02(pc)
|
|  Ambos callers son variantes del "constructor de entity" del player que
|  se ejecuta al arrancar una vida nueva / respawnear. Esta funcion es un
|  segundo paso de inicializacion que corren ambas variantes justo despues
|  de cargar la config de nivel ($138FE) y antes de limpiar el flag de
|  pausa y marcar el spawn como activo.
|
|  Que hace (campos de struct Entity en a6, offsets tal como los usa el
|  resto del proyecto -- ver include/mslug.h):
|
|      +14 = +16                    copia word (probablemente "posicion
|                                   anterior" <- "posicion actual", o
|                                   "vida maxima" <- "vida actual")
|      +5b |= 0x04                 set flag (bit 2)
|      +6b |= 0x40                 set flag (bit 6)
|      +60 = &Sub_00032500          instala puntero a handler/tabla en
|                                   +60 (mismo slot que otros "instaladores
|                                   de handler" del proyecto, p.ej. Wave S)
|      +32 = +33 = 0xFF             dos contadores/temporizadores a "sin
|                                   activar" (0xFF suele ser centinela de
|                                   "cooldown inactivo" en este codebase)
|      +38 = 0x8010                 flags de estado: bit15 set + bit4 set
|                                   (bit15 sobreescribe todo el word, bit4
|                                   se OR-ea aparte -- dos escrituras
|                                   separadas en el original, preservado
|                                   tal cual para el match byte-a-byte)
|      jsr $5E98A                   helper no matcheado aun (probable
|                                   reset de fisica/velocidad)
|      +80 = 0x0A                   contador (10 unidades -- podria ser
|                                   "hits para invulnerabilidad" o similar)
|      +82 = 0x0000                 contador/timer a cero
|      +85 = 0x01                   flag activo
|      +8c = +87 = 0                dos flags a cero
|      jsr $8F6D2                   helper no matcheado aun
|      +72 = +36 = 0                dos words a cero (posiblemente
|                                   velocidad/momentum)
|      +91 = +21 = 0                dos flags a cero
|      +71 = +70 = 0                dos bytes a cero (posiblemente
|                                   contadores de combo/input)
|      +78 = 1                      flag activo (distinto del +78 que
|                                   pone PlayerEntitySpawn antes de
|                                   llamar aqui -- se pisa a proposito)
|      +7c = +7e = 0                dos words a cero (posiblemente
|                                   offsets de camara/shake)
|      +13 &= ~0x08                 clear flag (bit 3)
|      jsr $517AA                   helper no matcheado aun
|      lea $776E2, a1; jsr Task_AllocFromFreeList(a1)
|                                   tail-call: encola un template de tarea
|                                   en $776E2 usando el allocator comun
|                                   (idioma "lea tpl,a1; jsr $4AE" repetido
|                                   en todo el proyecto, ver Wave H/S/NN)
|
|  No hay rts explicito: la funcion termina en el jsr $4AE (el propio
|  Task_AllocFromFreeList hace el rts final por el caller). Es el mismo
|  idioma de "instalador de tarea" documentado en otras waves: la ultima
|  instruccion es una jsr, no una rts, porque quien realmente retorna al
|  caller original es la funcion instalada via a1.
|
|  Los tres jsr a helpers no matcheados ($5E98A, $8F6D2, $517AA) quedan
|  con placeholder en tools/symbols.py (Sub_0005E98A, Sub_0008F6D2,
|  Sub_000517AA) para no bloquear este match; son candidatos naturales
|  para la siguiente ola (usar tools/rank_candidates.py).
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  PlayerEntity_InitAuxState_032A02
        .type   PlayerEntity_InitAuxState_032A02, @function
        .section .text.PlayerEntity_InitAuxState_032A02, "ax", @progbits

PlayerEntity_InitAuxState_032A02:
        move.w  0x16(a6), 0x14(a6)      | +000  copia word +16 -> +14
        bset.b  #2, 0x5b(a6)            | +006  set flag +5b bit 2
        bset.b  #6, 0x6b(a6)            | +00c  set flag +6b bit 6
        move.l  #0x32500, 0x60(a6)      | +012  instala handler +60
        move.b  #0xff, 0x32(a6)         | +01a  +32 = 0xFF (cooldown off)
        move.b  #0xff, 0x33(a6)         | +020  +33 = 0xFF (cooldown off)
        move.w  #0x8000, 0x38(a6)       | +026  +38 = 0x8000 (overwrite)
        ori.w   #0x10, 0x38(a6)         | +02c  +38 |= 0x0010
        jsr     Sub_0005E98A            | +032  helper (aun no matcheado)
        move.b  #0xa, 0x80(a6)          | +038  +80 = 10
        move.w  #0x0, 0x82(a6)          | +03e  +82 = 0
        move.b  #0x1, 0x85(a6)          | +044  +85 = 1
        clr.b   0x8c(a6)                | +04a  +8c = 0
        clr.b   0x87(a6)                | +04e  +87 = 0
        jsr     Sub_0008F6D2            | +052  helper (aun no matcheado)
        clr.w   0x72(a6)                | +058  +72 = 0
        clr.w   0x36(a6)                | +05c  +36 = 0
        clr.b   0x91(a6)                | +060  +91 = 0
        clr.b   0x21(a6)                | +064  +21 = 0
        move.b  #0x0, 0x71(a6)          | +068  +71 = 0
        move.b  #0x0, 0x70(a6)          | +06e  +70 = 0
        move.b  #0x1, 0x78(a6)          | +074  +78 = 1
        move.w  #0x0, 0x7c(a6)          | +07a  +7c = 0
        move.w  #0x0, 0x7e(a6)          | +080  +7e = 0
        bclr.b  #3, 0x13(a6)            | +086  clear flag +13 bit 3
        jsr     Sub_000517AA            | +08c  helper (aun no matcheado)
        lea.l   0x776e2.l, a1           | +092  a1 = &TaskTpl_0776E2
        jsr     ThunkTarget_0004ae      | +098  Task_AllocFromFreeList(a1)

        .size   PlayerEntity_InitAuxState_032A02, .-PlayerEntity_InitAuxState_032A02
