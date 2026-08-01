| ============================================================================
|  Metal Slug 1 - asm/table_lookup.s
|  ----------------------------------------------------------------------------
|  Wave S (Entity/Sprite helpers) - funcion #3
|
|  Table_LookupPointerBounded  @ $000772  (64 bytes, 35 callers)
|
|  Resolvedor generico de tabla de punteros indexada por d0.w, con
|  guardia de cota superior y assertion via trap #15.
|
|  Entrada (registros absolutos, convencion no-C):
|      a0 : puntero base a una tabla de punteros de 32 bits
|      a1 : cota superior de la tabla, o ENTITY_NIL (0xFFFFFFFF) para
|           desactivar la comprobacion de rango
|      a6 : puntero a la entidad actual (destino del write final)
|      d0 : indice (word) - se multiplica por 4 al entrar y luego se
|           usa como word-index en (a0,d0.w) para leer el puntero
|
|  Salida:
|      a1 : puntero leido de tabla[indice]
|      (a6): se escribe a1 solo si a1 != ENTITY_NIL
|
|  Firma C conceptual (no reproducible por GCC 1:1):
|      void Table_LookupPointerBounded(
|          struct Entity *a6,   /* entidad destino, campo +0 */
|          void         **a0,   /* tabla base */
|          void          *a1,   /* cota superior o ENTITY_NIL */
|          unsigned short d0    /* indice (word) */);
|
|  Hallazgos forenses (asm a mano):
|    1. nop de padding intercalados en $78c/$78e/$792 - un compilador
|       jamas emite nops asi.
|    2. cmpa.l a0,a1 duplicado en $786 y $790 con nops entre medias.
|    3. cmpa.l #-1,a1 duplicado en $79a y $7a4 con beq detras de cada uno.
|    4. trap #15 (opcode 0x4E4F, vector 47) como assertion fatal de rango.
|  ============================================================================

        .text
        .globl  Table_LookupPointerBounded
        .type   Table_LookupPointerBounded, @function
        .section .text.Table_LookupPointerBounded, "ax", @progbits

Table_LookupPointerBounded:
        lsl.w   #2, d0                 | +00  d0 = index * 4
        cmpa.l  #-1, a1                | +02  a1 == ENTITY_NIL ?
        beq.w   .Ldo_lookup             | +08  si, saltar guardia de rango
        andi.l  #0xffff, d0            | +0c  d0 &= 0xFFFF (word)
        adda.l  d0, a0                 | +12  a0 += d0 (avanzar hasta slot)
        cmpa.l  a0, a1                 | +14  a1 > a0 ?
        bgt.w   .Ldo_lookup             | +16  si, dentro de rango
        nop                             | +1a  padding manual
        nop                             | +1c  padding manual
        cmpa.l  a0, a1                 | +1e  re-check (deliberado)
        nop                             | +20  padding manual
        trap    #15                     | +22  ASSERTION: out of bounds
.Ldo_lookup:
        movea.l (a0, d0.w), a1         | +24  a1 = *(a0 + d0.w)
        cmpa.l  #-1, a1                | +28  a1 == ENTITY_NIL ?
        beq.w   .Lret                   | +2e  si, salir sin escribir
        cmpa.l  #-1, a1                | +32  re-check duplicado
        beq.w   .Lret                   | +38  si, salir sin escribir
        move.l  a1, (a6)               | +3c  entity[0] = a1
.Lret:
        rts                             | +3e
        .size   Table_LookupPointerBounded, .-Table_LookupPointerBounded
