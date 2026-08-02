| ============================================================================
|  Metal Slug 1 - asm/entity_group_spawn_linked_from_template_list_065c94.s
|  ----------------------------------------------------------------------------
|  Wave RR#5 - constructor de grupo de entities enlazadas desde una lista
|  de templates. Usa Entity_MirrorDeltaByFacing_065D32 (Wave RR#4, mismo
|  archivo de wave) como sub-rutina.
|
|  EntityGroup_SpawnLinkedFromTemplateList_065C94  @ $065C94  (84 bytes,
|  5 callers)
|
|  ---------- Mapa de callers -------------------------------------------------
|
|      JsrPcThunk_065eee  (thunk, ya matcheado) -> jsr $65c94(pc)
|      JsrPcThunk_065efa  (thunk, ya matcheado) -> jsr $65c94(pc)
|      JsrPcThunk_065f06  (thunk, ya matcheado) -> jsr $65c94(pc)
|      JsrPcThunk_065f12  (thunk, ya matcheado) -> jsr $65c94(pc)
|      JsrPcThunk_065f1e  (thunk, ya matcheado) -> jsr $65c94(pc)
|
|  5 thunks distintos apuntan aqui -- el mayor numero de callers directos
|  de toda la Wave RR, consistente con un helper "de infraestructura"
|  reutilizado por varios constructores de entity (vehiculos/enemigos
|  multi-parte con sub-entities enlazadas, ver slot_parent/slot_child en
|  include/mslug.h).
|
|  Parametro de entrada:
|      a1 = puntero al PRIMER nodo de una lista de "templates de spawn"
|           en formato *(a1) == 0xFFFFFFFF marca el fin de lista. Cada
|           nodo mide 12 bytes (0xC), stride confirmado por
|           `adda.l #0xc, a1` en el loop:
|
|             +0x00 (long)  id/puntero de enlace  (copiado tal cual al
|                                                   nuevo task->+0x00)
|             +0x04 (word)  delta_x               (espejado por facing
|                                                   antes de sumarse)
|             +0x06 (word)  delta_y               (sumado sin espejar)
|             +0x08 (word)  delta_flags3a          (solo se usa el byte
|                                                   bajo, EOR sobre
|                                                   new_task->flags3a)
|             +0x0a (word)  delta_flags38          (sumado a
|                                                   new_task->flags38)
|
|  Que hace (por cada nodo hasta el centinela):
|
|      while (*(long*)a1 != 0xFFFFFFFF) {
|          long node_id = *(long*)a1;
|          Task *a0 = Task_AllocFromFreeList(&Jsr5B6ThenJmpScheduler_064d8a);
|                                              nuevo task, handler instalado
|                                              = la funcion (usada solo como
|                                              direccion, no se llama aqui)
|          Entity_CopyTransform();              copia flags11/pos_x/pos_y/
|                                              flags38/flags3a desde el
|                                              "molde" activo al nuevo task
|                                              (mismo helper de Wave S#4)
|          a0->+0x00 = node_id;                 restaura el id de enlace
|                                              guardado antes de las jsr
|
|          d0 = node->delta_x;                  (a1 + 0x4, word)
|          d0 = Entity_MirrorDeltaByFacing_065D32(d0);   espeja si a6 mira
|                                                        a la izquierda
|          a0->pos_x += d0;
|
|          a0->pos_y += node->delta_y;          (a1 + 0x6, SIN espejar)
|          a0->flags3a ^= (u8)node->delta_flags3a;   (a1 + 0x8, byte bajo)
|          a0->flags38 += node->delta_flags38;   (a1 + 0xa)
|
|          a1 += 0xc;                            siguiente nodo
|      }
|      return;   (rts explicito -- no es tail-call, hay loop antes)
|
|  Interpretacion: "constructor de grupo enlazado": recorre una tabla de
|  templates terminada en centinela y crea un nuevo task/entity por cada
|  entrada, copiando el estado base del "molde" activo (via
|  Entity_CopyTransform, que opera implicitamente sobre a6/contexto
|  activo) y aplicandole offsets de posicion/flags especificos de cada
|  sub-parte -- el patron tipico para instanciar las piezas de un
|  vehiculo o enemigo multi-segmento (torretas, orugas, remolques) con
|  posiciones relativas que se espejan automaticamente segun la
|  orientacion del "padre". El campo delta_x se espeja (mundo real:
|  offset horizontal relativo al frente del vehiculo) mientras delta_y
|  no lo necesita (la orientacion en este juego es solo horizontal).
|
|  Nota tecnica: `lea.l Jsr5B6ThenJmpScheduler_064d8a(pc), a1` toma la
|  DIRECCION de una funcion ya matcheada (14 B, Wave M) para usarla como
|  "molde de handler" pasado a Task_AllocFromFreeList -- mismo idioma
|  "lea tpl,a1; jsr Task_AllocFromFreeList" documentado en Wave QQ#1 y
|  Wave H/S, aqui con un handler de codigo real en vez de un blob de
|  datos puro.
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  EntityGroup_SpawnLinkedFromTemplateList_065C94
        .type   EntityGroup_SpawnLinkedFromTemplateList_065C94, @function
        .section .text.EntityGroup_SpawnLinkedFromTemplateList_065C94, "ax", @progbits

EntityGroup_SpawnLinkedFromTemplateList_065C94:
.Lloop:
        move.l  (a1), d0                        | +000  d0 = *(long*)a1
        cmpi.l  #0xffffffff, d0                 | +002  centinela de fin?
        beq.w   .Ldone                          | +008
        movem.l d0/a1, -(a7)                    | +00c  guarda node_id y
                                                 |       cursor de lista
        lea.l   Jsr5B6ThenJmpScheduler_064d8a(pc), a1  | +010  a1 = &molde-handler
        jsr     Task_AllocFromFreeList           | +016  a0 = nuevo task
        jsr     Entity_CopyTransform              | +01a  copia estado base
        movem.l (a7)+, d0/a1                     | +01e  restaura node_id/a1
        move.l  d0, (a0)                         | +022  new->+0 = node_id
        move.w  0x4(a1), d0                      | +024  d0 = node->delta_x
        jsr     Entity_MirrorDeltaByFacing_065D32(pc) | +028  espeja segun facing
        add.w   d0, 0x22(a0)                     | +02c  new->pos_x += d0
        move.w  0x6(a1), d0                      | +030  d0 = node->delta_y
        add.w   d0, 0x24(a0)                     | +034  new->pos_y += d0
        move.w  0x8(a1), d0                      | +038  d0 = node->delta_flags3a
        eor.b   d0, 0x3a(a0)                     | +03c  new->flags3a ^= d0
        move.w  0xa(a1), d0                      | +040  d0 = node->delta_flags38
        add.w   d0, 0x38(a0)                     | +044  new->flags38 += d0
        adda.l  #0xc, a1                         | +048  a1 += 12 (siguiente)
        bra.b   .Lloop                           | +04e
.Ldone:
        rts                                      | +050

        .size   EntityGroup_SpawnLinkedFromTemplateList_065C94, .-EntityGroup_SpawnLinkedFromTemplateList_065C94
