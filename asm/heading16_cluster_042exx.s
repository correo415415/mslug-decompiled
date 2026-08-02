| ============================================================================
|  Metal Slug 1 - asm/heading16_cluster_042exx.s
|  ----------------------------------------------------------------------------
|  Wave VV (parte 3/3) - set de animaciones de 16 rumbos + handler + clones
|  Cluster $042E8A..$0434C2, 5 entradas, 1 592 B.
|
|  Heading16AnimSet_042E8A (1 332 B de DATOS): 16 tablas de animacion
|  (una por rumbo/heading de 22.5 grados) para el interprete
|  $28CD4/$28D70, SIN header $0900 (secuencias de frames puras: el
|  handler ya tiene el sprite montado). Cada tabla termina en $0100 +
|  puntero ABSOLUTO a la tabla IDLE comun $43354, que a su vez termina
|  en el opcode $1600 (HOLD: congela el ultimo frame, sin loop).
|  Al final, tabla de 16 punteros ($4337E) indexada por el handler.
|
|  Heading16Sprite_Handler_0433BE: sprite decorativo dirigido por RAM
|  global: $106F29 (byte de control: <0 = quieto, 0 = girar en Y +
|  reaplicar idle, >0 = tomar rumbo) y $106F28 (rumbo 0..15). Se
|  auto-despawnea al salir de pantalla por la derecha (x+$30 >= $1A0).
|
|  TrackTargetLatch75/78: clones del tracker tri-estado con la PRIMERA
|  rama INVERTIDA (`bcs.w` donde todos los demas usan `bcc.w`) - la
|  rama "hay atacante" queda en el else. Retornan con rts propio.
|
|  Entity_CmpDepthToParent_0434B2: clon byte-exacto del comparador de
|  profundidad de $042390, cayendo en su pareja de islas CCR
|  ($0434C2/$0434C8).
| ============================================================================

| ----------------------------------------------------------------------------
|  Heading16AnimSet_042E8A  @ $042E8A  (1 332 B, DATOS)
|
|  16 tablas (7 frames palindromos dur=2 la mayoria; las diagonales
|  usan attr $0209 = flip) + tabla IDLE de 4 frames (3 de dur=4 y el
|  ultimo dur=1, tiles de otra pagina $25xxxx) + tabla de punteros.
|  Transcripcion VERIFICADA contra la ROM por script (asserts por
|  frame y por puntero).
| ----------------------------------------------------------------------------
        .section .text.Heading16AnimSet_042E8A, "ax", @progbits
        .global Heading16AnimSet_042E8A
        .global Heading16IdleAnim_043354
        .global Heading16PtrTable_04337E
Heading16AnimSet_042E8A:
.Lh00:                                          | +000  heading  0
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00230e6c
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00230e80
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00230e94
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00230ea8
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00230e94
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00230e80
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00230e6c
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh01:                                          | +04c  heading  1
        .short  0x0002, 0x0208                  | frame 0
        .long   0x002311c6
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x002311da
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x002311ee
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00231202
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x002311ee
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x002311da
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x002311c6
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh02:                                          | +098  heading  2
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00230ad8
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00230aec
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00230b00
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00230b14
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00230b00
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00230aec
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00230ad8
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh03:                                          | +0e4  heading  3
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00233cae
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00233cc2
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00233cd6
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00233cee
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00233cd6
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00233cc2
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00233cae
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh04:                                          | +130  heading  4
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00232556
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00232570
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x0023258a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x0023259e
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x0023258a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00232570
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00232556
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh05:                                          | +17c  heading  5
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00230fd6
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00230fea
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00230ffe
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00231012
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00231026
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x0023103a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x0023104e
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh06:                                          | +1c8  heading  6
        .short  0x0002, 0x0209                  | frame 0
        .long   0x00231266
        .short  0xffff
        .short  0x0002, 0x0209                  | frame 1
        .long   0x00231252
        .short  0xffff
        .short  0x0002, 0x0209                  | frame 2
        .long   0x0023123e
        .short  0xffff
        .short  0x0002, 0x0209                  | frame 3
        .long   0x0023122a
        .short  0xffff
        .short  0x0002, 0x0209                  | frame 4
        .long   0x0023123e
        .short  0xffff
        .short  0x0002, 0x0209                  | frame 5
        .long   0x00231252
        .short  0xffff
        .short  0x0002, 0x0209                  | frame 6
        .long   0x00231266
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh07:                                          | +214  heading  7
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00231b58
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00231b6c
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00231b80
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00231b94
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00231ba8
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00231c2c
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00231c40
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh08:                                          | +260  heading  8
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00230fc2
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00230fae
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00230f9a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00230f86
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00230f9a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00230fae
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00230fc2
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh09:                                          | +2ac  heading  9
        .short  0x0002, 0x0208                  | frame 0
        .long   0x0023098e
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x002309a6
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x002309ba
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x002309ce
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00230a94
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00230aac
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00230ac4
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh10:                                          | +2f8  heading 10
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00231c2c
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00231c40
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00231c54
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00231c68
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00231c54
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00231c40
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00231c2c
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh11:                                          | +344  heading 11
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00232efa
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00232f0e
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00232f22
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00232f36
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00232f4a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00232f5e
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00232f72
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh12:                                          | +390  heading 12
        .short  0x0002, 0x0208                  | frame 0
        .long   0x002336fe
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00233712
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00233726
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00233638
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x0023364c
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00233660
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x0023367a
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh13:                                          | +3dc  heading 13
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00231ab8
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00231acc
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00231ae0
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00231af4
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00231ae0
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00231acc
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00231ab8
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh14:                                          | +428  heading 14
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00250e6a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x00250e56
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00250dd6
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x00250df2
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00250e06
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x00250e1a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00250e2e
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 7
        .long   0x00250e42
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

