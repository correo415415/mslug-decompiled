| ============================================================================
|  Metal Slug 1 - asm/entity_probe_revert_ccr_02788c.s
|  ----------------------------------------------------------------------------
|  Wave Z - #5 (cluster probe/collision $027xxx)
|
|  Entity_ProbeRevertCcr_02788C  @ $02788C  (118 bytes, 1 caller)
|
|  Variante extendida del cluster probe/revert. Guarda CUATRO campos del
|  entity (a6) en scratch RAM (offsets $22, $82, $26, $27), invoca la
|  colision PC-rel en $27036 y bifurca segun CCR:
|
|    - Si bcc (colision NO, path exitoso): restaura los 4 campos y termina
|      con `ori #$11, ccr` (setea X y C bits del CCR = "fallo publico").
|    - Si bcs (colision SI, path fallido): restaura los 4 campos en OTRO
|      orden (word primero, byte despues, alternando entre los dos pares) y
|      termina con `andi #$EE, ccr` (limpia X y C bits = "exito publico").
|
|  Ambas ramas hacen `andi.w #$3FF, $82(a6)` (clamp 10-bit) y `jsr $28108(pc)`
|  (post-hook comun) antes del `ori/andi ccr; rts`. La inversion del CCR
|  respecto a la logica visible es intencional: el bit C se usa como flag
|  de retorno para el caller y las ramas lo publican con signo opuesto al
|  del branch que las selecciono. Idioma clasico ya visto en Wave T#7-T#15.
|
|  Firma C conceptual:
|
|      /* Snapshot de 4 campos ($22, $82, $26, $27) del entity, ejecuta
|       * la colision PC-rel en $27036 y bifurca segun CCR. Ambas ramas
|       * restauran los 4 campos, aplican clamp 10-bit al $82 y ejecutan
|       * el post-hook comun. Retorna con CCR bit C invertido respecto al
|       * branch tomado (protocolo "invert CCR" del cluster T#7-T#15). */
|      /* void */ int Entity_ProbeRevertCcr_02788c(struct Entity *self /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Retorno por CCR con inversion explicita: `ori #$11, ccr` (path
|       aparentemente exitoso publica FALLO) y `andi #$EE, ccr` (path
|       aparentemente fallido publica EXITO). GCC devuelve el resultado
|       en d0.
|    2. La rama "colision SI" restaura los 4 campos en ORDEN DIFERENTE al
|       de la rama "colision NO": alterna `$22 $26 $82 $27` en vez del
|       secuencial `$22 $82 $26 $27`. GCC generaria el mismo orden en
|       ambas ramas o factorizaria el restore en una funcion comun.
|    3. `jsr $27036(pc)` es un jsr PC-relativo con offset negativo ($F78A
|       = -2166) - target antes de la propia funcion. Idioma "callback
|       hacia atras" hand-coded.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ProbeRevertCcr_02788C
        .type   Entity_ProbeRevertCcr_02788C, @function
        .section .text.Entity_ProbeRevertCcr_02788C, "ax", @progbits

Entity_ProbeRevertCcr_02788C:
        movea.l #0x108080, a5                  | +00  a5 = BASE arena global
        move.w  0x22(a6), -0x1148(a5)          | +06  scratch[-$1148] = self->field22
        move.w  0x82(a6), -0x1146(a5)          | +0c  scratch[-$1146] = self->field82
        move.b  0x26(a6), -0x1144(a5)          | +12  scratch[-$1144] = self->field26
        move.b  0x27(a6), -0x1143(a5)          | +18  scratch[-$1143] = self->field27
        jsr     .Lcollision(pc)                | +1e  Sub_00027036  (colision)
        bcc.w   .Lcolision_no                  | +22  if (!C)  path NO-colision
                                              |
                                              | ---- rama "colision SI": restaura en orden
                                              |      alternado y publica exito (andi ccr) ----
        move.w  -0x1148(a5), 0x22(a6)          | +26  self->field22 (word)
        move.w  -0x1146(a5), 0x82(a6)          | +2c  self->field82 (word)
        move.b  -0x1144(a5), 0x26(a6)          | +32  self->field26 (byte)
        move.b  -0x1143(a5), 0x27(a6)          | +38  self->field27 (byte)
        andi.w  #0x3ff, 0x82(a6)               | +3e  self->field82 &= 0x3FF
        jsr     .Lposthook(pc)                 | +44  FUN_00028108
        ori.b   #0x11, ccr                     | +48  CCR |= 0x11 (setear X+C)
        rts                                    | +4c
                                              |
.Lcolision_no:
                                              | ---- rama "colision NO": restaura en orden
                                              |      alternado word/byte y publica fallo
                                              |      (ori ccr) ----
        move.w  -0x1148(a5), 0x22(a6)          | +4e  self->field22 (word)
        move.b  -0x1144(a5), 0x26(a6)          | +54  self->field26 (byte)
        move.w  -0x1146(a5), 0x82(a6)          | +5a  self->field82 (word)
        move.b  -0x1143(a5), 0x27(a6)          | +60  self->field27 (byte)
        andi.w  #0x3ff, 0x82(a6)               | +66  self->field82 &= 0x3FF
        jsr     .Lposthook(pc)                 | +6c  FUN_00028108
        andi.b  #0xee, ccr                     | +70  CCR &= 0xEE (limpiar X+C)
        rts                                    | +74

        .equ    .Lcollision, Sub_00027036
        .equ    .Lposthook,  Entity_ApplyFadeShade_028108

        .size   Entity_ProbeRevertCcr_02788C, .-Entity_ProbeRevertCcr_02788C
