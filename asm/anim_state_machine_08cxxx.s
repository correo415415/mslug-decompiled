| ============================================================================
|  Metal Slug 1 - asm/anim_state_machine_08cxxx.s
|  ----------------------------------------------------------------------------
|  Wave GG batch 2 - cluster maquina de estados de animacion en $08Cxxx
|
|  Este cluster de 6 funciones ($08C008..$08C2B7, 688 B) implementa una
|  maquina de estados F1 -> F2 -> F3 -> F4 -> F5 -> F6 con enlaces por
|  "self-replace handler" (`lea $<next>(pc), a1; move.l a1, (a6)`) y una
|  cola comun de 3 jsr (tail): `$967c0(pc) + $436de.l + $28d70.l + rts`.
|
|  Estructura:
|      F1 $08C008 (342 B) - initializer: sprite setup completo (position,
|                            flags, task-adds x6 con templates $8c2b8/322/
|                            37e/3da/436/5b2), publica $335A6 en $100440
|                            (slot P1), y arranca la LUT-driven state
|                            machine leyendo $2C072C[$34(a6)*2].
|      F2 $08C15E ( 78 B) - LUT-driven: lee $2C072C, actualiza $32(a6),
|                            avanza $34(a6) por 4, transiciona a F3.
|      F3 $08C1AC ( 62 B) - jsr $8bc74(pc) probe, si C=1 llama a MMIO
|                            blitter $5da9c(#$7084,$2320,$20,$19), reset
|                            $34(a6), transiciona a F4.
|      F4 $08C1EA ( 80 B) - LUT-driven sobre $2C07AC[], subq.w #1 sobre
|                            resultado, actualiza $32(a6), transiciona a F5.
|      F5 $08C23A ( 92 B) - LUT-driven sobre $2C07AC[], actualiza $33(a6),
|                            si $34>=$3F entonces jsr $2308(#$80) + jsr
|                            $5239E(#$2) + timer $70=$3C, transiciona a F6.
|      F6 $08C296 ( 34 B) - decrementa timer $70(a6), al llegar a 0 clr
|                            $106ED2, tail comun (queda esperando final).
|
|  Todos comparten tail comun:
|      jsr     $967c0.l                      | shared setup (Attract_Sub_setup_967C0)
|      jsr     $436de.l                      | Geom_Proj_Clamp (Wave FF#2)
|      jsr     $28d70.l                      | ThunkTarget_028d70 (Entity_HasLinkedSlots via thunk)
|      rts
|
|  Firma C conceptual (F1 como ejemplo):
|
|      /* Handler de arranque de la maquina de estados de animacion del
|       * player-icon en el menu principal / gameplay. Configura sprite
|       * completo, encadena 6 task-handlers, y arranca la LUT-driven
|       * state machine. La LUT $2C072C es una tabla de word-values que
|       * modulan el brillo (channel $33(a6)) segun un contador de fase
|       * ($34(a6)). Cuando $34 alcanza $3F, la fase termina y el handler
|       * se auto-reemplaza por F2. */
|      void Anim_State_F1_08C008(void);
|      void Anim_State_F2_08C15E(void);  // ... F6
|
|  Cross-links:
|    - jsr $436de.l = Geom_Proj_Clamp_0436DE (Wave FF#2, 13-caller helper).
|      Los 6 handlers son 6 mas de los 13 callers identificados en FF.
|    - jsr $28d70.l = Entity_HasLinkedSlots (Wave S#2, via ThunkTarget).
|    - jsr $2352 = InputGuardCall219c (Wave A#4).
|    - jsr $23c68 = Sub_00023C68 (usado en F1 con opcode $2b).
|    - move.l #$335a6, $100440.l = publica handler player $335A6 en slot P1.
|      Recordar que Attract_DoubleCheck_400_Publish_001846 (Wave FF#1) hace
|      lo mismo pero con $2575C. Este es otro puntero al mismo pipeline.
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. F1 hace 6 `lea $<offset>(pc), a1; jsr $4ae` consecutivos con
|       distintos templates. Los offsets PC-rel de cada `lea` estan
|       precisamente calculados para apuntar a los sub-templates
|       ($8c2b8/322/37e/3da/436/5b2), impossible en GCC sin macros.
|    2. F1..F5 comparten literalmente la misma tail (`jsr $967c0.l;
|       jsr $436de.l; jsr $28d70.l; rts`) - 20 bytes duplicados x 6 =
|       120 bytes de tail. GCC factorizaria via `bra tail_common`.
|    3. F3 usa `movea.w #$7084, a1` (cargar word con sign-extend en a1)
|       para pasar como argumento al blitter $5da9c. GCC habria usado
|       `move.l #$7084, a1` (2 bytes mas pero mas explicito).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

| ============================================================================
| F1: Anim_State_F1_08C008  @ $08C008  (342 B) - initializer + LUT stage 0
| ============================================================================
        .globl  Anim_State_F1_08C008
        .type   Anim_State_F1_08C008, @function
        .section .text.Anim_State_F1_08C008, "ax", @progbits

Anim_State_F1_08C008:
        jsr     0x22c8.l                       | +00  Sub_000022C8 (init)
        move.w  #0x2b, d0                      | +06  d0 = $2B (opcode)
        jsr     0x2352.l                       | +0a  InputGuardCall219c(#$2b)
        move.w  #0x12e, d1                     | +10  d1 = $12E
        jsr     0x236e.l                       | +14  ThunkTarget_00236e(#$12e)
        lea.l   0x2f2dbe.l, a0                 | +1a  a0 = sprite template ptr
        jsr     0x28cd4.l                      | +20  Sub_00028CD4(a0) (sprite install)
        move.w  #0xa0, 0x22(a6)                | +26  entity.pos_x = $A0
        move.w  #0x1c3, 0x24(a6)               | +2c  entity.pos_y = $1C3
        clr.w   0x26(a6)                       | +32  entity.pos_z = 0
        move.w  #0x4000, 0x38(a6)              | +36  entity.flags_38 = $4000
        bset.b  #0x6, 0x12(a6)                 | +3c  entity.flags_12 |= $40
        move.b  #0x9, d0                       | +42  d0 = 9
        jsr     0x43568.l                      | +46  Sub_00043568(9)
        move.w  #0x0, 0x70(a6)                 | +4c  entity.timer_70 = 0
        move.w  #0x0, 0x76(a6)                 | +52  entity.field_76 = 0
        move.l  #0x0, 0x78(a6)                 | +58  entity.field_78 = 0
        lea.l   0x2ef8aa.l, a1                 | +60  a1 = sub-template ptr
        move.l  a1, 0x7c(a6)                   | +66  entity.field_7c = a1
        move.l  #0x0, 0x90(a6)                 | +6a  entity.field_90 = 0
        movea.l a6, a1                         | +72  a1 = a6 (self)
        adda.l  #0x90, a1                      | +74  a1 = &entity.field_90
        move.l  a1, 0x72(a6)                   | +7a  entity.field_72 = &self.field_90
        lea.l   0x2ae50.l, a1                  | +7e  a1 = task template $2AE50
        jsr     0x4ae.l                        | +84  scheduler_add(a1) -> a0 = new task
        move.w  #0xa0, 0x22(a0)                | +8a  new_task.pos_x = $A0
        move.w  #0x1ff, 0x24(a0)               | +90  new_task.pos_y = $1FF
        move.b  #0x3, 0x98(a0)                 | +96  new_task.field_98 = 3
        lea.l   0x100440.l, a0                 | +9c  a0 = &SLOT_P1
        move.l  #0x335a6, (a0)                 | +a2  SLOT_P1 = TaskHandler_000335A6
        jsr     0x5fe.l                        | +a8  scheduler_publish (Wave R)
                                              | (6 task-adds via PC-rel lea)
        lea.l   .Lf1_tpl1(pc), a1              | +ae  a1 = ptr $8c2b8
        jsr     0x4ae.l                        | +b2  scheduler_add(a1)
        lea.l   .Lf1_tpl2(pc), a1              | +b8  a1 = ptr $8c322
        jsr     0x4ae.l                        | +bc
        lea.l   .Lf1_tpl3(pc), a1              | +c2  a1 = ptr $8c37e
        jsr     0x4ae.l                        | +c6
        lea.l   .Lf1_tpl4(pc), a1              | +cc  a1 = ptr $8c3da
        jsr     0x4ae.l                        | +d0
        lea.l   .Lf1_tpl5(pc), a1              | +d6  a1 = ptr $8c436
        jsr     0x4ae.l                        | +da
        lea.l   .Lf1_tpl6(pc), a1              | +e0  a1 = ptr $8c5b2
        jsr     0x4ae.l                        | +e4
                                              |
        move.b  #0x0, 0x32(a6)                 | +ea  entity.field_32 = 0
        move.b  #0x0, 0x33(a6)                 | +f0  entity.field_33 = 0
        move.w  #0x0, 0x34(a6)                 | +f6  entity.field_34 = 0
                                              |
                                              | ---- self-replace handler: F1 vuelve al mismo LUT stage ----
        lea.l   .Lf1_loop(pc), a1              | +fc  ptr a "continuation"
        move.l  a1, (a6)                       | +100 entity.handler = a1
                                              |
.Lf1_loop:                                     | $08C10A
        lea.l   0x2c072c.l, a1                 | +102 a1 = &LUT_2C072C[0]
        move.w  0x34(a6), d0                   | +108 d0 = phase counter
        lsl.w   #0x1, d0                       | +10c d0 <<= 1  (word index)
        move.w  (a1, d0.w), d1                 | +10e d1 = LUT[phase]
        cmpi.w  #0xff, d1                      | +112 clamp d1 to $FF
        ble.w   .Lf1_no_clamp                  | +116
        move.w  #0xff, d1                      | +11a d1 = $FF
.Lf1_no_clamp:                                 | $08C126
        move.b  d1, 0x33(a6)                   | +11e entity.field_33 = d1
        addq.w  #0x8, 0x34(a6)                 | +122 phase += 8
        cmpi.w  #0x3f, 0x34(a6)                | +126 if (phase <= $3F)
        ble.w   .Lf1_tail                      | +12c   go to tail (loop)
                                              | else transition to F2:
        move.w  #0x0, 0x34(a6)                 | +130 phase = 0
        move.b  #0xff, 0x33(a6)                | +136 field_33 = $FF (finalized)
        lea.l   .Lf1_next(pc), a1              | +13c a1 = &Anim_State_F2_08C15E
        move.l  a1, (a6)                       | +140 entity.handler = F2
.Lf1_tail:                                     | $08C14A
        jsr     0x967c0.l                      | +142 Attract_Sub_setup_967C0
        jsr     0x436de.l                      | +148 Geom_Proj_Clamp
        jsr     0x28d70.l                      | +14e ThunkTarget_028d70
        rts                                    | +154

        .equ    .Lf1_tpl1, Sub_0008C2B8
        .equ    .Lf1_tpl2, Sub_0008C322
        .equ    .Lf1_tpl3, Sub_0008C37E
        .equ    .Lf1_tpl4, Sub_0008C3DA
        .equ    .Lf1_tpl5, Sub_0008C436
        .equ    .Lf1_tpl6, Sub_0008C5B2
        .equ    .Lf1_next, Anim_State_F2_08C15E

        .size   Anim_State_F1_08C008, .-Anim_State_F1_08C008

| ============================================================================
| F2: Anim_State_F2_08C15E  @ $08C15E  (78 B) - LUT stage 1 ($32 channel)
| ============================================================================
        .globl  Anim_State_F2_08C15E
        .type   Anim_State_F2_08C15E, @function
        .section .text.Anim_State_F2_08C15E, "ax", @progbits

Anim_State_F2_08C15E:
        lea.l   0x2c072c.l, a1                 | +00  a1 = &LUT_2C072C[0]
        move.w  0x34(a6), d0                   | +06
        lsl.w   #0x1, d0                       | +0a
        move.w  (a1, d0.w), d1                 | +0c
        cmpi.w  #0xff, d1                      | +10
        ble.w   .Lf2_no_clamp                  | +14
        move.w  #0xff, d1                      | +18
.Lf2_no_clamp:                                 | $08C17A
        move.b  d1, 0x32(a6)                   | +1c  field_32 = d1
        addq.w  #0x4, 0x34(a6)                 | +20  phase += 4
        cmpi.w  #0x3f, 0x34(a6)                | +24
        ble.w   .Lf2_tail                      | +2a
        move.b  #0xff, 0x32(a6)                | +2e  field_32 = $FF (finalized)
        lea.l   .Lf2_next(pc), a1              | +34  a1 = &F3
        move.l  a1, (a6)                       | +38
.Lf2_tail:                                     | $08C198
        jsr     0x967c0.l                      | +3a
        jsr     0x436de.l                      | +40
        jsr     0x28d70.l                      | +46
        rts                                    | +4c

        .equ    .Lf2_next, Anim_State_F3_08C1AC

        .size   Anim_State_F2_08C15E, .-Anim_State_F2_08C15E

| ============================================================================
| F3: Anim_State_F3_08C1AC  @ $08C1AC  (62 B) - probe + optional blitter
| ============================================================================
        .globl  Anim_State_F3_08C1AC
        .type   Anim_State_F3_08C1AC, @function
        .section .text.Anim_State_F3_08C1AC, "ax", @progbits

Anim_State_F3_08C1AC:
        jsr     Sub_0008BC74(pc)               | +00  probe (returns CCR-C)
        bcc.w   .Lf3_tail                      | +04  if (!C) skip blitter
        movea.w #0x7084, a1                    | +08  a1 = &VRAM $7084 (sign-ext)
        move.w  #0x2320, d0                    | +0c  d0 = tile-id $2320
        move.w  #0x20, d1                      | +10  d1 = width $20
        move.w  #0x19, d2                      | +14  d2 = height $19
        jsr     0x5da9c.l                      | +18  ThunkTarget_05da9c (blitter)
        move.w  #0x0, 0x34(a6)                 | +1e  phase = 0
        lea.l   .Lf3_next(pc), a1              | +24  a1 = &F4
        move.l  a1, (a6)                       | +28
.Lf3_tail:                                     | $08C1D6
        jsr     0x967c0.l                      | +2a
        jsr     0x436de.l                      | +30
        jsr     0x28d70.l                      | +36
        rts                                    | +3c

        .equ    .Lf3_next, Anim_State_F4_08C1EA

        .size   Anim_State_F3_08C1AC, .-Anim_State_F3_08C1AC

| ============================================================================
| F4: Anim_State_F4_08C1EA  @ $08C1EA  (80 B) - LUT stage 2 ($32 channel decay)
| ============================================================================
        .globl  Anim_State_F4_08C1EA
        .type   Anim_State_F4_08C1EA, @function
        .section .text.Anim_State_F4_08C1EA, "ax", @progbits

Anim_State_F4_08C1EA:
        lea.l   0x2c07ac.l, a1                 | +00  a1 = &LUT_2C07AC[0]
        move.w  0x34(a6), d0                   | +06
        lsl.w   #0x1, d0                       | +0a
        move.w  (a1, d0.w), d1                 | +0c
        subq.w  #0x1, d1                       | +10  d1 -= 1 (bias)
        bcc.w   .Lf4_no_clamp0                 | +12  if no borrow, skip
        clr.w   d1                             | +16  else d1 = 0
.Lf4_no_clamp0:                                | $08C202
        move.b  d1, 0x32(a6)                   | +18  field_32 = d1
        addq.w  #0x4, 0x34(a6)                 | +1c  phase += 4
        cmpi.w  #0x3f, 0x34(a6)                | +20
        ble.w   .Lf4_tail                      | +26
        move.b  #0x0, 0x32(a6)                 | +2a  field_32 = 0 (finalized)
        move.w  #0x0, 0x34(a6)                 | +30  phase = 0
        lea.l   .Lf4_next(pc), a1              | +36  a1 = &F5
        move.l  a1, (a6)                       | +3a
.Lf4_tail:                                     | $08C226
        jsr     0x967c0.l                      | +3c
        jsr     0x436de.l                      | +42
        jsr     0x28d70.l                      | +48
        rts                                    | +4e

        .equ    .Lf4_next, Anim_State_F5_08C23A

        .size   Anim_State_F4_08C1EA, .-Anim_State_F4_08C1EA

| ============================================================================
| F5: Anim_State_F5_08C23A  @ $08C23A  (92 B) - LUT stage 3 + trigger events
| ============================================================================
        .globl  Anim_State_F5_08C23A
        .type   Anim_State_F5_08C23A, @function
        .section .text.Anim_State_F5_08C23A, "ax", @progbits

Anim_State_F5_08C23A:
        lea.l   0x2c07ac.l, a1                 | +00  a1 = &LUT_2C07AC[0]
        move.w  0x34(a6), d0                   | +06
        lsl.w   #0x1, d0                       | +0a
        move.w  (a1, d0.w), d1                 | +0c
        move.b  d1, 0x33(a6)                   | +10  field_33 = d1
        addq.w  #0x8, 0x34(a6)                 | +14  phase += 8
        cmpi.w  #0x3f, 0x34(a6)                | +18
        ble.w   .Lf5_tail                      | +1e
                                              | ---- phase >= $40: fire "sound + fade" ----
        move.b  #0x80, d0                      | +22  d0 = $80 (opcode SFX)
        jsr     0x2308.l                       | +26  Sub_00002308(d0) (audio trigger)
        move.w  #0x2, d0                       | +2c  d0 = 2
        jsr     0x5239e.l                      | +30  ThunkTarget_05239e(#2) (fade)
        move.w  #0x3c, 0x70(a6)                | +36  timer_70 = $3C (60 frames)
        move.b  #0x0, 0x33(a6)                 | +3c  field_33 = 0 (finalized)
        lea.l   .Lf5_next(pc), a1              | +42  a1 = &F6
        move.l  a1, (a6)                       | +46
.Lf5_tail:                                     | $08C282
        jsr     0x967c0.l                      | +48
        jsr     0x436de.l                      | +4e
        jsr     0x28d70.l                      | +54
        rts                                    | +5a

        .equ    .Lf5_next, Anim_State_F6_08C296

        .size   Anim_State_F5_08C23A, .-Anim_State_F5_08C23A

| ============================================================================
| F6: Anim_State_F6_08C296  @ $08C296  (34 B) - final timer-decrement stage
| ============================================================================
        .globl  Anim_State_F6_08C296
        .type   Anim_State_F6_08C296, @function
        .section .text.Anim_State_F6_08C296, "ax", @progbits

Anim_State_F6_08C296:
        subq.w  #0x1, 0x70(a6)                 | +00  timer_70--
        cmpi.w  #0x0, 0x70(a6)                 | +04
        bgt.w   .Lf6_tail                      | +0a  if (timer > 0) tail
        clr.b   0x106ed2.l                     | +0e  else clear pending flag
.Lf6_tail:                                     | $08C2AA
        jsr     0x967c0.l                      | +14
        jsr     0x436de.l                      | +1a
        rts                                    | +20
                                              | NB: F6 tail solo tiene 2 jsr (no
                                              | llama a $28d70 como F1-F5). El rts
                                              | queda EXACTAMENTE en el offset esperado.

        .size   Anim_State_F6_08C296, .-Anim_State_F6_08C296
