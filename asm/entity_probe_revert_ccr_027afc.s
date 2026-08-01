| ============================================================================
|  Metal Slug 1 - asm/entity_probe_revert_ccr_027afc.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #5 (cluster probe/collision $027xxx, cuarta variante)
|
|  Entity_ProbeRevertCcr_027AFC  @ $027AFC  (106 bytes, 1 caller)
|
|  Cuarta variante del cluster probe/revert $027xxx (T#7-T#15 + Z#4-Z#6).
|  Comparte con Z#6 la firma "campos $22/$24/$26/$27, sin clamp 10-bit" y
|  con Z#5 la bifurcacion `bcs` (path C=1 = "colision SI"). La colision
|  callback es PC-rel a $272A8 (target propio, distinto de Z#5=$27036 y
|  Z#6=$26B56).
|
|  Ambas ramas restauran los 4 campos en el MISMO orden secuencial
|  (contrastando con Z#5 que las alternaba). Termina con `ori #$11, ccr;
|  rts` en $027B60 = absorbe SetXN_027b60 (Wave N-ccr_helpers). 16 FP.
|
|  Firma C conceptual:
|
|      /* Snapshot de 4 campos ($22, $24, $26, $27) del entity, ejecuta
|       * colision PC-rel $272A8 y bifurca segun CCR (bcs). Ambas ramas
|       * restauran los 4 campos en orden secuencial, ejecutan post-hook
|       * comun $28108, y publican CCR. */
|      /* void */ int Entity_ProbeRevertCcr_027afc(struct Entity *self);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ProbeRevertCcr_027AFC
        .type   Entity_ProbeRevertCcr_027AFC, @function
        .section .text.Entity_ProbeRevertCcr_027AFC, "ax", @progbits

Entity_ProbeRevertCcr_027AFC:
        movea.l #0x108080, a5                  | +00  a5 = BASE arena global
        move.w  0x22(a6), -0x1148(a5)          | +06  scratch[-$1148] = field22
        move.w  0x24(a6), -0x1146(a5)          | +0c  scratch[-$1146] = field24
        move.b  0x26(a6), -0x1144(a5)          | +12  scratch[-$1144] = field26
        move.b  0x27(a6), -0x1143(a5)          | +18  scratch[-$1143] = field27
        jsr     .Lcollision(pc)                | +1e  Sub_000272A8  (colision)
        bcs.w   .Lcolision_si                  | +22  if (C=1) path colision-SI
                                              |
                                              | ---- rama "colision NO": restaura + andi ccr ----
        move.w  -0x1148(a5), 0x22(a6)          | +26  restore field22
        move.w  -0x1146(a5), 0x24(a6)          | +2c  restore field24
        move.b  -0x1144(a5), 0x26(a6)          | +32  restore field26
        move.b  -0x1143(a5), 0x27(a6)          | +38  restore field27
        jsr     .Lposthook(pc)                 | +3e  FUN_00028108
        andi.b  #0xee, ccr                     | +42  CCR &= 0xEE (colision NO publico)
        rts                                    | +46
                                              |
.Lcolision_si:
                                              | ---- rama "colision SI": restaura + ori ccr ----
        move.w  -0x1148(a5), 0x22(a6)          | +48  restore field22
        move.w  -0x1146(a5), 0x24(a6)          | +4e  restore field24
        move.b  -0x1144(a5), 0x26(a6)          | +54  restore field26
        move.b  -0x1143(a5), 0x27(a6)          | +5a  restore field27
        jsr     .Lposthook(pc)                 | +60  FUN_00028108
        ori.b   #0x11, ccr                     | +64  CCR |= 0x11 (colision SI publico)
        rts                                    | +68

        .equ    .Lcollision, Sub_000272A8
        .equ    .Lposthook,  FUN_00028108

        .size   Entity_ProbeRevertCcr_027AFC, .-Entity_ProbeRevertCcr_027AFC
