| ============================================================================
|  Metal Slug 1 - asm/input_aim_tables_05d316.s
|  ----------------------------------------------------------------------------
|  Wave BBB - TABLAS DE ANGULO DE PUNTERIA + BACKEND DE LECTURA DE INPUT.
|  Cierra ENTERO el hueco $05D316..$05D6AA (27 entradas, 916 B) entre las
|  islas ClearXN_05d310 y JsrPcThunk_05d6aa.
|
|  MITAD DATOS (688 B) - las "tablas de arrays" reales de la punteria:
|
|  * AimAngleTable_* ($05D326..$05D586): 19 tablas de 16 words (32 B).
|    Son EXACTAMENTE las tablas que consume Ent_AimUpdate_045022 /
|    Ent_AimInit_045412 (Wave XX, ent_aim_input_044f8a.s): entrada =
|    tabla[(estado & $F)*2] = word de ANGULO OBJETIVO en unidades
|    $10000 = vuelta completa ($4000 = 90 grados); $FFFF = sin cambio,
|    $8000 en slot flip. Asignacion por arma (refs verificadas desde el
|    codigo matcheado de $0450xx-$0454xx):
|      $5D326 base pistola/default      $5D346 rocket (40/50/30: abanico)
|      $5D386 init dir4 (todo 0)        $5D3A6 HMG (78/78/80: casi 180)
|      $5D4C6 HMG bit5 de $7B(a6)       $5D3C6..$5D526 juego de 8 tablas
|      del lanzallamas/shotgun (una por direccion: 20/40/60/80/A0/C0/E0/
|      00, mas variantes 30/50/B0/D0 y diagonales suaves D8/A8).
|    $5D366/$5D4A6/$5D566 no tienen ref desde codigo matcheado aun
|    (variantes espejo del mismo juego).
|
|  * AimDirRows_05D316 (16 B) y SpawnRows_05D586/96/A6 (16 B c/u):
|    filas de 4 bytes terminadas en $FF. Las tres ultimas son los
|    destinos de la tabla de punteros de 5 longs en $02A5B8
|    ($02A5B8/BC/C0 -> $5D5A6, $02A5C4 -> $5D586, $02A5C8 -> $5D596).
|
|  MITAD CODIGO (244 B) - el backend de lectura de layout de input:
|
|  * InputLayout_ReadField2_05D5B6 / InputLayout_ReadField0_05D616:
|    versiones COMPLETAS del idioma InputMask_ReadCtxSwitchPlayer (Wave
|    U): eligen buffer de input por player ($6d(a6): 1 -> $100300,
|    2 -> $1003A0, default demo $5CC08), cortocircuitan a rts si no hay
|    replay (bit7 de $100000 apagado y $44($1001C0) armado), aplican el
|    override de demo y extraen del byte d1 del layout el campo de
|    bits 2-3 (>>2) o 0-1 respectivamente. Llamadas via jsr abs.l desde
|    codigo aun sin matchear ($02AB30, $02CA9A, ...).
|    HALLAZGO - BUG DE SNK en $05D628: la rama P1 de ReadField0 hace
|    `movea.l $72(a1),a2` ANTES del `lea $100300,a1` (orden invertido
|    respecto a ReadField2 y a los 3 backends de Wave U): lee el puntero
|    del buffer con el a1 VIEJO. Quedo compilado asi en la ROM final.
|
|  * InputCtx_DemoOverride_05D674: el "probe real" pendiente desde Wave
|    U (era el defsym forward Sub_00005D674, llamado por los 3
|    InputMask_ReadCtxSwitchPlayer_* via jsr pc y por Ent_InputSample /
|    turret via jsr abs.l): si el modo replay $10E39D esta armado,
|    redirige a2 a los buffers de grabacion $10E2E2/$10E2E8 (segun a2
|    fuera $10E200 o no).
|
|  * DebugHex_SetupA1_05D6A0: cabecera de 10 B (a1=$7063 destino VRAM
|    fix-layer, d1=$300 atributo) que cae por fall-through en el thunk
|    JsrPcThunk_05d6aa -> Sprite_HexFormat4_05D6C2 (volcado hex debug).
|
|  Todo byte-exacto contra la ROM (verificado por match_batch).
| ============================================================================

        .globl  AimDirRows_05D316
        .section .text.AimDirRows_05D316, "ax", @progbits
