| ============================================================================
|  Metal Slug 1 - asm/collision_probes_051cxx.s
|  ----------------------------------------------------------------------------
|  Wave KK batch 2 - probes de colision del cluster de camara + handler MMIO.
|
|  Contenido (4 funciones, 526 bytes):
|
|      $051C08   Collision_ProbeRange_051C08     120 B  probe rango completo
|      $051C82   Collision_ProbeX_051C82         110 B  probe una columna
|      $051CF6   Collision_ProbeY_051CF6         136 B  probe una fila
|      $051F94   TileMap_HandlerInline_051F94    160 B  handler MMIO $3C0000
|
|  Cierra los tres probes CCR de camara referenciados por los hooks
|  Probe08/82/F6 (JJ#1) y el handler inline que TransformCommit_MMIO_051F30
|  (KK#1) pasa por a0 al dispatcher generico $1F4A.
|
|  ---------- Mapa de callers -----------------------------------------------
|
|      CameraHook_Probe08_043DF4  -> Collision_ProbeRange_051C08 (JJ#1)
|      CameraHook_Probe82_043E0E  -> Collision_ProbeX_051C82     (JJ#1)
|      CameraHook_ProbeF6_043E24  -> Collision_ProbeY_051CF6     (JJ#1)
|      TransformCommit_MMIO_051F30 -> TileMap_HandlerInline_051F94 (KK#1,
|                                     pasado por a0 a Fn_00001F4A)
|
|  ---------- Convencion CCR de los tres probes ------------------------------
|
|  Los tres retornan por CCR-C, con distincion clara entre "colision
|  detectada" (C set) y "sin colision" (C clear). El idioma es:
|
|      ori.b   #$1, ccr    ; rama COLISION  (retorna d0..d6 preservados)
|      rts
|      andi.b  #$FE, ccr   ; rama NO_COLISION (o simplemente rts en Probe08)
|      rts
|
|  Absorben SIETE falsos positivos de Waves N (CCR helpers) y B
|  (Stub_00051C80), documentados en cada funcion.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  Collision_ProbeRange_051C08  @ $051C08  (120 bytes)
|  Callee de CameraHook_Probe08_043DF4 (Wave JJ#1) via jsr abs.l.
|  Absorbe FPs: Stub_00051C80 (rama sin colision) y SetC_051c7a
|  (rama colision) - los 6+2 B del final son epilogos propios.
| ---------------------------------------------------------------------------
|
        .globl  Collision_ProbeRange_051C08
        .type   Collision_ProbeRange_051C08, @function
        .section .text.Collision_ProbeRange_051C08, "ax", @progbits
Collision_ProbeRange_051C08:
        movea.l 0xe(a0), a1                     | +00  a1 = collision_target
        move.w  0x4(a0), d0                     | +04  d0 = x_accum lo
        move.w  0x8(a0), d1                     | +08  d1 = y_accum lo
        add.w   0x16(a0), d0                    | +0c  d0 += x_offset
        asr.w   #0x4, d0                        | +10  d0 >>= 4  (pixel->tile)
        add.w   0x18(a0), d1                    | +12  d1 += y_offset
        asr.w   #0x4, d1                        | +16  d1 >>= 4
        move.w  d0, 0x1e(a0)                    | +18  cache_x = d0
        move.w  d1, 0x20(a0)                    | +1c  cache_y = d1
        move.w  d0, d2                          | +20  d2 = d0
        move.w  d1, d3                          | +22  d3 = d1
        move.w  0x1a(a0), d4                    | +24  d4 = width
        move.w  0x1c(a0), d5                    | +28  d5 = height
        bsr.w   Fn_00051D84                     | +2c  probe basico
        bcc.w   .Lprobe_range_none              | +30  if (!C) no colision
        movem.w d0-d6, -(a7)                    | +34  save d0-d6
        movem.w d0/d4, -(a7)                    | +38  save d0/d4 (loop)
        subq.w  #0x1, d4                        | +3c  --d4  (dbra)
        lea.l   0x52(a0), a2                    | +3e  a2 = &tile_map_A
        lea.l   0x32(a0), a3                    | +42  a3 = &tile_map_B
.Lprobe_range_loop:                             | $051C4E
        bsr.w   Fn_00051BA8                     | +46  apply_basico()
        move.w  d0, d7                          | +4a  d7 = current X
        add.w   0x22(a0), d7                    | +4c  d7 += base
        andi.w  #0x1f, d7                       | +50  d7 mod 32
        move.b  0x26(a0), (a2, d7.w)            | +54  tile_A[d7] = flag
        move.b  0x27(a0), (a3, d7.w)            | +5a  tile_B[d7] = flag
        addq.w  #0x1, d0                        | +60  ++d0
        dbra    d4, .Lprobe_range_loop          | +62  loop columnas
        movem.w (a7)+, d0/d4                    | +66  restore d0/d4
        bsr.w   ThunkTarget_051de2                     | +6a  commit()
        movem.w (a7)+, d0-d6                    | +6e  restore d0-d6
        ori.b   #0x1, ccr                       | +72  CCR-C = 1 (colision)
        rts                                     | +76
.Lprobe_range_none:                             | $051C80
        rts                                     | +78  (CCR-C ya en 0 por bcc)

        .size   Collision_ProbeRange_051C08, .-Collision_ProbeRange_051C08

|
| ---------------------------------------------------------------------------
|  Collision_ProbeX_051C82  @ $051C82  (110 bytes)
|  Callee de CameraHook_Probe82_043E0E (Wave JJ#1).
|  Absorbe FPs: SetC_051cea (colision) y ClearC_051cf0 (no colision).
| ---------------------------------------------------------------------------
|
        .globl  Collision_ProbeX_051C82
        .type   Collision_ProbeX_051C82, @function
        .section .text.Collision_ProbeX_051C82, "ax", @progbits
Collision_ProbeX_051C82:
        movea.l 0xe(a0), a1                     | +00  a1 = collision_target
        move.w  0x4(a0), d0                     | +04  d0 = x_accum lo
        move.w  0x8(a0), d1                     | +08  d1 = y_accum lo
        add.w   0x16(a0), d0                    | +0c  d0 += x_offset
        asr.w   #0x4, d0                        | +10  d0 >>= 4
        add.w   0x18(a0), d1                    | +12  d1 += y_offset
        asr.w   #0x4, d1                        | +16  d1 >>= 4
        cmp.w   0x1e(a0), d0                    | +18  if (d0 == cache_x)
        beq.w   .Lprobe_x_none                  | +1c    skip probe
        move.w  d0, 0x1e(a0)                    | +20  cache_x = d0
        move.w  (a0), d2                        | +24  d2 = flags word 0
        bmi.w   .Lprobe_x_set_d2                | +26  if (bit15 set)
        add.w   0x1a(a0), d0                    | +2a    d0 += width
        subq.w  #0x1, d0                        | +2e    --d0  (borde derecho)
.Lprobe_x_set_d2:                               | $051CB2
        move.w  d0, d2                          | +30  d2 = d0
        move.w  d1, d3                          | +32  d3 = d1
        moveq   #0x1, d4                        | +34  d4 = 1 (una columna)
        move.w  0x1c(a0), d5                    | +36  d5 = height
        bsr.w   Fn_00051D84                     | +3a  probe basico
        bcc.w   .Lprobe_x_none                  | +3e  if (!C) skip
        movem.w d0-d6, -(a7)                    | +42  save d0-d6
        bsr.w   Fn_00051BA8                     | +46  apply_basico()
        move.w  d0, d7                          | +4a  d7 = X
        add.w   0x22(a0), d7                    | +4c  d7 += base
        andi.w  #0x1f, d7                       | +50  d7 mod 32
        move.b  0x26(a0), 0x52(a0, d7.w)        | +54  tile_A[d7] = flag
        move.b  0x27(a0), 0x32(a0, d7.w)        | +5a  tile_B[d7] = flag
        bsr.w   ThunkTarget_051de2                     | +60  commit()
        movem.w (a7)+, d0-d6                    | +64  restore d0-d6
        ori.b   #0x1, ccr                       | +68  CCR-C = 1 (colision)
        rts                                     | +6c
.Lprobe_x_none:                                 | $051CF0
        andi.b  #0xfe, ccr                      | +6e  CCR-C = 0
        rts                                     | +72

        .size   Collision_ProbeX_051C82, .-Collision_ProbeX_051C82

|
| ---------------------------------------------------------------------------
|  Collision_ProbeY_051CF6  @ $051CF6  (136 bytes)
|  Callee de CameraHook_ProbeF6_043E24 (Wave JJ#1).
|  Clon estructural de ProbeX con ejes intercambiados y bucle dbra.
|  Absorbe FPs: SetC_051d78 (colision) y ClearC_051d7e (no colision).
| ---------------------------------------------------------------------------
|
        .globl  Collision_ProbeY_051CF6
        .type   Collision_ProbeY_051CF6, @function
        .section .text.Collision_ProbeY_051CF6, "ax", @progbits
Collision_ProbeY_051CF6:
        movea.l 0xe(a0), a1                     | +00  a1 = collision_target
        move.w  0x4(a0), d0                     | +04  d0 = x_accum lo
        move.w  0x8(a0), d1                     | +08  d1 = y_accum lo
        add.w   0x16(a0), d0                    | +0c  d0 += x_offset
        asr.w   #0x4, d0                        | +10  d0 >>= 4
        add.w   0x18(a0), d1                    | +12  d1 += y_offset
        asr.w   #0x4, d1                        | +16  d1 >>= 4
        cmp.w   0x20(a0), d1                    | +18  if (d1 == cache_y)
        beq.w   .Lprobe_y_none                  | +1c    skip probe
        move.w  d1, 0x20(a0)                    | +20  cache_y = d1
        move.w  0x2(a0), d2                     | +24  d2 = flags word 2
        bmi.w   .Lprobe_y_set_d2                | +28  if (bit15 set)
        add.w   0x1c(a0), d1                    | +2c    d1 += height
        subq.w  #0x1, d1                        | +30    --d1 (borde inferior)
.Lprobe_y_set_d2:                               | $051D28
        move.w  d0, d2                          | +32  d2 = d0
        move.w  d1, d3                          | +34  d3 = d1
        move.w  0x1a(a0), d4                    | +36  d4 = width
        moveq   #0x1, d5                        | +3a  d5 = 1 (una fila)
        bsr.w   Fn_00051D84                     | +3c  probe basico
        bcc.w   .Lprobe_y_none                  | +40  if (!C) skip
        movem.w d0-d6, -(a7)                    | +44  save d0-d6
        movem.w d0/d4, -(a7)                    | +48  save d0/d4
        subq.w  #0x1, d4                        | +4c  --d4 (dbra)
        lea.l   0x52(a0), a2                    | +4e  a2 = &tile_map_A
        lea.l   0x32(a0), a3                    | +52  a3 = &tile_map_B
.Lprobe_y_loop:                                 | $051D4C
        bsr.w   Fn_00051BA8                     | +56  apply_basico()
        move.w  d0, d7                          | +5a  d7 = X
        add.w   0x22(a0), d7                    | +5c  d7 += base
        andi.w  #0x1f, d7                       | +60  d7 mod 32
        move.b  0x26(a0), (a2, d7.w)            | +64  tile_A[d7] = flag
        move.b  0x27(a0), (a3, d7.w)            | +6a  tile_B[d7] = flag
        addq.w  #0x1, d0                        | +70  ++d0
        dbra    d4, .Lprobe_y_loop              | +72  loop columnas
        movem.w (a7)+, d0/d4                    | +76  restore d0/d4
        bsr.w   ThunkTarget_051de2                     | +7a  commit()
        movem.w (a7)+, d0-d6                    | +7e  restore d0-d6
        ori.b   #0x1, ccr                       | +82  CCR-C = 1 (colision)
        rts                                     | +86
.Lprobe_y_none:                                 | $051D7E
        andi.b  #0xfe, ccr                      | +88  CCR-C = 0
        rts                                     | +8c

        .size   Collision_ProbeY_051CF6, .-Collision_ProbeY_051CF6

|
| ---------------------------------------------------------------------------
|  TileMap_HandlerInline_051F94  @ $051F94  (160 bytes)
|  Handler pasado por a0 al dispatcher Fn_00001F4A desde
|  TransformCommit_MMIO_051F30 (KK#1).
| ---------------------------------------------------------------------------
|
        .globl  TileMap_HandlerInline_051F94
        .type   TileMap_HandlerInline_051F94, @function
        .section .text.TileMap_HandlerInline_051F94, "ax", @progbits
TileMap_HandlerInline_051F94:
        move.w  #0x8201, d0                     | +00  d0 = SCB3 base
        add.w   0x28(a6), d0                    | +04  d0 += sprite_id
        move.w  0x2a(a6), d1                    | +08  d1 = tile_row_start
        move.w  0x30(a6), d2                    | +0c  d2 = hw_y_offset
        move.w  d0, d5                          | +10  d5 = d0
        add.w   d1, d5                          | +12  d5 = d0 + d1
        move.w  d5, 0x106ee4.l                  | +14  shadow_addr = d5
        move.w  d5, 0x3c0000.l                  | +1a  VRAM_ADDR = d5
        move.w  #0x0, 0x3c0002.l                | +20  VRAM_DATA = 0
        addq.b  #0x1, d1                        | +28  ++d1
        andi.b  #0x1f, d1                       | +2a  d1 mod 32
        moveq   #0xb, d4                        | +2e  d4 = 11 (shift count)
        move.w  d1, d3                          | +30  d3 = d1
        lsl.w   d4, d3                          | +32  d3 <<= 11
        add.w   0x2e(a6), d3                    | +34  d3 += hw_x_offset
.Lhandler_loop:                                 | $051FCC
        cmp.w   0x2c(a6), d1                    | +38  if (d1 == tile_row_end)
        beq.w   .Lhandler_done                  | +3c    salir
        move.w  d0, d5                          | +40  d5 = d0
        add.w   d1, d5                          | +42  d5 = d0 + d1
        move.w  d5, 0x106ee4.l                  | +44  shadow_addr = d5
        move.w  d5, 0x3c0000.l                  | +4a  VRAM_ADDR = d5
        move.b  0x52(a6, d1.w), d6              | +50  d6 = tile_map_A[d1]
        lsl.w   d4, d6                          | +54  d6 <<= 11
        add.w   d2, d6                          | +56  d6 += hw_y_offset
        or.b    0x32(a6, d1.w), d6              | +58  d6 |= tile_map_B[d1]
        move.w  d6, 0x3c0002.l                  | +5c  VRAM_DATA = d6
        addi.w  #0x200, d5                      | +62  d5 += $200
        move.w  d5, 0x106ee4.l                  | +66  shadow_addr = d5
        move.w  d5, 0x3c0000.l                  | +6c  VRAM_ADDR = d5
        addq.b  #0x1, d1                        | +72  ++d1
        andi.w  #0x1f, d1                       | +74  d1 mod 32
        move.w  d3, 0x3c0002.l                  | +78  VRAM_DATA = d3
        addi.w  #0x800, d3                      | +7e  d3 += $800
        bra.b   .Lhandler_loop                  | +82
.Lhandler_done:                                 | $052018
        move.w  d0, d4                          | +84  d4 = d0
        add.w   d1, d4                          | +86  d4 = d0 + d1
        move.w  d4, 0x106ee4.l                  | +88  shadow_addr = d4
        move.w  d4, 0x3c0000.l                  | +8e  VRAM_ADDR = d4
        move.w  #0x0, 0x3c0002.l                | +94  VRAM_DATA = 0 (term)
        rts                                     | +9c

        .size   TileMap_HandlerInline_051F94, .-TileMap_HandlerInline_051F94
