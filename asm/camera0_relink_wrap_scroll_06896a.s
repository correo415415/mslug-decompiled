| ============================================================================
|  Metal Slug 1 - asm/camera0_relink_wrap_scroll_06896a.s
|  ----------------------------------------------------------------------------
|  Wave RR#3 - relink + wrap del scroll de la camara principal (camera[0]).
|
|  Camera0_RelinkAndWrapScroll_06896A  @ $06896A  (112 bytes, 3 callers)
|
|  Termina SIN rts propio: la ultima instruccion cae por fall-through en
|  el epilogo compartido SetXN_0689da (`ori.b #0x11, ccr; rts`, ya
|  matcheado). El salto temprano tambien es directo a otro epilogo
|  compartido ya matcheado, ClearXN_0689e0 (`andi.b #0xEE, ccr; rts`).
|  Ninguno de los dos requiere symbol nuevo: son entradas REGISTRY ya
|  existentes referenciadas por nombre.
|
|  ---------- Mapa de callers -------------------------------------------------
|
|      JsrPcThunk_068398  (thunk, ya matcheado) -> jsr $6896a(pc)
|      JsrPcThunk_0683ae  (thunk, ya matcheado) -> jsr $6896a(pc)
|      JsrPcThunk_0683ea  (thunk, ya matcheado) -> jsr $6896a(pc)
|
|  Campos usados (struct camera, base $106F6C = camera[0], confirmados por
|  Wave JJ batch 1 en asm/camera_apply_cluster_043daa.s):
|
|      +0x74 scale_x   (usado por CameraApplyOne, no tocado aqui)
|      +0x76 scale_y   (idem)
|      +0x78 linked    (puntero; CameraApplyOne lo lee como "cam->linked")
|
|  Que hace:
|
|      d0 = a6->+0x72 - 8;                       ("gatillo" de re-anclaje,
|                                                  campo por-entidad, misma
|                                                  zona +0x72 vista en Wave
|                                                  QQ#1 como posible
|                                                  velocidad/momentum)
|      if (d0 > (s16)$106F50) goto ClearXN_0689e0;
|                                                  $106F50 = "camera_x"
|                                                  secundaria / contador de
|                                                  scroll (ver
|                                                  coord_camera_cluster_
|                                                  043f5e.s); si el gatillo
|                                                  todavia no alcanza al
|                                                  contador, aborta sin
|                                                  re-anclar (early exit
|                                                  directo al epilogo
|                                                  "Clear" de otra funcion)
|
|      a0 = &camera[0]                    ($106F6C)
|      a1 = &Tpl_2C84BA                   (direccion ROM, probable template
|                                          de datos; no analizada, no
|                                          referenciada por symbol porque
|                                          solo se usa su direccion, no se
|                                          llama)
|      a2 = $20A000                        (direccion VRAM/banco de tiles)
|      camera[0]->linked = a2;             re-vincula la camara al banco
|                                          $20A000 (probable "banco B" del
|                                          fix layer / background, alternando
|                                          con el banco previo para doble
|                                          buffer de scroll infinito)
|
|      d0 = (s16)$106F50 - a6->+0x72 - 0x140;    delta X recalculado
|                                                 (0xFEC0 = -0x140 en 16 bits,
|                                                  visto como suma en el
|                                                  disassembly)
|      d1 = $106F54 + 0x30 - 0x40;                delta Y recalculado
|                                                 ($106F54 = "camera_y"
|                                                  secundaria, mismo par que
|                                                  $106F50)
|      Sub_00051B3E(d0, d1);                       helper sin analizar aun
|                                                  (probable "publicar
|                                                  deltas de reanclaje")
|
|      a0 = &camera[0];
|      CameraHook_Probe08_043DF4();                 hook de camara YA
|                                                  matcheado (Wave JJ#1);
|                                                  referenciado por nombre
|
|      a0 = &camera[0];
|      d0 = 0x0f;  d1 = 0x00;
|      Sub_00051B1C(a0, d0, d1);                    helper sin analizar aun
|                                                  (probable "seleccionar
|                                                  banco/slot de camara")
|
|      $106F50 -= 0x140;                            resta el ancho de
|                                                  pantalla NTSC en pixeles
|                                                  (320 = 0x140) del
|                                                  contador de scroll --
|                                                  patron clasico de "wrap"
|                                                  de scroll infinito por
|                                                  reciclado de banco
|      [fall-through -> SetXN_0689da: ori.b #0x11,ccr; rts]
|
|  Interpretacion: al detectar que el scroll de fondo (contador $106F50)
|  ha avanzado una pantalla completa relativo al "gatillo" de la entidad
|  en a6, esta rutina re-ancla la camara principal a un nuevo banco de
|  tiles ($20A000), recalcula los deltas de reanclaje, dispara los hooks
|  de aplicacion de camara ya conocidos, y envuelve ("wrap") el contador
|  de scroll restandole el ancho de pantalla -- el mecanismo tipico de
|  scroll de fondo infinito con doble banco en hardware Neo Geo.
|
|  $51B3E y $51B1C quedan como candidatos naturales para
|  tools/rank_candidates.py; no se anaden a tools/symbols.py porque se
|  referencian con direccion absoluta literal (jsr abs.l), que no
|  requiere symbol para el match byte-a-byte.
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  Camera0_RelinkAndWrapScroll_06896A
        .type   Camera0_RelinkAndWrapScroll_06896A, @function
        .section .text.Camera0_RelinkAndWrapScroll_06896A, "ax", @progbits

Camera0_RelinkAndWrapScroll_06896A:
        move.w  0x72(a6), d0                    | +000  d0 = a6->+72
        subq.w  #8, d0                          | +004  d0 -= 8 (gatillo)
        cmp.w   0x106f50.l, d0                  | +006  d0 vs camera_x_sec
        bgt.w   ClearXN_0689e0                  | +00c  aun no toca: sale
        lea.l   0x106f6c.l, a0                  | +010  a0 = &camera[0]
        lea.l   0x2c84ba.l, a1                  | +016  a1 = &Tpl_2C84BA
        lea.l   0x20a000.l, a2                  | +01c  a2 = banco VRAM
        move.l  a2, 0x78(a0)                    | +022  camera[0]->linked = a2
        move.w  0x106f50.l, d0                  | +026  d0 = camera_x_sec
        sub.w   0x72(a6), d0                    | +02a  d0 -= a6->+72
        addi.w  #0xfec0, d0                     | +02e  d0 += -0x140
        move.w  0x106f54.l, d1                  | +032  d1 = camera_y_sec
        addi.w  #0x30, d1                       | +036  d1 += 0x30
        addi.w  #0xffc0, d1                     | +03a  d1 += -0x40
        jsr     0x51b3e.l                       | +03e  Sub_00051B3E(d0,d1)
        lea.l   0x106f6c.l, a0                  | +044  a0 = &camera[0]
        jsr     CameraHook_Probe08_043DF4        | +04a
        lea.l   0x106f6c.l, a0                  | +050  a0 = &camera[0]
        move.b  #0xf, d0                        | +056
        move.b  #0x0, d1                        | +05a
        jsr     0x51b1c.l                       | +05e  Sub_00051B1C(a0,d0,d1)
        subi.w  #0x140, 0x106f50.l              | +064  wrap: camera_x_sec -= 320

        .size   Camera0_RelinkAndWrapScroll_06896A, .-Camera0_RelinkAndWrapScroll_06896A