AimDirRows_05D316:
        .byte   0xFF, 0x08, 0x18, 0xFF                 | $05D316 fila 0
        .byte   0x10, 0x0C, 0x14, 0xFF                 | $05D31A fila 1
        .byte   0x00, 0x04, 0x1C, 0xFF                 | $05D31E fila 2
        .byte   0xFF, 0xFF, 0xFF, 0xFF                 | $05D322 fin
        .size   AimDirRows_05D316, .-AimDirRows_05D316

        .globl  AimAngleTable_05D326
        .section .text.AimAngleTable_05D326, "ax", @progbits
AimAngleTable_05D326:                                  | base pistola/default
        .word   0xFFFF,0x4000,0xC000,0xFFFF, 0x8000,0x6000,0xA000,0xFFFF
        .word   0x0000,0x2000,0xE000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D326, .-AimAngleTable_05D326

        .globl  AimAngleTable_05D346
        .section .text.AimAngleTable_05D346, "ax", @progbits
AimAngleTable_05D346:                                  | rocket (Ent_AimUpdate +16a)
        .word   0x4000,0x4000,0x4000,0xFFFF, 0x5000,0x5000,0x5000,0xFFFF
        .word   0x3000,0x3000,0x3000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D346, .-AimAngleTable_05D346

        .globl  AimAngleTable_05D366
        .section .text.AimAngleTable_05D366, "ax", @progbits
AimAngleTable_05D366:                                  | 90 fijo (sin ref matcheada)
        .word   0x4000,0x4000,0x4000,0xFFFF, 0x4000,0x4000,0x4000,0xFFFF
        .word   0x4000,0x4000,0x4000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D366, .-AimAngleTable_05D366

        .globl  AimAngleTable_05D386
        .section .text.AimAngleTable_05D386, "ax", @progbits
AimAngleTable_05D386:                                  | init dir4 (Ent_AimInit +48)
        .word   0x0000,0x0000,0x0000,0xFFFF, 0x0000,0x0000,0x0000,0xFFFF
        .word   0x0000,0x0000,0x0000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D386, .-AimAngleTable_05D386

        .globl  AimAngleTable_05D3A6
        .section .text.AimAngleTable_05D3A6, "ax", @progbits
AimAngleTable_05D3A6:                                  | HMG (Ent_AimUpdate +05a)
        .word   0x7800,0x7800,0x8000,0xFFFF, 0x7800,0x7800,0x8000,0xFFFF
        .word   0x7800,0x7800,0x8000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D3A6, .-AimAngleTable_05D3A6

        .globl  AimAngleTable_05D3C6
        .section .text.AimAngleTable_05D3C6, "ax", @progbits
AimAngleTable_05D3C6:                                  | flame/shotgun dir 45
        .word   0x2000,0x2000,0x2000,0xFFFF, 0x2000,0x2000,0x2000,0xFFFF
        .word   0x2000,0x2000,0x2000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D3C6, .-AimAngleTable_05D3C6

        .globl  AimAngleTable_05D3E6
        .section .text.AimAngleTable_05D3E6, "ax", @progbits
AimAngleTable_05D3E6:                                  | flame/shotgun dir 90
        .word   0x4000,0x4000,0x4000,0xFFFF, 0x4000,0x4000,0x4000,0xFFFF
        .word   0x4000,0x4000,0x4000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D3E6, .-AimAngleTable_05D3E6

        .globl  AimAngleTable_05D406
        .section .text.AimAngleTable_05D406, "ax", @progbits
