| ============================================================================
|  Metal Slug 1 - asm/input_evt_thunks.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - cluster completo
|
|  Cluster de 28 thunks de eventos de input en $05cdfc..$05cfa7 (428 B).
|  Cada entrada codifica (mask_bit, layer, opt_ctx_ptr) y salta al backend
|  comun ($05cfa8 sin a2 o $05cff8 con a2 precargado en $10E200/$10E206).
|
|  Firma comun de una entrada:
|      move.b  #<mask>, d1        ; 12 3c 00 XX  (bit de evento en d1)
|      [ ori.b #<ori>,  d1  ]     ; 00 01 00 XX  (opcional: combina bits)
|      move.w  #<layer>, d0       ; 30 3c 00 XX  (grupo 2 o 3)
|      [ lea   <ctx>, a2   ]      ; 45 f9 XX XX XX XX  (P1=$10E200 / P2=$10E206)
|      bra.w   <backend>          ; 60 00 XX XX  ($05cfa8 sin a2 o $05cff8 con a2)
|
|  Tamano por entrada: 12 B (sin ori ni lea), 16 B (con ori), 18 B (con lea a2).
|  Los tres tamanos coexisten dentro del mismo cluster --> firma inequivoca de
|  asm a mano. GCC habria emitido 28 funciones iguales via switch/tabla, o una
|  unica funcion con parametros. La eleccion de 28 thunks distintos con longitud
|  variable es del programador humano.
|
|  Hallazgo forense: el bit d1 va recorriendo potencias de 2 ($01..$80) y
|  combinaciones OR ($05, $06, $09, $0A, $0C), es decir un ENUM de eventos
|  de input. layer d0 = 2 o 3 indica "grupo" (probable dispositivo o menu
|  activo). ctx a2 = $10E200 vs $10E206 = buffers P1 y P2 (offset 6 = 6 bytes
|  por jugador --> encaja con "5-6 slots de input por jugador").
|
|  Callers directos matcheados (desde codigo ya registrado en waves anteriores):
|      $05cdfc -> 1 (JsrAbsThunk_02abb2)
|      $05cef8 -> 3 (JmpAbsThunk_02abc6, JmpAbsThunk_0330ca, JmpAbsThunk_039410)
|      $05cf04 -> 2 (JmpAbsThunk_032e3c, JmpAbsThunk_0393ce)
|      $05cf6c -> 1 (JsrPcThunk_05d30a)
|  El resto de entradas tienen callers en zonas aun no matcheadas (los descubrira
|  el scanner cuando decompilemos su callgraph).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 bare-metal + GAS m68k con
|              --register-prefix-optional.
|  ============================================================================

        .text

| #00  mask=$f0  layer=3  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cdfc
        .type   InputEvtThunk_05cdfc, @function
        .section .text.InputEvtThunk_05cdfc, "ax", @progbits
InputEvtThunk_05cdfc:
        move.b  #0xf0, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cdfc, .-InputEvtThunk_05cdfc

| #01  mask=$e0  layer=3  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05ce08
        .type   InputEvtThunk_05ce08, @function
        .section .text.InputEvtThunk_05ce08, "ax", @progbits
InputEvtThunk_05ce08:
        move.b  #0xe0, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05ce08, .-InputEvtThunk_05ce08

| #02  mask=$10  layer=3  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce14
        .type   InputEvtThunk_05ce14, @function
        .section .text.InputEvtThunk_05ce14, "ax", @progbits
InputEvtThunk_05ce14:
        move.b  #0x10, d1
        move.w  #0x3, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce14, .-InputEvtThunk_05ce14

| #03  mask=$20  layer=3  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce26
        .type   InputEvtThunk_05ce26, @function
        .section .text.InputEvtThunk_05ce26, "ax", @progbits
InputEvtThunk_05ce26:
        move.b  #0x20, d1
        move.w  #0x3, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce26, .-InputEvtThunk_05ce26

| #04  mask=$40  layer=3  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce38
        .type   InputEvtThunk_05ce38, @function
        .section .text.InputEvtThunk_05ce38, "ax", @progbits
InputEvtThunk_05ce38:
        move.b  #0x40, d1
        move.w  #0x3, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce38, .-InputEvtThunk_05ce38

| #05  mask=$80  layer=3  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce4a
        .type   InputEvtThunk_05ce4a, @function
        .section .text.InputEvtThunk_05ce4a, "ax", @progbits
InputEvtThunk_05ce4a:
        move.b  #0x80, d1
        move.w  #0x3, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce4a, .-InputEvtThunk_05ce4a

| #06  mask=$10  layer=3  a2=$10e206  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce5c
        .type   InputEvtThunk_05ce5c, @function
        .section .text.InputEvtThunk_05ce5c, "ax", @progbits
InputEvtThunk_05ce5c:
        move.b  #0x10, d1
        move.w  #0x3, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce5c, .-InputEvtThunk_05ce5c

| #07  mask=$20  layer=3  a2=$10e206  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce6e
        .type   InputEvtThunk_05ce6e, @function
        .section .text.InputEvtThunk_05ce6e, "ax", @progbits
InputEvtThunk_05ce6e:
        move.b  #0x20, d1
        move.w  #0x3, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce6e, .-InputEvtThunk_05ce6e

| #08  mask=$40  layer=3  a2=$10e206  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce80
        .type   InputEvtThunk_05ce80, @function
        .section .text.InputEvtThunk_05ce80, "ax", @progbits
InputEvtThunk_05ce80:
        move.b  #0x40, d1
        move.w  #0x3, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce80, .-InputEvtThunk_05ce80

| #09  mask=$80  layer=3  a2=$10e206  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ce92
        .type   InputEvtThunk_05ce92, @function
        .section .text.InputEvtThunk_05ce92, "ax", @progbits
InputEvtThunk_05ce92:
        move.b  #0x80, d1
        move.w  #0x3, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ce92, .-InputEvtThunk_05ce92

| #10  mask=$01  layer=2  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05cea4
        .type   InputEvtThunk_05cea4, @function
        .section .text.InputEvtThunk_05cea4, "ax", @progbits
InputEvtThunk_05cea4:
        move.b  #0x01, d1
        move.w  #0x2, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05cea4, .-InputEvtThunk_05cea4

| #11  mask=$02  layer=2  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ceb6
        .type   InputEvtThunk_05ceb6, @function
        .section .text.InputEvtThunk_05ceb6, "ax", @progbits
InputEvtThunk_05ceb6:
        move.b  #0x02, d1
        move.w  #0x2, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ceb6, .-InputEvtThunk_05ceb6

| #12  mask=$08  layer=2  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05cec8
        .type   InputEvtThunk_05cec8, @function
        .section .text.InputEvtThunk_05cec8, "ax", @progbits
InputEvtThunk_05cec8:
        move.b  #0x08, d1
        move.w  #0x2, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05cec8, .-InputEvtThunk_05cec8

| #13  mask=$04  layer=2  a2=$10e200  backend=$05cff8  size=18B
        .globl  InputEvtThunk_05ceda
        .type   InputEvtThunk_05ceda, @function
        .section .text.InputEvtThunk_05ceda, "ax", @progbits
InputEvtThunk_05ceda:
        move.b  #0x04, d1
        move.w  #0x2, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05cff8
        .size   InputEvtThunk_05ceda, .-InputEvtThunk_05ceda

| #14  mask=$01  layer=2  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05ceec
        .type   InputEvtThunk_05ceec, @function
        .section .text.InputEvtThunk_05ceec, "ax", @progbits
InputEvtThunk_05ceec:
        move.b  #0x01, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05ceec, .-InputEvtThunk_05ceec

| #15  mask=$02  layer=2  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cef8
        .type   InputEvtThunk_05cef8, @function
        .section .text.InputEvtThunk_05cef8, "ax", @progbits
InputEvtThunk_05cef8:
        move.b  #0x02, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cef8, .-InputEvtThunk_05cef8

| #16  mask=$08  layer=2  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cf04
        .type   InputEvtThunk_05cf04, @function
        .section .text.InputEvtThunk_05cf04, "ax", @progbits
InputEvtThunk_05cf04:
        move.b  #0x08, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf04, .-InputEvtThunk_05cf04

| #17  mask=$04  layer=2  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cf10
        .type   InputEvtThunk_05cf10, @function
        .section .text.InputEvtThunk_05cf10, "ax", @progbits
InputEvtThunk_05cf10:
        move.b  #0x04, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf10, .-InputEvtThunk_05cf10

| #18  mask=$04 | ori=$08 (eff=$0c)  layer=2  a2=-  backend=$05cfa8  size=16B
        .globl  InputEvtThunk_05cf1c
        .type   InputEvtThunk_05cf1c, @function
        .section .text.InputEvtThunk_05cf1c, "ax", @progbits
InputEvtThunk_05cf1c:
        move.b  #0x04, d1
        ori.b   #0x08, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf1c, .-InputEvtThunk_05cf1c

| #19  mask=$01 | ori=$08 (eff=$09)  layer=2  a2=-  backend=$05cfa8  size=16B
        .globl  InputEvtThunk_05cf2c
        .type   InputEvtThunk_05cf2c, @function
        .section .text.InputEvtThunk_05cf2c, "ax", @progbits
InputEvtThunk_05cf2c:
        move.b  #0x01, d1
        ori.b   #0x08, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf2c, .-InputEvtThunk_05cf2c

| #20  mask=$02 | ori=$08 (eff=$0a)  layer=2  a2=-  backend=$05cfa8  size=16B
        .globl  InputEvtThunk_05cf3c
        .type   InputEvtThunk_05cf3c, @function
        .section .text.InputEvtThunk_05cf3c, "ax", @progbits
InputEvtThunk_05cf3c:
        move.b  #0x02, d1
        ori.b   #0x08, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf3c, .-InputEvtThunk_05cf3c

| #21  mask=$01 | ori=$04 (eff=$05)  layer=2  a2=-  backend=$05cfa8  size=16B
        .globl  InputEvtThunk_05cf4c
        .type   InputEvtThunk_05cf4c, @function
        .section .text.InputEvtThunk_05cf4c, "ax", @progbits
InputEvtThunk_05cf4c:
        move.b  #0x01, d1
        ori.b   #0x04, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf4c, .-InputEvtThunk_05cf4c

| #22  mask=$02 | ori=$04 (eff=$06)  layer=2  a2=-  backend=$05cfa8  size=16B
        .globl  InputEvtThunk_05cf5c
        .type   InputEvtThunk_05cf5c, @function
        .section .text.InputEvtThunk_05cf5c, "ax", @progbits
InputEvtThunk_05cf5c:
        move.b  #0x02, d1
        ori.b   #0x04, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf5c, .-InputEvtThunk_05cf5c

| #23  mask=$08  layer=3  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cf6c
        .type   InputEvtThunk_05cf6c, @function
        .section .text.InputEvtThunk_05cf6c, "ax", @progbits
InputEvtThunk_05cf6c:
        move.b  #0x08, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf6c, .-InputEvtThunk_05cf6c

| #24  mask=$04  layer=3  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cf78
        .type   InputEvtThunk_05cf78, @function
        .section .text.InputEvtThunk_05cf78, "ax", @progbits
InputEvtThunk_05cf78:
        move.b  #0x04, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf78, .-InputEvtThunk_05cf78

| #25  mask=$01  layer=3  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cf84
        .type   InputEvtThunk_05cf84, @function
        .section .text.InputEvtThunk_05cf84, "ax", @progbits
InputEvtThunk_05cf84:
        move.b  #0x01, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf84, .-InputEvtThunk_05cf84

| #26  mask=$02  layer=3  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cf90
        .type   InputEvtThunk_05cf90, @function
        .section .text.InputEvtThunk_05cf90, "ax", @progbits
InputEvtThunk_05cf90:
        move.b  #0x02, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf90, .-InputEvtThunk_05cf90

| #27  mask=$0c  layer=3  a2=-  backend=$05cfa8  size=12B
        .globl  InputEvtThunk_05cf9c
        .type   InputEvtThunk_05cf9c, @function
        .section .text.InputEvtThunk_05cf9c, "ax", @progbits
InputEvtThunk_05cf9c:
        move.b  #0x0c, d1
        move.w  #0x3, d0
        bra.w   InputMask_CheckChannelAvail_05cfa8
        .size   InputEvtThunk_05cf9c, .-InputEvtThunk_05cf9c
