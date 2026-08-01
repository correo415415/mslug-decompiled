| ============================================================================
|  Metal Slug 1 - asm/sprite_allocator_0139xx.s
|  ----------------------------------------------------------------------------
|  Wave JJ batch 2 - asignador de sprites hardware Neo Geo $01390E..$0139FD.
|
|  Contenido (6 funciones, 240 bytes):
|
|      $01390E   Scratch_Alloc_01390E             68 B  particiona los pools
|      $013952   Spawn_TypeB_013952               46 B  alloc pool B (bajo)
|      $013980   SpriteTrapGuard_013980            2 B  trap #$F compartido
|      $013982   Spawn_TypeA_013982               26 B  alloc pool A (alto)
|      $01399C   SpriteRange_DisableAll_01399C    34 B  apaga rango de sprites
|      $0139BE   SpriteRange_InitChain_0139BE     64 B  arma cadena de sprites
|
|  Cierra el trio de callees que `SceneLoader_Main_043568` (Wave HH#1) invoca
|  en sus dos pasadas sobre el bytecode de escena, y que hasta ahora solo
|  existian como externals `Scratch_Alloc_01390E` / `Spawn_TypeA_013982` /
|  `Spawn_TypeB_013952` en `symbols.py`.
|
|  ---------- CONTEXTO HARDWARE: los 380 sprites del Neo Geo ----------------
|
|  El Neo Geo dispone de 381 sprites hardware (indices 0..$17C). Cada uno se
|  describe en cuatro bancos de la VRAM, accedidos por el puerto MMIO
|  $3C0000 (direccion) / $3C0002 (dato) / $3C0004 (autoincremento):
|
|      SCB1  $0000..$7FFF   tile map del sprite (64 tiles por sprite)
|      SCB2  $8000..$81FF   coeficiente de shrink  ($FFF = sin reduccion)
|      SCB3  $8200..$83FF   Y, bit sticky (bit 6) y altura (bits 5..0)
|      SCB4  $8400..$85FF   X
|
|  El **bit sticky** de SCB3 encadena un sprite al anterior: un objeto ancho
|  se compone de N sprites contiguos donde el primero es no-sticky y los
|  N-1 restantes son sticky. Esto explica exactamente la estructura de
|  `SpriteRange_InitChain_0139BE` (ver abajo).
|
|  ---------- Scratch_Alloc_01390E -------------------------------------------
|
|  Particiona los 381 sprites hardware en DOS POOLS que crecen en sentidos
|  opuestos, y apaga todos los sprites antes de empezar la escena.
|
|      /* Reparte los sprites hardware entre los dos pools de la escena.
|       * @param d0  tamanyo reservado para el pool B (contador tipo B)
|       * @param d1  tamanyo reservado intermedio
|       * @param d2  tamanyo reservado en cabeza (subop del terminador) */
|      void Scratch_Alloc(u16 count_b, u16 mid, u16 head) {
|          SpriteSys_Reset();                  /* $5A88A */
|          g_unk_10E1F4  = 0;
|          g_poolB_cur   = 0;                  /* $10E1FE */
|          g_poolB_limit = count_b;            /* $10E1F6 */
|          u16 t = 0x17C - head;               /* 380 - d2 */
|          g_mark_10E1F8 = t;
|          t -= mid;                           /* -= d1 */
|          g_mark_10E1FA = t;
|          g_poolA_cur   = t;                  /* $10E1FC */
|          SpriteRange_DisableAll(0, 0x17B);   /* apaga los 380 */
|      }
|
|  Mapa de los globales del asignador (todos words en $10E1Fx):
|      $10E1F4  reservado / reset a 0 (proposito aun sin identificar)
|      $10E1F6  limite superior del pool B
|      $10E1F8  marca 380 - head
|      $10E1FA  marca 380 - head - mid
|      $10E1FC  cursor del pool A (crece hacia arriba desde la marca)
|      $10E1FE  cursor del pool B (crece hacia arriba desde 0)
|
|  Absorbe el FP `JsrPcThunk_01394c` (Wave J): los 6 B en $01394C..$013951
|  son la cola `jsr $1399C(pc); rts` de esta funcion.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. La cadena `move.w #$17C,d0 / sub.w d2,d0 / move.w d0,X / sub.w d1,d0
|       / move.w d0,Y / move.w d0,Z` reutiliza d0 como acumulador vivo entre
|       cinco escrituras a memoria. GCC habria materializado cada expresion
|       por separado o mantenido los temporales en registros distintos.
|    2. Cuatro `move.w #$0, abs.l` (10 B c/u) en lugar de `clr.w abs.l`
|       (6 B). Mismo idioma "publicacion explicita" ya documentado en
|       SceneLoader_Main (HH#1).
|    3. `jsr $1399C(pc)` PC-rel 16-bit (4 B) al helper contiguo.
|
|  ---------- HALLAZGO FORENSE: COLA COMPARTIDA ENTRE SPAWNERS --------------
|
|  Los dos spawners comparten su mitad final. `Spawn_TypeA_013982` termina
|  con `bra.b $01396E`, que es una direccion **dentro del cuerpo de
|  `Spawn_TypeB_013952`** (offset +$1C):
|
|      $013952  Spawn_TypeB: cabecera propia (cursor B, limite $10E1F6)
|      $01396E  .Lspawn_tail:  <-- cola COMPARTIDA
|                   move.w d3, d1
|                   subq.w #1, d1
|                   movem.l d0-d1, -(a7)
|                   jsr $139BE(pc)        ; arma la cadena de sprites
|                   movem.l (a7)+, d0-d1
|                   rts
|      $013982  Spawn_TypeA: cabecera propia (cursor A, limite $17C)
|                   ...
|                   bra.b $01396E         ; --> salta a la cola de TypeB
|
|  Es el SEGUNDO caso de codigo compartido entre funciones detectado en
|  Wave JJ (el primero fue el epilogo `rts` cruzado de los hooks de camara,
|  batch 1). Aqui no es solo el epilogo: son SEIS instrucciones completas,
|  incluida la llamada al inicializador de cadena.
|
|  Ambos spawners comparten ademas la guarda de desbordamiento
|  `SpriteTrapGuard_013980` (`trap #$F`), alcanzada por `bhi.w` desde TypeB
|  y por `bhi.b` desde TypeA.
|
|  ---------- Spawn_TypeB_013952 / Spawn_TypeA_013982 ------------------------
|
|      /* Reserva `count` sprites hardware consecutivos del pool indicado y
|       * los arma como una cadena (primero no-sticky, resto sticky).
|       * Dispara trap #$F si el pool se desborda.
|       * @param  d0  numero de sprites a reservar
|       * @return d0  primer indice reservado
|       *         d1  ultimo indice reservado */
|      void Spawn_TypeB(u16 count) {            /* pool B: 0 .. limite */
|          u16 first = g_poolB_cur;
|          u16 last  = first + count;
|          if (last > g_poolB_limit) trap(0xF);
|          g_poolB_cur = last;
|          goto spawn_tail;                     /* cola compartida */
|      }
|      void Spawn_TypeA(u16 count) {            /* pool A: marca .. 380 */
|          u16 first = g_poolA_cur;
|          u16 last  = first + count;
|          if (last > 0x17C) trap(0xF);
|          g_poolA_cur = last;
|          goto spawn_tail;                     /* bra.b $01396E */
|      }
|      /* spawn_tail: */
|          SpriteRange_InitChain(first, last - 1);
|
|  Correspondencia con el bytecode de escena (HH#1): las entradas con
|  `type == 0` llaman a TypeA y las de cualquier otro valor no-cero a TypeB,
|  lo que reparte los objetos de la escena entre los dos extremos del banco
|  de sprites — tipicamente fondo/decorado en un pool y actores en el otro.
|
|  Por que NO son rederivables por GCC 1:1:
|    1. El `bra.b` de TypeA al interior de TypeB. GCC nunca emite un salto
|       al cuerpo de otra funcion; a lo sumo haria inlining o una llamada.
|    2. `trap #$F` como manejador de desbordamiento compartido, alcanzado
|       desde dos funciones distintas con anchos de branch distintos
|       (`bhi.w` 4 B en TypeB, `bhi.b` 2 B en TypeA). La asimetria es un
|       artefacto de ensamblado en una pasada: cuando se ensambla TypeB el
|       destino aun esta por delante sin resolver.
|    3. `movem.l d0-d1, -(a7)` / `movem.l (a7)+, d0-d1` para preservar el
|       par de retorno sobre la llamada. GCC habria usado dos `move.l`.
|
|  ---------- SpriteRange_DisableAll_01399C ----------------------------------
|
|  Apaga los sprites del rango [d0, d1] escribiendo 0 en su entrada SCB3
|  (altura 0 y sticky limpio = sprite no dibujado).
|
|      /* Desactiva los sprites hardware del rango indicado. */
|      void SpriteRange_DisableAll(u16 first, u16 last) {
|          u16 scb3 = 0x8201 + first;
|          u16 end  = 0x8201 + last;
|          VRAM_ADDR_INC = 0;                   /* $3C0004 = 0 */
|          do {
|              vram_write(scb3, 0);             /* SCB3[i] = 0 */
|              ++scb3;
|          } while (end >= scb3);
|      }
|
|  Llamado por `Scratch_Alloc` con (0, $17B) para limpiar el banco completo
|  al cargar la escena.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `movem.w d0/d2, (a0)` con a0 = $3C0000 escribe DOS words seguidos
|       (direccion y dato) en un unico opcode, aprovechando el puerto MMIO
|       de la VRAM. Es el idioma "movem como escritura MMIO pareada" ya
|       documentado en Waves CC#2, DD y II#1. GCC no lo genera.
|    2. `movea.l #$3C0000, a0` (6 B) recargado dentro de cada helper en vez
|       de mantenerse en un registro global del modulo.
|    3. `clr.w $3C0004.l` fija el autoincremento a 0 porque el bucle escribe
|       la direccion explicitamente en cada iteracion. Detalle de hardware
|       que GCC desconoce.
|
|  ---------- SpriteRange_InitChain_0139BE -----------------------------------
|
|  Arma el rango [d0, d1] como una CADENA de sprites: el primero queda
|  no-sticky y todos los siguientes sticky, con shrink desactivado en todos.
|  Es el nucleo del "objeto multi-sprite" del Neo Geo.
|
|      /* Arma los sprites [first, last] como una cadena hardware. */
|      void SpriteRange_InitChain(u16 first, u16 last) {
|          u16 scb3 = 0x8201 + first;           /* Y / sticky / altura */
|          u16 end  = 0x8201 + last;
|          u16 scb2 = 0x8001 + first;           /* shrink */
|          vram_write(scb3++, 0x000);           /* cabeza: NO sticky */
|          vram_write(scb2++, 0xFFF);           /* sin shrink */
|          while (end >= scb3) {
|              vram_write(scb3++, 0x040);       /* cola: sticky (bit 6) */
|              vram_write(scb2++, 0xFFF);       /* sin shrink */
|          }
|      }
|
|  El valor $040 es exactamente el bit 6 de SCB3 (sticky) con altura 0, y
|  $FFF es el coeficiente de shrink neutro de SCB2. La cabeza escribe $000
|  para romper la cadena anterior.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. La primera pareja de escrituras esta FUERA del bucle y la condicion
|       de salida (`cmp.w d0,d1 / bcs.w`) esta en MEDIO del cuerpo, no al
|       principio ni al final. Es un bucle rotado a mano que GCC no produce
|       con esta forma exacta.
|    2. Cuatro constantes precargadas en d3/d4/d5 ($000, $FFF, $40) antes
|       del bucle para poder usar `movem.w` pareado. GCC habria emitido
|       inmediatos en cada escritura.
|    3. `bcs.w` (4 B) para un salto de 14 bytes hacia delante conviviendo
|       con un `bra.b` (2 B) hacia atras en el mismo bucle.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  Scratch_Alloc_01390E  @ $01390E  (68 bytes)
|  Callee de SceneLoader_Main_043568 (Wave HH#1), pasada 1.
|  Absorbe FP: JsrPcThunk_01394c (Wave J) = cola jsr $1399C(pc) + rts.
| ---------------------------------------------------------------------------
|
        .globl  Scratch_Alloc_01390E
        .type   Scratch_Alloc_01390E, @function
        .section .text.Scratch_Alloc_01390E, "ax", @progbits
Scratch_Alloc_01390E:
        jsr     Fn_0005A88A                     | +00  SpriteSys_Reset()
        move.w  #0x0, 0x10e1f4.l                | +06  g_unk_10E1F4  = 0
        move.w  #0x0, 0x10e1fe.l                | +0e  g_poolB_cur   = 0
        move.w  d0, 0x10e1f6.l                  | +16  g_poolB_limit = count_b
        move.w  #0x17c, d0                      | +1c  d0 = 380 (max sprites)
        sub.w   d2, d0                          | +20  d0 -= head
        move.w  d0, 0x10e1f8.l                  | +22  g_mark_10E1F8 = d0
        sub.w   d1, d0                          | +28  d0 -= mid
        move.w  d0, 0x10e1fa.l                  | +2a  g_mark_10E1FA = d0
        move.w  d0, 0x10e1fc.l                  | +30  g_poolA_cur   = d0
        move.w  #0x0, d0                        | +36  d0 = 0   (primer sprite)
        move.w  #0x17b, d1                      | +3a  d1 = 379 (ultimo)
        jsr     SpriteRange_DisableAll_01399C(pc) | +3e apaga los 380
        rts                                     | +42

        .size   Scratch_Alloc_01390E, .-Scratch_Alloc_01390E

|
| ---------------------------------------------------------------------------
|  Spawn_TypeB_013952  @ $013952  (46 bytes)
|  Reserva del pool B (crece desde 0 hasta g_poolB_limit).
|  Exporta SpawnTail_01396E: la cola de 6 instrucciones que reutiliza TypeA.
| ---------------------------------------------------------------------------
|
        .globl  Spawn_TypeB_013952
        .globl  SpawnTail_01396E
        .type   Spawn_TypeB_013952, @function
        .section .text.Spawn_TypeB_013952, "ax", @progbits
Spawn_TypeB_013952:
        move.w  0x10e1fe.l, d2                  | +00  d2 = g_poolB_cur
        move.w  d2, d3                          | +06  d3 = d2
        add.w   d0, d3                          | +08  d3 = cur + count
        cmp.w   0x10e1f6.l, d3                  | +0a  if (d3 > limit)
        bhi.w   SpriteTrapGuard_013980          | +10    -> trap #$F
        move.w  d3, 0x10e1fe.l                  | +14  g_poolB_cur = d3
        move.w  d2, d0                          | +1a  d0 = primer indice
SpawnTail_01396E:                               | $01396E  COLA COMPARTIDA
        move.w  d3, d1                          | +1c  d1 = ultimo + 1
        subq.w  #0x1, d1                        | +1e  d1 = ultimo indice
        movem.l d0-d1, -(a7)                    | +20  preserva el par
        jsr     SpriteRange_InitChain_0139BE(pc)| +24  arma la cadena
        movem.l (a7)+, d0-d1                    | +28  restaura el par
        rts                                     | +2c

        .size   Spawn_TypeB_013952, .-Spawn_TypeB_013952

|
| ---------------------------------------------------------------------------
|  SpriteTrapGuard_013980  @ $013980  (2 bytes)
|  Guarda de desbordamiento compartida por ambos spawners.
| ---------------------------------------------------------------------------
|
        .globl  SpriteTrapGuard_013980
        .type   SpriteTrapGuard_013980, @function
        .section .text.SpriteTrapGuard_013980, "ax", @progbits
SpriteTrapGuard_013980:
        trap    #0xf                            | +00  pool agotado: abortar

        .size   SpriteTrapGuard_013980, .-SpriteTrapGuard_013980

|
| ---------------------------------------------------------------------------
|  Spawn_TypeA_013982  @ $013982  (26 bytes)
|  Reserva del pool A (crece desde la marca hasta 380).
|  Termina saltando a la cola COMPARTIDA de Spawn_TypeB ($01396E).
| ---------------------------------------------------------------------------
|
        .globl  Spawn_TypeA_013982
        .type   Spawn_TypeA_013982, @function
        .section .text.Spawn_TypeA_013982, "ax", @progbits
Spawn_TypeA_013982:
        move.w  0x10e1fc.l, d2                  | +00  d2 = g_poolA_cur
        move.w  d2, d3                          | +06  d3 = d2
        add.w   d0, d3                          | +08  d3 = cur + count
        cmpi.w  #0x17c, d3                      | +0a  if (d3 > 380)
        bhi.b   SpriteTrapGuard_013980          | +0e    -> trap #$F
        move.w  d3, 0x10e1fc.l                  | +10  g_poolA_cur = d3
        move.w  d2, d0                          | +16  d0 = primer indice
        bra.b   SpawnTail_01396E                | +18  --> cola de Spawn_TypeB

        .size   Spawn_TypeA_013982, .-Spawn_TypeA_013982

|
| ---------------------------------------------------------------------------
|  SpriteRange_DisableAll_01399C  @ $01399C  (34 bytes)
|  Escribe 0 en SCB3 de cada sprite del rango: altura 0 => no dibujado.
| ---------------------------------------------------------------------------
|
        .globl  SpriteRange_DisableAll_01399C
        .type   SpriteRange_DisableAll_01399C, @function
        .section .text.SpriteRange_DisableAll_01399C, "ax", @progbits
SpriteRange_DisableAll_01399C:
        addi.w  #0x8201, d0                     | +00  d0 = SCB3 + first
        addi.w  #0x8201, d1                     | +04  d1 = SCB3 + last
        moveq   #0x0, d2                        | +08  d2 = 0 (altura/sticky)
        clr.w   0x3c0004.l                      | +0a  VRAM autoinc = 0
        movea.l #0x3c0000, a0                   | +10  a0 = puerto VRAM
.Ldis_loop:                                     | $0139B2
        movem.w d0/d2, (a0)                     | +16  SCB3[i] = 0 (MMIO par)
        addq.w  #0x1, d0                        | +1a  ++i
        cmp.w   d0, d1                          | +1c  while (last >= i)
        bcc.b   .Ldis_loop                      | +1e
        rts                                     | +20

        .size   SpriteRange_DisableAll_01399C, .-SpriteRange_DisableAll_01399C

|
| ---------------------------------------------------------------------------
|  SpriteRange_InitChain_0139BE  @ $0139BE  (64 bytes)
|  Arma [first,last] como cadena: cabeza no-sticky, resto sticky (bit 6),
|  shrink neutro ($FFF) en todos. Nucleo del objeto multi-sprite Neo Geo.
| ---------------------------------------------------------------------------
|
        .globl  SpriteRange_InitChain_0139BE
        .type   SpriteRange_InitChain_0139BE, @function
        .section .text.SpriteRange_InitChain_0139BE, "ax", @progbits
SpriteRange_InitChain_0139BE:
        move.w  d0, d2                          | +00  d2 = first
        addi.w  #0x8201, d0                     | +02  d0 = SCB3 + first
        addi.w  #0x8201, d1                     | +06  d1 = SCB3 + last
        addi.w  #0x8001, d2                     | +0a  d2 = SCB2 + first
        moveq   #0x0, d3                        | +0e  d3 = $000 (no sticky)
        move.w  #0xfff, d4                      | +10  d4 = $FFF (sin shrink)
        move.w  #0x40, d5                       | +14  d5 = $040 (sticky bit6)
        movea.l #0x3c0000, a0                   | +18  a0 = puerto VRAM
        movem.w d0/d3, (a0)                     | +1e  SCB3[first] = $000
        addq.w  #0x1, d0                        | +22  ++scb3
        movem.w d2/d4, (a0)                     | +24  SCB2[first] = $FFF
        addq.w  #0x1, d2                        | +28  ++scb2
.Lchain_test:                                   | $0139E8
        cmp.w   d0, d1                          | +2a  if (last < scb3)
        bcs.w   .Lchain_done                    | +2c    fin de la cadena
        movem.w d0/d5, (a0)                     | +30  SCB3[i] = $040 (sticky)
        addq.w  #0x1, d0                        | +34  ++scb3
        movem.w d2/d4, (a0)                     | +36  SCB2[i] = $FFF
        addq.w  #0x1, d2                        | +3a  ++scb2
        bra.b   .Lchain_test                    | +3c
.Lchain_done:                                   | $0139FC
        rts                                     | +3e

        .size   SpriteRange_InitChain_0139BE, .-SpriteRange_InitChain_0139BE
