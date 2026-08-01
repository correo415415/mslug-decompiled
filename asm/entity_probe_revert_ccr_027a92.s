| ============================================================================
|  Metal Slug 1 - asm/entity_probe_revert_ccr_027a92.s
|  ----------------------------------------------------------------------------
|  Wave Z - #6 (cluster probe/collision $027xxx)
|
|  Entity_ProbeRevertCcr_027A92  @ $027A92  (106 bytes, 1 caller)
|
|  Variante gemela de Entity_ProbeRevertCcr_02788C (Z#5): mismo layout de
|  snapshot y post-hook, pero usa el campo $24 en lugar del $82 y opera
|  contra la colision PC-rel en $26B56 en vez de $27036.
|
|  Difiere ademas en dos detalles clave respecto a Z#5:
|
|    1. La rama de bifurcacion es INVERSA: `bcs` en lugar de `bcc`. Es
|       decir, aqui "C=1" salta al path fallido; en Z#5 "C=0" salta al
|       path exitoso. Los callers de cada uno esperan la convencion opuesta.
|    2. Ninguna rama aplica clamp `andi.w #$3FF, $24`. El campo $24 no
|       necesita el mask 10-bit que si necesitaba $82 (ambos son coords,
|       pero $24 tiene rango mayor o esta pre-clampado por otro helper).
|
|  Firma C conceptual:
|
|      /* Snapshot de 4 campos ($22, $24, $26, $27) del entity, ejecuta
|       * la colision PC-rel en $26B56 y bifurca segun CCR (rama inversa
|       * a Z#5). Ambas ramas restauran los 4 campos y ejecutan el post-hook
|       * comun. Retorna con CCR bit C invertido respecto al branch tomado. */
|      /* void */ int Entity_ProbeRevertCcr_027a92(struct Entity *self /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Simetria casi perfecta con Z#5 pero con field24 en vez de field82
|       y una convencion de retorno CCR opuesta. Es un clon manual del
|       helper vecino - GCC habria factorizado el nucleo comun.
|    2. Ausencia del clamp 10-bit `andi.w #$3FF, ...` en ambas ramas
|       demuestra que el campo $24 tiene tratamiento distinto al $82 en
|       este cluster. Estructura clave para inferir mslug.h:
|
|         struct Entity {
|             ...
|             uint16 field22;     // coord (10-bit clamp en $82)
|             uint16 field24;     // coord (sin clamp aqui)
|             uint8  field26;
|             uint8  field27;
|             ...
|             uint16 field82;     // coord (10-bit clamp obligatorio)
|         };
|    3. `bcs.w` vs `bcc.w` con el mismo template de restore es una tecnica
|       de asm hand-coded para invertir el sentido de la comparacion sin
|       reescribir todo el helper - habria bastado con permutar los dos
|       bloques de restore.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ProbeRevertCcr_027A92
        .type   Entity_ProbeRevertCcr_027A92, @function
        .section .text.Entity_ProbeRevertCcr_027A92, "ax", @progbits

Entity_ProbeRevertCcr_027A92:
        movea.l #0x108080, a5                  | +00  a5 = BASE arena global
        move.w  0x22(a6), -0x1148(a5)          | +06  scratch[-$1148] = self->field22
        move.w  0x24(a6), -0x1146(a5)          | +0c  scratch[-$1146] = self->field24
        move.b  0x26(a6), -0x1144(a5)          | +12  scratch[-$1144] = self->field26
        move.b  0x27(a6), -0x1143(a5)          | +18  scratch[-$1143] = self->field27
        jsr     .Lcollision(pc)                | +1e  Sub_00026B56  (colision)
        bcs.w   .Lcolision_si                  | +22  if (C=1)  path colision-SI
                                              |
                                              | ---- rama "colision NO": restaura en orden
                                              |      secuencial y publica exito (andi ccr) ----
        move.w  -0x1148(a5), 0x22(a6)          | +26  self->field22 (word)
        move.w  -0x1146(a5), 0x24(a6)          | +2c  self->field24 (word)
        move.b  -0x1144(a5), 0x26(a6)          | +32  self->field26 (byte)
        move.b  -0x1143(a5), 0x27(a6)          | +38  self->field27 (byte)
        jsr     .Lposthook(pc)                 | +3e  FUN_00028108
        andi.b  #0xee, ccr                     | +42  CCR &= 0xEE (limpiar X+C)
        rts                                    | +46
                                              |
.Lcolision_si:
                                              | ---- rama "colision SI": restaura en el mismo
                                              |      orden secuencial y publica fallo (ori ccr) ----
        move.w  -0x1148(a5), 0x22(a6)          | +48  self->field22 (word)
        move.w  -0x1146(a5), 0x24(a6)          | +4e  self->field24 (word)
        move.b  -0x1144(a5), 0x26(a6)          | +54  self->field26 (byte)
        move.b  -0x1143(a5), 0x27(a6)          | +5a  self->field27 (byte)
        jsr     .Lposthook(pc)                 | +60  FUN_00028108
        ori.b   #0x11, ccr                     | +64  CCR |= 0x11 (setear X+C)
        rts                                    | +68

        .equ    .Lcollision, Sub_00026B56
        .equ    .Lposthook,  FUN_00028108

        .size   Entity_ProbeRevertCcr_027A92, .-Entity_ProbeRevertCcr_027A92