AimAngleTable_05D406:                                  | flame dir 135 (init flame)
        .word   0x6000,0x6000,0x6000,0xFFFF, 0x6000,0x6000,0x6000,0xFFFF
        .word   0x6000,0x6000,0x6000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D406, .-AimAngleTable_05D406

        .globl  AimAngleTable_05D426
        .section .text.AimAngleTable_05D426, "ax", @progbits
AimAngleTable_05D426:                                  | flame/shotgun dir 180
        .word   0x8000,0x8000,0x8000,0xFFFF, 0x8000,0x8000,0x8000,0xFFFF
        .word   0x8000,0x8000,0x8000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D426, .-AimAngleTable_05D426

        .globl  AimAngleTable_05D446
        .section .text.AimAngleTable_05D446, "ax", @progbits
AimAngleTable_05D446:                                  | flame/shotgun dir 315
        .word   0xE000,0xE000,0xE000,0xFFFF, 0xE000,0xE000,0xE000,0xFFFF
        .word   0xE000,0xE000,0xE000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D446, .-AimAngleTable_05D446

        .globl  AimAngleTable_05D466
        .section .text.AimAngleTable_05D466, "ax", @progbits
AimAngleTable_05D466:                                  | flame/shotgun dir 270
        .word   0xC000,0xC000,0xC000,0xFFFF, 0xC000,0xC000,0xC000,0xFFFF
        .word   0xC000,0xC000,0xC000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D466, .-AimAngleTable_05D466

        .globl  AimAngleTable_05D486
        .section .text.AimAngleTable_05D486, "ax", @progbits
AimAngleTable_05D486:                                  | flame/shotgun dir 225
        .word   0xA000,0xA000,0xA000,0xFFFF, 0xA000,0xA000,0xA000,0xFFFF
        .word   0xA000,0xA000,0xA000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D486, .-AimAngleTable_05D486

        .globl  AimAngleTable_05D4A6
        .section .text.AimAngleTable_05D4A6, "ax", @progbits
AimAngleTable_05D4A6:                                  | 180 fijo (sin ref matcheada)
        .word   0x8000,0x8000,0x8000,0xFFFF, 0x8000,0x8000,0x8000,0xFFFF
        .word   0x8000,0x8000,0x8000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D4A6, .-AimAngleTable_05D4A6

        .globl  AimAngleTable_05D4C6
        .section .text.AimAngleTable_05D4C6, "ax", @progbits
AimAngleTable_05D4C6:                                  | HMG bit5 / shotgun 67.5
        .word   0x3000,0x3000,0x3000,0xFFFF, 0x3000,0x3000,0x3000,0xFFFF
        .word   0x3000,0x3000,0x3000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D4C6, .-AimAngleTable_05D4C6

        .globl  AimAngleTable_05D4E6
        .section .text.AimAngleTable_05D4E6, "ax", @progbits
AimAngleTable_05D4E6:                                  | shotgun 112.5
        .word   0x5000,0x5000,0x5000,0xFFFF, 0x5000,0x5000,0x5000,0xFFFF
        .word   0x5000,0x5000,0x5000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D4E6, .-AimAngleTable_05D4E6

        .globl  AimAngleTable_05D506
        .section .text.AimAngleTable_05D506, "ax", @progbits
AimAngleTable_05D506:                                  | shotgun 292.5
        .word   0xD000,0xD000,0xD000,0xFFFF, 0xD000,0xD000,0xD000,0xFFFF
        .word   0xD000,0xD000,0xD000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D506, .-AimAngleTable_05D506

        .globl  AimAngleTable_05D526
        .section .text.AimAngleTable_05D526, "ax", @progbits
AimAngleTable_05D526:                                  | shotgun 247.5
        .word   0xB000,0xB000,0xB000,0xFFFF, 0xB000,0xB000,0xB000,0xFFFF
        .word   0xB000,0xB000,0xB000,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D526, .-AimAngleTable_05D526

        .globl  AimAngleTable_05D546
        .section .text.AimAngleTable_05D546, "ax", @progbits
