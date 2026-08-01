| ============================================================================
|  Metal Slug 1 - asm/entity_trail_record_099812.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity/Sprite helpers) - funcion #6
|
|  Entity_TrailRecord_099812  @ $099812  (184 bytes, 7 callers)
|
|  Registra una muestra en el "trail buffer" (ring buffer de 16 x 12 B en
|  $10E3BE) que utilizan los efectos de estela / partículas de humo /
|  destello de disparo para saber donde estaba la entidad hace N frames.
|
|  Estructura del buffer (a documentar en mslug.h):
|
|      TRAIL_BUFFER_BASE  = $10E3BE
|      TRAIL_TAIL_INDEX_W = $10E47E   (word: byte offset del "tail" de lectura)
|      TRAIL_HEAD_WRAP_W  = $10E480   (word: byte offset donde el tail wrapea)
|      TRAIL_HEAD_INDEX_W = $10E482   (word: byte offset del cursor de escritura)
|      TRAIL_ID_COUNTER_B = $10E484   (byte: contador ciclico de "id de trail",
|                                     modulo 15)
|
|      struct TrailSlot {          -- 12 B
|          uint8_t  id;            -- +0    id del trail (nibble alto de flags6c)
|          uint8_t  _pad;          -- +1
|          uint16_t pos_x;         -- +2    posicion X actual
|          uint16_t pos_y;         -- +4    posicion Y actual
|          uint16_t w6;            -- +6    parametro opaco del llamador (d2)
|          uint16_t dx;            -- +8    delta X vs. muestra previa
|          uint16_t dy;            -- +a    delta Y vs. muestra previa
|      };
|
|  Firma C conceptual:
|
|      /* Registra la posicion actual (d0=x, d1=y, d2=w6) del entity 'a6'
|       * en el ring buffer de trails. Si flags6c(a6).hi == 0xF, asigna
|       * primero un id nuevo; si no, busca el slot con id==flags6c.hi y
|       * calcula el delta contra su ultima muestra. */
|      void Entity_TrailRecord(uint16_t x   /*d0*/,
|                              uint16_t y   /*d1*/,
|                              uint16_t w6  /*d2*/,
|                              struct Entity *a6);
|
|  Notas forenses:
|    - lsr.b #4,d7 sobre un scratch d7 previamente cargado con move.b
|      completo del field (sin zero-extend con moveq #0,d7 previo) es
|      idioma clasico de asm 68000: el codigo se apoya en que el byte
|      alto de d7 puede quedar sucio porque solo se compara/enmascara la
|      parte baja. GCC habria emitido moveq #0,d7 antes del load.
|    - move.b #0,d7 (4 B) donde GCC habria emitido clr.b d7 (2 B) es
|      otra evidencia forense: el codigo original usa la forma larga
|      probablemente para tener un slot de 4 B de codigo intercambiable
|      con otra constante durante desarrollo.
|    - clr.w d5 + clr.w d6 en dos sitios distintos que convergen en el
|      mismo epilogo (bra.w $99896) es tipico de "vaciar deltas cuando
|      no hay historia" en un tracker; GCC habria factorizado.
|    - Los tres callers directos identificados por scan_unmatched_callees
|      son escritos como bsr.w (rango 16-bit): consistente con un helper
|      grande usado por rutinas cercanas en el segmento $099xxx.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_TrailRecord_099812
        .type   Entity_TrailRecord_099812, @function
        .section .text.Entity_TrailRecord_099812, "ax", @progbits

Entity_TrailRecord_099812:
        lea     0x10e3be.l, a0          | +000  a0 = TRAIL_BUFFER_BASE
        move.b  0x6c(a6), d7            | +006  d7 = entity->flags6c
        lsr.b   #4, d7                  | +00c  d7 = trail_id (nibble alto)
        cmpi.b  #0xf, d7                | +00e  primer registro? (id == 0xF marca "sin asignar")
        bne.w   .Lhas_id                | +012  no: ir a busqueda por id

|--- Rama A: primera muestra, asigna id nuevo -------------------------------
        move.b  0x10e484.l, d7          | +016  d7 = TRAIL_ID_COUNTER
        addq.b  #1, d7                  | +01c  d7 += 1
        cmpi.b  #0xe, d7                | +01e  d7 < 14 ?
        bcs.w   .Lstore_id              | +022  si: usar d7 tal cual
        move.b  #0, d7                  | +026  no: wrap a 0 (contador cíclico mod 15)
.Lstore_id:
        move.b  d7, 0x10e484.l          | +02a  TRAIL_ID_COUNTER = d7
        move.b  d7, d6                  | +030  d6 = d7 (temp)
        lsl.b   #4, d6                  | +032  d6 <<= 4 (nibble alto)
        move.b  0x6c(a6), d5            | +034  d5 = entity->flags6c
        andi.b  #0xf, d5                | +038  preservar nibble bajo
        or.b    d6, d5                  | +03c  fusionar
        move.b  d5, 0x6c(a6)            | +03e  entity->flags6c = nuevo id | nibble bajo original
        clr.w   d5                      | +042  dx = 0 (no hay muestra previa)
        clr.w   d6                      | +044  dy = 0
        bra.w   .Lstore_slot            | +046  saltar al epilogo comun

|--- Rama B: id ya asignado, buscar slot y calcular delta -------------------
|  Nota forense: cada iteracion del bucle vuelve a comparar el tail contra
|  head_wrap ($10e480), no solo el slot.id. Es una salvaguarda contra bucle
|  infinito: si el tail alcanza al head_wrap, no hay muestra previa y se
|  aborta con dx=dy=0. GCC no habria puesto la relectura del tail dentro
|  del bucle; habria cacheado el limite en un registro.
.Lhas_id:
        move.w  0x10e47e.l, d3          | +04a  d3 = TRAIL_TAIL_INDEX (cursor de lectura)
.Lcheck_wrap:
        cmp.w   0x10e480.l, d3          | +050  ¿ tail == head_wrap ? (fin de historia)
        bne.w   .Lprobe_slot            | +056  no: sondear el slot actual
        clr.w   d5                      | +05a  si: no hay historia, dx=dy=0
        clr.w   d6                      | +05c
        bra.w   .Lstore_slot            | +05e
.Lprobe_slot:
        cmp.b   (a0, d3.w), d7          | +062  slot[d3].id == d7 ?
        beq.w   .Lfound                 | +066  si: calcular delta
        addi.w  #0xc, d3                | +06a  d3 += 12 (siguiente slot)
        cmpi.w  #0xc0, d3               | +06e  d3 < 192 ?
        bcs.w   .Lno_wrap               | +072  si: seguir sin wrap
        clr.w   d3                      | +076  no: wrap a 0
.Lno_wrap:
        bra.b   .Lcheck_wrap            | +078  reintentar chequeando tail == head_wrap
.Lfound:
        move.w  d0, d5                  | +07a  d5 = x actual
        sub.w   0x2(a0, d3.w), d5       | +07c  d5 -= slot.pos_x  -> dx
        move.w  d1, d6                  | +080  d6 = y actual
        sub.w   0x4(a0, d3.w), d6       | +082  d6 -= slot.pos_y  -> dy

|--- Epilogo comun: escribir el nuevo slot en TRAIL_HEAD y avanzar cursor ---
.Lstore_slot:
        move.w  0x10e482.l, d3          | +086  d3 = TRAIL_HEAD_INDEX (cursor escritura)
        move.b  d7, (a0, d3.w)          | +08c  slot.id     = d7
        move.w  d0, 0x2(a0, d3.w)       | +090  slot.pos_x  = d0
        move.w  d1, 0x4(a0, d3.w)       | +094  slot.pos_y  = d1
        move.w  d2, 0x6(a0, d3.w)       | +098  slot.w6     = d2
        move.w  d5, 0x8(a0, d3.w)       | +09c  slot.dx     = d5
        move.w  d6, 0xa(a0, d3.w)       | +0a0  slot.dy     = d6
        addi.w  #0xc, d3                | +0a4  d3 += 12 (siguiente slot)
        cmpi.w  #0xc0, d3               | +0a8  d3 < 192 ?
        bcs.w   .Lhead_no_wrap          | +0ac  si: mantener
        clr.w   d3                      | +0b0  no: wrap a 0
.Lhead_no_wrap:
        move.w  d3, 0x10e482.l          | +0b2  TRAIL_HEAD_INDEX = d3
        rts                             | +0b8
        .size   Entity_TrailRecord_099812, .-Entity_TrailRecord_099812
