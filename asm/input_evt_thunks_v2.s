| ============================================================================
|  Metal Slug 1 - asm/input_evt_thunks_v2.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - sub-cluster v2 ($05D00E..$05D031)
|
|  Segundo micro-cluster de 3 thunks de eventos de input en $05D00E..$05D031
|  (36 B). Estructura identica a los 28 thunks del cluster #1 pero con menos
|  entradas y solo el backend "CheckChannel" (sin variante con a2 precargada).
|
|  Firma comun (12 B por entrada):
|      move.b  #<mask>, d1      ; 12 3c 00 XX
|      move.w  #<layer>, d0     ; 30 3c 00 XX
|      bra.w   InputMask_CheckChannelAvail_05d032   ; 60 00 XX XX
|
|  Layer siempre = $3 (grupo 3, sin variante grupo 2 como el cluster #1).
|  Mascaras: $30 ($10|$20), $60 ($20|$40), $a0 ($20|$80) - todas OR de
|  potencias de 2, sin bit unico. Es el detector de COMBINACIONES.
|  ============================================================================

        .text

| #00  mask=$30  layer=3  backend=$5d032  size=12B
        .globl  InputEvtThunk_05d00e
        .type   InputEvtThunk_05d00e, @function
        .section .text.InputEvtThunk_05d00e, "ax", @progbits
InputEvtThunk_05d00e:
        move.b  #0x30, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05d032
        .size   InputEvtThunk_05d00e, .-InputEvtThunk_05d00e

| #01  mask=$60  layer=3  backend=$5d032  size=12B
        .globl  InputEvtThunk_05d01a
        .type   InputEvtThunk_05d01a, @function
        .section .text.InputEvtThunk_05d01a, "ax", @progbits
InputEvtThunk_05d01a:
        move.b  #0x60, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05d032
        .size   InputEvtThunk_05d01a, .-InputEvtThunk_05d01a

| #02  mask=$a0  layer=3  backend=$5d032  size=12B
        .globl  InputEvtThunk_05d026
        .type   InputEvtThunk_05d026, @function
        .section .text.InputEvtThunk_05d026, "ax", @progbits
InputEvtThunk_05d026:
        move.b  #0xa0, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05d032
        .size   InputEvtThunk_05d026, .-InputEvtThunk_05d026
