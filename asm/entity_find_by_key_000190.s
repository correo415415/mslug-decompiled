| ============================================================================
|  Metal Slug 1 - asm/entity_find_by_key_000190.s
|  ----------------------------------------------------------------------------
|  Wave X (post-allocator: HUD debug + comparadores + arranque) - funcion #5
|
|  Entity_FindByKey_000190  @ $000190  (58 bytes, 2+ callers via $0001A4)
|
|  Busca una entrada en una tabla de descriptores de 8 B que coincida con
|  una tripleta de claves (byte, byte, byte) en $ace..$ad0(a5). Al inicio
|  escribe d0 al puerto MMIO $300001 (probablemente el WATCHDOG_RESET del
|  Neo Geo MVS, direccion documentada en decomp.wiki y ngdevkit como
|  "$300001 = watchdog kick").
|
|  Estructura de la tabla (a documentar en mslug.h):
|
|      TableEntry {              -- 8 B por entrada
|          uint8_t  eos_marker;  -- +0 (byte)  0xFF = end-of-list
|          uint8_t  pad0;        -- +1
|          uint16_t key_a;       -- +2 (word, byte-key comparado con $ace(a5))
|          uint16_t key_b;       -- +4 (word, byte-key comparado con $acf(a5))
|          uint16_t key_c_full;  -- +6 (comparado como word con $ad0(a5))
|      };
|
|  Layout de globals en $a04(a5) y siguientes:
|      $a04(a5) : puntero long a la tabla
|      $a08(a5) : num entries - 1 (word, usado como contador dbra)
|      $ace(a5) : key_a esperada (byte)
|      $acf(a5) : key_b esperada (byte)
|      $ad0(a5) : key_c_full esperada (byte)
|
|  Algoritmo:
|      a0 = *(long*)$a04(a5)                 -- tabla base
|      d7 = *(word*)$a08(a5)                 -- count - 1
|      MMIO_WATCHDOG_KICK = d0                -- $300001.b = d0 (kick watchdog)
|  .Lloop:
|      d1 = a0->eos_marker                   -- (a0).w
|      if (d1 == 0xFF) goto .Lnext            -- end marker: skip
|      d0 = a0->key_a                        -- $2(a0).w
|      if (d0 != $ace(a5).b) goto .Lnext      -- key_a mismatch
|      d0 = a0->key_b                        -- $4(a0).w
|      if (d0 != $acf(a5).b) goto .Lnext      -- key_b mismatch
|      if (d1 != $ad0(a5).b) goto .Lfound     -- key_c match found
|      goto .Lnext
|  .Lnext:
|      a0 += 8                                -- siguiente entry
|      dbra d7, .Lloop                        -- d7--; loop
|      d3 = d7                                -- d3 = -1 (no encontrado)
|  .Lfound:
|      rts                                    -- d3 = idx restante (o -1 si no found)
|
|  Firma C conceptual:
|
|      /* Busca en la tabla ($a04(a5)) una entry cuya tripleta de claves
|       * coincida con $ace/$acf/$ad0(a5). Devuelve en d3 el indice
|       * (contador dbra restante) o d3=-1 si no encontrado. Kick al
|       * watchdog $300001 antes de buscar. */
|      int Entity_FindByKey(uint8_t watchdog_val /*d0*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. move.b d0, $300001.l al inicio de la funcion es un "watchdog kick"
|       del Neo Geo MVS. La direccion $300001 es el puerto oficial de
|       reset del watchdog documentado en el manual del MVS. GCC nunca
|       emite writes MMIO a direcciones fijas por su cuenta.
|    2. cmpi.b #$ff sobre d1 (que se cargo con move.w (a0),d1) usa solo
|       el byte bajo pero deja el byte alto activo. Idioma clasico de
|       "leer word, comparar byte" que aprovecha que en 68000 cmpi.b
|       solo consume 8 bits del registro. GCC habria hecho move.b (a0),d1
|       explicito.
|    3. move.w d7, d3 al FINAL del bucle sirve como "codigo de retorno".
|       Si no encontro, d7 vale -1 (dbra sale con d7=-1), y d3=-1
|       -> "not found". Si encontro, d3 conserva el valor de d7 en el
|       momento del match (idx restante desde el final). Idioma sutil
|       que GCC habria expresado como un ternary explicito.
|    4. dbra d7, $198 (target = $198 = inicio del bucle, +0x08 desde el
|       arranque de la funcion) - saltar al load del contador NO al
|       inicio, es la optimizacion "cache el a0 base tras el kick".
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_FindByKey_000190
        .type   Entity_FindByKey_000190, @function
        .section .text.Entity_FindByKey_000190, "ax", @progbits

Entity_FindByKey_000190:
        movea.l 0xa04(a5), a0                | +00  a0 = *(long*)$a04(a5)  (tabla base)
        move.w  0xa08(a5), d7                | +04  d7 = *(word*)$a08(a5)  (count - 1)
.Lloop:
        move.b  d0, 0x300001.l               | +08  MMIO $300001.b = d0 (watchdog kick por iter!)
        move.w  (a0), d1                     | +0e  d1 = a0->eos_marker (word)
        cmpi.b  #0xff, d1                    | +10  ¿ end-of-list ?
        beq.b   .Lnext                       | +14  si: skip esta entry
        move.w  0x2(a0), d0                  | +16  d0 = a0->key_a
        cmp.b   0xace(a5), d0                | +1a  ¿ match key_a ?
        bne.b   .Lnext                       | +1e  no: skip
        move.w  0x4(a0), d0                  | +20  d0 = a0->key_b
        cmp.b   0xacf(a5), d0                | +24  ¿ match key_b ?
        bne.b   .Lnext                       | +28  no: skip
        cmp.b   0xad0(a5), d1                | +2a  ¿ match key_c ?
        beq.b   .Lfound                      | +2e  si: encontrado
.Lnext:
        addq.l  #0x8, a0                     | +30  a0 += 8 (siguiente entry)
        dbra    d7, .Lloop                   | +32  d7--; loop
        move.w  d7, d3                       | +36  d3 = d7 (-1 si no found)
.Lfound:
        rts                                  | +38
        .size   Entity_FindByKey_000190, .-Entity_FindByKey_000190
