| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Callbacks del op 08 de animación incrustados en el banco de datos alto
|  Región: $19C956 / $19CB32 / $1B6114 / $1B611C  (4 entradas)
| ============================================================================
|
|  A) RESUMEN
|  ----------
|  El op 08 de los scripts de sprite (`08 00 .l rutina`) llama con jsr a una
|  rutina de código. Cuatro de esas rutinas están dentro de los módulos de
|  datos gráficos (CPU $29C956, $29CB32, $2B6114, $2B611C), entre los scripts
|  que las usan; el censo las marcaba como "otro".
|
|   AnimOp08_Sfx28_19c956     d1 = $28 y sigue en JsrAbsThunk_19c95a
|                             (jsr Entity_AllocSpriteSlot_00236E; rts).
|   AnimOp08_MortarShell_19cb32
|                             proyectil de mortero (script $29CBFE): vx =
|                             ±$100 según orientación, ay = $1FE, vy = -$22,
|                             +$2C = 0, vx += rnd(0..$7F) - $3F, y sigue en
|                             SetTaskW_19cb64 (vx -> +$28; rts).
|   AnimOp08_AddY24_1b6114 /  +$24 (Y) += / -= $24.
|   AnimOp08_SubY24_1b611c
|
|  B) EVIDENCIA
|  -----------
|  Destinos de op 08 de scripts recorridos por sprites.walk (dos para
|  $29C956 / $29CB32, uno para cada $2B61xx); desensamblado limpio hasta rts.
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  AnimOp08_Sfx28_19c956  @ $19C956  (4 B)  -> sigue en JsrAbsThunk_19c95a
| ----------------------------------------------------------------------------
        .section .text.AnimOp08_Sfx28_19c956, "ax", @progbits
        .global AnimOp08_Sfx28_19c956
AnimOp08_Sfx28_19c956:
        move.w  #0x28,d1                        | +000

| ----------------------------------------------------------------------------
|  AnimOp08_MortarShell_19cb32  @ $19CB32  (50 B)  -> sigue en SetTaskW_19cb64
| ----------------------------------------------------------------------------
        .section .text.AnimOp08_MortarShell_19cb32, "ax", @progbits
        .global AnimOp08_MortarShell_19cb32
AnimOp08_MortarShell_19cb32:
        move.w  #0xff00,d0                      | +000
        jsr     Entity_NegIfFacing_05dca4       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x1fe,0x2a(a6)                 | +00e
        move.w  #0xffde,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        jsr     RNG_LFSRStep_SelfSeed_05E9B6    | +020
        move.w  #0x7f,d0                        | +026
        subi.w  #0x3f,d0                        | +02a
        add.w   0x28(a6),d0                     | +02e

| ----------------------------------------------------------------------------
|  AnimOp08_AddY24_1b6114  @ $1B6114  (8 B)
| ----------------------------------------------------------------------------
        .section .text.AnimOp08_AddY24_1b6114, "ax", @progbits
        .global AnimOp08_AddY24_1b6114
AnimOp08_AddY24_1b6114:
        addi.w  #0x24,0x24(a6)                  | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  AnimOp08_SubY24_1b611c  @ $1B611C  (8 B)
| ----------------------------------------------------------------------------
        .section .text.AnimOp08_SubY24_1b611c, "ax", @progbits
        .global AnimOp08_SubY24_1b611c
AnimOp08_SubY24_1b611c:
        subi.w  #0x24,0x24(a6)                  | +000
        rts                                     | +006
