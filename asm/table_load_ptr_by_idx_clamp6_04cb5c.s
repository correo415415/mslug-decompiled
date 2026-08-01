| ============================================================================
|  Metal Slug 1 - asm/table_load_ptr_by_idx_clamp6_04cb5c.s
|  ----------------------------------------------------------------------------
|  Wave Y - #5
|
|  Table_LoadPtrByIdxClamp6_04CB5C  @ $04CB5C  (44 bytes)
|
|  Publica en $1081B2 el puntero de una tabla de 6 entradas long-word
|  situada en $04CB44 (PC-rel), indexada por el byte $106ECE. Si el indice
|  es >= 6, el puntero queda como sentinel $FFFFFFFF (que ya se escribio
|  al comienzo, antes de la comprobacion).
|
|  Firma C conceptual:
|
|      /* Carga table[idx] en el slot global; sentinel -1 si idx>=6. */
|      void Table_LoadPtrByIdxClamp6(void);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Se ESCRIBE primero el sentinel $FFFFFFFF y solo despues se comprueba
|       el rango. GCC ordenaria: primero comprobar, luego una unica escritura.
|       Idioma "publish default, patch on success" tipico de asm hand-coded.
|    2. Escalado del indice con dos add.w d0,d0 (x4) en vez de asl.w #2, d0
|       o lsl.w #2, d0. GCC preferiria un shift explicito.
|    3. Tabla accedida por lea.l pc-rel absoluta ($4cb44) con base $04CB76
|       (pc del lea), lo que da desp $FFCC = -52. Esta a 24 bytes ANTES
|       de la propia funcion, dentro de la anterior; caso clasico de
|       datos incrustados en el fragmento de otro helper.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Table_LoadPtrByIdxClamp6_04CB5C
        .type   Table_LoadPtrByIdxClamp6_04CB5C, @function
        .section .text.Table_LoadPtrByIdxClamp6_04CB5C, "ax", @progbits

Table_LoadPtrByIdxClamp6_04CB5C:
        move.l  #0xffffffff, 0x1081b2.l        | +00  global_ptr = -1 (sentinel default)
        moveq   #0x0, d0                       | +0a  d0 = 0
        move.b  0x106ece.l, d0                 | +0c  d0 = idx (byte)
        cmpi.b  #0x6, d0                       | +12  if (idx >= 6)
        bcc.w   .Lout                          | +16     goto .Lout   (unsigned cmp)
        lea     .LTable(pc), a0                | +1a  a0 = &table[0]  (pc-rel, back -52)
        add.w   d0, d0                         | +1e  d0 *= 2
        add.w   d0, d0                         | +20  d0 *= 4  (idx*sizeof(long))
        move.l  (a0, d0.w), 0x1081b2.l         | +22  global_ptr = table[idx]
.Lout:
        rts                                    | +2a

        .equ    .LTable, PtrTable6_04CB44

        .size   Table_LoadPtrByIdxClamp6_04CB5C, .-Table_LoadPtrByIdxClamp6_04CB5C
