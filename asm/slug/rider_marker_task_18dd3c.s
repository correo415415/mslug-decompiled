| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Tarea del marcador de jinete del Slug (isla de código en el banco alto)
|  Región: $18DD3C..$18DD96  (90 B, 1 entrada)
| ============================================================================
|
|  A) RESUMEN
|  ----------
|  Rutina de 90 B incrustada en medio del banco de datos gráficos
|  (Data_18db78 / Data_18dd96, CPU $28DD3C). Slug_SpawnRiderMarker_02ad70 la
|  instala como tarea (`lea $28DD3C,a1; jsr $4AE; jsr $5DD02`) en X=$30 o
|  X=$B8 (jugador 1/2), Y=$174. Reserva una paleta dinámica (d1=3), fija
|  +$32/+$33 = $7F, prioridad $E000, el bit 4 de +$38, instala el script de
|  sprite $28DC6E (las 14 cifras/banderas "0001 0208 0026 xxxx FFFF") y se
|  queda esperando a Entity_HasLinkedSlots; termina (Scheduler) cuando el
|  byte +$3B del padre supera $3C.
|
|  B) EVIDENCIA
|  -----------
|  Desensamblado limpio de $28DD3C a $28DD94 (rts), dos referencias desde
|  helpers_0295a6.s y el censo, que lo marcaba como "otro".
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  SlugRiderMarker_Task_18dd3c  @ $18DD3C  (90 B)
| ----------------------------------------------------------------------------
        .section .text.SlugRiderMarker_Task_18dd3c, "ax", @progbits
        .global SlugRiderMarker_Task_18dd3c
SlugRiderMarker_Task_18dd3c:
        move.w  #0x3,d1                         | +000
        jsr     PalSlot_AllocDynamic_0029f2     | +004
        move.b  #0x7f,d0                        | +00a
        move.b  d0,0x32(a6)                     | +00e
        move.b  d0,0x33(a6)                     | +012
        move.w  #0xe000,d0                      | +016
        jsr     EntitySetField38AndUpdate       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x10,0x38(a6)                  | +026
        lea     Data_18db78+0xf6(pc),a0         | +02c  -> $28DC6E (script)
        jsr     EntitySetSpriteMap              | +030
        jsr     EntityResetState                | +036
        lea     .L18dd7e(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L18dd7e:
        jsr     Entity_HasLinkedSlots           | +042
        cmpi.b  #0x3c,0x3b(a6)                  | +048
        bls.w   .L18dd94                        | +04e
        jmp     Scheduler_Main_0518             | +052
.L18dd94:
        rts                                     | +058
