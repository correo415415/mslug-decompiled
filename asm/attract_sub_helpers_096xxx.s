| ============================================================================
|  Metal Slug 1 - asm/attract_sub_helpers_096xxx.s
|  ----------------------------------------------------------------------------
|  Wave HH batch 2 - cluster de sub-helpers attract en $096xxx.
|
|  Contenido (6 funciones + 1 tabla, 404 bytes):
|
|      $0967C0   SelectPositive_TwoSlots_0967C0    62 B  min-magnitud 2-slot
|      $0969C2   AttractCuller_Cam0_0969C2         76 B  cull+blit cam0
|      $096A0E   AttractCuller_Cam1_096A0E         76 B  cull+blit cam1 (clon)
|      $096A5A   Viewport_CoordToScreen_096A5A     38 B  coord->pantalla
|      $096B24   DebugTriggers_TwoBits_096B24      90 B  DIP bits 2/7
|      $096B7E   ClampAndLookup8_096B7E      30+32=62 B  clamp[0..7] + jmp-tbl
|
|  Todos son sub-helpers del cluster attract cuyos consumidores directos ya
|  estan matcheados en Wave GG (7 handlers state $096xxx). SelectPositive es
|  llamado 6 veces (top #3 de scan_unmatched_callees, arrastrado por las 6
|  ATTRACT_STATE macros de Wave GG batch 1).
|
|  ---------- SelectPositive_TwoSlots_0967C0 ---------------------------------
|
|  Recupera min-magnitud componentwise sobre los slots player P1/P2 en
|  $100440/$1004E0. La funcion $5E2D8 (aun unmatched) computa un par (d0,d1)
|  a partir de un slot player; SelectPositive elige, POR CADA componente
|  (d0 y d1 separados), el valor POSITIVO entre P1 y P2. Es decir:
|
|      /* Fusiona los resultados de $5E2D8 sobre P1 y P2, prefiriendo por
|       * cada componente el valor positivo si el otro es negativo. */
|      Pair SelectPositive_TwoSlots(void) {
|          Pair p1 = fn_5E2D8(&slot_P1);   /* (d0, d1) */
|          Pair p2 = fn_5E2D8(&slot_P2);   /* (d0, d1) */
|
|          long x = p2.d0;       /* actual = P2 */
|          if (x < 0) x = p1.d0; /* si P2 negativo, prefiere P1 */
|          if (p1.d0 < 0) p1.d0 = x;  /* si P1 negativo, sustituye por x */
|          long y = p2.d1;
|          if (y < 0) y = p1.d1;
|          if (p1.d1 < 0) p1.d1 = y;
|          return (Pair){p1.d0, p1.d1};
|      }
|
|  Semantica de la funcion resultante: "toma el jugador VIVO (con coordenada
|  positiva) para calcular la coordenada del follower/HUD de attract mode".
|
|  Callers conocidos: 6 (top #3 de scan_unmatched_callees), invocada por los
|  handlers ATTRACT_STATE de Wave GG batch 1 (Attract_State0/1/2/3/5/7).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Doble jsr $5E2D8 con `move.l d0,d2; move.l d1,d3` intermedio para
|       preservar (d0,d1) tras el primer jsr. GCC habria usado la pila o
|       registros callee-saved distintos, y el idioma "usar d2/d3 como
|       backup de d0/d1" es inequivocamente hand-coded.
|    2. 4 tests `tst.l dX; bpl.w .Lskip; move.l dY, dX` con `bpl.w` de
|       desplazamiento 4 bytes (formato largo cuando bastaba `bpl.b` de
|       2 B). GCC habria elegido siempre la forma corta. El bpl.w es
|       un tell de asm hand-coded.
|    3. Los cuatro pares tst/bpl/move estan simetricamente entrelazados
|       (d0<->d2, d2<->d0, d1<->d3, d3<->d1) — patron de "commutar
|       elegido y descarte" que en C se escribiria con dos MAX en lugar
|       de este intercambio manual.
|
|  ---------- AttractCuller_Cam0_0969C2 / _Cam1_096A0E -----------------------
|
|  Dos culler+blit clonicos de la lista de sprites del attract. Cada uno
|  recorre una lista de "attract sprite descriptors" (14 B por entrada:
|  centinela $FFFF al final) referenciada por el opcode `$21(a6)` del task
|  en curso, y por cada entrada:
|
|    1. Test de viewport: `x = entry.x - (camera_x_hi + $140)`; si x>=0
|       (fuera de pantalla) skip.
|    2. Blit: llama a $5DCCE con a1 apuntando a la entrada.
|    3. Avanza el cursor `$70(a6) += $14` (siguiente entrada).
|    4. Loop hasta encontrar `$FFFF` (centinela).
|
|  Cam0 usa camera_x en $106F50 (sistema principal), Cam1 usa camera_x en
|  $106F5C (sistema secundario). Es la MISMA funcion escrita DOS VECES
|  con la unica diferencia del slot de camera. Patron "no factoriza"
|  ya visto en Wave BB batch 2 (dos frame selectors contiguos $057044/
|  $05707A) y Wave Z#5/#6 (Entity_ProbeRevertCcr gemelas).
|
|      /* Cull+blit de la lista de sprites de attract del task actual,
|       * usando la camera N (N=0 o 1). */
|      void AttractCuller_CamN(void);
|
|  Callers conocidos: 1 cada uno (invocados por dos handlers attract
|  distintos que aun no estan disponibles como funciones cerradas).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `clr.l d4; move.w $70(a6), d4` es un zero-extend word->long
|       explicito. GCC habria emitido `moveq #0, d4; move.w …, d4` o
|       `moveu.w` (68020+). En 68000 el idioma clr.l+move.w es el mas
|       compacto pero no lo emite GCC (usa mov.w abs,d4 + moveq #0,dX).
|    2. `move.l abs.l, d1; swap d1; addi.w #$140, d1; sub.w d1, d0`
|       es un "extraer hi_word + sumar + restar" hand-coded. GCC
|       cargaria hi_word con addq/lsr o con acceso word directo.
|    3. `andi.l #$FFFF, d4; adda.l d4, a1` en lugar de `adda.w d4, a1`
|       (que hace sign-extend). Es un zero-extend a1 += (u16)d4
|       explicito, otro giro forense hand-coded.
|    4. Los dos clones son BYTE-A-BYTE identicos salvo el operando
|       absoluto de `move.l $106F50/$106F5C, d1`. Ninguna rama de GCC
|       inline con parametro produciria dos copias identicas de 76 B.
|
|  ---------- Viewport_CoordToScreen_096A5A ----------------------------------
|
|  Conversion de coordenadas viewport-space -> screen-space usada por los
|  cullers y el debug HUD. Recibe un par (d0,d1) en camera-space y devuelve
|  el par transformado en pantalla:
|
|      /* Convierte (x,y) camera-space a screen-space aplicando la camera
|       * global (cam_x=$106F50, cam_y=$106F54, altura de pantalla=$180). */
|      void Viewport_CoordToScreen(void) {
|          d0 = (short)(d0 - (camera_x >> 16));   ++d0;
|          d1 = (short)(d1 - (camera_y >> 16));
|          d1 = (short)($180 - d1);   ++d1;   /* Y-flip Neo Geo */
|      }
|
|  El "Y-flip Neo Geo" ($180 - Y) es el mismo idioma documentado en Wave
|  CC batch 1 (coord_camera_cluster_043f5e.s). Los `addq.w #1, d0` finales
|  son ajustes de half-pixel del sprite frame.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Doble `asr.l #8, dX` en lugar de `asr.l #16, dX` para extraer
|       hi_word con sign-extend. GCC habria emitido `swap; ext.l` o
|       `asr.l #16` directo (una sola instruccion en 68000).
|    2. `move.w #$180, d4; sub.w d1, d4; move.w d4, d1` en lugar de
|       `move.w #$180, d1; sub.w …, d1`. La reutilizacion de d4 como
|       temporal es puramente estilistica.
|
|  ---------- DebugTriggers_TwoBits_096B24 -----------------------------------
|
|  Dos triggers debug simetricos gated por DIP switches en $100001. Cada
|  uno espeta una tarea al scheduler ($4AE = Task_AllocFromFreeList, T#4)
|  con un template en $86586 (bit 2) o $86582 (bit 7), pero solo si dos
|  probes ($5CD18 y $5D11C) retornan carry=set.
|
|      void DebugTriggers_TwoBits(void) {
|          if (DIP_BIT2 && $5CD18() && $5D11C())
|              Task_Alloc(template_86586);
|          if (DIP_BIT7 && $5CD18() && $5D11C())
|              Task_Alloc(template_86582);
|      }
|
|  Callers conocidos: 6+ (invocado por todos los handlers ATTRACT_STATE
|  como parte de la macro attract-tick).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Los dos bloques son literalmente identicos salvo por el bit
|       probado (2 vs 7) y el template ($86586 vs $86582). En C se
|       habria factorizado en helper con dos calls. Aqui la macro asm
|       DEBUG_TRIGGER n, template las expandio inline dos veces.
|    2. `bcc.w` en lugar de `bcc.b` para saltos < 128 bytes: 6 B en vez
|       de 2 B por salto, GCC siempre elige la forma corta.
|    3. `btst.b #N, $100001.l` con desplazamiento absoluto largo (10 B)
|       en lugar de `btst.b #N, $100001.w` (6 B). El uso del modo abs.l
|       cuando abs.w bastaria es otro giro hand-coded.
|
|  ---------- ClampAndLookup8_096B7E + JumpTable_096B9C ----------------------
|
|  Clampea d0 a [0..7] y hace fetch de un puntero de handler en la tabla
|  embebida $096B9C[8]. Devuelve el puntero en a0. Como el `rts` final se
|  usa DESPUES de `movea.l (a0, d0.w), a0`, el efecto es equivalente a
|  "return handler_ptr in a0" — el caller normalmente hace `jmp (a0)` o
|  usa a0 directamente como base de una segunda deref.
|
|      /* Devuelve el N-esimo handler de la jump-table de 8 entradas,
|       * clamped a [0..7]. */
|      void (*ClampAndLookup8(u8 n))(void);
|
|  La tabla contiene 8 punteros a handlers en $096BBC..$0975A2:
|
|      slot 0: $00096BBC     slot 4: $000972CC
|      slot 1: $00096CAE     slot 5: $00097422
|      slot 2: $00096FA8     slot 6: $00097500
|      slot 3: $0009718A     slot 7: $000975A2
|
|  Esto sugiere que los 8 targets son handlers de un sub-dispatcher attract
|  cuyo indice se calcula en algun punto del pipeline (probablemente el
|  cluster $0969C2/$096A0E via `$21(a6)` = attract_state opcode).
|
|  Callers conocidos: 2 (jsr pc-rel desde $096A12 y $0969C6).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Clamp asimetrico `cmpi.b #7, d0; ble; move.b #7, d0`: no clampa
|       por abajo (asume d0 >= 0). GCC habria emitido clamp bilateral.
|    2. `andi.l #$FF, d0` de zero-extend byte->long redundante tras el
|       clamp que ya garantiza d0 <= 7. Es "defensive coding" del asm.
|    3. `asl.l #2, d0; lea $096B9C(pc), a0; movea.l (a0, d0.w), a0; rts`
|       es el idioma jump-table clasico 68000. GCC 13 emite este patron
|       para switch denso, PERO habria emitido `jmp (a0)` no `rts`. El
|       `rts` aqui es una micro-optimizacion: si el caller usa `jsr` en
|       lugar de `jmp`, se ahorra el push de PC del jmp indirecto.
|    4. La tabla `.long` esta INMEDIATAMENTE despues del `rts` sin
|       alineamiento a 4 (el rts ya deja en offset $096B9A par). GCC
|       habria usado `.section .rodata` separada.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  SelectPositive_TwoSlots_0967C0  @ $0967C0  (62 bytes)
| ---------------------------------------------------------------------------
|
        .globl  SelectPositive_TwoSlots_0967C0
        .type   SelectPositive_TwoSlots_0967C0, @function
        .section .text.SelectPositive_TwoSlots_0967C0, "ax", @progbits
SelectPositive_TwoSlots_0967C0:
        lea.l   0x100440.l, a0                 | +00  a0 = &slot_P1
        jsr     SlotExtractCoords_05E2D8                   | +06  (d0,d1) = fn_5E2D8(P1)
        move.l  d0, d2                         | +0c  d2 = P1.d0 (backup)
        move.l  d1, d3                         | +0e  d3 = P1.d1 (backup)
        lea.l   0x1004e0.l, a0                 | +10  a0 = &slot_P2
        jsr     SlotExtractCoords_05E2D8                   | +16  (d0,d1) = fn_5E2D8(P2)
        tst.l   d0                             | +1c  if (P2.d0 >= 0)
        bpl.w   .Lx_pos                        | +1e    skip
        move.l  d2, d0                         | +22    P2.d0 = P1.d0 (backup)
.Lx_pos:                                       | $0967E4
        tst.l   d2                             | +24  if (P1.d0 >= 0)
        bpl.w   .Lx_done                       | +26    skip
        move.l  d0, d2                         | +2a    P1.d0 = elegido
.Lx_done:                                      | $0967EC
        tst.l   d1                             | +2c  if (P2.d1 >= 0)
        bpl.w   .Ly_pos                        | +2e    skip
        move.l  d3, d1                         | +32    P2.d1 = P1.d1 (backup)
.Ly_pos:                                       | $0967F4
        tst.l   d3                             | +34  if (P1.d1 >= 0)
        bpl.w   .Ly_done                       | +36    skip
        move.l  d1, d3                         | +3a    P1.d1 = elegido
.Ly_done:                                      | $0967FC
        rts                                    | +3c

        .size   SelectPositive_TwoSlots_0967C0, .-SelectPositive_TwoSlots_0967C0

|
| ---------------------------------------------------------------------------
|  AttractCuller_Cam0_0969C2  @ $0969C2  (76 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractCuller_Cam0_0969C2
        .type   AttractCuller_Cam0_0969C2, @function
        .section .text.AttractCuller_Cam0_0969C2, "ax", @progbits
AttractCuller_Cam0_0969C2:
.Lcam0_top:
        move.b  0x21(a6), d0                   | +00  d0 = task.opcode (=lista)
        jsr     ClampAndLookup8_096B7E(pc)     | +04  (jsr pc-rel d16, 4 B)
        clr.l   d4                             | +08  d4 = 0 (zero-ext)
        move.w  0x70(a6), d4                   | +0a  d4 = cursor byte-offset
.Lcam0_loop:                                   | $0969D0
        cmpi.w  #0xffff, (a0, d4.w)            | +0e  if (entry[0] == $FFFF)
        beq.w   .Lcam0_done                    | +14    goto done (fin lista)
        move.w  0x2(a0, d4.w), d0              | +18  d0 = entry.x_world
        move.l  0x106f50.l, d1                 | +1c  d1 = camera0 long
        swap    d1                             | +22  d1 = camera0 hi_word
        addi.w  #0x140, d1                     | +24  d1 += viewport_x_right
        sub.w   d1, d0                         | +28  d0 = x - right_edge
        cmpi.w  #0x0, d0                       | +2a  if (d0 >= 0)  # off-screen
        bge.w   .Lcam0_done                    | +2e    goto done (skip resto)
        movea.l a0, a1                         | +32  a1 = lista base
        andi.l  #0xffff, d4                    | +34  d4 = zero-ext(cursor)
        adda.l  d4, a1                         | +3a  a1 = &entry actual
        jsr     Fn_0005DCCE                    | +3c  blit sprite (a1)
        addi.w  #0x14, 0x70(a6)                | +42  task.cursor += 20
        bra.b   .Lcam0_top                     | +48  loop
.Lcam0_done:                                   | $096A0C
        rts                                    | +4a

        .size   AttractCuller_Cam0_0969C2, .-AttractCuller_Cam0_0969C2

|
| ---------------------------------------------------------------------------
|  AttractCuller_Cam1_096A0E  @ $096A0E  (76 bytes, clon de _Cam0)
| ---------------------------------------------------------------------------
|
        .globl  AttractCuller_Cam1_096A0E
        .type   AttractCuller_Cam1_096A0E, @function
        .section .text.AttractCuller_Cam1_096A0E, "ax", @progbits
AttractCuller_Cam1_096A0E:
.Lcam1_top:
        move.b  0x21(a6), d0                   | +00  d0 = task.opcode
        jsr     ClampAndLookup8_096B7E(pc)     | +04  (jsr pc-rel d16, 4 B)
        clr.l   d4                             | +08  d4 = 0
        move.w  0x70(a6), d4                   | +0a  d4 = cursor
.Lcam1_loop:                                   | $096A1C
        cmpi.w  #0xffff, (a0, d4.w)            | +0e  if (entry[0] == $FFFF)
        beq.w   .Lcam1_done                    | +14    goto done
        move.w  0x2(a0, d4.w), d0              | +18  d0 = entry.x_world
        move.l  0x106f5c.l, d1                 | +1c  d1 = camera1 long
        swap    d1                             | +22  d1 = camera1 hi_word
        addi.w  #0x140, d1                     | +24  d1 += viewport_x_right
        sub.w   d1, d0                         | +28  d0 = x - right_edge
        cmpi.w  #0x0, d0                       | +2a  if (d0 >= 0)
        bge.w   .Lcam1_done                    | +2e    goto done
        movea.l a0, a1                         | +32  a1 = lista base
        andi.l  #0xffff, d4                    | +34  d4 = zero-ext(cursor)
        adda.l  d4, a1                         | +3a  a1 = &entry actual
        jsr     Fn_0005DCCE                    | +3c  blit sprite (a1)
        addi.w  #0x14, 0x70(a6)                | +42  task.cursor += 20
        bra.b   .Lcam1_top                     | +48  loop
.Lcam1_done:                                   | $096A58
        rts                                    | +4a

        .size   AttractCuller_Cam1_096A0E, .-AttractCuller_Cam1_096A0E

|
| ---------------------------------------------------------------------------
|  Viewport_CoordToScreen_096A5A  @ $096A5A  (38 bytes)
| ---------------------------------------------------------------------------
|
        .globl  Viewport_CoordToScreen_096A5A
        .type   Viewport_CoordToScreen_096A5A, @function
        .section .text.Viewport_CoordToScreen_096A5A, "ax", @progbits
Viewport_CoordToScreen_096A5A:
        move.l  0x106f50.l, d2                 | +00  d2 = camera_x_long
        asr.l   #0x8, d2                       | +06  d2 >>= 8  (fixed->int)
        asr.l   #0x8, d2                       | +08  d2 >>= 8  (=camera_x_hi)
        sub.w   d2, d0                         | +0a  d0 = x - camera_x_hi
        addq.w  #0x1, d0                       | +0c  ++d0
        move.l  0x106f54.l, d3                 | +0e  d3 = camera_y_long
        asr.l   #0x8, d3                       | +14  d3 >>= 8
        asr.l   #0x8, d3                       | +16  d3 >>= 8  (=camera_y_hi)
        sub.w   d3, d1                         | +18  d1 = y - camera_y_hi
        move.w  #0x180, d4                     | +1a  d4 = screen_height
        sub.w   d1, d4                         | +1e  d4 = 384 - y  (Y-flip)
        move.w  d4, d1                         | +20  d1 = d4
        addq.w  #0x1, d1                       | +22  ++d1
        rts                                    | +24

        .size   Viewport_CoordToScreen_096A5A, .-Viewport_CoordToScreen_096A5A

|
| ---------------------------------------------------------------------------
|  DebugTriggers_TwoBits_096B24  @ $096B24  (90 bytes)
| ---------------------------------------------------------------------------
|
        .globl  DebugTriggers_TwoBits_096B24
        .type   DebugTriggers_TwoBits_096B24, @function
        .section .text.DebugTriggers_TwoBits_096B24, "ax", @progbits
DebugTriggers_TwoBits_096B24:
        btst.b  #0x2, 0x100001.l               | +00  if (!DIP_BIT2)
        beq.w   .Lskip_bit2                    | +08    skip trigger A
        jsr     Fn_00005CD18                   | +0c  probe1()
        bcc.w   .Lskip_bit2                    | +12  if (!C) skip
        jsr     Fn_00005D11C                   | +16  probe2()
        bcc.w   .Lskip_bit2                    | +1c  if (!C) skip
        lea.l   0x86586.l, a1                  | +20  a1 = template A
        jsr     Task_AllocFromFreeList         | +26  scheduler_add(A)
.Lskip_bit2:                                   | $096B50
        btst.b  #0x7, 0x100001.l               | +2c  if (!DIP_BIT7)
        beq.w   .Lskip_bit7                    | +34    skip trigger B
        jsr     Fn_00005CD18                   | +38  probe1()
        bcc.w   .Lskip_bit7                    | +3e  if (!C) skip
        jsr     Fn_00005D11C                   | +42  probe2()
        bcc.w   .Lskip_bit7                    | +48  if (!C) skip
        lea.l   0x86582.l, a1                  | +4c  a1 = template B
        jsr     Task_AllocFromFreeList         | +52  scheduler_add(B)
.Lskip_bit7:                                   | $096B7C
        rts                                    | +58

        .size   DebugTriggers_TwoBits_096B24, .-DebugTriggers_TwoBits_096B24

|
| ---------------------------------------------------------------------------
|  ClampAndLookup8_096B7E  @ $096B7E  (30 bytes codigo + 32 bytes tabla)
| ---------------------------------------------------------------------------
|
        .globl  ClampAndLookup8_096B7E
        .type   ClampAndLookup8_096B7E, @function
        .section .text.ClampAndLookup8_096B7E, "ax", @progbits
ClampAndLookup8_096B7E:
        cmpi.b  #0x7, d0                       | +00  if (d0 <= 7)
        ble.w   .Lin_range                     | +04    skip clamp
        move.b  #0x7, d0                       | +08    d0 = 7
.Lin_range:                                    | $096B8A
        andi.l  #0xff, d0                      | +0c  d0 = zero-ext byte->long
        asl.l   #0x2, d0                       | +12  d0 <<= 2  (=idx*4)
        lea.l   .Ljump_table(pc), a0           | +14  a0 = &tabla[0]
        movea.l (a0, d0.w), a0                 | +18  a0 = tabla[idx]
        rts                                    | +1c  return (a0)

.Ljump_table:                                  | $096B9C  (8 punteros de 4 B)
        .long   0x00096bbc                     | slot 0
        .long   0x00096cae                     | slot 1
        .long   0x00096fa8                     | slot 2
        .long   0x0009718a                     | slot 3
        .long   0x000972cc                     | slot 4
        .long   0x00097422                     | slot 5
        .long   0x00097500                     | slot 6
        .long   0x000975a2                     | slot 7

        .size   ClampAndLookup8_096B7E, .-ClampAndLookup8_096B7E
