| ============================================================================
|  Metal Slug 1 - asm/entity_apply_fade_shade_028108.s
|  ----------------------------------------------------------------------------
|  Wave QQ#2 - helper compartido de "shade"/fade sobre flags +38.
|
|  Entity_ApplyFadeShade_028108  @ $028108  (44 bytes, 9 callers)
|
|  ---------- Mapa de callers -----------------------------------------------
|
|      EntitySetField38AndUpdate         -> jmp  $28108(pc)  (tail-call)
|      ActorCtxWrapper_02783a            -> jsr  $28108(pc)
|      Entity_Probe_Scratch_02785C       -> jsr  $28108(pc)
|      Entity_ProbeRevertCcr_02788C (x2) -> jsr  $28108(pc)
|      Entity_ProbeRevertCcr_027A92 (x2) -> jsr  $28108(pc)
|      Entity_ProbeRevertCcr_027AFC (x2) -> jsr  $28108(pc)
|
|  9 callers, todos dentro del cluster de probes/CCR de entidades ($0277xx-
|  $027Bxx) mas un tail-call desde EntitySetField38AndUpdate ($028134,
|  ya matcheado). Es un helper compartido de "aplicar sombreado/fade" al
|  word de flags +38 de la entidad en a6.
|
|  Que hace (campos de struct Entity en a6):
|
|      d6 = entity->+24                      (word, "contador de vida del
|                                             efecto" o similar timer)
|      d5 = entity->+38 - 0x4000
|      if ((uint16_t)d5 >= 0x7FFF) return;    guard de rango: solo actua
|                                             si entity->+38 esta en
|                                             [0x4000, 0xBFFE] (aprox);
|                                             fuera de rango, no-op.
|      d6 &= 0xFF                             d6 = contador de fade (0-255)
|      d5 = 0xFF - d6                         d5 = "fade inverso" (255 al
|                                             empezar, 0 al terminar)
|      d5 <<= 5                               d5 = d5 * 32  (desplaza el
|                                             valor a los bits 5-12)
|      entity->+38 &= 0xE01F                  limpia bits 5-12 de +38
|                                             (preserva bits 0-4 y 13-15)
|      entity->+38 |= d5                      escribe el nuevo valor de
|                                             fade en esos bits
|
|  Interpretacion: +38 es un word de flags de hardware/sprite (ya visto
|  en Wave QQ#1 con valores como 0x8010); los bits 5-12 (mascara 0x1FE0)
|  parecen codificar un nivel de "shade"/brillo de 8 bits que decae con
|  el contador +24 -- tipico de un efecto de flash/fade al morir o al
|  recibir dano. El guard de rango en +38 evita aplicar el fade si el
|  word de flags esta en un estado que no corresponde a "modo fade
|  activo" (los bits altos de +38 fuera de [0x40,0xBF] act. como region
|  de guarda).
|
|  Sin rts explicito en el tail-call (EntitySetField38AndUpdate hace
|  jmp, no jsr): el rts de esta funcion es el que retorna al caller
|  original de EntitySetField38AndUpdate tambien.
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ApplyFadeShade_028108
        .type   Entity_ApplyFadeShade_028108, @function
        .section .text.Entity_ApplyFadeShade_028108, "ax", @progbits

Entity_ApplyFadeShade_028108:
        move.w  0x24(a6), d6                   | +000  d6 = entity->+24
        move.w  0x38(a6), d5                   | +004  d5 = entity->+38
        subi.w  #0x4000, d5                    | +008  d5 -= 0x4000
        cmpi.w  #0x7fff, d5                     | +00c  d5 (uns) vs 0x7FFF
        bcc.w   .Lout_of_range                  | +010  fuera de rango: salir
        andi.w  #0xff, d6                       | +014  d6 = contador (0-255)
        move.w  #0xff, d5                       | +018  d5 = 0xFF
        sub.w   d6, d5                          | +01c  d5 = 0xFF - contador
        asl.w   #5, d5                          | +01e  d5 <<= 5
        andi.w  #0xe01f, 0x38(a6)               | +020  limpia bits 5-12 de +38
        or.w    d5, 0x38(a6)                    | +026  funde el nuevo shade
.Lout_of_range:
        rts                                     | +02a

        .size   Entity_ApplyFadeShade_028108, .-Entity_ApplyFadeShade_028108
