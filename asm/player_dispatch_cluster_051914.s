| ============================================================================
|  Metal Slug 1 - asm/player_dispatch_cluster_051914.s
|  ----------------------------------------------------------------------------
|  Wave AA batch 1 - #1..#4
|
|  Cluster de 4 helpers contiguos del pipeline de estado por-jugador, en el
|  rango $051914..$051AA3 (294 B netos). Comparte gramatica con el ya
|  matcheado `Player_StateDispatch_0519BE` (Z2#3) y `BCD_AddClamp99999999`
|  (Z2#4), que quedan fisicamente INSERTADOS entre los helpers de esta
|  oleada. Por eso el escaner reportaba "~276 B real" para un unico target
|  en $051914: era un cluster contiguo con dos huecos ya cerrados.
|
|  Layout tras Wave AA batch 1:
|
|     $051914  Player_DispatchStateBySlot_051914       (134 B, AA1 #1)
|     $05199A  Player_BuildTableAddrOnly_05199A        ( 36 B, AA1 #2)
|     $0519BE  Player_StateDispatch_0519BE             ( 82 B, Z2#3, YA)
|     $051A10  BCD_AddClamp99999999_051A10             ( 24 B, Z2#4, YA)
|     $051A28  Player_DispatchOrLoadFromSlot50_051A28  ( 94 B, AA1 #3)
|     $051A86  Player_IncCounterAt7_051A86             ( 30 B, AA1 #4)
|
|  Todos los helpers de la familia:
|    - Salvan a0-a2 con `movem.l a0-a2, -(a7)` (idioma no-ABI GCC).
|    - Seleccionan el ctx del jugador segun bit 7 / valor de `$6E` o `$68`:
|         < 0  -> salida
|         == 0 -> a0 = $106E94  (P1_ctx)
|         != 0 -> a0 = $106E9C  (P2_ctx)
|    - Construyen puntero a la jump-table `$5188C(pc)` de 32 entradas
|      long-word con `(idx & $1F) *4 + 4`.
|    - Ejecutan la triada `bsr $51862; bsr $51A10 (BCD); suba.w #8,a0;
|      bsr $51828` como accion de estado.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|  ----------------------------------------------------------------------------
|  #1  Player_DispatchStateBySlot_051914  @ $051914  (134 B, 1 caller)
|
|  Variante "por slot externo" del dispatcher. Recibe el puntero a un slot
|  candidato en `a1` (uno de $100440 / $1004E0 / $100580 o cualquier otro
|  slot de jugador) y usa `a6` como entity de contexto. Publica el flag P1/P2
|  derivado de `$68(a6)` en `$6E(a1)` y, si el slot pasa un prerrequisito de
|  flag, ejecuta la accion de estado indexada por `$58(a1)` (el idx viene
|  del propio slot que se esta dispatchando, no del a6).
|
|  Firma C conceptual:
|
|      /* Dispatcha la state action de un slot externo `slot`, usando
|       * `self` como entity de contexto (proveedor del signo P1/P2 en
|       * $68). Sale silenciosamente si slot es uno de los 3 slots
|       * "canonicos" $100440/$1004E0/$100580, si $58(slot)==0, si $68(a6)
|       * es negativo, o si el bit 1 de $12(slot) esta apagado. */
|      void Player_DispatchStateBySlot(struct Entity *slot /*a1*/,
|                                      struct Entity *self /*a6*/);
|
|  Idiomas hand-coded incompatibles con GCC:
|    - Tres `cmpa.l #$XXXXXX,a1; beq.w $51998` como un switch sin table.
|    - `move.b d0, $6E(a1)` (byte a offset $6E del slot; publica bit sign).
|    - `movem.l a0-a2, -(a7)` / `movem.l (a7)+, a0-a2` para preservar sobre
|      la triada de bsrs (no ABI GCC).
|    - `move.w d1, -(a7)` para preservar `d1` sobre `bsr $51862` (offset $70
|      de la state-table entry se computa despues).
|    - `bsr.w` a las tres funciones vecinas ($51862, $51A10, $51828), no
|      `jsr.l` (asm hand-coded elige la codificacion mas corta con conocimiento
|      exacto de la distancia).
|  ----------------------------------------------------------------------------

        .globl  Player_DispatchStateBySlot_051914
        .type   Player_DispatchStateBySlot_051914, @function
        .section .text.Player_DispatchStateBySlot_051914, "ax", @progbits

Player_DispatchStateBySlot_051914:
        cmpa.l  #0x100440, a1                  | +00  if (slot == P1_MAIN)
        beq.w   .L1_exit                       | +06     goto exit
        cmpa.l  #0x1004e0, a1                  | +0a  if (slot == P2_MAIN)
        beq.w   .L1_exit                       | +10     goto exit
        cmpa.l  #0x100580, a1                  | +14  if (slot == P3_SLOT)
        beq.w   .L1_exit                       | +1a     goto exit
        move.b  0x58(a1), d1                   | +1e  d1 = slot->state_idx
        beq.w   .L1_exit                       | +22  if (!d1) goto exit
        move.b  0x68(a6), d0                   | +26  d0 = self->player_flag
        bmi.w   .L1_exit                       | +2a  if (d0 < 0) goto exit
        move.b  d0, 0x6e(a1)                   | +2e  slot->player_ctx = d0
        btst.b  #0x1, 0x12(a1)                 | +32  if (!(slot->flags12 & 2))
        beq.w   .L1_exit                       | +38     goto exit
        movem.l a0-a2, -(a7)                   | +3c  push a0-a2
        tst.b   d0                             | +40  d0 == 0 ?
        bne.w   .L1_p2                         | +42  goto P2 branch
        lea.l   0x106e94.l, a0                 | +46  a0 = P1_ctx
        bra.w   .L1_loaded                     | +4c  goto loaded
.L1_p2:
        lea.l   0x106e9c.l, a0                 | +50  a0 = P2_ctx
.L1_loaded:
        lea.l   0x1081b6.l, a1                 | +54  a1 = state_buffer (overwrite)
        move.w  d1, -(a7)                      | +5a  push d1 (preserve state_idx
                                              |             sobre .Lprep)
        bsr.w   .L1_prep                       | +5c  Sub_00051862 (prep)
        move.w  (a7)+, d1                      | +60  restore d1
        lea     .L1_jt(pc), a2                 | +62  a2 = &StateJT[0]
        andi.w  #0x1f, d1                      | +66  d1 &= 0x1F
        add.w   d1, d1                         | +6a  d1 *= 2
        add.w   d1, d1                         | +6c  d1 *= 4 (long stride)
        addq.w  #0x4, d1                       | +6e  d1 += 4 (skip slot 0)
        adda.w  d1, a2                         | +70  a2 = &StateJT[idx+1]
        bsr.w   .L1_bcd                        | +72  BCD_AddClamp99999999
        suba.w  #0x8, a0                       | +76  a0 -= 8 (undo shift)
        bsr.w   .L1_post                       | +7a  Sub_00051828 (post-jt)
        movem.l (a7)+, a0-a2                   | +7e  pop a0-a2
.L1_exit:
        rts                                    | +82

        .equ    .L1_prep, Sub_00051862
        .equ    .L1_jt,   StateJumpTable_05188C
        .equ    .L1_bcd,  BCD_AddClamp99999999_051A10
        .equ    .L1_post, Sub_00051828

        .size   Player_DispatchStateBySlot_051914, .-Player_DispatchStateBySlot_051914


|  ----------------------------------------------------------------------------
|  #2  Player_BuildTableAddrOnly_05199A  @ $05199A  (36 B, 1 caller)
|
|  Variante "solo direccion": calcula el mismo puntero a la state-table
|  `$5188C(pc)` que #1, pero NO ejecuta la state action. Publica el puntero
|  final en `d6` como salida y retorna. Sirve para leer/inspeccionar la
|  entrada de la state-table sin dispararla.
|
|  Nota forense: `movem.l a0-a2, -(a7)` seguido de `movem.l (a7)+, a0-a2`
|  al final es superfluo (solo usa a2), pero el asm-hand-coded reusa la
|  misma silueta que #1 y #3 para uniformidad. Los usan ambos pop/push aunque
|  entre medias solo modifique a2.
|
|  Firma C conceptual:
|
|      /* Devuelve la direccion de la entrada de la state-table para el
|       * slot `slot`, indexada por $58(slot) & $1F, con offset +$4 (skip
|       * primer slot). No dispara ninguna accion. */
|      void *Player_BuildTableAddrOnly(struct Entity *slot /*a1*/,
|                                      struct Entity *self /*a6*/);
|      /* return via d6 (asm-only convencion). */
|  ----------------------------------------------------------------------------

        .globl  Player_BuildTableAddrOnly_05199A
        .type   Player_BuildTableAddrOnly_05199A, @function
        .section .text.Player_BuildTableAddrOnly_05199A, "ax", @progbits

Player_BuildTableAddrOnly_05199A:
        move.b  0x58(a1), d1                   | +00  d1 = slot->state_idx
        move.b  0x68(a6), d0                   | +04  d0 = self->player_flag (no usado)
        movem.l a0-a2, -(a7)                   | +08  push a0-a2
        lea     .L2_jt(pc), a2                 | +0c  a2 = &StateJT[0]
        andi.w  #0x1f, d1                      | +10  d1 &= 0x1F
        add.w   d1, d1                         | +14  d1 *= 2
        add.w   d1, d1                         | +16  d1 *= 4 (long stride)
        addq.w  #0x4, d1                       | +18  d1 += 4 (skip slot 0)
        adda.w  d1, a2                         | +1a  a2 = &StateJT[idx+1]
        move.l  a2, d6                         | +1c  d6 = a2 (publica pointer)
        movem.l (a7)+, a0-a2                   | +1e  pop a0-a2
        rts                                    | +22

        .equ    .L2_jt, StateJumpTable_05188C

        .size   Player_BuildTableAddrOnly_05199A, .-Player_BuildTableAddrOnly_05199A


|  ----------------------------------------------------------------------------
|  #3  Player_DispatchOrLoadFromSlot50_051A28  @ $051A28  (94 B, 1 caller)
|
|  Variante "por sub-slot": lee un puntero a slot desde `$50(a6)` (campo
|  chain-to-child) y, si no es ENTITY_NIL, publica `$68(child) -> $6E(a6)`.
|  Luego reusa el mismo backbone que #1: seleccion P1/P2 por $6E(a6), triada
|  bsr $51862 + BCD + $51828 con `movem.l d0/a0-a2, -(a7)` (preserva un
|  registro de datos EXTRA respecto a #1 y #2). La accion continua a partir
|  de `$051A44` (segunda push de `movem.l d0/a0-a2, -(a7)`), lo que hace
|  esta funcion re-entrable por `$051A44` como ENTRY POINT SECUNDARIO.
|
|  Firma C conceptual:
|
|      /* Dispatcha la state action del sub-slot apuntado por $50(self);
|       * si el sub-slot es ENTITY_NIL, sale silenciosamente. Si el
|       * sub-slot esta presente, copia su $68 al $6E de self antes de
|       * ejecutar el pipeline. */
|      void Player_DispatchOrLoadFromSlot50(struct Entity *self /*a6*/);
|
|  Nota: la etiqueta $51A44 es un segundo entry point. Cuando se salta ahi
|  directamente, el caller ya provee `$6E(a6)` valido y evita la carga
|  desde $50(a6). Se expone como `Player_DispatchOrLoadFromSlot50_ENTRY_051A44`
|  a traves de symbols.py (referencia solo interna, no hay callers externos
|  matcheados aun).
|  ----------------------------------------------------------------------------

        .globl  Player_DispatchOrLoadFromSlot50_051A28
        .type   Player_DispatchOrLoadFromSlot50_051A28, @function
        .section .text.Player_DispatchOrLoadFromSlot50_051A28, "ax", @progbits

Player_DispatchOrLoadFromSlot50_051A28:
        movem.l d0/a0-a2, -(a7)                | +00  push d0/a0-a2
        movea.l 0x50(a6), a0                   | +04  a0 = self->child_slot
        cmpa.l  #0xffffffff, a0                | +08  if (a0 == ENTITY_NIL)
        beq.w   .L3_exit                       | +0e     goto exit
        move.b  0x68(a0), 0x6e(a6)             | +12  self->player_flag =
                                              |         child->player_flag_src
        bra.w   .L3_shared                     | +18  skip alterno; ir al pipeline
                                              |
                                              | ---- entry alterno $51A44 ----
                                              |     (caller inicia AQUI cuando
                                              |      ya tiene $6E(a6) listo:
                                              |      empuja d0/a0-a2 y sigue)
.Lentry_alt_051A44:
        movem.l d0/a0-a2, -(a7)                | +1c  push d0/a0-a2 (entry alt)
.L3_shared:
        tst.b   0x6e(a6)                       | +20  if (self->player_flag < 0)
        bmi.w   .L3_exit                       | +24     goto exit
        bne.w   .L3_p2                         | +28  goto P2 branch
        lea.l   0x106e94.l, a0                 | +2c  a0 = P1_ctx
        bra.w   .L3_loaded                     | +32  goto loaded
.L3_p2:
        lea.l   0x106e9c.l, a0                 | +36  a0 = P2_ctx
.L3_loaded:
        lea.l   0x1081b6.l, a1                 | +3c  a1 = state_buffer
        lea.l   0x1081ba.l, a2                 | +42  a2 = state_buffer + 4
        move.l  d0, (a2)                       | +48  publica d0 en (a2)
        bsr.w   .L3_prep                       | +4a  Sub_00051862 (prep)
        addq.w  #0x4, a2                       | +4e  a2 += 4
        bsr.b   .L3_bcd                        | +50  BCD_AddClamp99999999
        subq.w  #0x8, a0                       | +52  a0 -= 8 (undo shift)
        bsr.w   .L3_post                       | +54  Sub_00051828 (post-jt)
.L3_exit:
        movem.l (a7)+, d0/a0-a2                | +58  pop d0/a0-a2
        rts                                    | +5c

        .equ    .L3_prep, Sub_00051862
        .equ    .L3_bcd,  BCD_AddClamp99999999_051A10
        .equ    .L3_post, Sub_00051828

        .size   Player_DispatchOrLoadFromSlot50_051A28, .-Player_DispatchOrLoadFromSlot50_051A28


|  ----------------------------------------------------------------------------
|  #4  Player_IncCounterAt7_051A86  @ $051A86  (30 B, 1 caller)
|
|  Incrementa el contador byte en `$7(a1)` con clamp inferior a 9 (usando
|  `bcs.w` sobre 10 = "no ha rebasado") y termina con tail-call a
|  `Player_CounterSaturateByte_05170C` (Wave W#11) que aplica el clamp
|  superior definitivo en el buffer global.
|
|  Es la contrapartida "incremento por-slot" del clamp global de W#11: cada
|  frame se llama primero a #4 (incremento en el slot), luego W#11 (clamp
|  saturado en el buffer per-player).
|
|  Firma C conceptual:
|
|      /* Incrementa slot->counter_at7 con clamp a [0..9]; luego llama al
|       * clamp saturado a $FF en el buffer per-player. */
|      void Player_IncCounterAt7(struct Entity *slot /*a1*/);
|  ----------------------------------------------------------------------------

        .globl  Player_IncCounterAt7_051A86
        .type   Player_IncCounterAt7_051A86, @function
        .section .text.Player_IncCounterAt7_051A86, "ax", @progbits

Player_IncCounterAt7_051A86:
        move.b  0x7(a1), d0                    | +00  d0 = slot->counter7
        addq.b  #0x1, d0                       | +04  ++d0
        cmpi.b  #0xa, d0                       | +06  if (d0 < 10)
        bcs.w   .L4_store                      | +0a     goto store
        move.b  #0x9, d0                       | +0e  d0 = 9 (clamp)
.L4_store:
        move.b  d0, 0x7(a1)                    | +12  slot->counter7 = d0
        jsr     .L4_saturate.l                 | +16  Player_CounterSaturateByte
                                              |         (W#11, tail-hook)
        rts                                    | +1c

        .equ    .L4_saturate, Player_CounterSaturateByte_05170C

        .size   Player_IncCounterAt7_051A86, .-Player_IncCounterAt7_051A86
