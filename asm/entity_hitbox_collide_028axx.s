| ============================================================================
|  Metal Slug 1 - asm/entity_hitbox_collide_028axx.s
|  ----------------------------------------------------------------------------
|  Wave SS#2 + SS#3 - nucleo del sistema de colision entity-vs-entity.
|
|  Dos funciones intimamente acopladas (caller + callee, ambas en el cluster
|  $028Axx..$028Cxx que Wave QQ#2 ya toco con Entity_ApplyFadeShade_028108):
|
|    Entity_HitboxCollide_028A96   @ $028A96  (114 B, 4 callers pc-rel:
|                                   $0289B4/$0289BE/$0289CC/$0289E2, todos
|                                   dentro del bucle de barrido de entities
|                                   aun no matcheado en $0289xx)
|    Hitbox_OverlapTestXY_028B14   @ $028B14  (268 B, 1 caller: SS#2)
|
|  Absorbe 3 falsos positivos de Wave N (helpers CCR que en realidad son
|  los epilogos internos de SS#3):
|      ClearXN_028b7c  ($028B7C, 6 B)  rama "sin solape en X"
|      ClearXN_028c14  ($028C14, 6 B)  rama "sin solape en Y"
|      SetXN_028c1a    ($028C1A, 6 B)  rama "solape confirmado"
|  (ClearXN_028b08 / SetXN_028b0e permanecen como islas matcheadas entre
|  ambas funciones: son los epilogos de SS#2, alcanzados por branch externo,
|  igual que el idioma de epilogo compartido documentado en Wave JJ#1.)
|
|  ---------- Firma C conceptual (SS#2) --------------------------------------
|
|    /* a6 = entity atacante, a0 = descriptor de hitbox del atacante,
|       a1 = entity candidata a victima.
|       Retorno via CCR: carry/X set = hubo colision.  */
|    bool Entity_HitboxCollide(Entity *att /*a6*/, HitboxDesc *box /*a0*/,
|                              Entity *victim /*a1*/)
|    {
|        if (victim == att)                return false;   // no self-hit
|        HitboxDesc *vbox = victim->hitbox_60;              // +0x60
|        if (vbox == NIL)                  return false;
|        if (vbox->type != 0x80)           return false;   // byte +0 del desc
|        if (victim->flags69 & BIT(3))     return false;   // inmune/ya visto
|        // saltar cabecera de 6 B de ambos descriptores:
|        if (!Hitbox_OverlapTestXY(att, box+6h, victim, vbox+6h)) return false;
|        // d7/d6 salen del test = lado relativo (0/1) de cada entity:
|        if (!(victim->flags6b & BIT(6)))
|            att->flags69    |= BIT(d7+1);   // bit 1 o 2 segun lado
|        victim->flags69     |= BIT(d6+1);
|        victim->flags69     &= ~BIT(2);
|        if (box->byte1 == 2)                // +1 del descriptor: "tipo 2"
|            victim->flags69 |= BIT(2);
|        return true;
|    }
|
|  ---------- Firma C conceptual (SS#3) --------------------------------------
|
|    /* a6/a1 = entities, a4 = extents del atacante (tras cabecera),
|       a3 = extents de la victima.  Layout extents: +0/+2 = minX/maxX,
|       +4/+6 = minY/maxY, relativos a la posicion de la entity.
|       Devuelve carry set si los rectangulos (espejados por facing/flip
|       de cada entity) se solapan en X y en Y.  Efecto lateral: d7/d6 =
|       slt/sge del centro relativo (que lado ocupa cada uno).  */
|    bool Hitbox_OverlapTestXY(Entity *att /*a6*/, s16 *abox /*a4*/,
|                              Entity *victim /*a1*/, s16 *vbox /*a3*/)
|
|  HALLAZGO FORENSE MAYOR - ASSERTIONS DE DEBUG `trap #15` NOP-PATCHED:
|  SS#3 contiene CUATRO bloques identicos de la forma
|
|        cmp.w   dY, dX
|        blt.w   .Lok          | asercion: min < max
|        nop
|        nop
|        cmp.w   dY, dX        | (repite el cmp - resto de macro)
|        nop
|        trap    #15           | breakpoint de debugger si min >= max
|    .Lok:
|
|  Primera evidencia directa del proyecto de una MACRO DE ASSERT del
|  build de desarrollo de Nazca: los `nop` sugieren instrucciones del
|  cuerpo original de la macro parcheadas/condicionalmente ensambladas
|  (p.ej. un `move` de contexto para el debugger eliminado en release,
|  dejando el esqueleto cmp/trap).  `trap #15` era el vector clasico de
|  breakpoint de los ICE/monitores 68000 (tambien usado por el monitor
|  de MAME dev).  El juego ASUME cajas bien formadas (min<max); la
|  asercion solo dispara con datos corruptos.
|
|  Espejado por orientacion (mismo idioma que Wave RR#1/#2):
|      bit 0 de flags3a = facing horizontal -> neg + exg de minX/maxX
|      bit 1 de flags3a = flip vertical     -> neg + exg de minY/maxY
|  El eje X ademas calcula el solape-interseccion [max(min), min(max)]
|  y deriva d7/d6 (lado relativo) con slt/sge - valores que el caller
|  SS#2 usa como NUMERO DE BIT (+1) en los flags69 de cada entity:
|  bit1 = "golpeado por la izquierda", bit2 = "por la derecha"
|  (interpretacion tentativa; consistente con addq.b #1 + bset dinamico).
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text

| ----------------------------------------------------------------------------
|  SS#2  Entity_HitboxCollide_028A96  ($028A96..$028B07, 114 B)
|  Sale por branch a las islas matcheadas ClearXN_028b08 (false) /
|  SetXN_028b0e (true); no tiene rts propio.
| ----------------------------------------------------------------------------
        .globl  Entity_HitboxCollide_028A96
        .type   Entity_HitboxCollide_028A96, @function
        .section .text.Entity_HitboxCollide_028A96, "ax", @progbits

Entity_HitboxCollide_028A96:
        cmpa.l  a6, a1                          | +000  victima == atacante ?
        beq.w   ClearXN_028b08                  | +002  si: return false
        movea.l 0x60(a1), a2                    | +006  a2 = victim->hitbox_60
        cmpa.l  #0xffffffff, a2                 | +00a  descriptor NIL ?
        beq.w   ClearXN_028b08                  | +010  si: return false
        cmpi.b  #0x80, (a2)                     | +014  tipo != 0x80 ?
        bne.w   ClearXN_028b08                  | +018  si: return false
        btst.b  #3, 0x69(a1)                    | +01c  victima inmune ?
        bne.w   ClearXN_028b08                  | +022  si: return false
        movea.l a2, a3                          | +026  a3 = vbox
        adda.l  #6, a3                          | +028  ... + cabecera 6 B
        movea.l a0, a4                          | +02e  a4 = abox
        adda.l  #6, a4                          | +030  ... + cabecera 6 B
        jsr     Hitbox_OverlapTestXY_028B14(pc) | +036  test geometrico
        bcs.w   .Lhit                           | +03a  carry: hay solape
        bra.w   ClearXN_028b08                  | +03e  no: return false
.Lhit:
        addq.b  #1, d7                          | +042  d7 = lado att (0/1) +1
        btst.b  #6, 0x6b(a1)                    | +044  victima "no marca"?
        bne.w   .Lskip_att                      | +04a
        bset.b  d7, 0x69(a6)                    | +04e  att->flags69 |= bit d7
.Lskip_att:
        addq.b  #1, d6                          | +052  d6 = lado victim +1
        bset.b  d6, 0x69(a1)                    | +054  victim->flags69 |= bit d6
        bclr.b  #2, 0x69(a1)                    | +058  limpia bit "tipo 2"
        cmpi.b  #2, 0x1(a0)                     | +05e  box->byte1 == 2 ?
        bne.w   .Ldone                          | +064
        bset.b  #2, 0x69(a1)                    | +068  marca "tipo 2"
.Ldone:
        bra.w   SetXN_028b0e                    | +06e  return true (X/C set)

| ----------------------------------------------------------------------------
|  SS#3  Hitbox_OverlapTestXY_028B14  ($028B14..$028C1F, 268 B)
|  Absorbe ClearXN_028b7c / ClearXN_028c14 / SetXN_028c1a como epilogos.
| ----------------------------------------------------------------------------
        .globl  Hitbox_OverlapTestXY_028B14
        .type   Hitbox_OverlapTestXY_028B14, @function
        .section .text.Hitbox_OverlapTestXY_028B14, "ax", @progbits

Hitbox_OverlapTestXY_028B14:
        | ---- eje X, atacante: d0/d1 = minX/maxX espejados ------------------
        move.w  (a4), d0                        | +000  d0 = abox->minX
        move.w  0x2(a4), d1                     | +002  d1 = abox->maxX
        btst.b  #0, 0x3a(a6)                    | +006  att mira a la izq ?
        beq.w   .Lax_ok                         | +00c
        neg.w   d0                              | +010  espejo horizontal:
        neg.w   d1                              | +012  niega e intercambia
        exg     d0, d1                          | +014
.Lax_ok:
        | ---- ASSERT(minX < maxX)  [macro debug nop-patched, ver cabecera] --
        cmp.w   d1, d0                          | +016
        blt.w   .Lax_assert_ok                  | +018
        nop                                     | +01c  (cuerpo de macro
        nop                                     | +01e   parcheado a nop)
        cmp.w   d1, d0                          | +020
        nop                                     | +022
        trap    #15                             | +024  breakpoint debugger
.Lax_assert_ok:
        | ---- eje X, victima: d2/d3 = minX/maxX espejados -------------------
        move.w  (a3), d2                        | +026  d2 = vbox->minX
        move.w  0x2(a3), d3                     | +028  d3 = vbox->maxX
        btst.b  #0, 0x3a(a1)                    | +02c  victima mira a izq ?
        beq.w   .Lvx_ok                         | +032
        neg.w   d2                              | +036
        neg.w   d3                              | +038
        exg     d2, d3                          | +03a
.Lvx_ok:
        | ---- ASSERT(minX < maxX) victima -----------------------------------
        cmp.w   d3, d2                          | +03c
        blt.w   .Lvx_assert_ok                  | +03e
        nop                                     | +042
        nop                                     | +044
        cmp.w   d3, d2                          | +046
        nop                                     | +048
        trap    #15                             | +04a
.Lvx_assert_ok:
        | ---- a coordenadas mundo y test de solape en X ---------------------
        add.w   0x22(a6), d0                    | +04c  += att->pos_x
        add.w   0x22(a6), d1                    | +050
        add.w   0x22(a1), d2                    | +054  += victim->pos_x
        add.w   0x22(a1), d3                    | +058
        cmp.w   d1, d2                          | +05c  victim.min > att.max ?
        bgt.w   .Lno_overlap_x                  | +05e  si: sin solape
        cmp.w   d3, d0                          | +062  att.min <= victim.max ?
        ble.w   .Lx_overlap                     | +064  si: solapan en X
.Lno_overlap_x:                                 |       (= ClearXN_028b7c absorbido)
        andi.b  #0xEE, ccr                      | +068  clear X/N/C = false
        rts                                     | +06c
.Lx_overlap:
        | ---- interseccion [max(min), min(max)] + lado relativo -------------
        cmp.w   d2, d0                          | +06e  d0 = max(d0, d2)
        bcc.w   .Lkeep_min                      | +070
        move.w  d2, d0                          | +074
.Lkeep_min:
        cmp.w   d3, d1                          | +076  d1 = min(d1, d3)
        bcs.w   .Lkeep_max                      | +078
        move.w  d3, d1                          | +07c
.Lkeep_max:
        move.w  0x22(a1), d4                    | +07e  d4 = victim.x + vbox->maxX
        move.w  0x22(a6), d5                    | +082  d5 = att.x + abox->maxX
        add.w   0x2(a2), d4                     | +086  (a2 sigue = vbox base+0)
        add.w   0x2(a0), d5                     | +08a  (a0 sigue = abox base+0)
        cmp.w   d4, d5                          | +08e  que borde derecho queda
        slt     d7                              | +090  d7 = (att < victim) ? -1:0
        sge     d6                              | +092  d6 = complementario
        | ---- eje Y, atacante ------------------------------------------------
        move.w  0x4(a4), d0                     | +094  d0 = abox->minY
        move.w  0x6(a4), d1                     | +098  d1 = abox->maxY
        btst.b  #1, 0x3a(a6)                    | +09c  att flip vertical ?
        beq.w   .Lay_ok                         | +0a2
        neg.w   d0                              | +0a6
        neg.w   d1                              | +0a8
        exg     d0, d1                          | +0aa
.Lay_ok:
        | ---- ASSERT(minY < maxY) atacante ----------------------------------
        cmp.w   d1, d0                          | +0ac
        blt.w   .Lay_assert_ok                  | +0ae
        nop                                     | +0b2
        nop                                     | +0b4
        cmp.w   d1, d0                          | +0b6
        nop                                     | +0b8
        trap    #15                             | +0ba
.Lay_assert_ok:
        | ---- eje Y, victima --------------------------------------------------
        move.w  0x4(a3), d2                     | +0bc  d2 = vbox->minY
        move.w  0x6(a3), d3                     | +0c0  d3 = vbox->maxY
        btst.b  #1, 0x3a(a1)                    | +0c4  victima flip vertical ?
        beq.w   .Lvy_ok                         | +0ca
        neg.w   d2                              | +0ce
        neg.w   d3                              | +0d0
        exg     d2, d3                          | +0d2
.Lvy_ok:
        | ---- ASSERT(minY < maxY) victima -----------------------------------
        cmp.w   d3, d2                          | +0d4
        blt.w   .Lvy_assert_ok                  | +0d6
        nop                                     | +0da
        nop                                     | +0dc
        cmp.w   d3, d2                          | +0de
        nop                                     | +0e0
        trap    #15                             | +0e2
.Lvy_assert_ok:
        | ---- a coordenadas mundo y test de solape en Y ---------------------
        add.w   0x24(a6), d0                    | +0e4  += att->pos_y
        add.w   0x24(a6), d1                    | +0e8
        add.w   0x24(a1), d2                    | +0ec  += victim->pos_y
        add.w   0x24(a1), d3                    | +0f0
        cmp.w   d1, d2                          | +0f4  victim.min > att.max ?
        bgt.w   .Lno_overlap_y                  | +0f6  si: sin solape
        cmp.w   d3, d0                          | +0fa  att.min <= victim.max ?
        ble.w   .Loverlap                       | +0fc  si: SOLAPE TOTAL
.Lno_overlap_y:                                 |       (= ClearXN_028c14 absorbido)
        andi.b  #0xEE, ccr                      | +100  clear X/N/C = false
        rts                                     | +104
.Loverlap:                                      |       (= SetXN_028c1a absorbido)
        ori.b   #0x11, ccr                      | +106  set X/C = true (hit)
        rts                                     | +10a
