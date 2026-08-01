| ============================================================================
|  Metal Slug 1 - asm/debug_draw_hud_vars_096a80.s
|  ----------------------------------------------------------------------------
|  Wave X (post-allocator: HUD debug + comparadores + arranque) - funcion #1
|
|  Debug_DrawHUDVars_096A80  @ $096A80  (164 bytes, 2 callers)
|
|  Vuelca 7 variables globales del engine al Fix Layer (tile-map de texto
|  del Neo Geo en $3C0000) como 4 chars hex ASCII cada una. Solo se ejecuta
|  cuando el bit 0 del byte $100001 esta activo (probablemente un DIP
|  switch de debug o el flag "SLOT DEBUG MODE" del MVS BIOS).
|
|  Variables volcadas (coordenada VRAM (col base) -> global):
|      $7146  <- $106E88.w       (probable frame counter)
|      $72E3  <- $106F5C.w       (probable RNG state)
|      $7383  <- $106F50 hi.w    (timer high word)
|      $7423  <- $106F54 hi.w    (otro timer high word)
|      $7384  <- $106F50 lo.w    (via helper $5D8F2 + $5D6C2)
|      $7424  <- $106F54 lo.w    (via helper $5D8F2 + $5D6C2)
|      $7385  <- $10E39C.b       (probable "current level")
|
|  El helper $5D8F2 aparece llamado entre dos `andi.l #$ffff, d0` y el
|  jsr a $5D6C2 (Sprite_HexFormat4, W#3). Probablemente prepara la
|  posicion VRAM en a1 antes del blit; sera objetivo de la Wave X#2.
|
|  Firma C conceptual:
|
|      /* Renderiza el HUD de debug con las 7 variables globales del
|       * engine al Fix Layer. Solo activo si DEBUG_FLAG en $100001.b0 = 1. */
|      void Debug_DrawHUDVars(void);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. movea.l/movea.w con immediate largo (#$7146, #$72E3, etc.) para
|       cargar direcciones VRAM fijas en a1. GCC habria usado `lea $7146.w,
|       a1` (4 B) o `lea abs_addr, a1` (6 B). Aqui se usa `movea` (6 B) por
|       consistencia estilistica con los offsets `$72e3` (word que se
|       sign-extend a $ffff_72e3 al meterse en a1). Este es un truco
|       forense: los tile map offsets del Fix Layer son negativos con
|       signo cuando el word tiene bit 15 activo. GCC no lo emitiria por
|       falta de bit-exactness con lea.w.
|    2. swap d0 explicito entre carga del long y jsr al backend hex,
|       tres veces seguidas. GCC habria emitido bit-shift o direct
|       high-word address. Aqui `swap d0` es el opcode preferido en 68000
|       para intercambiar high/low de un data register (2 B, 4 ciclos).
|    3. Doble jsr `$5D8F2 + $5D6C2` intercalado con `andi.l #$ffff` en dos
|       sitios (posicion +$5E y +$82). Idioma "prepara-y-blit" que
|       reutiliza W#3 como backend puro sin params extra en registros.
|    4. btst.b #0, $100001.l al arranque como gate: es el idioma clasico
|       de "debug mode enable" del Neo Geo. La direccion $100001 es el
|       byte alto de $100000 (Backup RAM del cartucho MVS), donde el
|       BIOS y la aplicacion pueden intercambiar flags.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Debug_DrawHUDVars_096A80
        .type   Debug_DrawHUDVars_096A80, @function
        .section .text.Debug_DrawHUDVars_096A80, "ax", @progbits

Debug_DrawHUDVars_096A80:
        btst.b  #0x0, 0x100001.l              | +00  ¿ DEBUG_FLAG activo ?
        beq.w   .Ldone                         | +08  no: skip todo el HUD
        movea.l #0x7146, a1                    | +0c  a1 = VRAM offset "frame counter"
        move.w  0x106e88.l, d0                 | +12  d0 = frame counter
        jsr     Sprite_HexFormat4_05D6C2       | +18  (W#3) formatea 4 hex chars
        movea.w #0x72e3, a1                    | +1e  a1 = VRAM "RNG state"
        move.w  0x106f5c.l, d0                 | +22  d0 = RNG state
        jsr     Sprite_HexFormat4_05D6C2       | +28  (W#3)
        movea.w #0x7383, a1                    | +2e  a1 = VRAM "timer0 hi"
        move.l  0x106f50.l, d0                 | +32  d0 = timer0_long
        swap    d0                              | +38  d0 = hi word
        jsr     Sprite_HexFormat4_05D6C2       | +3a  (W#3)
        movea.w #0x7423, a1                    | +40  a1 = VRAM "timer1 hi"
        move.l  0x106f54.l, d0                 | +44  d0 = timer1_long
        swap    d0                              | +4a  d0 = hi word
        jsr     Sprite_HexFormat4_05D6C2       | +4c  (W#3)
        movea.w #0x7384, a1                    | +52  a1 = VRAM "timer0 lo"
        move.l  0x106f50.l, d0                 | +56  d0 = timer0_long
        swap    d0                              | +5c  swap para obtener hi..
        andi.l  #0xffff, d0                    | +5e  ..y limpiar a solo lo/hi (word)
        jsr     Sub_00005D8F2                  | +64  helper "prep VRAM/params"
        jsr     Sprite_HexFormat4_05D6C2       | +6a  (W#3)
        movea.w #0x7424, a1                    | +70  a1 = VRAM "timer1 lo"
        move.l  0x106f54.l, d0                 | +74  d0 = timer1_long
        swap    d0                              | +7a
        andi.l  #0xffff, d0                    | +7c
        jsr     Sub_00005D8F2                  | +82  helper
        jsr     Sprite_HexFormat4_05D6C2       | +88  (W#3)
        movea.w #0x7385, a1                    | +8e  a1 = VRAM "current level"
        move.b  0x10e39c.l, d0                 | +92  d0 = current_level (byte)
        andi.w  #0xff, d0                      | +98  d0 &= 0xFF (zero-extend)
        jsr     Sprite_HexFormat4_05D6C2       | +9c  (W#3)
.Ldone:
        rts                                    | +a2
        .size   Debug_DrawHUDVars_096A80, .-Debug_DrawHUDVars_096A80
