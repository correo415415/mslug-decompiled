| ============================================================================
|  Metal Slug 1 - asm/sprite_setup_dispatch_05ca2a.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #13 (cluster 4-entry)
|
|  Sprite_Dispatch_05CA2A / _05CA4E / _05CA60 / _05CAC0
|      @ $05CA2A / $05CA4E / $05CA60 / $05CAC0   (164 bytes totales, 5 callers)
|
|  Cluster de 4 trampolines contiguos que preparan argumentos para el
|  backend en $5A9E6 (variante _sinperspectiva) o $5A9D6 (variante
|  _crossbank). Todos los entry points comparten el mismo prologo
|  parcial y convergen por bra/fall-through interno. Ninguno tiene rts
|  propio: son 4 configuraciones de "shape" del argumento hacia el
|  backend de renderizado.
|
|  Entry A ($05CA2A, 36 B, salida bra.w $5A9E6):
|      d5 = flags3a & 3               -- 2 bits de "shape/flip"
|      d0 = pos_x_long                 -- entity->pos_x_long (32 bit)
|      d1 = flags38                    -- animation frame flags
|      d2 = last_slot                  -- $14(a6)
|      d3 = anchor_y                   -- $33(a6)
|      d4 = anchor_x                   -- $32(a6)
|      a0 = slot_parent                -- $3c(a6)
|      bra.w Sprite_Blit_5A9E6          -- backend estandar
|
|  Entry B ($05CA4E, 18 B, salida bra.b $5CA32 al cuerpo de A):
|      d5 = (flags3a & 3) | 4          -- activa bit 2 = "cross-bank flag"
|      a1 = &Sprite_Blit_5A9D6         -- backend alternativo por a1
|      bra.b .A_step2                   -- cae en el "move.l pos_x_l,d0" de A
|
|  Entry C ($05CA60, 96 B, salida bra.w $5A9E6 con perspective):
|      d5 = flags3a & 3
|      d0 = pos_x_long
|      d3 = anchor_y
|      d4 = anchor_x
|      if (state12.bit2)               -- ¿ entity con perspective activo ?
|          d1 = d0 low
|          d0 = pos_x_high             -- swap.l d0
|          d0 -= camera_x ($10E1E4)
|          d6 = d4 + 1
|          d0 = (d0 * d6) >> 8          -- multiplicacion Q8.8
|          d0 += camera_x
|          d1 -= camera_y ($10E1E6)
|          d6 = d3 + 1
|          d1 = (d1 * d6) >> 8
|          d1 += camera_y
|          d0 high = d0, d0 low = d1    -- recombine long
|      d1 = flags38
|      d2 = last_slot
|      a0 = slot_parent
|      bra.w Sprite_Blit_5A9E6
|
|  Entry D ($05CAC0, 14 B, salida bra.b $5CA68 al cuerpo de C):
|      d5 = (flags3a & 3) | 4          -- cross-bank flag
|      bra.b .C_step2                   -- cae en el "move.l pos_x_l,d0" de C
|
|  Firma C conceptual (4 wrappers cerca del mismo backend):
|
|      /* A: dispatch normal, sin perspective. */
|      void Sprite_Dispatch_Normal      (struct Entity *a6);
|      /* B: cross-bank flag + backend alternativo por a1. */
|      void Sprite_Dispatch_CrossBank   (struct Entity *a6);
|      /* C: dispatch con perspective (parallax por profundidad). */
|      void Sprite_Dispatch_Perspective (struct Entity *a6);
|      /* D: cross-bank + perspective. */
|      void Sprite_Dispatch_PerspXBank  (struct Entity *a6);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Cuatro entry points en un solo bloque de 164 B, todos globales,
|       ninguno con rts. GCC no emite funciones-sin-rts salvo en muy
|       especificas transformaciones tail-call, y no permite dos entry
|       points en la misma funcion. Idioma tipico de asm hand-coded que
|       reutiliza codigo entre variantes de un mismo backend.
|    2. muls.w + asr.l #8 = multiplicacion Q8.8 fixed-point sin overflow
|       check. GCC habria usado __mulsi3 o __divsi3 con signed division;
|       aqui el codigo confia en que d6=d4+1 no desborda (d4 = anchor_x
|       byte, max = 256).
|    3. Recombine long con `swap d0; clr.w d0; move.w d1,d0`: idioma de
|       reconstruccion de long word en 68000 tras trabajar sobre las dos
|       halves independientemente. GCC habria usado two 16-bit registers
|       o packing via bitwise.
|    4. Fall-through de B a A y de D a C con bra.b (salto corto <128 B).
|       El disp negativo `60d2` y `609a` codifican -46 y -102 exactos, con
|       ninguna alineacion posible bajo GCC.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Sprite_Dispatch_05CA2A
        .globl  Sprite_Dispatch_05CA4E
        .globl  Sprite_Dispatch_05CA60
        .globl  Sprite_Dispatch_05CAC0
        .type   Sprite_Dispatch_05CA2A, @function
        .type   Sprite_Dispatch_05CA4E, @function
        .type   Sprite_Dispatch_05CA60, @function
        .type   Sprite_Dispatch_05CAC0, @function
        .section .text.Sprite_Dispatch_05CA2A, "ax", @progbits