.Lh15:                                          | +47e  heading 15
        .short  0x0002, 0x0208                  | frame 0
        .long   0x00231854
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 1
        .long   0x0023173c
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 2
        .long   0x00231750
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 3
        .long   0x0023176a
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 4
        .long   0x00231750
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 5
        .long   0x0023173c
        .short  0xffff
        .short  0x0002, 0x0208                  | frame 6
        .long   0x00231854
        .short  0xffff
        .short  0x0100                          | terminador
        .long   Heading16IdleAnim_043354        | loop ptr ABSOLUTO

Heading16IdleAnim_043354:                       | +4ca  tabla IDLE (sin loop)
        .short  0x0004, 0x0208                  | frame 0
        .long   0x002311b2
        .short  0xffff
        .short  0x0004, 0x0208                  | frame 1
        .long   0x00250d90
        .short  0xffff
        .short  0x0004, 0x0208                  | frame 2
        .long   0x00250da4
        .short  0xffff
        .short  0x0001, 0x0208                  | frame 3
        .long   0x00250db8
        .short  0xffff
        .short  0x1600                          | terminador HOLD (no loop!)

Heading16PtrTable_04337E:                       | +4f4  16 punteros
        .long   .Lh00                          | heading  0 -> $042e8a
        .long   .Lh01                          | heading  1 -> $042ed6
        .long   .Lh02                          | heading  2 -> $042f22
        .long   .Lh03                          | heading  3 -> $042f6e
        .long   .Lh04                          | heading  4 -> $042fba
        .long   .Lh05                          | heading  5 -> $043006
        .long   .Lh06                          | heading  6 -> $043052
        .long   .Lh07                          | heading  7 -> $04309e
        .long   .Lh08                          | heading  8 -> $0430ea
        .long   .Lh09                          | heading  9 -> $043136
        .long   .Lh10                          | heading 10 -> $043182
        .long   .Lh11                          | heading 11 -> $0431ce
        .long   .Lh12                          | heading 12 -> $04321a
        .long   .Lh13                          | heading 13 -> $043266
        .long   .Lh14                          | heading 14 -> $0432b2
        .long   .Lh15                          | heading 15 -> $043308

        .size Heading16AnimSet_042E8A, .-Heading16AnimSet_042E8A

| ----------------------------------------------------------------------------
|  void Heading16Sprite_Handler_0433BE(Task *t /*a6*/)  @ $0433BE  (160 B)
|
|  Setup: facing por stat +0x98, prioridad $8000, flags 38, sonido $E,
|  aplica la tabla IDLE (pc-rel!) e instala el bucle. Bucle dirigido
|  por RAM global:
|    $106F29 < 0  -> nada (solo fisica+anim)
|    $106F29 == 0 -> girar en Y (bchg facing) + reaplicar IDLE
|    $106F29 > 0  -> rumbo = $106F28 & $F, indexa la tabla de punteros
|                    (pc-rel) y aplica la anim del rumbo
|  Despawn cuando x+$30 >= $1A0 (sale por la derecha).
| ----------------------------------------------------------------------------
        .section .text.Heading16Sprite_Handler_0433BE, "ax", @progbits
        .global Heading16Sprite_Handler_0433BE
Heading16Sprite_Handler_0433BE:
        tst.b   0x98(a6)                        | +000  stat de facing
        beq.w   .Lhs_prio                       | +004
        bset    #0, 0x3a(a6)                    | +008  mira derecha
.Lhs_prio:
        move.w  #0x8000, d0                     | +00e
        jsr     0x28134.l                       | +012  prioridad
        andi.w  #0xffe3, 0x38(a6)               | +018
        ori.w   #0x18, 0x38(a6)                 | +01e
        move.w  #0xe, d1                        | +024
        jsr     0x236e.l                        | +028  sonido
        lea     Heading16IdleAnim_043354(pc), a0 | +02e  tabla idle
        jsr     0x28cd4.l                       | +032  aplicar
        lea     .Lhs_run(pc), a1                | +038
        move.l  a1, (a6)                        | +03c
