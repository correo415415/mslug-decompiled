| ============================================================================
|  Metal Slug 1 - asm/init_master_subsystems_0020e2.s
|  ----------------------------------------------------------------------------
|  Wave Y - #8  (arranque post-BIOS)
|
|  Init_MasterSubsystems_0020E2  @ $0020E2  (68 bytes, 1 caller)
|
|  Secuencia canonica de arranque del juego tras la inicializacion BIOS.
|  Invoca CINCO subsistemas hijos en orden fijo, con un watchdog kick al
|  puerto Neo Geo $300001 INTERCALADO entre cada llamada (tecnica clasica
|  para que la MMU del watchdog no rearme el sistema durante las fases
|  de init largas de cada subsistema).
|
|  La secuencia termina con una llamada final a Global_SetDualFlagFrom10FD82
|  ($000FFE, Y#4), sincronizando las dos flags gemelas $1081BF/$1081C0.
|
|  Firma C conceptual:
|
|      /* Fase master de inicializacion post-BIOS. Ejecuta los 5 subsistemas
|       * base del juego (por identificar semanticamente cada uno) con
|       * watchdog kick entre llamadas y publica el par de flags globales
|       * al final. */
|      void Init_MasterSubsystems(uint8_t watchdog_seed /*d0*/);
|
|  Subsistemas invocados (por identificar):
|      $01379A : Sub_01379A - subsistema #1
|      $013D12 : Sub_013D12 - subsistema #2
|      $05A824 : Sub_05A824 - subsistema #3
|      $000410 : Sub_000410 - subsistema #4
|      $05CC0E : Sub_05CC0E - subsistema #5
|      $000FFE : Global_SetDualFlagFrom10FD82 (Y#4)  - publicacion final
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. El patron `move.b d0, $300001.l` entre CADA jsr es un watchdog kick
|       del Neo Geo. GCC nunca intercalaria una escritura absoluta al mismo
|       puerto entre llamadas independientes.
|    2. Los cinco jsr abs.l tienen todos forma larga (4E B9 xx xx xx xx),
|       incluida la llamada a $000410 cuyo target cabria perfectamente en
|       abs.w corto (16-bit con signo). El asm original NO optimiza:
|       homogeneidad de codificacion como decision de estilo, ademas de
|       coherencia byte-a-byte para el matcher.
|    3. La secuencia termina con `jsr $ffe.l; rts` en vez de `jmp $ffe.l`
|       (tail-call). GCC hubiese emitido tail-call si $ffe fuese noreturn.
|    4. Absorbio JsrAbsThunk_00211e (Wave I): los ultimos 8 B de la funcion
|       (`jsr $ffe.l; rts`) fueron erroneamente contabilizados como thunk
|       independiente. Octavo falso positivo del proyecto, mismo patron
|       que JsrAbsThunk_000762 (absorbido por Entity_AllocFromFreeList_0006FE,
|       Wave W#16).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Init_MasterSubsystems_0020E2
        .type   Init_MasterSubsystems_0020E2, @function
        .section .text.Init_MasterSubsystems_0020E2, "ax", @progbits

Init_MasterSubsystems_0020E2:
        move.b  d0, 0x300001.l                 | +00  watchdog kick
        jsr     0x1379a.l                      | +06  Sub_01379A
        move.b  d0, 0x300001.l                 | +0c  watchdog kick
        jsr     0x13d12.l                      | +12  Sub_013D12
        move.b  d0, 0x300001.l                 | +18  watchdog kick
        jsr     0x5a824.l                      | +1e  Sub_05A824
        move.b  d0, 0x300001.l                 | +24  watchdog kick
        jsr     0x410.l                        | +2a  Sub_000410  (forma abs.l larga)
        move.b  d0, 0x300001.l                 | +30  watchdog kick
        jsr     0x5cc0e.l                      | +36  Sub_05CC0E
        jsr     0xffe.l                        | +3c  Global_SetDualFlagFrom10FD82 (Y#4)
        rts                                    | +42

        .size   Init_MasterSubsystems_0020E2, .-Init_MasterSubsystems_0020E2
