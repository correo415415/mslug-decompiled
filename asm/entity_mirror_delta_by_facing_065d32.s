| ============================================================================
|  Metal Slug 1 - asm/entity_mirror_delta_by_facing_065d32.s
|  ----------------------------------------------------------------------------
|  Wave RR#4 - helper de espejado de delta usado por
|  EntityGroup_SpawnLinkedFromTemplateList_065C94 (Wave RR#5, mismo archivo
|  de wave, ver ese .s para el caller).
|
|  Entity_MirrorDeltaByFacing_065D32  @ $065D32  (12 bytes, 1 caller)
|
|  Sin rts propio: termina por FALL-THROUGH en SetXN_065D3E
|  (`ori.b #0x11, ccr; rts`, ya matcheado) cuando a6 mira a la derecha,
|  o salta directo a ClearXN_065d44 (`andi.b #0xEE, ccr; rts`, ya
|  matcheado) cuando mira a la izquierda. Ambos son simbolos REGISTRY ya
|  existentes; no hace falta symbol nuevo.
|
|  ---------- Caller ----------------------------------------------------------
|
|      EntityGroup_SpawnLinkedFromTemplateList_065C94 (Wave RR#5)
|          -> jsr $65d32(pc)
|
|  Que hace:
|
|      if (a6->flags3a & 1) d0 = -d0;    espeja d0 (delta) si a6 mira a la
|                                        izquierda -- mismo flag +0x3a usado
|                                        por RR#1/RR#2 y por
|                                        Entity_CopyTransform
|      [fall-through / branch a epilogo CCR compartido -- el resultado en
|       d0 vuelve al caller via rts de SetXN/ClearXN; las flags X/N del
|       CCR tambien quedan visibles para el caller, aunque en este caso
|       concreto RR#5 no las usa]
|
|  Interpretacion: micro-helper generico de "espejar un delta segun la
|  orientacion de a6", extraido a funcion propia porque se llama desde
|  al menos un punto con jsr $(pc) explicito en vez de estar inline --
|  candidato a mas callers todavia no localizados (solo tenemos
|  constancia de 1 en el codigo ya matcheado).
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_MirrorDeltaByFacing_065D32
        .type   Entity_MirrorDeltaByFacing_065D32, @function
        .section .text.Entity_MirrorDeltaByFacing_065D32, "ax", @progbits

Entity_MirrorDeltaByFacing_065D32:
        btst.b  #0, 0x3a(a6)                    | +000  a6 mira a la izquierda?
        beq.w   ClearXN_065d44                  | +006  no: clear ccr + rts
        neg.w   d0                              | +00a  si: d0 = -d0
                                                 |       [fall-through -> SetXN_065D3E]

        .size   Entity_MirrorDeltaByFacing_065D32, .-Entity_MirrorDeltaByFacing_065D32
