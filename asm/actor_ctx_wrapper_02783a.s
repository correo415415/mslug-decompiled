| ============================================================================
|  Metal Slug 1 - asm/actor_ctx_wrapper_02783a.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #5
|
|  ActorCtxWrapper_02783a  @ $02783a  (34 bytes, 9 callers)
|
|  Envoltura de invocacion en torno al "actor context block" en $108080.
|  Fija a5 = $108080, ejecuta un preparador ($440e4), preserva d5 y d6
|  con dos movem.w consecutivos de UN SOLO registro cada uno, delega
|  en el procesador central FUN_00028108 y restaura d6/d5 en orden
|  inverso antes del rts.
|
|  El bloque $108080 aparece como base pointer de a5 en varios wrappers
|  contiguos (aqui, $02785c, $027cee...) - es el "contexto del actor
|  activo" del engine. Los campos usados por los callers de a5 son
|  desplazamientos negativos (p.ej. -$1148(a5), -$1146(a5) en $027cee),
|  compatibles con un bloque centrado alrededor de $108080 con extension
|  hacia atras (regs de VRAM y otros MMIO del Neo Geo caen en direcciones
|  bajas cercanas: $3c0000 fix layer, $3c0004 sprite ctrl, etc.).
|
|  Firma C conceptual (no reproducible por GCC 1:1):
|      void ActorCtxWrapper_02783a(void);
|      Efectos: a5 = &g_actor_ctx (=$108080),
|               ejecuta prep_$440e4() y actor_process_$28108(),
|               preservando d5 y d6 alrededor de este ultimo.
|
|  Hallazgos forenses (asm a mano):
|    1. movem.w d5,-(a7)  (opcode 48a7 0400)  seguido de
|       movem.w d6,-(a7)  (opcode 48a7 0200):
|       dos movem con mascara de UN solo bit cada una. GCC habria
|       emitido move.w dN,-(a7) individuales (2 B en lugar de 4 B)
|       o un unico movem con mascara 0600 (2 registros). La eleccion
|       de dos movem separados de 1 registro solo tiene sentido si
|       el codigo se genero desde un macro-assembler con la forma
|       "PUSH.w dN" implementada como movem.w dN,-(sp).
|    2. jsr $440e4.l usa forma abs.l (6 B) mientras que jsr $28108(pc)
|       usa PC-rel corto (4 B), en la misma funcion. Mixing es firma
|       de asm a mano; GCC elige uno u otro globalmente segun flags.
|    3. movea.l #$108080, a5 con constante empotrada: los C-thunks
|       del proyecto usan lea abs.l + carga indirecta, no movea.l#imm.
|  ============================================================================

        .text
        .globl  ActorCtxWrapper_02783a
        .type   ActorCtxWrapper_02783a, @function
        .section .text.ActorCtxWrapper_02783a, "ax", @progbits

ActorCtxWrapper_02783a:
        movea.l #0x108080, a5           | +00  2a 7c 00 10 80 80    a5 = &g_actor_ctx
        jsr     0x440e4                 | +06  4e b9 00 04 40 e4    prep_$440e4()
        movem.w d5, -(a7)               | +0c  48 a7 04 00          push d5 (word)
        movem.w d6, -(a7)               | +10  48 a7 02 00          push d6 (word)
        jsr     .Lactor_process(pc)     | +14  4e ba 08 b8          -> $028108 (PC-rel)
        movem.w (a7)+, d6               | +18  4c 9f 00 40          pop  d6
        movem.w (a7)+, d5               | +1c  4c 9f 00 20          pop  d5
        rts                             | +20  4e 75
        .size   ActorCtxWrapper_02783a, .-ActorCtxWrapper_02783a

| ----- Etiqueta PC-relativa hacia $028108 (FUN_00028108, ya simbolizado).
| Distancia: $028108 - ($02784e + 2) = $028108 - $027850 = $8B8. Cabe
| en desplazamiento con signo 16-bit. GAS usa el simbolo directamente.
        .equ    .Lactor_process, Entity_ApplyFadeShade_028108
