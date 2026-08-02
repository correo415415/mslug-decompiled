| ============================================================================
|  Metal Slug 1 - asm/entity_check_box_overlap_with_selector_0798ac.s
|  ----------------------------------------------------------------------------
|  Wave RR#2 - segunda de un par de rutinas de deteccion de solape de caja.
|  Ver Entity_CheckActiveBoxOverlap_072C98 (Wave RR#1) para la variante
|  "hermana" y la comparacion estructural completa.
|
|  Entity_CheckBoxOverlapWithSelector_0798AC  @ $0798AC  (164 bytes, 1 caller)
|
|  ---------- Caller ---------------------------------------------------------
|
|      JsrPcThunk_0798a6  (thunk de 6 B, ya matcheado) -> jsr $798ac(pc)
|
|  Parametros (registros de entrada):
|      a6 = entity de referencia ("yo")
|      a1 = entity candidata a probar ("otro"); centinela NIL (-1) = no-op
|      a2 = puntero a caja EXPLICITO (a diferencia de RR#1, que la lee de
|           a6->+94; aqui el caller la pasa directamente)
|
|  Layout de la caja apuntada por a2 (mismo primer campo que en RR#1, mas
|  dos campos nuevos):
|      +0x00 box_x0   (word)
|      +0x02 box_x1   (word)
|      +0x04 box_y0   (word)
|      +0x06 box_y1   (word)   -- tambien usado como cota superior final
|      +0x08 mode     (byte)   -- selector de 2 bits (mode & 3) para la
|                                tabla de 4 punteros a funcion de la fase
|                                final (ver abajo)
|
|  Que hace:
|
|      if (a1 == NIL) return;
|
|      d0 = box->x0;  d1 = box->x1;          (SIN espejado por flags3a --
|                                              a diferencia de RR#1, esta
|                                              version no consulta +0x3a
|                                              en la fase de rango)
|      d0 += a6->pos_x;  d1 += a6->pos_x;
|      if (a1->pos_x < d0 || a1->pos_x > d1) return;
|
|      d0 = box->y0;  d1 = box->y1;
|      d0 += a6->pos_y;  d1 += a6->pos_y;
|      if (a1->pos_y < d0 || a1->pos_y > d1) return;
|
|      -- fase final: calcula una distancia horizontal con "zona muerta"
|         de medio ancho de caja, luego aplica una de 4 funciones
|         seleccionables antes de combinar con la componente Y --
|
|      d0 = box->x0 + a6->pos_x - a1->pos_x;      (borde_izq - a1.x)
|      d2 = (box->x1 - box->x0) >> 1;              (medio ancho de caja)
|      if (d0 > d2) {                               (a1 esta mas alla de la
|                                                     mitad => usar borde_der)
|          d1 = -box->x0 + a6->pos_x;               (nota: reusa box->x0,
|                                                     NO box->x1 -- posible
|                                                     idiosincrasia/bug del
|                                                     original, preservado
|                                                     tal cual)
|          d0 = a1->pos_x - d1;
|      }
|      d0 = -d0;
|
|      d3 = box->mode & 3;
|      a3 = SelectorTable_2DF4AA[d3];               (tabla de 4 punteros a
|                                                     funcion en ROM, sin
|                                                     nombre asignado --
|                                                     direccionamiento
|                                                     absoluto + jsr (a3)
|                                                     indirecto, no requiere
|                                                     symbol para el match)
|      d0 = a3();                                    (la funcion seleccionada
|                                                     recibe/devuelve d0;
|                                                     candidatas no
|                                                     analizadas todavia)
|
|      d0 += a6->pos_y - a1->pos_y;
|      if (d0 > box->y1) d0 = box->y1;               (clamp superior)
|      a1->+0x8e = d0;
|
|  Interpretacion tentativa: variante "generica" de la deteccion de caja
|  de RR#1, pensada para reutilizarse con distintas cajas/comportamientos
|  via el selector de 2 bits en box->+8 (probablemente una de 4 formas
|  distintas de resolver el "borde mas cercano" segun el tipo de target).
|  Mismo campo +0x8e de destino que RR#1, reforzando la hipotesis de que
|  es un campo compartido de "resultado de deteccion" en la entidad
|  candidata.
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_CheckBoxOverlapWithSelector_0798AC
        .type   Entity_CheckBoxOverlapWithSelector_0798AC, @function
        .section .text.Entity_CheckBoxOverlapWithSelector_0798AC, "ax", @progbits

Entity_CheckBoxOverlapWithSelector_0798AC:
        move.l  a1, d0                          | +000  d0 = a1 (test centinela)
        cmpi.l  #0xffffffff, d0                 | +002
        beq.w   .Lret                           | +008
        move.w  (a2), d0                        | +00c  d0 = box->x0
        move.w  0x2(a2), d1                     | +00e  d1 = box->x1
        move.w  0x22(a6), d2                    | +012  d2 = a6->pos_x
        add.w   d2, d0                          | +016
        add.w   d2, d1                          | +018
        move.w  0x22(a1), d2                    | +01a  d2 = a1->pos_x
        cmp.w   d0, d2                          | +01e
        blt.w   .Lret                           | +020
        cmp.w   d1, d2                           | +024
        bgt.w   .Lret                           | +026
        move.w  0x4(a2), d0                     | +02a  d0 = box->y0
        move.w  0x6(a2), d1                     | +02e  d1 = box->y1
        move.w  0x24(a6), d2                    | +032  d2 = a6->pos_y
        add.w   d2, d0                          | +036
        add.w   d2, d1                          | +038
        move.w  0x24(a1), d2                    | +03a  d2 = a1->pos_y
        cmp.w   d0, d2                          | +03e
        blt.w   .Lret                           | +040
        cmp.w   d1, d2                           | +044
        bgt.w   .Lret                           | +046
        move.w  (a2), d0                        | +04a  d0 = box->x0
        move.w  d0, d1                          | +04c  d1 = box->x0 (copia)
        add.w   0x22(a6), d0                    | +04e  d0 += a6->pos_x
        sub.w   0x22(a1), d0                    | +052  d0 -= a1->pos_x
        move.w  0x2(a2), d2                     | +056  d2 = box->x1
        sub.w   d1, d2                          | +058  d2 -= box->x0 (ancho)
        asr.w   #1, d2                          | +05a  d2 = ancho / 2
        cmp.w   d2, d0                          | +05c
        ble.w   .Lnear                          | +05e  d0 <= medio-ancho: salta
        move.w  (a2), d1                        | +062  d1 = box->x0
        neg.w   d1                              | +064
        add.w   0x22(a6), d1                    | +066
        move.w  0x22(a1), d0                    | +06a  d0 = a1->pos_x
        sub.w   d1, d0                          | +06e
.Lnear:
        neg.w   d0                              | +070
        move.b  0x8(a2), d3                     | +072  d3 = box->mode
        andi.w  #3, d3                           | +076  d3 &= 3
        lsl.w   #2, d3                           | +07a  d3 *= 4 (indice x4)
        lea.l   0x2df4aa.l, a3                  | +07c  a3 = &SelectorTable_2DF4AA
        movea.l (a3, d3.w), a3                  | +082  a3 = tabla[d3]
        jsr     (a3)                            | +086  d0 = a3()  (indirecto,
                                                 |        sin symbol necesario)
        add.w   0x24(a6), d0                    | +088  d0 += a6->pos_y
        sub.w   0x24(a1), d0                    | +08c  d0 -= a1->pos_y
        cmp.w   0x6(a2), d0                     | +090  d0 vs box->y1
        ble.w   .Lstore                         | +094
        move.w  0x6(a2), d0                     | +098  clamp: d0 = box->y1
.Lstore:
        move.w  d0, 0x8e(a1)                    | +09c  a1->+8e = d0
.Lret:
        rts                                     | +0a2

        .size   Entity_CheckBoxOverlapWithSelector_0798AC, .-Entity_CheckBoxOverlapWithSelector_0798AC
