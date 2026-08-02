| ============================================================================
|  Metal Slug 1 - asm/player_dispatch_3slots_028998.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #2
|
|  Player_Dispatch3Slots_028998  @ $028998  (94 bytes)
|
|  Dispatcher por los 3 slots de jugador del juego ($100440, $1004E0, $100580).
|  Prerrequisitos:
|    - $60(a6) != $FFFF  (link ptr valido)
|    - byte 0 del target ($60(a6)) == $80  (marca de "activo")
|
|  Si algun prerequisito falla, cae en $289F0 (`andi #$EE, ccr; rts`).
|
|  Si todos pasan, invoca $28A96(pc) con cada uno de los tres slots en a1.
|  Termina con `ori #$11, ccr; rts` (path exito) o `andi #$EE, ccr; rts`
|  (path por defecto en $289F0). Absorbe ClearXN_0289f0. 17 FP.
|
|  Firma C conceptual:
|
|      /* Verifica que el link ptr $60(self) apunte a un target activo
|       * (byte 0 == $80), y en tal caso dispatcha llamadas al helper
|       * $28A96 sobre los 3 slots de jugador conocidos en un orden fijo
|       * segun el resultado del probe del slot 1. Retorno por CCR. */
|      /* void */ int Player_Dispatch3Slots(struct Entity *self);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Player_Dispatch3Slots_028998
        .type   Player_Dispatch3Slots_028998, @function
        .section .text.Player_Dispatch3Slots_028998, "ax", @progbits

Player_Dispatch3Slots_028998:
        cmpi.w  #0xffff, 0x60(a6)              | +00  if (self->link == $FFFF)
        beq.w   .Lexit_default                 | +06     goto default
        movea.l 0x60(a6), a0                   | +0a  a0 = *link
        cmpi.b  #0x80, (a0)                    | +0e  if (*a0 != $80)
        bne.w   .Lexit_default                 | +12     goto default
                                              |
                                              | ---- primer helper: slot 3 ($100580) ----
        lea.l   0x100580.l, a1                 | +16  a1 = P3_slot
        jsr     .Lhelper(pc)                   | +1c  Entity_HitboxCollide_028A96
                                              |
                                              | ---- helper sobre slot 1 ($100440) ----
        lea.l   0x100440.l, a1                 | +20  a1 = P1_slot
        jsr     .Lhelper(pc)                   | +26  Entity_HitboxCollide_028A96
        bcs.w   .Lslot1_active                 | +2a  if (C) slot 1 activo
                                              |
                                              | ---- path "slot 1 no": chequea slot 2 ($1004E0) ----
        lea.l   0x1004e0.l, a1                 | +2e  a1 = P2_slot
        jsr     .Lhelper(pc)                   | +34  Entity_HitboxCollide_028A96
        bcc.w   .Lexit_default                 | +38  if (!C) exit default
        bra.w   .Lexit_success                 | +3c  goto success
                                              |
.Lslot1_active:
        movem.l a1, -(a7)                      | +40  push a1 (backup)
        lea.l   0x1004e0.l, a1                 | +44  a1 = P2_slot
        jsr     .Lhelper(pc)                   | +4a  Entity_HitboxCollide_028A96
        movem.l (a7)+, a1                      | +4e  pop a1
                                              |
.Lexit_success:
        ori.b   #0x11, ccr                     | +52  CCR |= 0x11 (success)
        rts                                    | +56
                                              |
.Lexit_default:
        andi.b  #0xee, ccr                     | +58  CCR &= 0xEE (default)
        rts                                    | +5c

        .equ    .Lhelper, Entity_HitboxCollide_028A96

        .size   Player_Dispatch3Slots_028998, .-Player_Dispatch3Slots_028998
