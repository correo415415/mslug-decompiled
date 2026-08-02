| ============================================================================
|  Metal Slug 1 - asm/entity_check_active_box_overlap_072c98.s
|  ----------------------------------------------------------------------------
|  Wave RR#1 - primera de un par de rutinas de deteccion de solape de caja
|  (la otra es Entity_CheckBoxOverlapWithSelector_0798AC, Wave RR#2 -- misma
|  familia, ver ese archivo para la comparacion completa).
|
|  Entity_CheckActiveBoxOverlap_072C98  @ $072C98  (144 bytes, 1 caller)
|
|  ---------- Caller ---------------------------------------------------------
|
|      JsrPcThunk_072c92  (thunk de 6 B, ya matcheado) -> jsr $72c98(pc)
|
|  Solo un caller conocido en el ROM (via thunk PC-relativo); el 6 B thunk
|  en si mismo puede tener multiples referencias externas que no hemos
|  rastreado, tipico de esta familia JsrPcThunk_*.
|
|  Parametros (registros de entrada, convencion "entity en a6"):
|      a6 = entity que ejecuta la comprobacion ("yo")
|      a1 = entity candidata a probar ("otro"); centinela NIL (-1) = no-op
|      (a2 NO es parametro -- se recalcula desde a6->+94, ver abajo)
|
|  Que hace:
|
|      if (a1 == NIL) return;                      guard de centinela
|
|      a2 = *(a6 + 0x94)                            puntero a "caja activa"
|                                                    de a6 (campo nuevo, no
|                                                    documentado hasta ahora
|                                                    en include/mslug.h;
|                                                    candidato a
|                                                    "active_box" o similar)
|      d0 = box->+0x00   (word)                     borde "izquierdo" caja
|      d1 = box->+0x02   (word)                     borde "derecho" caja
|      if (a6->flags3a & 1) { exg d0,d1; neg d0; neg d1; }
|                                                    espejo horizontal si a6
|                                                    mira a la izquierda
|                                                    (mismo flag +0x3a que
|                                                    Entity_CopyTransform)
|      d0 += a6->pos_x;  d1 += a6->pos_x;            bordes en coords mundo
|      if (a1->pos_x < d0 || a1->pos_x > d1) return; fuera del rango X: sale
|
|      d0 = box->+0x04;  d1 = box->+0x06;            borde superior/inferior
|      d0 += a6->pos_y;  d1 += a6->pos_y;
|      if (a1->pos_y < d0 || a1->pos_y > d1) return; fuera del rango Y: sale
|
|      d0 = box->+0x00 + a6->pos_x - a1->pos_x;
|      if (a6->flags3a & 1) {
|          d1 = -box->+0x00 + a6->pos_x;
|          d0 = a1->pos_x - d1;
|      }
|      d0 = -d0;
|      d0 += a6->pos_y - a1->pos_y;
|      a1->+0x8e = d0;         (word, campo nuevo, ver nota abajo)
|
|  Interpretacion tentativa: rutina de deteccion "esta a1 dentro de la
|  caja activa de a6" (caja rectangular relativa a a6->pos_x/pos_y, ya
|  espejada por la orientacion de a6), analoga a un "hitbox/hurtbox
|  check" o zona de deteccion de IA. Si a1 cae dentro de la caja, escribe
|  en a1->+0x8e un valor compuesto (mezcla componentes X e Y, tal cual
|  surge de seguir los bytes -- no es una distancia euclidiana ni un
|  "overlap depth" clasico, mas bien parece una clave/offset usado por
|  otra funcion consumidora que aun no hemos localizado). El campo
|  +0x8e y el puntero +0x94 quedan documentados aqui como primera
|  evidencia; se anadiran a include/mslug.h cuando un segundo consumidor
|  independiente confirme el layout.
|
|  Comparese con Entity_CheckBoxOverlapWithSelector_0798AC (RR#2): misma
|  estructura general de "check rango X, check rango Y, calcular y
|  guardar en target->+8e", pero esa variante recibe la caja como
|  parametro explicito (a2) en vez de leerla de a6->+94, no aplica el
|  espejado por flags3a en el chequeo de rango, y usa una tabla de 4
|  punteros a funcion (byte box->+8 & 3) para la fase final en vez del
|  espejado condicional que usa esta version.
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_CheckActiveBoxOverlap_072C98
        .type   Entity_CheckActiveBoxOverlap_072C98, @function
        .section .text.Entity_CheckActiveBoxOverlap_072C98, "ax", @progbits

Entity_CheckActiveBoxOverlap_072C98:
        move.l  a1, d0                          | +000  d0 = a1 (para test centinela)
        cmpi.l  #0xffffffff, d0                 | +002  d0 == NIL ?
        beq.w   .Lret                           | +008  si, salir
        movea.l 0x94(a6), a2                    | +00c  a2 = a6->+94 (caja activa)
        move.w  (a2), d0                        | +010  d0 = box->+0
        move.w  0x2(a2), d1                     | +012  d1 = box->+2
        btst.b  #0, 0x3a(a6)                    | +016  a6 mira a la izquierda?
        beq.w   .Lskip_flip1                    | +01c
        exg     d0, d1                          | +020  espejo: intercambia
        neg.w   d0                              | +022  y niega ambos bordes
        neg.w   d1                              | +024
.Lskip_flip1:
        move.w  0x22(a6), d2                    | +026  d2 = a6->pos_x
        add.w   d2, d0                          | +02a  d0 = borde_izq mundo
        add.w   d2, d1                          | +02c  d1 = borde_der mundo
        move.w  0x22(a1), d2                    | +02e  d2 = a1->pos_x
        cmp.w   d0, d2                          | +032
        blt.w   .Lret                           | +034  a1.x < borde_izq: sale
        cmp.w   d1, d2                           | +038
        bgt.w   .Lret                           | +03a  a1.x > borde_der: sale
        move.w  0x4(a2), d0                     | +03e  d0 = box->+4
        move.w  0x6(a2), d1                     | +042  d1 = box->+6
        move.w  0x24(a6), d2                    | +046  d2 = a6->pos_y
        add.w   d2, d0                          | +04a
        add.w   d2, d1                          | +04c
        move.w  0x24(a1), d2                    | +04e  d2 = a1->pos_y
        cmp.w   d0, d2                          | +052
        blt.w   .Lret                           | +054  a1.y < borde_sup: sale
        cmp.w   d1, d2                           | +058
        bgt.w   .Lret                           | +05a  a1.y > borde_inf: sale
        move.w  (a2), d0                        | +05e  d0 = box->+0
        add.w   0x22(a6), d0                    | +060  d0 += a6->pos_x
        sub.w   0x22(a1), d0                    | +064  d0 -= a1->pos_x
        btst.b  #0, 0x3a(a6)                    | +068  espejo (2a vez)?
        beq.w   .Lskip_flip2                    | +06e
        move.w  (a2), d1                        | +072  d1 = box->+0
        neg.w   d1                              | +074  d1 = -box->+0
        add.w   0x22(a6), d1                    | +076  d1 += a6->pos_x
        move.w  0x22(a1), d0                    | +07a  d0 = a1->pos_x
        sub.w   d1, d0                          | +07e  d0 -= d1
.Lskip_flip2:
        neg.w   d0                              | +080
        add.w   0x24(a6), d0                    | +082  d0 += a6->pos_y
        sub.w   0x24(a1), d0                    | +086  d0 -= a1->pos_y
        move.w  d0, 0x8e(a1)                    | +08a  a1->+8e = d0
.Lret:
        rts                                     | +08e

        .size   Entity_CheckActiveBoxOverlap_072C98, .-Entity_CheckActiveBoxOverlap_072C98