AimAngleTable_05D546:                                  | diag suave 303.75
        .word   0xD800,0xD800,0xD800,0xFFFF, 0xD800,0xD800,0xD800,0xFFFF
        .word   0xD800,0xD800,0xD800,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D546, .-AimAngleTable_05D546

        .globl  AimAngleTable_05D566
        .section .text.AimAngleTable_05D566, "ax", @progbits
AimAngleTable_05D566:                                  | diag suave 236.25 (sin ref)
        .word   0xA800,0xA800,0xA800,0xFFFF, 0xA800,0xA800,0xA800,0xFFFF
        .word   0xA800,0xA800,0xA800,0xFFFF, 0xFFFF,0xFFFF,0xFFFF,0xFFFF
        .size   AimAngleTable_05D566, .-AimAngleTable_05D566

        .globl  SpawnRows_05D586
        .section .text.SpawnRows_05D586, "ax", @progbits
SpawnRows_05D586:                                      | <- puntero $02A5C4
        .byte   0x00, 0x01, 0x01, 0xFF                 | $05D586
        .byte   0x00, 0x01, 0x00, 0xFF                 | $05D58A
        .byte   0x00, 0x00, 0x01, 0xFF                 | $05D58E
        .byte   0xFF, 0xFF, 0xFF, 0xFF                 | $05D592 fin
        .size   SpawnRows_05D586, .-SpawnRows_05D586

        .globl  SpawnRows_05D596
        .section .text.SpawnRows_05D596, "ax", @progbits
SpawnRows_05D596:                                      | <- puntero $02A5C8
        .byte   0x00, 0x01, 0x01, 0xFF                 | $05D596
        .byte   0x00, 0x00, 0x01, 0xFF                 | $05D59A
        .byte   0x00, 0x01, 0x00, 0xFF                 | $05D59E
        .byte   0xFF, 0xFF, 0xFF, 0xFF                 | $05D5A2 fin
        .size   SpawnRows_05D596, .-SpawnRows_05D596

        .globl  SpawnRows_05D5A6
        .section .text.SpawnRows_05D5A6, "ax", @progbits
SpawnRows_05D5A6:                                      | <- punteros $02A5B8/BC/C0
        .byte   0x00, 0x01, 0x01, 0xFF                 | $05D5A6
        .byte   0x00, 0x00, 0x00, 0xFF                 | $05D5AA
        .byte   0x00, 0x00, 0x00, 0xFF                 | $05D5AE
        .byte   0xFF, 0xFF, 0xFF, 0xFF                 | $05D5B2 fin
        .size   SpawnRows_05D5A6, .-SpawnRows_05D5A6

        .globl  InputLayout_ReadField2_05D5B6
        .type   InputLayout_ReadField2_05D5B6, @function
        .section .text.InputLayout_ReadField2_05D5B6, "ax", @progbits
InputLayout_ReadField2_05D5B6:
        move.w  #0x2, d1                               | +000  d1 = byte 2 del layout
        lea     Sub_00005CC08(pc), a2                  | +004  default: ctx demo $5CC08
        cmpi.b  #0x1, 0x6d(a6)                         | +008  ¿player 1?
        bne.w   .L5d5d2                                | +00e
        lea     0x100300.l, a1                         | +012  buf input P1
        movea.l 0x72(a1), a2                           | +018
.L5d5d2:
        cmpi.b  #0x2, 0x6d(a6)                         | +01c  ¿player 2?
        bne.w   .L5d5e6                                | +022
        lea     0x1003a0.l, a1                         | +026  buf input P2
        movea.l 0x72(a1), a2                           | +02c
