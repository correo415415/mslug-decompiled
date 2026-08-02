| ============================================================================
|  Metal Slug 1 - asm/entity_copy_anim_from_leader_06e2bc.s
|  ----------------------------------------------------------------------------
|  Wave SS#1 - helper de sincronizacion de sub-entity con su "leader".
|
|  Entity_CopyAnimFromLeader_06E2BC  @ $06E2BC  (66 bytes, 4 callers)
|
|  ---------- Callers --------------------------------------------------------
|
|      $06E244  jsr $6e2bc(pc)   (codigo aun no matcheado, cluster $06Exxx)
|      $06E26A  jsr $6e2bc(pc)   (idem)
|      JsrPcThunk_06e290 (matcheado, Wave J) -> jsr $6e2bc(pc); rts
|      JsrPcThunk_06e2b6 (matcheado, Wave J) -> jsr $6e2bc(pc); rts
|
|  El cluster $06E224.. (Entity_SpawnAndTag_06E224, ya matcheado) crea
|  sub-entities enlazadas; este helper es el que las mantiene en sincronia
|  visual con la entity "lider".
|
|  ---------- Firma C conceptual ---------------------------------------------
|
|    /* a6 = sub-entity (contexto activo), a0 = destino a actualizar
|       (en los callers observados a0 es la propia sub-entity o su
|       espejo de render).  Sin retorno de valor.  */
|    void Entity_CopyAnimFromLeader(Entity *self /*a6*/, Entity *dst /*a0*/)
|    {
|        Entity *leader = self->leader_50;        // +0x50 (link al lider)
|        dst->pos_x      = leader->pos_x;          // +0x22 (word)
|        dst->pos_y      = leader->pos_y;          // +0x24 (word)
|        dst->field_38   = leader->field_38;       // +0x38 (word)
|        dst->field_30   = leader->field_30;       // +0x30 (word)
|        dst->field_3b   = leader->field_3b;       // +0x3b (byte)
|        dst->field_46   = leader->field_46;       // +0x46 (byte)
|        dst->field_47   = leader->field_47;       // +0x47 (byte)
|        dst->anim_3c    = leader->anim_3c;        // +0x3c (long)
|        dst->anim_40    = leader->anim_40;        // +0x40 (long)
|        dst->field_30   = leader->field_30;       // +0x30 REPETIDO (!)
|    }
|
|  Nota forense: la copia de +0x30 aparece DOS veces (offsets +016 y +03a
|  del cuerpo).  Un compilador habria eliminado la primera como store
|  muerto; su presencia es evidencia adicional de asm escrito a mano (o de
|  una macro de copia editada sin limpiar).  El campo +0x50 ("leader") es
|  el mismo link que usa EntityGroup_SpawnLinkedFromTemplateList_065C94
|  (Wave RR#5) al construir cadenas de sub-entities.
|
|  Los pares +0x3c/+0x40 se copian como longs y +0x46/+0x47 como bytes
|  consecutivos: consistente con el bloque "animacion" del struct Entity
|  (frame ptr + timer) documentado parcialmente en include/mslug.h.
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_CopyAnimFromLeader_06E2BC
        .type   Entity_CopyAnimFromLeader_06E2BC, @function
        .section .text.Entity_CopyAnimFromLeader_06E2BC, "ax", @progbits

Entity_CopyAnimFromLeader_06E2BC:
        movea.l 0x50(a6), a1                    | +000  a1 = self->leader_50
        move.w  0x22(a1), 0x22(a0)              | +004  pos_x
        move.w  0x24(a1), 0x24(a0)              | +00a  pos_y
        move.w  0x38(a1), 0x38(a0)              | +010  field_38
        move.w  0x30(a1), 0x30(a0)              | +016  field_30 (1a vez)
        move.b  0x3b(a1), 0x3b(a0)              | +01c  field_3b
        move.b  0x46(a1), 0x46(a0)              | +022  field_46
        move.b  0x47(a1), 0x47(a0)              | +028  field_47
        move.l  0x3c(a1), 0x3c(a0)              | +02e  anim_3c (long)
        move.l  0x40(a1), 0x40(a0)              | +034  anim_40 (long)
        move.w  0x30(a1), 0x30(a0)              | +03a  field_30 (2a vez, store
        rts                                     | +040  muerto - ver cabecera)
