| ============================================================================
|  Metal Slug 1 - asm/input_evt_thunks_v3.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - sub-clusters #3 y #4 ($05D09A..$05D16F)
|
|  Sub-cluster #3 ($05D09A..$05D0E1, 72 B, 4 thunks):
|    mask=$F0 invariante, layer=2/3, a2=$10E200/$10E206, backend=$5D0E2.
|    Es el "cross-check de nibble alto" contra ambos jugadores.
|
|  Sub-cluster #4 ($05D0F8..$05D16F, 120 B, 8 thunks):
|    - Bloque #a (4 thunks con lea a2, 18 B c/u): mask=$01/$02/$04/$08,
|      layer=2, a2=$10E206, backend=$5D1C0 (TestChannelNibbleXor).
|    - Bloque #b (4 thunks sin lea a2, 12 B c/u): mismos mask, layer=2,
|      backend=$5D170 (CheckChannelAvail).
|
|  Firma comun (12 B sin lea, 18 B con lea):
|      move.b  #<mask>, d1      ; 12 3c 00 XX
|      move.w  #<layer>, d0     ; 30 3c 00 XX
|      [lea    <ctx>, a2]       ; 45 f9 XX XX XX XX  (bloque #a solamente)
|      bra.w   <backend>        ; 60 00 XX XX
|  ============================================================================

        .text

|--- Sub-cluster #3: 4 thunks (mask=$F0, layer 2/3, backend=$5d0e2) ---

| #00  mask=$f0  layer=3  a2=$10e200  backend=$5d0e2  size=18B
        .globl  InputEvtThunk_05d09a
        .type   InputEvtThunk_05d09a, @function
        .section .text.InputEvtThunk_05d09a, "ax", @progbits
InputEvtThunk_05d09a:
        move.b  #0xf0, d1
        move.w  #0x3, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05d0e2
        .size   InputEvtThunk_05d09a, .-InputEvtThunk_05d09a

| #01  mask=$f0  layer=3  a2=$10e206  backend=$5d0e2  size=18B
        .globl  InputEvtThunk_05d0ac
        .type   InputEvtThunk_05d0ac, @function
        .section .text.InputEvtThunk_05d0ac, "ax", @progbits
InputEvtThunk_05d0ac:
        move.b  #0xf0, d1
        move.w  #0x3, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelBit_05d0e2
        .size   InputEvtThunk_05d0ac, .-InputEvtThunk_05d0ac

| #02  mask=$f0  layer=2  a2=$10e200  backend=$5d0e2  size=18B
        .globl  InputEvtThunk_05d0be
        .type   InputEvtThunk_05d0be, @function
        .section .text.InputEvtThunk_05d0be, "ax", @progbits
InputEvtThunk_05d0be:
        move.b  #0xf0, d1
        move.w  #0x2, d0
        lea     0x10e200.l, a2
        bra.w   InputMask_TestChannelBit_05d0e2
        .size   InputEvtThunk_05d0be, .-InputEvtThunk_05d0be

| #03  mask=$f0  layer=2  a2=$10e206  backend=$5d0e2  size=18B
        .globl  InputEvtThunk_05d0d0
        .type   InputEvtThunk_05d0d0, @function
        .section .text.InputEvtThunk_05d0d0, "ax", @progbits
InputEvtThunk_05d0d0:
        move.b  #0xf0, d1
        move.w  #0x2, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelBit_05d0e2
        .size   InputEvtThunk_05d0d0, .-InputEvtThunk_05d0d0

|--- Sub-cluster #4a: 4 thunks con lea a2=$10e206 (backend=$5d1c0) ---

| #04  mask=$01  layer=2  a2=$10e206  backend=$5d1c0  size=18B
        .globl  InputEvtThunk_05d0f8
        .type   InputEvtThunk_05d0f8, @function
        .section .text.InputEvtThunk_05d0f8, "ax", @progbits
InputEvtThunk_05d0f8:
        move.b  #0x1, d1
        move.w  #0x2, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelNibbleXor_05d1c0
        .size   InputEvtThunk_05d0f8, .-InputEvtThunk_05d0f8

| #05  mask=$02  layer=2  a2=$10e206  backend=$5d1c0  size=18B
        .globl  InputEvtThunk_05d10a
        .type   InputEvtThunk_05d10a, @function
        .section .text.InputEvtThunk_05d10a, "ax", @progbits
InputEvtThunk_05d10a:
        move.b  #0x2, d1
        move.w  #0x2, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelNibbleXor_05d1c0
        .size   InputEvtThunk_05d10a, .-InputEvtThunk_05d10a

| #06  mask=$08  layer=2  a2=$10e206  backend=$5d1c0  size=18B
        .globl  InputEvtThunk_05d11c
        .type   InputEvtThunk_05d11c, @function
        .section .text.InputEvtThunk_05d11c, "ax", @progbits
InputEvtThunk_05d11c:
        move.b  #0x8, d1
        move.w  #0x2, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelNibbleXor_05d1c0
        .size   InputEvtThunk_05d11c, .-InputEvtThunk_05d11c

| #07  mask=$04  layer=2  a2=$10e206  backend=$5d1c0  size=18B
        .globl  InputEvtThunk_05d12e
        .type   InputEvtThunk_05d12e, @function
        .section .text.InputEvtThunk_05d12e, "ax", @progbits
InputEvtThunk_05d12e:
        move.b  #0x4, d1
        move.w  #0x2, d0
        lea     0x10e206.l, a2
        bra.w   InputMask_TestChannelNibbleXor_05d1c0
        .size   InputEvtThunk_05d12e, .-InputEvtThunk_05d12e

|--- Sub-cluster #4b: 4 thunks sin lea a2 (backend=$5d170 CheckChannelAvail) ---

| #08  mask=$01  layer=2  backend=$5d170  size=12B
        .globl  InputEvtThunk_05d140
        .type   InputEvtThunk_05d140, @function
        .section .text.InputEvtThunk_05d140, "ax", @progbits
InputEvtThunk_05d140:
        move.b  #0x1, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05d170
        .size   InputEvtThunk_05d140, .-InputEvtThunk_05d140

| #09  mask=$02  layer=2  backend=$5d170  size=12B
        .globl  InputEvtThunk_05d14c
        .type   InputEvtThunk_05d14c, @function
        .section .text.InputEvtThunk_05d14c, "ax", @progbits
InputEvtThunk_05d14c:
        move.b  #0x2, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05d170
        .size   InputEvtThunk_05d14c, .-InputEvtThunk_05d14c

| #10  mask=$08  layer=2  backend=$5d170  size=12B
        .globl  InputEvtThunk_05d158
        .type   InputEvtThunk_05d158, @function
        .section .text.InputEvtThunk_05d158, "ax", @progbits
InputEvtThunk_05d158:
        move.b  #0x8, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05d170
        .size   InputEvtThunk_05d158, .-InputEvtThunk_05d158

| #11  mask=$04  layer=2  backend=$5d170  size=12B
        .globl  InputEvtThunk_05d164
        .type   InputEvtThunk_05d164, @function
        .section .text.InputEvtThunk_05d164, "ax", @progbits
InputEvtThunk_05d164:
        move.b  #0x4, d1
        move.w  #0x2, d0
        bra.w   InputMask_CheckChannelAvail_05d170
        .size   InputEvtThunk_05d164, .-InputEvtThunk_05d164
