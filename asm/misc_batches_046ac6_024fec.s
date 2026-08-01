| ============================================================================
|  Metal Slug 1 - asm/misc_batches_046ac6_024fec.s
|  ----------------------------------------------------------------------------
|  Wave HH batch 3 - dos helpers heterogeneos de la cola priorizada:
|
|      $046AC6   FixLayer_QuadBatch_046AC6         90 B  4 batches al Fix Layer
|      $024FEC   PlayerCtx_ResetTwoBlocks_024FEC   38 B  reset scratch player
|
|  ---------- FixLayer_QuadBatch_046AC6 --------------------------------------
|
|  Cuatro llamadas consecutivas a $5DA9C (backend blit-tilemap del Fix Layer,
|  ya identificado como helper del cluster CC batch 2) con parametros fijos
|  en (a1, d0, d1, d2):
|
|      #1  a1=$7000  d0=$20  d1=$28  d2=$2     (fila 0, cols  0..40, altura 2)
|      #2  a1=$701E  d0=$20  d1=$28  d2=$2     (fila 0, cols 30..70, altura 2)
|      #3  a1=$7000  d0=$20  d1=$1   d2=$20    (col 0, 1 col, altura 32)
|      #4  a1=$74E0  d0=$20  d1=$1   d2=$20    (col der, 1 col, altura 32)
|
|  Los offsets del Fix Layer ($7000 base + steps de $20 por fila y $1 por
|  columna en tile-map de 40x32 tiles) delatan:
|    - Batches #1/#2 pintan la BARRA superior HUD (dos mitades izq+der).
|    - Batches #3/#4 pintan dos COLUMNAS laterales (izq y der).
|
|  Este es el "frame overlay" del HUD del juego, invocado desde 4 puntos
|  distintos del pipeline. Todos usan el mismo tile-fill code $20 (indice
|  del tile transparente/negro del sfix.sfix).
|
|      /* Rellena el marco HUD del Fix Layer con el tile transparente. */
|      void FixLayer_QuadBatch(void);
|
|  Callers conocidos: 4 (top #4 de scan_unmatched_callees).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. 4 secuencias identicas de 5 instrucciones (movea + 3 move.w + jsr)
|       sin factorizar en un helper compartido. Es una macro asm
|       FILL_FIXLAYER a1, d0, d1, d2 expandida 4 veces.
|    2. `movea.w #$7000, a1` (4 B) usa modo immediate word con sign-extend.
|       GCC habria emitido `lea $7000.w, a1` o `move.l #$7000, a1` (6 B).
|    3. `movea.w #$74E0, a1` en el batch #4: valor $74E0 tiene bit 15
|       encendido, asi que sign-extend a $FFFF74E0. Curiosamente coincide
|       con la direccion "signed" del tile-map en el Fix Layer.
|    4. Todos los `move.w #$20, d0` usan 6-byte encoding immediate word
|       en lugar del `moveq #$20, d0` de 2 bytes (que GCC preferiria).
|       El asm original prefiere consistencia entre los 4 batches.
|
|  ---------- PlayerCtx_ResetTwoBlocks_024FEC --------------------------------
|
|  Reset simetrico de dos bloques scratch de 6 bytes cada uno (2 slots
|  player en $106EB0..$106EB5 y $106EB6..$106EBB), seguido de un gate
|  por `d0` que decide si limpiar el flag $106ECA:
|
|      /* Resetea los 2 slots player-context scratch. Si el player-count
|       * pasado en d0 es < 2, salta a $025012 (contexto multi-jugador
|       * complejo, otra funcion distinta contigua). Si >= 2, marca el
|       * flag ctx_mode = SINGLE en $106ECA y retorna. */
|      void PlayerCtx_ResetTwoBlocks(u16 player_count);
|
|  Callers conocidos: 3 (top #10 de scan_unmatched_callees).
|
|  Los dos bloques (12 bytes en total limpiados por clr.l/clr.w) miden
|  exactamente 6 bytes cada uno, encajando con el layout observado en
|  Wave X#1 (Debug_DrawHUDVars) donde $106EBx aparece como "player-context
|  registers". Cada slot contiene probablemente (u32 stateflags, u16 timer).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Dos secuencias `lea abs.l, a0; clr.l (a0)+; clr.w (a0)` con
|       carga separada del puntero, en lugar de una sola lea con
|       adda + clr repetidos. GCC habria unido las dos leas.
|    2. `bcs.w $25012` con desplazamiento largo (4 B) para saltar solo
|       $A bytes — GCC habria emitido `bcs.b` (2 B). El asm original
|       prefiere consistencia con el cluster $025xxx que usa branches
|       largos como convencion.
|    3. Fall-through a la siguiente funcion ($025012, no cerrada aqui):
|       el `bcs.w` bifurca a $025012 que es una funcion semanticamente
|       distinta pero contigua. Patron "salida alternativa hacia la
|       siguiente funcion" ya visto en Waves T y DD.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  FixLayer_QuadBatch_046AC6  @ $046AC6  (90 bytes)
| ---------------------------------------------------------------------------
|
        .globl  FixLayer_QuadBatch_046AC6
        .type   FixLayer_QuadBatch_046AC6, @function
        .section .text.FixLayer_QuadBatch_046AC6, "ax", @progbits
FixLayer_QuadBatch_046AC6:
        | ---- Batch #1: fila superior izq. ($7000, 40 cols, 2 filas)
        movea.w #0x7000, a1                    | +00  a1 = VRAM $7000
        move.w  #0x20, d0                      | +04  d0 = tile $20 (fill)
        move.w  #0x28, d1                      | +08  d1 = 40 (cols)
        move.w  #0x2, d2                       | +0c  d2 = 2 (rows)
        jsr     ThunkTarget_05da9c                   | +10  fill_tilemap()
        | ---- Batch #2: fila superior der. ($701E, 40 cols, 2 filas)
        movea.w #0x701e, a1                    | +16  a1 = VRAM $701E
        move.w  #0x20, d0                      | +1a  d0 = tile $20
        move.w  #0x28, d1                      | +1e  d1 = 40
        move.w  #0x2, d2                       | +22  d2 = 2
        jsr     ThunkTarget_05da9c                   | +26  fill_tilemap()
        | ---- Batch #3: columna izq. ($7000, 1 col, 32 rows)
        movea.w #0x7000, a1                    | +2c  a1 = VRAM $7000
        move.w  #0x20, d0                      | +30  d0 = tile $20
        move.w  #0x1, d1                       | +34  d1 = 1 (col)
        move.w  #0x20, d2                      | +38  d2 = 32 (rows)
        jsr     ThunkTarget_05da9c                   | +3c  fill_tilemap()
        | ---- Batch #4: columna der. ($74E0, 1 col, 32 rows)
        movea.w #0x74e0, a1                    | +42  a1 = VRAM $74E0
        move.w  #0x20, d0                      | +46  d0 = tile $20
        move.w  #0x1, d1                       | +4a  d1 = 1
        move.w  #0x20, d2                      | +4e  d2 = 32
        jsr     ThunkTarget_05da9c                   | +52  fill_tilemap()
        rts                                    | +58

        .size   FixLayer_QuadBatch_046AC6, .-FixLayer_QuadBatch_046AC6

|
| ---------------------------------------------------------------------------
|  PlayerCtx_ResetTwoBlocks_024FEC  @ $024FEC  (38 bytes)
| ---------------------------------------------------------------------------
|
        .globl  PlayerCtx_ResetTwoBlocks_024FEC
        .type   PlayerCtx_ResetTwoBlocks_024FEC, @function
        .section .text.PlayerCtx_ResetTwoBlocks_024FEC, "ax", @progbits
PlayerCtx_ResetTwoBlocks_024FEC:
        lea.l   0x106eb0.l, a0                 | +00  a0 = &player_ctx[0]
        clr.l   (a0)+                          | +06  ctx0.stateflags = 0
        clr.w   (a0)                           | +08  ctx0.timer      = 0
        lea.l   0x106eb6.l, a0                 | +0a  a0 = &player_ctx[1]
        clr.l   (a0)+                          | +10  ctx1.stateflags = 0
        clr.w   (a0)                           | +12  ctx1.timer      = 0
        cmpi.w  #0x2, d0                       | +14  if (player_count < 2)
        bcs.w   Fn_00025012                    | +18    goto $025012 (fall-thru
                                              |        a funcion vecina, no
                                              |        cerrada aqui)
        move.b  #0x0, 0x106eca.l               | +1c  ctx_mode = SINGLE
        rts                                    | +24

        .size   PlayerCtx_ResetTwoBlocks_024FEC, .-PlayerCtx_ResetTwoBlocks_024FEC
