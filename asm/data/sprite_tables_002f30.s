| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching — DATOS
|  Wave KKKKK
|  Región: $002F30..$0133B0  (66,688 B en 1 hueco(s), 3 entradas)
| ============================================================================
|
|  A. QUÉ HAY AQUÍ
|   Tablas de conversión de color del motor de paletas (video/palette_engine_
|     sprite_grid_0133b0.s). Las tres se ensamblan con .rept a partir de su
|     fórmula, sin datos literales:
|       $002F30..$012F30  ColorRGB555ToNeoGeo_002f30[32768]: palabra de color
|                         Neo Geo (bit bajo de cada canal en b14..b12, 4 bits
|                         altos en b11..b0) para cada RGB555 de 15 bits.
|       $012F30..$012FB0  ColorRamp_RowPtr_012f30[32]: punteros a las filas.
|       $012FB0..$0133B0  ColorRamp_012fb0[32][32]: fila k, columna v = v*k/31
|                         (división entera). Escala cada canal 0..31 en el
|                         fundido de paletas.
|
|  B. EVIDENCIA
|   `lea ColorRGB555ToNeoGeo_002f30(pc)` en PalSlot_UpdateAll_002c86 (boot) y
|     `lea $2F30.l` en Pal_LoadRaw16_0133b0 / PalAnim_Blend16_013480: d4 =
|     r<<10|g<<5|b, se lee la palabra y se escribe en la paleta en RAM.
|     `lea ColorRamp_RowPtr_012f30(pc)` + `movea.l (a0,d4.w)` + `move.b
|     (a0,d5.w)` en PalAnim_StepRGB_013504 y Pal_ApplyFade*_013624/013694.
|     Las 32768 + 32 + 1024 entradas se comprobaron contra la fórmula.
|
|  C. HIPÓTESIS / NOTAS
|   Antes el módulo tenía 428 etiquetas Data_/Str_/Zero_ dentro de las
|     tablas: eran valores de otros módulos (.dc.l del bytecode de animación,
|     etc.) que caían en este rango por casualidad. Se quitaron y esos .dc.l
|     quedan como número literal; no son punteros.
|
|  D. FORMATOS
|   ColorRGB555ToNeoGeo: .dc.w por índice r<<10|g<<5|b; en GAS m68k `|` abre
|     comentario, por eso los campos de bits se suman con `+`.
|     ColorRamp_RowPtr: .dc.l ColorRamp_012fb0 + 32*k.
|     ColorRamp: 32 filas de 32 B (1 B por valor 0..31).
|
|  E. HELPERS / USUARIOS
|   Pal_LoadRaw16_0133b0, PalAnim_Blend16_013480, PalAnim_StepRGB_013504,
|     Pal_ApplyFadeDarken/Lighten_013624/013694 (video/palette_engine_sprite_
|     grid_0133b0.s) y PalSlot_UpdateAll_002c86 (boot).
|   Rutinas con lea/move.l #abs a este módulo (6):
|     PalAnim_StepRGB_013504×3, PalSlot_UpdateAll_002c86×1,
|     Pal_LoadRaw16_0133b0×1, PalAnim_Blend16_013480×1,
|     Pal_ApplyFadeDarken_013624×1, Pal_ApplyFadeLighten_013694×1
|
|  F. ESTADO
|   Matching byte a byte (build_rom ✔). Censo (data_census.py, 66688 B):
|     tablas por fórmula 100.0%.
|   Entradas por prefijo: ColorRamp 2, ColorRGB555ToNeoGeo 1.
|   Pendiente: nada (módulo decodificado al 100 %).
|
        .text

| ----------------------------------------------------------------------------
|  ColorRGB555ToNeoGeo_002f30  @ $002F30  (65536 B)  [ref. desde código]
|  32768 palabras: índice i = r<<10 | g<<5 | b (5 bits por canal) ->
|  color Neo Geo: bits bajos r,g,b en b14..b12, 4 bits altos en b11..b0.
| ----------------------------------------------------------------------------
        .section .text.ColorRGB555ToNeoGeo_002f30, "ax", @progbits
        .global ColorRGB555ToNeoGeo_002f30
ColorRGB555ToNeoGeo_002f30:
        .set    .Li, 0
        .rept   32768
        .set    .Lr, .Li >> 10
        .set    .Lg, (.Li >> 5) & 31
        .set    .Lb, .Li & 31
        .dc.w   ((.Lr & 1) << 14) + ((.Lg & 1) << 13) + ((.Lb & 1) << 12) + ((.Lr >> 1) << 8) + ((.Lg >> 1) << 4) + (.Lb >> 1)
        .set    .Li, .Li + 1
        .endr

| ----------------------------------------------------------------------------
|  ColorRamp_RowPtr_012f30  @ $012F30  (128 B)  [ref. desde código]
|  32 punteros a las filas de ColorRamp_012fb0 (fila k = brillo k/31).
|  Uso: d4 = 4*k; movea.l (RowPtr,d4.w),a0; move.b (a0,v.w),d4.
| ----------------------------------------------------------------------------
        .section .text.ColorRamp_RowPtr_012f30, "ax", @progbits
        .global ColorRamp_RowPtr_012f30
ColorRamp_RowPtr_012f30:
        .set    .Lk, 0
        .rept   32
        .dc.l   ColorRamp_012fb0 + 32 * .Lk
        .set    .Lk, .Lk + 1
        .endr

| ----------------------------------------------------------------------------
|  ColorRamp_012fb0  @ $012FB0  (1024 B)
|  ColorRamp[k][v] = v*k/31 (división entera), k,v = 0..31.
| ----------------------------------------------------------------------------
        .section .text.ColorRamp_012fb0, "ax", @progbits
        .global ColorRamp_012fb0
ColorRamp_012fb0:
        .set    .Lk, 0
        .rept   32
        .set    .Lv, 0
        .rept   32
        .dc.b   (.Lv * .Lk) / 31
        .set    .Lv, .Lv + 1
        .endr
        .set    .Lk, .Lk + 1
        .endr
