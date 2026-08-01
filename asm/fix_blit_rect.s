| ============================================================================
|  Metal Slug 1 - asm/fix_blit_rect.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #3
|
|  Fix_BlitRectToFixLayer  @ $05da56  (70 bytes, 9 callers)
|
|  Vuelca un rectangulo (w x h) de tiles al Fix Layer del Neo Geo,
|  ubicado en la ventana de I/O empotrada en $3c0000.l (registro
|  MMIO REG_VRAMRW / REG_VRAMADDR expuesto como dos words consecutivos:
|  word-alto = VRAM addr, word-bajo = tile data + palette).
|
|  El Fix Layer usa direcciones "row-major" con incremento vertical
|  (word +$20 cada tile hacia abajo). El clipping vertical acepta solo
|  direcciones de VRAM en [$7000, $74FF] (una franja de 5 filas de 128
|  tiles, tipico HUD / SCORE bar del Fix Layer).
|
|  Entrada (registros absolutos, convencion no-C):
|      d0 : direccion inicial de VRAM (word) - columna
|      d1 : ancho en tiles (w) - se decrementa como contador outer
|      d2 : alto  en tiles (h) - se decrementa como contador outer del
|           bucle mas exterior (SI, hay tres bucles: outer w, middle h,
|           inner - ver notas)
|      a1 : puntero a la fuente de datos de tile (byte stream, +1 por
|           columna)
|
|  Salida: ninguna (retorna via rts). Modifica $3c0000/$3c0002 (MMIO).
|
|  Hallazgos forenses (asm a mano):
|    1. movem.w d3-d4, $3c0000.l  ->  registro-a-memoria absoluta.
|       GCC bare-metal m68k jamas emite movem hacia una direccion
|       absoluta corta; usa move.w individuales o memcpy inline.
|    2. Clipping asimetrico [$7000, $74FF] con cmpi.w sobre d3 tratado
|       como word aunque la fuente sea long (move.l a1,d3 al principio).
|       Este uso mixto de long/word sin extension explicita es firma
|       de asm a mano.
|    3. Preservado extra `move.w d4,-(a7)` DENTRO del bucle medio pero
|       fuera del bucle interno: patron no rederivable por GCC.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 bare-metal + GAS m68k con
|              --register-prefix-optional.
|  ============================================================================

        .text
        .globl  Fix_BlitRectToFixLayer
        .type   Fix_BlitRectToFixLayer, @function
        .section .text.Fix_BlitRectToFixLayer, "ax", @progbits

Fix_BlitRectToFixLayer:
        movem.l d3-d5, -(a7)            | +00  48 e7 1c 00   save d3..d5
        subq.w  #1, d1                  | +04  53 41         d1-- (dbra-style)
        subq.w  #1, d2                  | +06  53 42         d2--
        move.w  d0, d4                  | +08  38 00         d4 = VRAM addr base
        move.l  a1, d3                  | +0a  26 09         d3 = fuente base (long
                                        |                       tratado como word)
        move.w  d1, d5                  | +0c  3a 01         d5 = ancho copia
        move.w  d4, -(a7)               | +0e  3f 04         empuja d4 (base col)
.Linner:
        cmpi.w  #0x74ff, d3             | +10  0c 43 74 ff   d3 > $74FF ?
        bgt.w   .Linner_skip            | +14  6e 00 00 12   si, skip write
        cmpi.w  #0x7000, d3             | +18  0c 43 70 00   d3 < $7000 ?
        blt.w   .Linner_skip            | +1c  6d 00 00 0a   si, skip write
        movem.w d3-d4, 0x3c0000         | +20  48 b9 00 18 00 3c 00 00
                                        |               MMIO write: VRAM
.Linner_skip:
        addq.w  #1, d4                  | +28  52 44         d4++ (siguiente col)
        addi.w  #0x20, d3               | +2a  06 43 00 20   d3 += $20 (row step)
        dbra    d5, .Linner             | +2e  51 cd ff e0   loop d5
        move.w  (a7)+, d4               | +32  38 1f         restore d4
        addi.w  #0x10, d4               | +34  06 44 00 10   d4 += $10 (col group)
        adda.w  #1, a1                  | +38  d2 fc 00 01   a1++
        dbra    d2, .Louter_middle      | +3c  51 ca ff cc   loop d2 (a $05da60)
        movem.l (a7)+, d3-d5            | +40  4c df 00 38   restore d3..d5
        rts                             | +44  4e 75
        .size   Fix_BlitRectToFixLayer, .-Fix_BlitRectToFixLayer

| Etiqueta de destino del dbra d2 (regresa antes del empuje de d4).
| El dbra codifica desplazamiento -$34 = FFCC desde el PC del propio
| dbra+2 ($05da86+2 = $05da88; $05da88 + $ffcc = $05da60), justo la
| instruccion `move.l a1, d3`.
        .set    .Louter_middle, Fix_BlitRectToFixLayer + 0x0a