Sprite_Dispatch_05CA2A:
        move.b  0x3a(a6), d5                | +000  d5 = flags3a
        andi.b  #0x3, d5                    | +004  d5 &= 3
.A_step2:
        move.l  0x22(a6), d0                | +008  d0 = pos_x_long
        move.w  0x38(a6), d1                | +00c  d1 = flags38
        move.w  0x14(a6), d2                | +010  d2 = last_slot
        move.b  0x33(a6), d3                | +014  d3 = anchor_y
        move.b  0x32(a6), d4                | +018  d4 = anchor_x
        movea.l 0x3c(a6), a0                | +01c  a0 = slot_parent
        bra.w   Sprite_Blit_5A9E6           | +020  -> backend estandar

Sprite_Dispatch_05CA4E:
        move.b  0x3a(a6), d5                | +024  d5 = flags3a
        andi.b  #0x3, d5                    | +028  d5 &= 3
        ori.b   #0x4, d5                    | +02c  d5 |= 4 (cross-bank flag)
        lea     ThunkTarget_05a9d6(pc), a1  | +030  a1 = backend alternativo (nombre historico Wave I)
        bra.b   .A_step2                    | +034  fall-through al cuerpo de A

Sprite_Dispatch_05CA60:
        move.b  0x3a(a6), d5                | +036  d5 = flags3a
        andi.b  #0x3, d5                    | +03a  d5 &= 3
.C_step2:
        move.l  0x22(a6), d0                | +03e  d0 = pos_x_long
        move.b  0x33(a6), d3                | +042  d3 = anchor_y
        move.b  0x32(a6), d4                | +046  d4 = anchor_x
        btst.b  #0x2, 0x12(a6)              | +04a  ¿ state12 & 4 (perspective on) ?
        beq.w   .C_no_perspective           | +050  no: saltar el calc de proyeccion
        move.l  d0, d1                      | +054  d1 = pos_x_long (backup)
        swap    d0                          | +056  d0 = pos_x_hi (posicion X)
        sub.w   0x10e1e4.l, d0              | +058  d0 -= camera_x
        move.w  d4, d6                      | +05e  d6 = anchor_x
        addq.w  #0x1, d6                    | +060  d6++
        muls.w  d6, d0                      | +062  d0 = (X-cam_x) * (anchor+1)
        asr.l   #0x8, d0                    | +064  d0 >>= 8   (Q8.8)
        add.w   0x10e1e4.l, d0              | +066  d0 += camera_x
        sub.w   0x10e1e6.l, d1              | +06c  d1 -= camera_y
        move.w  d3, d6                      | +072  d6 = anchor_y
        addq.w  #0x1, d6                    | +074  d6++
        muls.w  d6, d1                      | +076  d1 = (Y-cam_y) * (anchor+1)
        asr.l   #0x8, d1                    | +078  d1 >>= 8   (Q8.8)
        add.w   0x10e1e6.l, d1              | +07a  d1 += camera_y
        swap    d0                          | +080  d0 hi = X projectada
        clr.w   d0                          | +082  d0 lo = 0
        move.w  d1, d0                      | +084  d0 lo = Y projectada
.C_no_perspective:
        move.w  0x38(a6), d1                | +086  d1 = flags38
        move.w  0x14(a6), d2                | +08a  d2 = last_slot
        movea.l 0x3c(a6), a0                | +08e  a0 = slot_parent
        bra.w   Sprite_Blit_5A9E6           | +092  -> backend estandar

Sprite_Dispatch_05CAC0:
        move.b  0x3a(a6), d5                | +096  d5 = flags3a
        andi.b  #0x3, d5                    | +09a  d5 &= 3
        ori.b   #0x4, d5                    | +09e  d5 |= 4 (cross-bank flag)
        bra.b   .C_step2                    | +0a2  fall-through al cuerpo de C
        .size   Sprite_Dispatch_05CA2A, .-Sprite_Dispatch_05CA2A
        .size   Sprite_Dispatch_05CA4E, .-Sprite_Dispatch_05CA4E
        .size   Sprite_Dispatch_05CA60, .-Sprite_Dispatch_05CA60
        .size   Sprite_Dispatch_05CAC0, .-Sprite_Dispatch_05CAC0
