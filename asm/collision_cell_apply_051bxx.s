| ============================================================================
|  Metal Slug 1 - asm/collision_cell_apply_051bxx.s
|  ----------------------------------------------------------------------------
|  Wave LL batch 1 - callees por-celda del cluster de colision/camara.
|
|  Contenido (2 funciones, 242 bytes):
|
|      $051BA8   CellApply_BidirScan_051BA8       96 B  cursor bidireccional
|      $051DE2   CellCommit_MMIO_051DE2          146 B  publica celda VRAM
|
|  Ambas eran los dos "externals" pendientes del cluster Wave KK batch 2
|  (collision_probes_051cxx.s). Wave KK las referenciaba via los aliases
|  externos Fn_00051BA8 y ThunkTarget_051de2, ambos declarados en
|  tools/symbols.py como --defsym al linker. Tras esta oleada:
|
|    - El simbolo canonico se define aqui (.text.<Sym>) con .globl.
|    - El alias externo antiguo se convierte en .set para que
|      collision_probes_051cxx.s NO necesite editarse (bsr.w Fn_00051BA8
|      y bsr.w ThunkTarget_051de2 siguen resolviendose).
|    - tools/symbols.py deja de definir esas 2 direcciones como --defsym
|      (comentadas con nota Wave LL, igual que Wave KK#1 hizo con
|      TransformCommit_MMIO_051F30).
|
|  ---------- Mapa de callers -----------------------------------------------
|
|      Collision_ProbeRange_051C08 (KK#2)  -> bsr.w Fn_00051BA8   ($51BA8)
|      Collision_ProbeX_051C82     (KK#2)  -> bsr.w Fn_00051BA8   ($51BA8)
|      Collision_ProbeY_051CF6     (KK#2)  -> bsr.w Fn_00051BA8   ($51BA8)
|
|      Collision_ProbeRange_051C08 (KK#2)  -> bsr.w ThunkTarget_051de2 ($51DE2)
|      Collision_ProbeX_051C82     (KK#2)  -> bsr.w ThunkTarget_051de2 ($51DE2)
|      Collision_ProbeY_051CF6     (KK#2)  -> bsr.w ThunkTarget_051de2 ($51DE2)
|
|  Es decir, los 3 probes de KK#2 cierran ahora su grafo completo: cada uno
|  llama a CellApply_BidirScan_051BA8 dentro del bucle .Lprobe_*_loop (una
|  vez por celda) y a CellCommit_MMIO_051DE2 tras salir del bucle (una vez
|  por probe positivo), sin externals residuales.
|
|  ---------- Descubrimientos arquitectonicos ------------------------------
|
|  1. 7o par de clones no factorizados del proyecto: la estructura del
|     bucle interior de CellCommit_MMIO_051DE2 (movem.w d0-d1/d4-d5 +
|     doble emit VRAM $3C0000/$3C0002 con timing $106EE4 + dbra sobre X e
|     Y) es identica a la del bucle .Lhandler_loop de
|     TileMap_HandlerInline_051F94 (Wave KK#2, 160 B). Difieren en el
|     origen de datos (linear stream via (a3)+ aqui, tile_map_A/B en
|     KK#2#4) y en si el emit escribe directo o pasa por lookup. El
|     patron macro es "publica un rectangulo de tiles al VRAM del BG
|     layer con timing MMIO por celda".
|
|  2. Layout struct sprite ampliado (a0):
|          $12(a0)  list_cursor    ptr a la entrada actual de la
|                                  lista de rangos ordenada
|          $22(a0)  x_base_tile    offset X en tiles (& $1F wrap)
|          $24(a0)  y_base_tile    offset Y en tiles
|          $26(a0)  cell_y_wrap    resultado del clamp Y modular
|          $27(a0)  cell_flags     flag byte publicado en la celda
|          $28(a0)  sprite_id_hw   sprite id hardware base
|          $52(a0)  tile_map_A[]   array 32 bytes tile map A
|                                  (usado como LUT en +$52,d0.w desde
|                                  CellCommit_MMIO)
|
|  3. Formato de la lista bidireccional en $12(a0) (CellApply_BidirScan):
|          entry[0] = word  d2_min_x
|          entry[1] = word  d3_max_x  (o $FFFF = sentinel fin-lista)
|          entry[2] = byte  cell_flag_A  (publicado en $27)
|          entry[3] = byte  cell_flag_B  (usado en calculo de $26)
|     Total: 6 bytes por entrada, pero el cursor avanza en steps de 4
|     (word-alineados: 2W hacia adelante = 4 B, 2W hacia atras = -4 B),
|     lo que implica que las entradas se solapan en el array (el flag_A
|     de una entrada es el max_x de la siguiente). Es un formato
|     "monotonic packed list" tipico del asm hand-coded.
|
|  ---------- Convencion de firma ------------------------------------------
|
|  Ambas funciones toman a0 = puntero al struct sprite/scanner activo
|  (mismo layout usado por los 3 probes de KK#2). Preservan a0 al salir.
|  No usan CCR como valor de retorno (CellApply_BidirScan sale con la
|  entrada actualizada en $12(a0); CellCommit_MMIO no retorna nada).
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  CellApply_BidirScan_051BA8  @ $051BA8  (96 bytes)
|
|  Aplicador de celda por lista bidireccional ordenada. Dado un candidato
|  d0 (columna/fila de tile), avanza el cursor $12(a0) hacia adelante o
|  hacia atras hasta encontrar la entrada cuyo rango [d2, d3] contiene d0.
|  Si encuentra: publica flag en $27(a0), calcula clamp Y en $26(a0), y
|  actualiza el cursor en $12(a0). Si no encuentra (rango 0..0 o
|  centinela $FFFF): actualiza cursor y sale sin publicar.
|
|  Idioma "cursor persistente": el cursor $12(a0) se re-publica siempre al
|  final (aunque no haya cambiado), y GCC no lo genera asi (mantendria
|  una variable local y solo escribiria si cambia). Es una escritura
|  incondicional para minimizar branches de escritura.
| ---------------------------------------------------------------------------
|
        .globl  CellApply_BidirScan_051BA8
        .globl  Fn_00051BA8                     | alias externo KK#2 (bsr.w)
        .type   CellApply_BidirScan_051BA8, @function
        .section .text.CellApply_BidirScan_051BA8, "ax", @progbits
CellApply_BidirScan_051BA8:
Fn_00051BA8:
        movem.l d2-d3/a1, -(a7)                 | +00  save d2/d3/a1
        movea.l 0x12(a0), a1                    | +04  a1 = list_cursor
        move.l  a1, d2                          | +08  d2 = a1  (NULL test)
        beq.w   .Lcell_apply_done               | +0a  if (!cursor) exit
.Lcell_apply_probe:                             | $051BB6
        move.w  (a1), d2                        | +0e  d2 = entry.min_x
        move.w  0x4(a1), d3                     | +10  d3 = entry.max_x
        cmp.w   d2, d0                          | +14  if (d0 >= d2)
        bcc.w   .Lcell_apply_check_high         | +16    goto check_high
        cmpi.w  #0x0, d2                        | +1a  if (d2 == 0)
        beq.w   .Lcell_apply_done               | +1e    exit (bottom sentinel)
        lea.l   -0x4(a1), a1                    | +22  a1 -= 4 (retrocede)
        bra.b   .Lcell_apply_probe              | +26  retry
.Lcell_apply_check_high:                        | $051BD0
        cmp.w   d3, d0                          | +28  if (d0 < d3)
        bcs.w   .Lcell_apply_hit                | +2a    goto hit
        cmpi.w  #0xffff, 0x4(a1)                | +2e  if (next.max_x == FFFF)
        beq.w   .Lcell_apply_done               | +34    exit (top sentinel)
        lea.l   0x4(a1), a1                     | +38  a1 += 4 (avanza)
        bra.b   .Lcell_apply_probe              | +3c  retry
.Lcell_apply_hit:                               | $051BE6
        move.b  0x2(a1), 0x27(a0)               | +3e  sprite.cell_flags = flagA
        move.b  0x3(a1), d2                     | +44  d2 = flagB
        add.w   0x24(a0), d2                    | +48  d2 += y_base_tile
        neg.w   d2                              | +4c  d2 = -d2
        andi.w  #0x1f, d2                       | +4e  d2 &= 31
        move.b  d2, 0x26(a0)                    | +52  sprite.cell_y_wrap = d2
.Lcell_apply_done:                              | $051BFE
        move.l  a1, 0x12(a0)                    | +56  publish cursor
        movem.l (a7)+, d2-d3/a1                 | +5a  restore
        rts                                     | +5e

        .size   CellApply_BidirScan_051BA8, .-CellApply_BidirScan_051BA8

|
| ---------------------------------------------------------------------------
|  CellCommit_MMIO_051DE2  @ $051DE2  (146 bytes)
|
|  Publica un rectangulo de celdas al VRAM del BG layer via el puerto
|  Neo Geo $3C0000/$3C0002 con el timing latch $106EE4 (protocolo
|  autoinc estandar de Neo Geo Fix/BG).
|
|  Firma conceptual:
|    void CellCommit_MMIO(struct sprite *a0,     ; entity/scanner ctx
|                         u16 d0,                ; base tile X
|                         u16 d1,                ; base tile Y
|                         u16 d2,                ; skip words per row
|                         u16 d3,                ; base tile row offset
|                         u32 d4,               ; width_minus_1   (word-safe)
|                         u32 d5,               ; height_minus_1  (word-safe)
|                         struct row *a1);       ; a1 = row-descriptor:
|                                                ;   +0 word skip
|                                                ;   +2 word row_width
|                                                ;   +4 long tile_stream_ptr
|
|  Loop doble: d5 (rows) x d4 (cols). Por celda emite 2 words al puerto
|  VRAM: uno con el low tile (a3)+ y otro con el alto (a3)+, con timing
|  latch en $106EE4 entre cada escritura. Aplica clipping modular X
|  ($1F) e Y ($3F), offset por sprite ($22,$24,$28), y LUT de fase
|  ($52,d0.w).
|
|  Comparte estructura de emit con TileMap_HandlerInline_051F94 (KK#2#4):
|  7o par de clones no factorizados. Difiere en fuente de datos (linear
|  stream (a3)+ aqui vs LUT tile_map_A/B en KK#2#4) y direccion del
|  bucle (X-outer,Y-inner aqui vs Y-outer,X-inner en KK#2#4).
|
|  Idioma "movem.w d0-d1/d4-d5, -(a7)": el estado (X,Y) del bucle
|  externo se preserva en pila y se restaura tras cada fila interna, en
|  vez de usar variables locales o mas registros - patron tipico del asm
|  hand-coded que necesita >8 datos vivos.
| ---------------------------------------------------------------------------
|
        .globl  CellCommit_MMIO_051DE2
        .globl  ThunkTarget_051de2              | alias externo KK#2 (bsr.w)
        .type   CellCommit_MMIO_051DE2, @function
        .section .text.CellCommit_MMIO_051DE2, "ax", @progbits
CellCommit_MMIO_051DE2:
ThunkTarget_051de2:
        andi.l  #0xffff, d4                     | +00  d4 = width_minus_1
        andi.l  #0xffff, d5                     | +06  d5 = height_minus_1
        move.w  0x2(a1), d6                     | +0c  d6 = row_width
        mulu.w  d6, d2                          | +10  d2 = skip * row_width
        add.w   d3, d2                          | +12  d2 += base_row_offset
        add.l   d2, d2                          | +14  d2 <<= 1   (byte offset)
        add.l   d2, d2                          | +16  d2 <<= 1   (long offset)
        movea.l 0x4(a1), a3                     | +18  a3 = tile_stream_ptr
        lea.l   (a3, d2.l), a3                  | +1c  a3 += byte offset
        sub.w   d5, d6                          | +20  d6 -= height_minus_1
        add.w   d6, d6                          | +22  d6 <<= 1
        add.w   d6, d6                          | +24  d6 <<= 1  (bytes per skip)
        subq.w  #0x1, d4                        | +26  --d4  (dbra)
        subq.w  #0x1, d5                        | +28  --d5  (dbra)
        add.w   0x22(a0), d0                    | +2a  d0 += x_base_tile
        add.w   0x24(a0), d1                    | +2e  d1 += y_base_tile
.Lcell_commit_row:                              | $051E14
        movem.w d0-d1/d4-d5, -(a7)              | +32  save loop state
        andi.w  #0x1f, d0                       | +36  d0 &= 31 (X wrap)
        add.b   0x52(a0, d0.w), d1              | +3a  d1 += phase LUT[d0]
        andi.w  #0x1f, d1                       | +3e  d1 &= 31 (Y wrap)
        add.w   0x28(a0), d0                    | +42  d0 += sprite_id_hw
        lsl.w   #0x6, d0                        | +46  d0 <<= 6
        addi.w  #0x40, d0                       | +48  d0 += $40 (BG base)
        movea.w d0, a1                          | +4c  a1 = VRAM word addr
        add.w   d1, d1                          | +4e  d1 <<= 1
.Lcell_commit_col:                              | $051E32
        lea.l   (a1, d1.w), a2                  | +50  a2 = a1 + d1  (recalc)
        move.w  a2, 0x106ee4.l                  | +54  latch shadow_addr
        move.w  a2, 0x3c0000.l                  | +5a  VRAM_ADDR = a2
        move.w  (a3)+, 0x3c0002.l               | +60  VRAM_DATA = stream[0]
        addq.w  #0x1, a2                        | +66  ++a2
        move.w  a2, 0x106ee4.l                  | +68  latch shadow_addr
        move.w  a2, 0x3c0000.l                  | +6e  VRAM_ADDR = a2
        move.w  (a3)+, 0x3c0002.l               | +74  VRAM_DATA = stream[1]
        addq.w  #0x2, d1                        | +7a  d1 += 2
        andi.w  #0x3f, d1                       | +7c  d1 &= 63
        dbra    d5, .Lcell_commit_col           | +80  loop cols (d5 times)
        movem.w (a7)+, d0-d1/d4-d5              | +84  restore loop state
        addq.w  #0x1, d0                        | +88  ++d0
        adda.w  d6, a3                          | +8a  a3 += skip per row
        dbra    d4, .Lcell_commit_row           | +8c  loop rows (d4 times)
        rts                                     | +90

        .size   CellCommit_MMIO_051DE2, .-CellCommit_MMIO_051DE2
