| ============================================================================
|  Metal Slug 1 - asm/entity_dispatch_opcode_nibble_032d00.s
|  ----------------------------------------------------------------------------
|  Wave Y - #6
|
|  Entity_DispatchOpcodeNibble_032D00  @ $032D00  (40 bytes)
|
|  Propaga el byte $87 desde el entity padre (a6->parent en $C(a6)) a este
|  entity, y selecciona un word campo del propio entity indexado por el
|  nibble bajo del opcode via una tabla de 16 word-desplazamientos en
|  $329EE (pc-rel). El word cargado se guarda en $14(a6).
|
|  Firma C conceptual:
|
|      /* Copia parent->opcode ($87) al entity actual, y usa el nibble
|       * bajo del opcode como indice a una tabla de offsets en $329EE.
|       * El word en (a6 + tabla[nibble]) se publica en $14(a6). */
|      void Entity_DispatchOpcodeNibble(struct Entity *self /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Indice a tabla externa via `move.w (a6, d0.w), d1` donde d0 viene
|       de OTRA tabla. Es una indireccion doble (nibble -> field offset ->
|       word value) que GCC no genera sin arrays anidados explicitos.
|    2. La tabla en $329EE contiene desplazamientos DENTRO del entity, no
|       punteros a funcion. Patente de tabla de despacho de campos, no de
|       vtable convencional.
|    3. lsl.w #1, d1 (`e349`) es la variante de rotacion con d5=0 implicito;
|       aqui se emite como `e349` = lsl.w #1,d1 con conteo inmediato de 1.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_DispatchOpcodeNibble_032D00
        .type   Entity_DispatchOpcodeNibble_032D00, @function
        .section .text.Entity_DispatchOpcodeNibble_032D00, "ax", @progbits

Entity_DispatchOpcodeNibble_032D00:
        movea.l 0xc(a6), a0                    | +00  a0 = self->parent
        move.b  0x87(a0), 0x87(a6)             | +04  self->opcode = parent->opcode
        moveq   #0x0, d1                       | +0a  d1 = 0
        move.b  0x87(a6), d1                   | +0c  d1 = opcode
        andi.w  #0xf, d1                       | +10  d1 = opcode & 0x0F
        lsl.w   #0x1, d1                       | +14  d1 *= 2  (index into word table)
        lea     .LOffsetTable(pc), a1          | +16  a1 = &offset_table[0]  (pc-rel)
        move.w  (a1, d1.w), d0                 | +1a  d0 = offset_table[nibble]
        move.w  (a6, d0.w), d1                 | +1e  d1 = *(a6 + d0)  (field word)
        move.w  d1, 0x14(a6)                   | +22  self->field_14 = d1
        rts                                    | +26

        .equ    .LOffsetTable, OpcodeOffsetTable_0329EE

        .size   Entity_DispatchOpcodeNibble_032D00, .-Entity_DispatchOpcodeNibble_032D00