.Lhs_run:
        move.b  0x106f29.l, d0                  | +03e  byte de control
        bmi.w   .Lhs_skip                       | +044  <0 -> nada
        bne.w   .Lhs_indexed                    | +048  >0 -> rumbo
        bchg    #0, 0x3a(a6)                    | +04c  ==0 -> girar
        lea     Heading16IdleAnim_043354(pc), a0 | +052
        jsr     0x28cd4.l                       | +056  reaplicar idle
        bra.w   .Lhs_skip                       | +05c
.Lhs_indexed:
        moveq   #0, d0                          | +060
        move.b  0x106f28.l, d0                  | +062  rumbo global
        andi.b  #0xf, d0                        | +068  0..15
        lsl.w   #2, d0                          | +06c  *4 (punteros)
        lea     Heading16PtrTable_04337E(pc), a0 | +06e  tabla pc-rel!
        movea.l (a0,d0.w), a0                   | +072  tabla del rumbo
        jsr     0x28cd4.l                       | +076  aplicar
.Lhs_skip:
        jsr     0x2783a.l                       | +07c  fisica
        jsr     0x28d70.l                       | +082  anim
        move.w  0x22(a6), d0                    | +088  x
        addi.w  #0x30, d0                       | +08c
        cmpi.w  #0x1a0, d0                      | +090  fuera por la derecha?
        bcs.w   .Lhs_rts                        | +094
        jmp     0x518.l                         | +098  despawn
.Lhs_rts:
        rts                                     | +09e

        .size Heading16Sprite_Handler_0433BE, .-Heading16Sprite_Handler_0433BE

| ----------------------------------------------------------------------------
|  void TrackTargetLatch75_04345E(Task *t /*a6*/)  @ $04345E  (42 B)
|
|  Clon del tracker tri-estado sobre +0x75 pero con la primera rama
|  INVERTIDA (`bcs.w`, no `bcc.w`): aqui cs = "sin atacante" va al
|  camino de comparacion y el else asigna el latch. SIN fase $28364.
|  rts propio (no depende de islas).
| ----------------------------------------------------------------------------
        .section .text.TrackTargetLatch75_04345E, "ax", @progbits
        .global TrackTargetLatch75_04345E
TrackTargetLatch75_04345E:
        jsr     0x27f08.l                       | +000  scan atacantes
        bcs.w   .Ll75_cmp                       | +006  INVERTIDA!
        move.b  d3, 0x75(a6)                    | +00a  latch = atacante
        bra.w   .Ll75_rts                       | +00e
.Ll75_cmp:
        cmp.b   0x75(a6), d0                    | +012
        bne.w   .Ll75_clear                     | +016
        move.b  d3, 0x75(a6)                    | +01a  refresca
        bra.w   .Ll75_rts                       | +01e
.Ll75_clear:
        move.b  #0xff, 0x75(a6)                 | +022  latch invalido
.Ll75_rts:
        rts                                     | +028

        .size TrackTargetLatch75_04345E, .-TrackTargetLatch75_04345E

| ----------------------------------------------------------------------------
|  void TrackTargetLatch78_043488(Task *t /*a6*/)  @ $043488  (42 B)
|
|  Clon byte-a-byte del anterior sobre el latch +0x78.
| ----------------------------------------------------------------------------
        .section .text.TrackTargetLatch78_043488, "ax", @progbits
        .global TrackTargetLatch78_043488
TrackTargetLatch78_043488:
        jsr     0x27f08.l                       | +000
        bcs.w   .Ll78_cmp                       | +006  INVERTIDA!
        move.b  d3, 0x78(a6)                    | +00a
        bra.w   .Ll78_rts                       | +00e
.Ll78_cmp:
        cmp.b   0x78(a6), d0                    | +012
        bne.w   .Ll78_clear                     | +016
        move.b  d3, 0x78(a6)                    | +01a
        bra.w   .Ll78_rts                       | +01e
.Ll78_clear:
        move.b  #0xff, 0x78(a6)                 | +022
.Ll78_rts:
        rts                                     | +028

        .size TrackTargetLatch78_043488, .-TrackTargetLatch78_043488

| ----------------------------------------------------------------------------
|  ccr Entity_CmpDepthToParent_0434B2(Task *t /*a6*/)  @ $0434B2  (16 B)
|
|  Clon byte-exacto de Entity_CmpDepthToParent_042390 con su propia
|  pareja de islas CCR: bcs -> SetXN_0434c8, fall -> ClearXN_0434c2.
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_0434B2, "ax", @progbits
        .global Entity_CmpDepthToParent_0434B2
Entity_CmpDepthToParent_0434B2:
        movea.l 0x8(a6), a1                     | +000  a1 = padre
        move.b  0x10(a6), d0                    | +004
        cmp.b   0x10(a1), d0                    | +008
        bcs.w   SetXN_0434c8                    | +00c  menor -> X/N set
        | cae en ClearXN_0434c2 (isla C)

        .size Entity_CmpDepthToParent_0434B2, .-Entity_CmpDepthToParent_0434B2
