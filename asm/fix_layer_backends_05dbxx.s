| ============================================================================
|  Metal Slug 1 - asm/fix_layer_backends_05dbxx.s
|  ----------------------------------------------------------------------------
|  Wave II batch 1 - cluster Fix Layer render backends $05DBxx + slot helper.
|
|  Contenido (4 funciones, 176 bytes):
|
|      $05DB1A   FixLayer_MultiRowBlit_05DB1A      34 B  lista rows $FFFF
|      $05DB3C   CompareField10_CCR_05DB3C         28 B  probe CCR bilateral
|      $05DB58   InstallListPubHead_05DB58         18 B  install head+size
|      $05E2D8   SlotExtractCoords_05E2D8          96 B  extract (x,y) slot
|
|  Este batch continua el cluster Fix Layer iniciado en Wave W (que ya cerro
|  `Fix_BlitRect_05DA9C` 60 B y `Fix_BlitStream_05DAD8` 66 B, los dos backends
|  base) y cerrado parcialmente en Waves CC#2 y HH#3 (los batchers que los
|  llaman). Wave II#1 cierra los 3 helpers contiguos restantes del cluster
|  ($05DB1A/$05DB3C/$05DB58) mas `SlotExtractCoords_05E2D8`, que es el helper
|  invocado por `SelectPositive_TwoSlots_0967C0` (HH#2) sobre ambos slots
|  player P1/P2 y que hasta ahora solo existia como external `Fn_00005E2D8`.
|
|  ---------- FixLayer_MultiRowBlit_05DB1A -----------------------------------
|
|  Recorre una lista de tile codes terminada en $FFFF, invocando el backend
|  fila-por-fila $5DA56 (Wave W, ya expuesto como ThunkTarget_05da56) por
|  cada elemento. Cada tile code leido de a2 se combina con d3 (base tile
|  guardado del d0 de entrada) mediante `or.w`, se blitea la fila completa,
|  y se avanza el cursor VRAM en $40 bytes.
|
|  El step de $40 es exactamente DOS filas de tile-map del Fix Layer (el
|  step por fila es $20, documentado en Wave W y confirmado en HH#3 con
|  los offsets $7000/$701E/$74E0 de FixLayer_QuadBatch). Es decir: cada
|  entrada de la lista pinta una banda de 2 filas de tiles.
|
|      /* Recorre lista de tile codes (terminada en $FFFF) y blitea cada
|       * uno como una banda de 2 filas del Fix Layer.
|       * @param a1  VRAM base (avanza $40 por entrada)
|       * @param a2  puntero a lista de u16 (tile codes, centinela $FFFF)
|       * @param d0  base tile code (aditivo por or.w con cada entrada)
|       */
|      void FixLayer_MultiRowBlit(u16 *vram, const u16 *tiles, u16 base);
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `rts` INTERMEDIO en $05DB26 (rama "lista vacia: primer elemento ya
|       es $FFFF") seguido de la continuacion del loop. Es una funcion con
|       DOS salidas donde la primera esta en medio del cuerpo. GCC habria
|       emitido una unica salida al final con un goto/label comun.
|    2. `movem.l d1-d3/a1, -(a7)` / `movem.l (a7)+, d1-d3/a1` con mascara
|       mixta de data+address registers en un solo opcode (4 B c/u). GCC
|       habria emitido 4 x `move.l reg, -(a7)` (8 B) en cada extremo.
|    3. `jsr $5DA56(pc)` con desplazamiento PC-rel 16-bit (4 B, opcode
|       $4EBA). El target esta a solo -218 B del caller. GCC bare-metal
|       siempre emite `jsr abs.l` (6 B, opcode $4EB9) por no conocer la
|       proximidad. Idioma documentado en Wave HH#2 — requiere sintaxis
|       explicita `jsr TARGET(pc)` en GAS para reproducirlo.
|    4. `bra.b` de vuelta al loop con desplazamiento negativo corto en
|       lugar de estructura do/while: patron de bucle infinito con salida
|       por el centinela, tipico del asm hand-coded.
|
|  ---------- CompareField10_CCR_05DB3C --------------------------------------
|
|  Probe CCR bilateral. Compara el byte $10 del contexto activo (a6) contra
|  el byte $10 del contexto enlazado en $8(a6). Retorno por CCR:
|
|      self.f10 <  linked.f10  ->  CCR-C SET    (rama `ori.b #$11, ccr`)
|      self.f10 >= linked.f10  ->  CCR-C CLEAR  (rama `andi.b #$EE, ccr`)
|
|      /* Compara byte $10 de dos contextos enlazados; retorno via CCR-C. */
|      bool_ccr CompareField10(void);   /* C set = self < linked */
|
|  El idioma "return via CCR" ya esta documentado en Waves N (ccr_helpers),
|  T, Z#5/#6 (Entity_ProbeRevertCcr) y DD (Clipping_Test_0999DE). Este es
|  el primero del proyecto que compara DOS contextos enlazados por $8(a6),
|  lo que sugiere que el campo $8 del context struct es un puntero "parent"
|  o "sibling" dentro de una lista enlazada de contextos.
|
|  Absorbe el falso positivo `ClearXN_05db4c` (Wave N): los 6 bytes en
|  $05DB4C..$05DB51 son la rama "greater-equal" de esta funcion, no un
|  helper CCR independiente.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Return via CCR: no hay valor en d0, sino set/clear del flag Carry.
|       GCC no puede expresarlo en C sin `__asm__` naked.
|    2. `andi.b #$EE, ccr` / `ori.b #$11, ccr`: manipulacion directa del
|       byte CCR con mascara. GCC nunca emite estos opcodes.
|    3. Dos brazos con `rts` propio en lugar de converger en salida unica:
|       28 B con dos epilogos en vez de 24 B con uno. El asm original
|       prefiere el path directo sobre el tamanyo.
|
|  ---------- InstallListPubHead_05DB58 --------------------------------------
|
|  Instala un descriptor de lista (a0) como "lista activa" del contexto (a6)
|  y publica su tamanyo, luego delega en la subrutina de re-init del cursor
|  $5DBC2 (Wave J, ya expuesta como PcThunkTarget_05dbc2).
|
|      /* Instala nueva lista activa en el contexto y reinicia su cursor.
|       * @param a0  puntero al descriptor de lista (header + payload)
|       */
|      void InstallListPubHead(ListDescriptor *desc) {
|          ctx->list_ptr  = desc;        /* $3C(a6) */
|          ctx->list_size = desc->size;  /* $46(a6) <- $6(a0) */
|          ListCursor_Reinit();          /* $5DBC2 */
|      }
|
|  Layout inferido del context struct (a6) — consistente con Waves S/W/BB:
|      $3C : void *list_ptr     (descriptor de lista activa)
|      $46 : u16   list_size    (numero de entradas restantes)
|
|  Absorbe el falso positivo `JsrPcThunk_05db64` (Wave J): los 6 bytes en
|  $05DB64..$05DB69 son la cola `jsr $5DBC2(pc); rts` de esta funcion.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Orden interleave hand-coded: escribe $3C(a6), LUEGO lee $6(a0),
|       LUEGO escribe $46(a6). GCC habria agrupado las dos lecturas de a0
|       antes de las dos escrituras a a6 (mejor scheduling de pipeline).
|    2. `jsr $5DBC2(pc)` PC-rel corto (4 B) + `rts` en vez de tail-call
|       `jmp $5DBC2(pc)` (4 B, ahorraria 2 B y un nivel de pila). El asm
|       original prefiere jsr+rts por uniformidad de convencion.
|
|  ---------- SlotExtractCoords_05E2D8 ---------------------------------------
|
|  Extrae las coordenadas (x, y) de un slot player con multiples guardas de
|  validez. Es el helper llamado por `SelectPositive_TwoSlots_0967C0` (HH#2)
|  una vez por cada slot player ($100440 = P1, $1004E0 = P2) para decidir
|  que jugador esta "vivo" y usar sus coordenadas en el attract mode.
|
|  Estructura:
|
|    1. Precarga de sentinela: d0 = d1 = $FFFFFFFF (via `moveq #-1`).
|       Si cualquier guarda falla, la funcion retorna con el sentinela
|       intacto y el caller (SelectPositive) lo detecta con `tst.l/bpl`.
|
|    2. Tres guardas de "slot invalido" sobre el primer long del slot:
|           (a0)[0] == $FFFFFFFF   -> slot libre / no inicializado
|           (a0)[0] == $52A        -> constante magica del engine
|           (a0)[0] == $400        -> constante magica del engine
|       Cualquiera de las tres retorna sentinela inmediatamente.
|
|    3. Gate por el modo global $106ECE:
|           == 3  -> path B ($05E320)
|           != 3  -> path A ($05E306)
|
|    4. Ambos paths hacen EXACTAMENTE lo mismo:
|           d0 = (u16) slot->x       ($22(a0), zero-extended)
|           si bit 7 de slot->flags  ($5B(a0)) esta SET:
|               d1 = (u16) slot->y   ($24(a0), zero-extended)
|           si no: d1 queda con el sentinela $FFFFFFFF
|
|      /* Extrae coordenadas (x, y) del slot player a0.
|       * @return d0 = x  ($FFFFFFFF si el slot es invalido)
|       *         d1 = y  ($FFFFFFFF si el slot es invalido o !flag_y) */
|      void SlotExtractCoords(PlayerSlot *slot);
|
|  Layout inferido del PlayerSlot (confirmado contra Waves AA/BB/GG que ya
|  usan estos mismos offsets sobre $100440/$1004E0):
|      $00 : u32  state_or_magic   (guardas $FFFFFFFF / $52A / $400)
|      $22 : u16  x                (coordenada mundo X)
|      $24 : u16  y                (coordenada mundo Y)
|      $5B : u8   flags            (bit 7 = "coordenada Y valida")
|
|  Callers conocidos: 2 (ambos desde SelectPositive_TwoSlots_0967C0, HH#2).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. **Los dos paths A y B son BYTE-A-BYTE IDENTICOS** (14 B cada uno,
|       mismos offsets, misma logica). El unico motivo de su existencia es
|       un tag semantico distinto en el fuente original (probablemente una
|       macro `EXTRACT_COORDS layout_tag` invocada con dos tags cuyo cuerpo
|       aun no divergia). GCC jamas duplicaria codigo identico sin motivo.
|       Es el CUARTO par de "clones no factorizados" del proyecto (tras
|       BB#2, Z#5/#6 y HH#2 Cam0/Cam1).
|    2. `moveq #-1, d0` / `moveq #-1, d1` (2 B c/u) para precargar el
|       sentinela $FFFFFFFF. GCC habria emitido `move.l #-1, dX` (6 B) o
|       `moveq #0, dX; not.l dX` (4 B). El uso de moveq con signo negativo
|       aprovecha el sign-extend del byte inmediato.
|    3. Tres `cmpi.l #const, (a0)` en cascada con constantes magicas del
|       engine ($FFFFFFFF/$52A/$400) sin factorizar en macro ni switch.
|    4. Salida unica en $05E336 alcanzada por CINCO `beq.w`/`bra.w`
|       distintos. Idioma "single exit via branch" ya visto en Waves T,
|       EE y FF.
|    5. `btst.b #7, $5B(a0)` sobre un campo de flags a offset no alineado
|       ($5B es impar) — GCC habria usado acceso word alineado + mascara.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  FixLayer_MultiRowBlit_05DB1A  @ $05DB1A  (34 bytes)
| ---------------------------------------------------------------------------
|
        .globl  FixLayer_MultiRowBlit_05DB1A
        .type   FixLayer_MultiRowBlit_05DB1A, @function
        .section .text.FixLayer_MultiRowBlit_05DB1A, "ax", @progbits
FixLayer_MultiRowBlit_05DB1A:
        move.w  d0, d3                          | +00  d3 = base tile code
.Lrow_loop:                                     | $05DB1C
        move.w  (a2)+, d0                       | +02  d0 = next tile code
        cmpi.w  #0xffff, d0                     | +04  if (d0 != $FFFF)
        bne.w   .Lrow_do                        | +08    procesar entrada
        rts                                     | +0c  RTS intermedio (fin lista)
.Lrow_do:                                       | $05DB28
        or.w    d3, d0                          | +0e  d0 |= base tile
        movem.l d1-d3/a1, -(a7)                 | +10  save d1-d3/a1
        jsr     ThunkTarget_05da56(pc)          | +14  blit fila (PC-rel, 4 B)
        movem.l (a7)+, d1-d3/a1                 | +18  restore d1-d3/a1
        adda.w  #0x40, a1                       | +1c  a1 += $40 (2 filas)
        bra.b   .Lrow_loop                      | +20  loop

        .size   FixLayer_MultiRowBlit_05DB1A, .-FixLayer_MultiRowBlit_05DB1A

|
| ---------------------------------------------------------------------------
|  CompareField10_CCR_05DB3C  @ $05DB3C  (28 bytes)
|  Absorbe FP: ClearXN_05db4c (Wave N) = rama greater-equal.
| ---------------------------------------------------------------------------
|
        .globl  CompareField10_CCR_05DB3C
        .type   CompareField10_CCR_05DB3C, @function
        .section .text.CompareField10_CCR_05DB3C, "ax", @progbits
CompareField10_CCR_05DB3C:
        movea.l 0x8(a6), a1                     | +00  a1 = ctx->linked
        move.b  0x10(a6), d0                    | +04  d0 = self.f10
        cmp.b   0x10(a1), d0                    | +08  compara self vs linked
        bcs.w   .Lcmp_less                      | +0c  if (self < linked)
        andi.b  #0xee, ccr                      | +10  CCR: C=0 (greater-equal)
        rts                                     | +14
.Lcmp_less:                                     | $05DB52
        ori.b   #0x11, ccr                      | +16  CCR: C=1 (less-than)
        rts                                     | +1a

        .size   CompareField10_CCR_05DB3C, .-CompareField10_CCR_05DB3C

|
| ---------------------------------------------------------------------------
|  InstallListPubHead_05DB58  @ $05DB58  (18 bytes)
|  Absorbe FP: JsrPcThunk_05db64 (Wave J) = cola jsr+rts.
| ---------------------------------------------------------------------------
|
        .globl  InstallListPubHead_05DB58
        .type   InstallListPubHead_05DB58, @function
        .section .text.InstallListPubHead_05DB58, "ax", @progbits
InstallListPubHead_05DB58:
        move.l  a0, 0x3c(a6)                    | +00  ctx->list_ptr = desc
        move.w  0x6(a0), d0                     | +04  d0 = desc->size
        move.w  d0, 0x46(a6)                    | +08  ctx->list_size = d0
        jsr     PcThunkTarget_05dbc2(pc)        | +0c  reinit cursor (PC-rel)
        rts                                     | +10

        .size   InstallListPubHead_05DB58, .-InstallListPubHead_05DB58

|
| ---------------------------------------------------------------------------
|  SlotExtractCoords_05E2D8  @ $05E2D8  (96 bytes)
|  Callee de SelectPositive_TwoSlots_0967C0 (Wave HH#2), antes Fn_00005E2D8.
| ---------------------------------------------------------------------------
|
        .globl  SlotExtractCoords_05E2D8
        .type   SlotExtractCoords_05E2D8, @function
        .section .text.SlotExtractCoords_05E2D8, "ax", @progbits
SlotExtractCoords_05E2D8:
        moveq   #-0x1, d0                       | +00  d0 = $FFFFFFFF (sentinel x)
        moveq   #-0x1, d1                       | +02  d1 = $FFFFFFFF (sentinel y)
        cmpi.l  #0xffffffff, (a0)               | +04  guarda 1: slot libre
        beq.w   .Lslot_done                     | +0a
        cmpi.l  #0x52a, (a0)                    | +0e  guarda 2: magic $52A
        beq.w   .Lslot_done                     | +14
        cmpi.l  #0x400, (a0)                    | +18  guarda 3: magic $400
        beq.w   .Lslot_done                     | +1e
        cmpi.b  #0x3, 0x106ece.l                | +22  if (global_mode == 3)
        beq.w   .Lpath_B                        | +2a    goto path B
| ---- path A: modo global != 3 (default) --------------------------------
        moveq   #0x0, d0                        | +2e  d0 = 0 (zero-extend)
        move.w  0x22(a0), d0                    | +30  d0 = slot->x
        btst.b  #0x7, 0x5b(a0)                  | +34  if (!(flags & bit7))
        beq.w   .Lslot_done                     | +3a    return (d1 = sentinel)
        moveq   #0x0, d1                        | +3e  d1 = 0 (zero-extend)
        move.w  0x24(a0), d1                    | +40  d1 = slot->y
        bra.w   .Lslot_done                     | +44
| ---- path B: modo global == 3 (clon estructural de path A) -------------
.Lpath_B:                                       | $05E320
        moveq   #0x0, d0                        | +48  d0 = 0 (zero-extend)
        move.w  0x22(a0), d0                    | +4a  d0 = slot->x
        btst.b  #0x7, 0x5b(a0)                  | +4e  if (!(flags & bit7))
        beq.w   .Lslot_done                     | +54    return (d1 = sentinel)
        moveq   #0x0, d1                        | +58  d1 = 0 (zero-extend)
        move.w  0x24(a0), d1                    | +5a  d1 = slot->y
.Lslot_done:                                    | $05E336
        rts                                     | +5e

        .size   SlotExtractCoords_05E2D8, .-SlotExtractCoords_05E2D8
