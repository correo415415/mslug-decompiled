| ============================================================================
|  Metal Slug 1 - asm/fix_blit_stream_05dad8.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #6
|
|  Fix_BlitStream_05DAD8  @ $05DAD8  (66 bytes, 4 callers)
|
|  Blit por stream de opcodes al Fix Layer (tile map de texto en VRAM
|  $3C0000). El stream apuntado por a2 consta de bytes que se interpretan:
|
|      $FE  --> EOF (fin de stream, restaura d1 y rts)
|      $FD  --> nueva linea (restaura d1, incrementa +1)
|      otro --> valor de tile a escribir en la posicion (d1) actual
|
|  El clipping vertical usa el rango [$7000..$74FF] en d1 (offset VRAM):
|  cualquier tile fuera del rango se salta sin escribir. Cada iteracion
|  avanza d1 += $20 (siguiente columna del tile map, 32 tiles/fila).
|
|  Entrada:
|      a2 : puntero al stream de opcodes
|      a1 : offset VRAM base (se copia a d1 al arrancar)
|      d0 : atributos de tile (palette/prio, ORed con el valor del stream)
|
|  Salida:
|      d1 : restaurado al valor original tras EOF
|
|  Firma C conceptual:
|
|      /* Renderiza un stream de tiles al Fix Layer con clipping vertical
|       * y opcodes de control (EOF=$FE, newline=$FD). */
|      void Fix_BlitStream(uint8_t *a2 /*stream*/,
|                          uint16_t base /*a1*/,
|                          uint16_t attrs /*d0*/);
|
|  Notas forenses:
|    1. `movem.w d1-d2, $3C0000.l` para escribir (address, data) al Fix
|       Layer en un solo movem: MMIO con lista de registros que ningun
|       compilador GCC emitiria (tercer caso confirmado del cluster
|       $3C0000, tras W#3 y W#4).
|    2. Uso de -(a7)/(a7)+ para preservar d1 durante el bucle: patron de
|       stack manual sin link/unlk (funcion leaf).
|    3. bra.b $5dadc reentra en `move.w d1,-(a7)` (push d1 al arranque de
|       cada linea), no en el read del stream. Idioma para reiniciar el
|       "cursor de linea" tras un opcode $FD.
|    4. cmpi.w #$74FF (bgt) + cmpi.w #$7000 (blt) es un clipping por
|       rango absoluto de VRAM: los tiles del Fix Layer viven en la fila
|       $7000..$74FF (approx 40 columnas x 32 filas = $500 slots, aunque
|       aqui solo aparecen 20 filas de 32 = 640 slots). Todo lo demas es
|       overscan y no debe escribirse.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Fix_BlitStream_05DAD8
        .type   Fix_BlitStream_05DAD8, @function
        .section .text.Fix_BlitStream_05DAD8, "ax", @progbits

Fix_BlitStream_05DAD8:
        moveq   #0x0, d2                | +00  d2 = 0 (zero-extend del tile)
        move.l  a1, d1                  | +02  d1 = base VRAM
.Lline_start:
        move.w  d1, -(a7)               | +04  push d1 (cursor de linea)
.Lchar_loop:
        move.b  (a2)+, d2               | +06  d2 = *(a2)++  (leer opcode)
        cmpi.b  #0xfe, d2               | +08  ¿ EOF ?
        beq.w   .Ldone                  | +0c  si: salir
        cmpi.b  #0xfd, d2               | +10  ¿ newline ?
        beq.w   .Lnewline               | +14  si: procesar nueva linea
        cmpi.w  #0x74ff, d1             | +18  d1 > $74FF ?
        bgt.w   .Lskip                  | +1c  si: skip clip
        cmpi.w  #0x7000, d1             | +20  d1 < $7000 ?
        blt.w   .Lskip                  | +24  si: skip clip
        or.w    d0, d2                  | +28  d2 |= attrs (palette/prio)
        movem.w d1/d2, 0x3c0000.l       | +2a  MMIO (address=d1, data=d2)
.Lskip:
        addi.w  #0x20, d1               | +32  d1 += 32 (siguiente columna)
        bra.b   .Lchar_loop             | +36  seguir leyendo stream
.Lnewline:
        move.w  (a7)+, d1               | +38  pop d1 (restaurar cursor)
        addq.w  #0x1, d1                | +3a  d1 += 1 (bajar 1 fila)
        bra.b   .Lline_start            | +3c  reiniciar cursor de linea
.Ldone:
        move.w  (a7)+, d1               | +3e  pop d1 (limpiar stack)
        rts                             | +40
        .size   Fix_BlitStream_05DAD8, .-Fix_BlitStream_05DAD8
