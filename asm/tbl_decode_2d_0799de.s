| ============================================================================
|  Metal Slug 1 - asm/tbl_decode_2d_0799de.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity/Sprite helpers) - funcion #9
|
|  Tbl_Decode2D_0799DE  @ $0799DE  (48 bytes, 4 callers)
|
|  Decodifica una entrada 2-D de la tabla apuntada por a0. La primera
|  palabra de la tabla es un magic word que identifica el formato:
|      *a0 == 2   ->  tabla "corta" (14 B header + slots)  -> salta a $79A0E
|      *a0 != 2   ->  tabla "larga" con indice compuesto por 2 dimensiones:
|                       d0 = (entity->flags11 & 3) * 4       (fila, 0/4/8/12)
|                       d0 += ($106ED1 & 2)                  (columna, 0/2)
|                       d1 = Sub_0007_99A4()                 (fila secundaria)
|                       d0 += (d1 & 7) * 16                  (interleave 3-bit)
|                       d0 = *(a0 + d0.w)                    (lookup word)
|                     -> retorna con d0 = valor decodificado
|
|  a0 se avanza 2 B en el cmpi (`cmpi.w #2,(a0)+`), asi que el resto de
|  la funcion opera sobre a0 = table_base+2 (offset de datos).
|
|  Firma C conceptual:
|
|      /* Decodifica una entrada 2-D de la tabla apuntada por a0 usando
|       * dos flags de estado del entity/global como coordenadas. */
|      uint16_t Tbl_Decode2D(struct DecodeTable *a0 /*avanzado 2 B*/,
|                            struct Entity *a6);
|
|  Notas forenses:
|    - cmpi.w #imm,(a0)+ con destino postincrement es una forma que GCC
|      solo emite en contextos muy especificos (tst con puntero). Aqui
|      la usa como "peek + advance" tipico de asm a mano.
|    - d0+=d0 seguido de d0+=d0 (shift by 2 en 2 pasos) donde GCC habria
|      emitido lsl.w #2,d0 (2 B) - la variante con dos add.w cuesta 4 B
|      pero tiene mejor latencia en 68000: 4+4 = 8 ciclos vs shift
|      variable 6+2*n = 10 ciclos. Es una micro-optimizacion manual.
|    - lsl.w #4,d1 seguido de add.w d1,d0 despues de haber usado d1 antes
|      del bsr y RECARGARLO despues es una pista fuerte: Sub_0007_99A4
|      *preserva d0* pero recalcula d1. La captura d0+=d0+d0 antes del
|      bsr es el "primer termino" del indice.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Tbl_Decode2D_0799DE
        .type   Tbl_Decode2D_0799DE, @function
        .section .text.Tbl_Decode2D_0799DE, "ax", @progbits

Tbl_Decode2D_0799DE:
        cmpi.w  #2, (a0)+               | +00  magic == 2 ? (peek + advance)
        beq.w   Tbl_DecodeShort_079A0E  | +04  si -> tabla corta
        move.b  0x11(a6), d0            | +08  d0 = entity->flags11
        andi.w  #3, d0                  | +0c  d0 &= 3  (0..3)
        add.w   d0, d0                  | +10  d0 *= 2
        add.w   d0, d0                  | +12  d0 *= 2  -> d0 = (flags11&3)*4
        move.b  0x106ed1.l, d1          | +14  d1 = global flag byte
        andi.w  #2, d1                  | +1a  d1 &= 2  (0 o 2)
        add.w   d1, d0                  | +1e  d0 += columna
        bsr.b   Sub_0007_99A4           | +20  d1 = subindice (Sub_0007_99A4 preserva d0)
        andi.w  #7, d1                  | +22  d1 &= 7  (0..7)
        lsl.w   #4, d1                  | +26  d1 *= 16
        add.w   d1, d0                  | +28  d0 += fila_secundaria
        move.w  (a0, d0.w), d0          | +2a  d0 = tabla[d0]
        rts                             | +2e
        .size   Tbl_Decode2D_0799DE, .-Tbl_Decode2D_0799DE
