| ============================================================================
|  Metal Slug 1 - asm/player_helpers_0336xx_0370xx.s
|  ----------------------------------------------------------------------------
|  Wave NN batch 3 - helpers del cluster player + maquina de animacion grande.
|
|  Contenido (5 funciones, 410 bytes):
|
|      $033638   EntityReset_Long48_Long60_033638    18 B  reset de 2 fields long
|      $03364A   PlayerEntitySpawn_Alt_03364A       106 B  spawn alternativo
|      $0336B4   SchedPublish_033638_A_0336B4         6 B  handler-tail micro
|      $0336C8   SchedPublish_033638_B_0336C8         6 B  handler-tail micro
|      $03705A   PlayerAnimState_03705A             274 B  maquina de animacion
|
|  ---------- Descripcion -------------------------------------------------
|
|  EntityReset_Long48_Long60: reset simple de $48(a6) y $60(a6) a $FFFFFFFF.
|  Es el handler "idle" publicado por SchedPublish_033638_A/B via
|  `move.l #$33638, (a6)` cuando el player entra en estado de reposo.
|
|  PlayerEntitySpawn_Alt: clon de PlayerEntitySpawn_0335A6 (NN#2) con
|  coords diferentes ($78/$1D8 vs $60/$171) y bra.w $33742 al final en
|  lugar de fall-through a SetTaskHandler_033630. 9o par de clones no
|  factorizados del proyecto.
|
|  SchedPublish_033638_A/B: dos handler-tail micro de 6 B cada uno que
|  publican $033638 (EntityReset) en (a6) y saltan al scheduler via
|  jsr $5B6.l + jmp $518.l. 10o par de clones no factorizados del
|  proyecto.
|
|  PlayerAnimState_03705A: maquina de estados de animacion del player
|  con 4 rutas paralelas controladas por flags $8C(a6) bits 1/2 y
|  $78(a6) (tipo de anim), $72(a6) (sub-estado), $70(a6) (sub-sub-estado).
|  Cada ruta publica un handler diferente en (a6) y setea flags en
|  $69(a6). Es la funcion de animacion mas grande decompilada del
|  proyecto hasta la fecha (274 B).
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  EntityReset_Long48_Long60_033638  @ $033638  (18 bytes)
| ---------------------------------------------------------------------------
|
        .globl  EntityReset_Long48_Long60_033638
        .type   EntityReset_Long48_Long60_033638, @function
        .section .text.EntityReset_Long48_Long60_033638, "ax", @progbits
EntityReset_Long48_Long60_033638:
        move.l  #-1, 0x48(a6)                   | +00  field $48 = $FFFFFFFF
        move.l  #-1, 0x60(a6)                   | +06  field $60 = $FFFFFFFF
        rts                                     | +0c

        .size   EntityReset_Long48_Long60_033638, .-EntityReset_Long48_Long60_033638

|
| ---------------------------------------------------------------------------
|  PlayerEntitySpawn_Alt_03364A  @ $03364A  (106 bytes)
|
|  Clon de PlayerEntitySpawn_0335A6 (NN#2) con coords ($78/$1D8) y
|  bra.w $33742 al final. 9o par de clones no factorizados.
| ---------------------------------------------------------------------------
|
        .globl  PlayerEntitySpawn_Alt_03364A
        .type   PlayerEntitySpawn_Alt_03364A, @function
        .section .text.PlayerEntitySpawn_Alt_03364A, "ax", @progbits
PlayerEntitySpawn_Alt_03364A:
        move.w  #0x78, 0x22(a6)                 | +00  x_coord = $78
        move.w  #0x1d8, 0x24(a6)                | +06  y_coord = $1D8
        jsr     Sub_00032AC8(pc)                | +0c  helper local (pc-rel)
        movem.w d2, -(a7)                       | +10  save d2
        move.w  #0x176, d1                      | +14  d1 = sprite_id body
        jsr     ThunkTarget_00236e              | +18  SpriteSetup (body)
        move.w  #0x190, d1                      | +1e  d1 = sprite_id acc1
        jsr     ThunkTarget_00236e              | +22  SpriteSetup (acc1)
        move.w  #0x191, d1                      | +28  d1 = sprite_id acc2
        jsr     ThunkTarget_00236e              | +2c  SpriteSetup (acc2)
        move.w  #0x1b, 0x1c(a6)                 | +32  entity_type = $1B
        jsr     ThunkTarget_0138fe              | +38  load level config
        jsr     Sub_00032A02(pc)                | +3e  helper local (pc-rel)
        bclr.b  #0x4, 0x12(a6)                  | +42  clear pause flag
        ori.w   #0x2, 0x38(a6)                  | +48  set spawn-active flag
        lea.l   TaskTpl_0394A8, a1              | +4e  a1 = &task_tpl
        jsr     ThunkTarget_0004ae              | +54  Task_Alloc
        jsr     ThunkTarget_05dd02              | +5a  Entity_CopyTransform (S)
        jsr     ThunkTarget_0517fe              | +60  init sprites hardware
        bra.w   Sub_00033742                    | +66  bra.w to $33742

        .size   PlayerEntitySpawn_Alt_03364A, .-PlayerEntitySpawn_Alt_03364A

|
| ---------------------------------------------------------------------------
|  SchedPublish_033638_A_0336B4  @ $0336B4  (6 bytes)
| ---------------------------------------------------------------------------
|
        .globl  SchedPublish_033638_A_0336B4
        .type   SchedPublish_033638_A_0336B4, @function
        .section .text.SchedPublish_033638_A_0336B4, "ax", @progbits
SchedPublish_033638_A_0336B4:
        move.l  #EntityReset_Long48_Long60_033638, (a6) | +00  publish idle handler
| Fall-through natural a Jsr5B6ThenJmpScheduler_0336ba (Wave M, ya matcheado):
| $0336BA: jsr $5B6.l; $0336C0: jmp $518.l  (14 B, no se incluyen aqui)

        .size   SchedPublish_033638_A_0336B4, .-SchedPublish_033638_A_0336B4

|
| ---------------------------------------------------------------------------
|  SchedPublish_033638_B_0336C8  @ $0336C8  (6 bytes)
|
|  10o par de clones no factorizados: gemelo byte-a-byte del anterior.
| ---------------------------------------------------------------------------
|
        .globl  SchedPublish_033638_B_0336C8
        .type   SchedPublish_033638_B_0336C8, @function
        .section .text.SchedPublish_033638_B_0336C8, "ax", @progbits
SchedPublish_033638_B_0336C8:
        move.l  #EntityReset_Long48_Long60_033638, (a6) | +00  publish idle handler
| Fall-through natural a Jsr5B6ThenJmpScheduler_0336ce (Wave M, ya matcheado):
| $0336CE: jsr $5B6.l; $0336D4: jmp $518.l  (14 B, no se incluyen aqui)

        .size   SchedPublish_033638_B_0336C8, .-SchedPublish_033638_B_0336C8

|
| ---------------------------------------------------------------------------
|  PlayerAnimState_03705A  @ $03705A  (274 bytes)
|
|  Maquina de estados de animacion del player. 4 rutas paralelas
|  controladas por:
|    $8C(a6) bit 2 -> skip (estado ya gestionado)
|    $8C(a6) bit 1 -> skip (estado ya gestionado)
|    $78(a6) == 8  -> ruta A (sub-estado $72)
|    $78(a6) != 8  -> ruta B (sub-estado $72)
|
|  Cada ruta publica un handler en (a6) via lea + move.l, setea flags
|  en $69(a6) bit 5, y actualiza $70/$72/$7C/$7E/$74 (campos de
|  sub-estado del TCB). Estructura clasica de "state selector con
|  publish de siguiente handler".
| ---------------------------------------------------------------------------
|
        .globl  PlayerAnimState_03705A
        .type   PlayerAnimState_03705A, @function
        .section .text.PlayerAnimState_03705A, "ax", @progbits
PlayerAnimState_03705A:
        btst.b  #0x2, 0x8c(a6)                  | +00  if flag $8C bit 2
        bne.w   .Lanim_ret                      | +06    skip (already handled)
        btst.b  #0x1, 0x8c(a6)                  | +0a  if flag $8C bit 1
        bne.w   .Lanim_ret                      | +10    skip (already handled)
        cmpi.b  #0x8, 0x78(a6)                  | +14  if anim_type != 8
        bne.w   .Lanim_route_b                  | +1a    goto route B
| ---- Ruta A: anim_type == 8 ----
        cmpi.w  #0x3, 0x72(a6)                  | +1e  if sub_state == 3
        beq.w   .Ltramp_ret                     | +24    already at target (via trampoline)
        andi.w  #0xfffe, 0x38(a6)               | +26  clear flag bit 0
        move.w  #0x3, 0x72(a6)                  | +2c  sub_state = 3
        cmpi.b  #0x26, 0x70(a6)                 | +32  if sub_sub == $26
        beq.w   .Lanim_a_state26                | +38    goto state26
.Lanim_a_state25:                               | $037098
        move.w  #0x2, 0x7c(a6)                  | +3c  field $7C = 2
        move.w  #0x2, 0x7e(a6)                  | +42  field $7E = 2
        move.b  #0x25, 0x70(a6)                 | +48  sub_sub = $25
        lea.l   Data_00279B0E, a0               | +4e  a0 = &data $279B0E
        move.l  -0x4(a0), 0x74(a6)              | +54  field $74 = *(a0-4)
        bra.w   .Lanim_a_check79                | +58
.Lanim_a_state26:                               | $0370BA
        move.w  #0x0, 0x7c(a6)                  | +5c  field $7C = 0
        move.w  #0x7, 0x7e(a6)                  | +62  field $7E = 7
        move.b  #0x26, 0x70(a6)                 | +68  sub_sub = $26
        lea.l   Data_00279B04, a0               | +6e  a0 = &data $279B04
        move.l  -0x4(a0), 0x74(a6)              | +74  field $74 = *(a0-4)
.Lanim_a_check79:                               | $0370D8
        cmpi.b  #0x18, 0x79(a6)                 | +78  if field $79 == $18
        bne.w   .Ltramp_ret                     | +7e    skip (via trampoline)
        bset.b  #0x5, 0x69(a6)                  | +82  set flag $69 bit 5
.Ltramp_ret:                                   | $0370E8 (trampoline to .Lanim_ret)
        bra.w   .Lanim_ret                      | +86
| ---- Ruta B: anim_type != 8 ----
.Lanim_route_b:                                 | $0370EC
        cmpi.w  #0x0, 0x72(a6)                  | +8a  if sub_state == 0
        beq.w   .Lanim_ret                      | +90    already at target
        move.w  #0x0, 0x72(a6)                  | +96  sub_state = 0
        cmpi.b  #0x26, 0x70(a6)                 | +9c  if sub_sub == $26
        beq.w   .Lanim_b_state26                | +a2    goto state26
.Lanim_b_state25:                               | $037106
        move.w  #0x0, 0x7c(a6)                  | +a6  field $7C = 0
        move.w  #0x2, 0x7e(a6)                  | +ac  field $7E = 2
        move.b  #0x25, 0x70(a6)                 | +b2  sub_sub = $25
        lea.l   Data_00279B22, a0               | +b8  a0 = &data $279B22
        move.l  -0x4(a0), 0x74(a6)              | +be  field $74 = *(a0-4)
        cmpi.b  #0x0, 0x71(a6)                  | +c4  if field $71 == 0
        bne.w   .Ltramp_b_check                 | +ca    skip (via trampoline)
        ori.w   #0x1, 0x38(a6)                  | +ce  set flag bit 0
.Ltramp_b_check:                               | $037134 (trampoline to .Lanim_b_check79)
        bra.w   .Lanim_b_check79                | +d2
.Lanim_b_state26:                               | $037138
        move.w  #0x0, 0x7c(a6)                  | +d6  field $7C = 0
        move.w  #0x7, 0x7e(a6)                  | +dc  field $7E = 7
        move.b  #0x26, 0x70(a6)                 | +e2  sub_sub = $26
        lea.l   Data_00279B18, a0               | +e8  a0 = &data $279B18
        move.l  -0x4(a0), 0x74(a6)              | +ee  field $74 = *(a0-4)
.Lanim_b_check79:                               | $037156
        cmpi.b  #0x81, 0x79(a6)                 | +f2  if field $79 == $81
        bne.w   .Lanim_ret                      | +f8    skip
        bset.b  #0x5, 0x69(a6)                  | +fc  set flag $69 bit 5
.Lanim_ret:                                     | $037166
        rts                                     | +100

        .size   PlayerAnimState_03705A, .-PlayerAnimState_03705A
