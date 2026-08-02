| ============================================================================
|  Metal Slug 1 - asm/entity_probe_scratch_02785c.s
|  ----------------------------------------------------------------------------
|  Wave Z - #4 (cluster probe/collision $027xxx, continuando linea T#7-T#15)
|
|  Entity_Probe_Scratch_02785C  @ $02785C  (48 bytes, 1 caller)
|
|  Variante minimalista del cluster probe/transform ($027xxx). Guarda los
|  campos $22 y $82 del entity (a6) en scratch RAM a5-relativa
|  (-$1148 y -$1146, ambos referidos a a5=$108080), invoca la colision en
|  $44182, restaura ambos campos, aplica clamp `andi.w #$3FF, $82(a6)`
|  (limita el bit-field a 10 bits) e invoca el post-hook comun en $28108
|  (ya expuesto como FUN_00028108).
|
|  Notese: a diferencia de los probe/revert de Wave T#7-T#15, este NO tiene
|  ramas simetricas segun CCR - restaura SIEMPRE. Es una variante "probe
|  informativo" que no bifurca segun resultado.
|
|  Firma C conceptual:
|
|      /* Snapshot campos $22 y $82 del entity, ejecuta colision de
|       * referencia $44182, restaura los campos (siempre), aplica clamp
|       * al bit-field $82 y post-hook comun. Sin bifurcacion segun CCR. */
|      void Entity_Probe_Scratch(struct Entity *self /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Uso de scratch RAM absoluta a5-relativa: `move.w $XX(a6), -$YYYY(a5)`
|       donde a5 = $108080. Idioma clasico de asm hand-coded del juego:
|       el area $10FF6E..$10FF6D (offsets -$1148/-$1146 desde $108080) es
|       un buffer global de scratch compartido. GCC usaria una variable
|       local en pila.
|    2. `andi.w #$3FF, $82(a6)` clampa un word field a 10 bits UTIL
|       ($3FF = 1023). Es un mask post-restore, indica que $82 es una
|       coordenada compacta (probable posX del mundo). Solo tiene sentido
|       aplicarlo tras la restauracion.
|    3. `jsr $28108(pc)` es un jsr PC-relativo corto de 4 B con
|       target = $028108 (FUN_00028108 en symbols.py). GCC habria emitido
|       `jsr $28108.l` de 6 B. La eleccion PC-rel demuestra optimizacion
|       manual del tamano.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_Probe_Scratch_02785C
        .type   Entity_Probe_Scratch_02785C, @function
        .section .text.Entity_Probe_Scratch_02785C, "ax", @progbits

Entity_Probe_Scratch_02785C:
        movea.l #0x108080, a5                  | +00  a5 = BASE arena global
        move.w  0x22(a6), -0x1148(a5)          | +06  scratch[-$1148] = self->field22
        move.w  0x82(a6), -0x1146(a5)          | +0c  scratch[-$1146] = self->field82
        jsr     0x44182.l                      | +12  Sub_00044182 (colision)
        move.w  -0x1148(a5), 0x22(a6)          | +18  self->field22 = scratch[-$1148]
        move.w  -0x1146(a5), 0x82(a6)          | +1e  self->field82 = scratch[-$1146]
        andi.w  #0x3ff, 0x82(a6)               | +24  self->field82 &= 0x3FF (10-bit clamp)
        jsr     .Lposthook(pc)                 | +2a  FUN_00028108 (post-hook comun)
        rts                                    | +2e

        .equ    .Lposthook, Entity_ApplyFadeShade_028108

        .size   Entity_Probe_Scratch_02785C, .-Entity_Probe_Scratch_02785C