.L5d5e6:
        moveq   #0x0, d0                               | +030
        btst    #0x7, 0x100000.l                       | +032  ¿replay activo?
        bne.w   .L5d606                                | +03a
        lea     0x1001c0.l, a4                         | +03e
        tst.b   0x44(a4)                               | +044  gate del slot $1001C0
        beq.w   .L5d606                                | +048
        nop                                            | +04c
        rts                                            | +04e  cortocircuito: d0=0
.L5d606:
        jsr     InputCtx_DemoOverride_05D674(pc)       | +050  override de demo en a2
        move.b  (a2,d1.w), d0                          | +054  byte d1 del layout
        andi.b  #0xc, d0                               | +058  campo bits 2-3
        asr.b   #2, d0                                 | +05c
        rts                                            | +05e
        .size   InputLayout_ReadField2_05D5B6, .-InputLayout_ReadField2_05D5B6

        .globl  InputLayout_ReadField0_05D616
        .type   InputLayout_ReadField0_05D616, @function
        .section .text.InputLayout_ReadField0_05D616, "ax", @progbits
InputLayout_ReadField0_05D616:
        move.w  #0x2, d1                               | +000
        lea     Sub_00005CC08(pc), a2                  | +004
        cmpi.b  #0x1, 0x6d(a6)                         | +008
        bne.w   .L5d632                                | +00e
        movea.l 0x72(a1), a2                           | +012  BUG SNK: lee $72 con el
        lea     0x100300.l, a1                         | +016  a1 VIEJO (orden invertido)
.L5d632:
        cmpi.b  #0x2, 0x6d(a6)                         | +01c
        bne.w   .L5d646                                | +022
        lea     0x1003a0.l, a1                         | +026
        movea.l 0x72(a1), a2                           | +02c
.L5d646:
        moveq   #0x0, d0                               | +030
        btst    #0x7, 0x100000.l                       | +032
        bne.w   .L5d666                                | +03a
        lea     0x1001c0.l, a4                         | +03e
        tst.b   0x44(a4)                               | +044
        beq.w   .L5d666                                | +048
        nop                                            | +04c
        rts                                            | +04e
.L5d666:
        jsr     InputCtx_DemoOverride_05D674(pc)       | +050
        move.b  (a2,d1.w), d0                          | +054
        andi.b  #0x3, d0                               | +058  campo bits 0-1
        rts                                            | +05c
        .size   InputLayout_ReadField0_05D616, .-InputLayout_ReadField0_05D616

        .globl  InputCtx_DemoOverride_05D674
        .type   InputCtx_DemoOverride_05D674, @function
        .section .text.InputCtx_DemoOverride_05D674, "ax", @progbits
InputCtx_DemoOverride_05D674:
        movem.l d0, -(a7)                              | +000
        tst.b   0x10e39d.l                             | +004  ¿modo replay armado?
        beq.w   .L5d69a                                | +00a
        move.l  a2, d0                                 | +00e
        lea     0x10e2e2.l, a2                         | +010  buffer grabacion A
        cmpi.l  #0x10e200, d0                          | +016  ¿venia del ctx $10E200?
        beq.w   .L5d69a                                | +01c
        lea     0x10e2e8.l, a2                         | +020  buffer grabacion B
.L5d69a:
        movem.l (a7)+, d0                              | +026
        rts                                            | +02a
        .size   InputCtx_DemoOverride_05D674, .-InputCtx_DemoOverride_05D674

        .globl  DebugHex_SetupA1_05D6A0
        .type   DebugHex_SetupA1_05D6A0, @function
        .section .text.DebugHex_SetupA1_05D6A0, "ax", @progbits
DebugHex_SetupA1_05D6A0:
        movea.l #0x7063, a1                            | +000  destino fix-layer
        move.w  #0x300, d1                             | +006  atributo/paleta
                                                       |       fall-through a
                                                       |       JsrPcThunk_05d6aa
        .size   DebugHex_SetupA1_05D6A0, .-DebugHex_SetupA1_05D6A0
