| ============================================================================
|  Metal Slug 1 - asm/scene_script_vm_0437da.s
|  ----------------------------------------------------------------------------
|  Wave WW - interprete de bytecode de scripts de escena/scroll + helpers.
|  Cluster $0437DA..$043D60 y $043E3A..$043F52, 5 entradas, 1 594 B.
|
|  LA FUNCION PRINCIPAL DEL SCROLL DEL JUEGO. SceneScriptVM_Frame_0437DA es
|  la VM que ejecuta cada frame el script de la escena activa (puntero de
|  programa en $10815C, cargado por SceneLoader_Main_043568 desde la tabla
|  $916C8): 23 opcodes despachados por jump-table pc-relativa, que colocan
|  camara, fijan limites de scroll, spawnean tareas y ceden el frame con el
|  opcode $02 (integracion de scroll + camara). Cierra el hueco de 1 414 B
|  que quedaba entre Geom_Proj_Clamp ($436DE) y ClearC_043d60, extendiendo
|  el megabloque contiguo hasta $04422A (~13 KB).
|
|  Formato de opcode: word en (PC)*4 como indice en la tabla de $04381C.
|  HALLAZGO: la tabla tiene 22 `bra.w` (ops $00..$15) y el op $16 "desborda"
|  la tabla aterrizando DIRECTAMENTE en el codigo inline de $043874 (el
|  handler mas caliente: fijar el progreso base del scroll). Ahorra un
|  bra.w a costa de soldar el ultimo handler a la tabla.
|
|  RAM del subsistema (contexto de scroll):
|    $10815C.l  PC del script de escena          $106F50.l  pos X (16.16)
|    $106F54.l  pos Y suavizada (16.16)          $106F58.l  pos Y cruda (16.16)
|    $106F5C.w  progreso maximo (high-water)     $106F5E/60/64  seeds de vel.
|    $108160/62.w  delta X/Y del frame           $108164/66.w  par op $10
|    $108168..70.w limites min/max X/Y + slope   $108172/74/76.w  base progreso
|    $108178.b  modo de eje (0/2/3/otros)        $108179.b  latch borde OFF
|    $10817A.l  acumulador smoothing camara-Y    $108182.w  delta smoothing
|
|  ---------- SceneScriptVM_Frame_0437DA  (1 288 B) ---------------------------
|
|  /* Un frame de la VM de script de escena.
|   *  d0.l/d1.l = velocidad de scroll X/Y (16.16) del caller, sustituida
|   *  por ($106F60,$106F64) si el latch $106F5E esta armado. */
|  void SceneScriptVM_Frame(long vel_x, long vel_y) {
|      if (scroll_seed_latch) { vel_x = seed_x; vel_y = seed_y; }
|      push(vel_x, vel_y);
|      Subsystem_77092(); Subsystem_8F172();      /* ticks por-frame */
|      Subsystem_997CC(); Subsystem_8F8E2();
|      for (;;) {
|          op = *script_pc;
|          switch (op) {                          /* jmp tabla(pc,d0.w) */
|          case 0x02:  /* fin de frame: integra scroll+camara y sale */
|              dx = Scroll_ClampToRange(pop(), &pos_x, min_x, max_x);
|              vy = pop();
|              if (slope) {                       /* tramo diagonal:    */
|                  vy = (slope<0 ? +dx : -dx)<<16;/* vel Y esclava de X */
|                  adj = (slope*(max_x-pos_x))>>8;
|                  min_y += adj; max_y += adj;
|              }
|              dy = Scroll_ClampToRange(vy, &pos_y, min_y, max_y);
|              progress = base + avance_segun_modo($108178);
|              if (progress > high_water) high_water = progress;
|              Camera_SmoothingIntegrate();
|              pos_y_suave = pos_y + smooth_acc;
|              dy += smooth_delta;
|              Sub_4CB88();
|              return CameraApplyAll4();          /* bra.w, tail-call  */
|          ...los otros 22 opcodes avanzan script_pc y hacen continue;
|          }
|      }
|  }
|
|  Detalles forenses:
|    - Preambulo: si $106F5E==0 se pushean los d0/d1 DEL CALLER sin tocar
|      (los seeds viajan en registros, el latch solo los sobreescribe).
|    - op $06 (wait-callback): `jsr 2(a1)` llama codigo embebido EN el
|      script; si devuelve d0!=0 cede el frame SIN avanzar el PC (re-poll
|      el proximo frame); si devuelve 0, el propio callback ya avanzo a1
|      y el PC se recarga de a1 (unico opcode donde el callee mueve el PC).
|    - op $0E: pushea el puntero, llama a Task_AllocFromFreeList (T#4) y
|      escribe el puntero DE VUELTA en el stream del script (script en RAM:
|      el slot tras el opcode es un parametro out).
|    - op $13 clampa via ClampD0ToRange ($219C, Wave A#1).
|    - op $0B llama $51F02 y, si el entity tiene continuacion en +$78,
|      limpia el mapa de colision via Buffer_ClearBlock1024L_043EDA.
|    - Los handlers estan en ROM en orden INVERSO al de la tabla (el op $16
|      pegado a la tabla, el op $02 al final) salvo dos trasposiciones:
|      op $13 ($439D8) entre op $0E y op $0D, y op $07 ($43A74) antes que
|      op $08 ($43A8C).
|
|  Por que NO es rederivable por GCC 1:1:
|    - switch con jump-table `jmp d8(pc,d0.w)` de bra.w: GCC emite tabla de
|      offsets .word + `add`+`jmp (a0)`, jamas una tabla de instrucciones.
|    - El desborde de tabla del op $16 es imposible de expresar en C.
|    - Los handlers releen a1 desde memoria una sola vez por iteracion y
|      escriben el PC ANTES de usar los parametros (orden manual).
|
|  ---------- SceneScript_EdgeArrivalTest_043CE2  (126 B) ---------------------
|
|  /* CCR: C=1 si el scroll YA llego al borde objetivo (o el chequeo esta
|   *  deshabilitado); C=0 si aun avanza. Cae en ClearC_043d60/SetC_043d66. */
|  bool SceneScript_EdgeArrivalTest(void) {
|      if (edge_latch_off || !flag_10E39A) return CARRY_SET;
|      switch (axis_mode) {
|      case 3:  return pos_y == max_y;     /* Y hacia adelante  */
|      case 2:  return pos_y == min_y;     /* Y hacia atras     */
|      case 0:  return pos_x == min_x;     /* X hacia atras     */
|      default: return pos_x == max_x;     /* X hacia adelante  */
|      }
|  }
|
|  Unico caller externo conocido: $08C8C6. Mismo patron de pareja de islas
|  CCR que TargetFacingTest ($0423AC, Wave VV) pero con 4 comparaciones.
|
|  ---------- Scroll_ClampToRange_043E3A  (78 B) -------------------------------
|
|  /* Integra un paso de scroll saturado.
|   *  d0.l=velocidad (16.16), d1.l=posicion (16.16),
|   *  d2.w=limite min (entero), d3.w=limite max (entero).
|   *  Devuelve d0.w=delta entero aplicado, d1.l=nueva posicion.
|   *  Los early-out saltan a la isla ClrWD0_00043E88 (delta=0). */
|  Preconvierte los limites a 16.16 (swap+clr / $FFFF en la parte baja del
|  max para clampear al ultimo subpixel). Si vel>0 y pos>=max, o vel<0 y
|  pos<min -> isla (ya saturado). Tras pos+=vel detecta el CRUCE del limite
|  comparando el signo de (limite-pos_nueva) XOR (limite-pos_vieja) via
|  `eor.l` + `bpl` y fija pos=limite. HALLAZGO: el max en 16.16 se
|  materializa con `move.w #$FFFF,d3` tras `swap` (36 3C FF FF), no con
|  moveq -1: solo la palabra baja debe quedar a unos.
|
|  ---------- CollMap_RowToLocalY_043EEC  (22 B) -------------------------------
|
|  /* d0.w = fila de celda -> Y local en pixeles (invertida).
|   *  ctx = $106F6C: +$08 y_origin, +$24 tile_y_base. */
|  short CollMap_RowToLocalY(short row) {
|      return -(((row + tile_y_base) << 4) - y_origin);
|  }
|
|  ---------- CollMap_TestPoint_043F02  (80 B) ---------------------------------
|
|  /* CCR: C=1 si el punto (d0.w,d1.w) cae en celda solida del mapa de
|   *  colision de 4 KB en $106F6C+$7C (el MISMO buffer que limpia
|   *  Buffer_ClearBlock1024L_043EDA y que resetea el op $0B de la VM).
|   *  Deja en d3/d4 el resto sub-celda (x&7, y&7) para el ajuste fino. */
|  Empaqueta (x,y) en un indice de 12 bits con celdas de 8x8 organizadas
|  en parejas: idx = rol3(x&$1F0) + ror2(y&$1F8) + ror3(x&8), es decir
|  bloques de 2 columnas x 64 filas. Lee el byte y cae en la pareja de
|  islas ClearC_043f52 / SetC_043f58 segun sea 0 o no.
|  Callers: $27DC2, $5B1DE, $5F122, $5F196, $5F2D0 (fisica de entidades).
| ============================================================================

| ----------------------------------------------------------------------------
|  SceneScriptVM_Frame_0437DA  @ $0437DA  (1 288 B)
| ----------------------------------------------------------------------------
|
        .globl  SceneScriptVM_Frame_0437DA
        .type   SceneScriptVM_Frame_0437DA, @function
        .section .text.SceneScriptVM_Frame_0437DA, "ax", @progbits
SceneScriptVM_Frame_0437DA:
        tst.w   0x106f5e.l                     | +000  latch de seeds armado?
        beq.w   .Lpush_seeds                   | +006  no: usa d0/d1 del caller
        move.l  0x106f60.l, d0                 | +00a  d0 = seed vel X
        move.l  0x106f64.l, d1                 | +010  d1 = seed vel Y
.Lpush_seeds:                                  | $0437F0
        movem.l d0-d1, -(a7)                   | +016  guarda vel X/Y del frame
        jsr     0x77092.l                      | +01a  tick subsistema 1
        jsr     0x8f172.l                      | +020  tick subsistema 2
        jsr     0x997cc.l                      | +026  tick subsistema 3
        jsr     0x8f8e2.l                      | +02c  tick subsistema 4
.Lfetch:                                       | $04380C  bucle de la VM
        movea.l 0x10815c.l, a1                 | +032  a1 = PC del script
        move.w  (a1), d0                       | +038  d0 = opcode
        add.w   d0, d0                         | +03a  *2
        add.w   d0, d0                         | +03c  *4 (entradas bra.w)
        jmp     .Lop_table(pc, d0.w)           | +03e  despacho pc-relativo
.Lop_table:                                    | $04381C  22 bra.w + desborde
        bra.w   .Lop00_bind_path               | op $00
        bra.w   .Lop01_set_fields              | op $01
        bra.w   .Lop02_end_frame               | op $02  (cede el frame)
        bra.w   .Lop03_warp                    | op $03
        bra.w   .Lop04_spawn_pair              | op $04
        bra.w   .Lop05_detach                  | op $05
        bra.w   .Lop06_wait_cb                 | op $06
        bra.w   .Lop07_call_ece                | op $07
        bra.w   .Lop08_call_ed6                | op $08
        bra.w   .Lop09_probe_ece               | op $09
        bra.w   .Lop0a_call_2b58               | op $0a
        bra.w   .Lop0b_cond_clear              | op $0b
        bra.w   .Lop0c_set_6eac                | op $0c
        bra.w   .Lop0d_call_inline             | op $0d
        bra.w   .Lop0e_alloc_task              | op $0e
        bra.w   .Lop0f_set_limits              | op $0f
        bra.w   .Lop10_set_6466                | op $10
        bra.w   .Lop11_pair_1b38               | op $11
        bra.w   .Lop12_set_axis                | op $12
        bra.w   .Lop13_clamp                   | op $13
        bra.w   .Lop14_call_newpc              | op $14
        bra.w   .Lop15_set_campos              | op $15
.Lop16_set_progress:                           | $043874  op $16: DESBORDE de
        lea.l   0x4(a1), a0                    | +09a  tabla (aterriza aqui)
        move.l  a0, 0x10815c.l                 | +09e  PC += 4
        move.w  0x2(a1), 0x106f5c.l            | +0a4  high-water = parametro
        move.w  0x106f5c.l, 0x108172.l         | +0ac  base progreso
        move.w  0x106f50.l, 0x108174.l         | +0b6  base X
        move.w  0x106f58.l, 0x108176.l         | +0c0  base Y
        bra.w   .Lfetch                        | +0ca
.Lop15_set_campos:                             | $0438A8  fijar camara (x,y)
        lea.l   0x6(a1), a0                    | +0ce
        move.l  a0, 0x10815c.l                 | +0d2  PC += 6
        move.w  0x2(a1), d0                    | +0d8  d0 = X entero
        move.w  0x4(a1), d1                    | +0dc  d1 = Y entero
        swap    d0                             | +0e0  a 16.16
        swap    d1                             | +0e2
        clr.w   d0                             | +0e4
        clr.w   d1                             | +0e6
        move.l  d0, 0x106f50.l                 | +0e8  pos X
        move.l  d1, 0x106f54.l                 | +0ee  pos Y suave
        move.l  d1, 0x106f58.l                 | +0f4  pos Y cruda
        move.w  0x106f5c.l, 0x108172.l         | +0fa  re-publica bases
        move.w  0x106f50.l, 0x108174.l         | +104
        move.w  0x106f58.l, 0x108176.l         | +10e
        clr.w   0x108160.l                     | +118  delta X = 0
        clr.w   0x108162.l                     | +11e  delta Y = 0
        bsr.w   Camera_ResetSmoothing_0434EA   | +124  smoothing a cero
        bra.w   .Lfetch                        | +128
.Lop14_call_newpc:                             | $043906  callback fija el PC
        jsr     0x2(a1)                        | +12c  codigo embebido
        move.l  a0, 0x10815c.l                 | +130  PC = a0 devuelto
        bra.w   .Lfetch                        | +136
.Lop12_set_axis:                               | $043914  modo de eje
        lea.l   0x4(a1), a0                    | +13a
        move.l  a0, 0x10815c.l                 | +13e  PC += 4
        move.b  0x2(a1), d0                    | +144
        move.b  d0, 0x108178.l                 | +148  modo = parametro byte
        move.w  0x106f5c.l, 0x108172.l         | +14e  re-publica bases
        move.w  0x106f50.l, 0x108174.l         | +158
        move.w  0x106f58.l, 0x108176.l         | +162
        bra.w   .Lfetch                        | +16c
.Lop11_pair_1b38:                              | $04394A  par de punteros
        lea.l   0xa(a1), a0                    | +170
        move.l  a0, 0x10815c.l                 | +174  PC += 10
        movea.l 0x2(a1), a0                    | +17a  a0 = ptr 1
        movea.l 0x6(a1), a1                    | +17e  a1 = ptr 2
        jsr     0x51b38.l                      | +182
        bra.w   .Lfetch                        | +188
.Lop10_set_6466:                               | $043966  par de words
        lea.l   0x6(a1), a0                    | +18c
        move.l  a0, 0x10815c.l                 | +190  PC += 6
        move.w  0x2(a1), 0x108164.l            | +196
        move.w  0x4(a1), 0x108166.l            | +19e
        bra.w   .Lfetch                        | +1a6
.Lop0f_set_limits:                             | $043984  limites de scroll
        lea.l   0xc(a1), a0                    | +1aa
        move.l  a0, 0x10815c.l                 | +1ae  PC += 12
        move.w  0x2(a1), 0x108168.l            | +1b4  min X
        move.w  0x4(a1), 0x10816a.l            | +1bc  min Y
        move.w  0x6(a1), 0x10816c.l            | +1c4  max X
        move.w  0x8(a1), 0x10816e.l            | +1cc  max Y
        move.w  0xa(a1), 0x108170.l            | +1d4  slope Y-por-X
        bra.w   .Lfetch                        | +1dc
.Lop0e_alloc_task:                             | $0439BA  spawn de tarea
        lea.l   0x6(a1), a0                    | +1e0
        move.l  a0, 0x10815c.l                 | +1e4  PC += 6
        movea.l 0x2(a1), a1                    | +1ea  a1 = handler
        move.l  a1, -(a7)                      | +1ee  preserva sobre el alloc
        jsr     Task_AllocFromFreeList         | +1f0  scheduler_add (T#4)
        movea.l (a7)+, a1                      | +1f6
        move.l  a1, (a0)                       | +1f8  out-param EN el script
        bra.w   .Lfetch                        | +1fa
.Lop13_clamp:                                  | $0439D8  clamp de un word
        lea.l   0x4(a1), a0                    | +1fe
        move.l  a0, 0x10815c.l                 | +202  PC += 4
        move.w  0x2(a1), d0                    | +208
        jsr     ClampD0ToRange                 | +20c  $219C (Wave A#1)
        bra.w   .Lfetch                        | +212
.Lop0d_call_inline:                            | $0439F0  callback con args
        lea.l   0xe(a1), a0                    | +216
        move.l  a0, 0x10815c.l                 | +21a  PC += 14
        lea.l   0x6(a1), a0                    | +220  a0 = &args inline
        movea.l 0x2(a1), a1                    | +224  a1 = funcion
        jsr     (a1)                           | +228
        bra.w   .Lfetch                        | +22a
.Lop0c_set_6eac:                               | $043A08  byte global
        lea.l   0x4(a1), a0                    | +22e
        move.l  a0, 0x10815c.l                 | +232  PC += 4
        move.b  0x2(a1), 0x106eac.l            | +238
        bra.w   .Lfetch                        | +240
.Lop0b_cond_clear:                             | $043A1E  clear condicional
        lea.l   0x6(a1), a0                    | +244
        move.l  a0, 0x10815c.l                 | +248  PC += 6
        movea.l 0x2(a1), a0                    | +24e  a0 = entity
        jsr     0x51f02.l                      | +252
        tst.l   0x78(a0)                       | +258  hay continuacion?
        beq.w   .Lfetch                        | +25c  no
        bsr.w   Buffer_ClearBlock1024L_043EDA  | +260  limpia mapa colision
        bra.w   .Lfetch                        | +264
.Lop0a_call_2b58:                              | $043A42  parser externo
        lea.l   0x2(a1), a0                    | +268  a0 = cursor en el script
        jsr     Sub_00002B58                   | +26c  consume datos, mueve a0
        addq.w  #2, a0                         | +272  +2 de alineacion
        move.l  a0, 0x10815c.l                 | +274  PC = cursor final
        bra.w   .Lfetch                        | +27a
.Lop09_probe_ece:                              | $043A58  probe + call
        lea.l   0x6(a1), a0                    | +27e
        move.l  a0, 0x10815c.l                 | +282  PC += 6
        movea.l 0x2(a1), a0                    | +288  a0 = entity
        bsr.w   CameraHook_Probe08_043DF4      | +28c
        jsr     0x51ece.l                      | +290
        bra.w   .Lfetch                        | +296
.Lop07_call_ece:                               | $043A74
        lea.l   0x6(a1), a0                    | +29a
        move.l  a0, 0x10815c.l                 | +29e  PC += 6
        movea.l 0x2(a1), a0                    | +2a4  a0 = entity
        jsr     0x51ece.l                      | +2a8
        bra.w   .Lfetch                        | +2ae
.Lop08_call_ed6:                               | $043A8C
        lea.l   0x6(a1), a0                    | +2b2
        move.l  a0, 0x10815c.l                 | +2b6  PC += 6
        movea.l 0x2(a1), a0                    | +2bc  a0 = entity
        jsr     0x51ed6.l                      | +2c0
        bra.w   .Lfetch                        | +2c6
.Lop06_wait_cb:                                | $043AA4  espera activa
        jsr     0x2(a1)                        | +2ca  callback embebido
        tst.b   d0                             | +2ce  sigue esperando?
        bne.w   .Lop02_end_frame               | +2d0  si: cede SIN avanzar PC
        move.l  a1, 0x10815c.l                 | +2d4  no: el cb ya avanzo a1
        bra.w   .Lfetch                        | +2da
.Lop05_detach:                                 | $043AB8  desacople de entity
        lea.l   0x6(a1), a0                    | +2de
        move.l  a0, 0x10815c.l                 | +2e2  PC += 6
        movea.l 0x2(a1), a0                    | +2e8  a0 = entity
        clr.l   0xe(a0)                        | +2ec  corta el enlace +$0E
        jsr     0x51ed6.l                      | +2f0
        jsr     TransformCommit_MMIO_051F30    | +2f6  commit MMIO (KK#1)
        bra.w   .Lfetch                        | +2fc
.Lop04_spawn_pair:                             | $043ADA  spawn con params
        lea.l   0x8(a1), a0                    | +300
        move.l  a0, 0x10815c.l                 | +304  PC += 8
        move.b  0x6(a1), d0                    | +30a  d0 = param A
        move.b  0x7(a1), d1                    | +30e  d1 = param B
        movea.l 0x2(a1), a0                    | +312  a0 = plantilla
        jsr     0x51b1c.l                      | +316
        bra.w   .Lfetch                        | +31c
.Lop03_warp:                                   | $043AFA  warp de camara
        lea.l   0x6(a1), a0                    | +320
        move.l  a0, 0x10815c.l                 | +324  PC += 6
        move.w  0x2(a1), d0                    | +32a  d0 = X entero
        move.w  0x4(a1), d1                    | +32e  d1 = Y entero
        move.b  #1, 0x108178.l                 | +332  modo eje = 1
        clr.w   0x106f5c.l                     | +33a  high-water = 0
        swap    d0                             | +340  a 16.16
        swap    d1                             | +342
        clr.w   d0                             | +344
        clr.w   d1                             | +346
        move.l  d0, 0x106f50.l                 | +348  pos X
        move.l  d1, 0x106f54.l                 | +34e  pos Y suave
        move.l  d1, 0x106f58.l                 | +354  pos Y cruda
        clr.w   0x108160.l                     | +35a  delta X = 0
        clr.w   0x108162.l                     | +360  delta Y = 0
        move.w  0x106f5c.l, 0x108172.l         | +366  re-publica bases
        move.w  0x106f50.l, 0x108174.l         | +370
        move.w  0x106f58.l, 0x108176.l         | +37a
        bsr.w   Camera_ResetSmoothing_0434EA   | +384
        bra.w   .Lfetch                        | +388
.Lop00_bind_path:                              | $043B66  bind entity a ruta
        lea.l   0x16(a1), a0                   | +38c
        move.l  a0, 0x10815c.l                 | +390  PC += 22
        movea.l 0x2(a1), a0                    | +396  a0 = entity
        move.l  0xe(a1), 0x78(a0)              | +39a  continuacion en +$78
        move.w  0x106f50.l, d0                 | +3a0  d0 = scroll X
        move.w  0x106f58.l, d1                 | +3a6  d1 = scroll Y
        add.w   0x10817a.l, d1                 | +3ac  + smoothing actual
        sub.w   0x12(a1), d0                   | +3b2  - anclas del script
        sub.w   0x14(a1), d1                   | +3b6
        lea.l   0x6(a1), a1                    | +3ba  a1 = &datos de ruta
        jsr     0x51b3e.l                      | +3be
        bra.w   .Lfetch                        | +3c4
.Lop01_set_fields:                             | $043BA2  campos 72/74/76
        lea.l   0xc(a1), a0                    | +3c8
        move.l  a0, 0x10815c.l                 | +3cc  PC += 12
        movea.l 0x2(a1), a0                    | +3d2  a0 = entity
        move.b  0x6(a1), 0x72(a0)              | +3d6
        move.w  0x8(a1), 0x74(a0)              | +3dc
        move.w  0xa(a1), 0x76(a0)              | +3e2
        bra.w   .Lfetch                        | +3e8
.Lop02_end_frame:                              | $043BC6  fin de frame
        move.l  (a7)+, d0                      | +3ec  d0 = vel X guardada
        move.l  0x106f50.l, d1                 | +3ee  d1 = pos X
        move.w  0x108168.l, d2                 | +3f4  d2 = min X
        move.w  0x10816c.l, d3                 | +3fa  d3 = max X
        bsr.w   Scroll_ClampToRange_043E3A     | +400  integra X
        move.w  d0, 0x108160.l                 | +404  delta X del frame
        move.l  d1, 0x106f50.l                 | +40a  nueva pos X
        move.l  (a7)+, d0                      | +410  d0 = vel Y guardada
        move.l  0x106f58.l, d1                 | +412  d1 = pos Y
        move.w  0x10816a.l, d2                 | +418  d2 = min Y
        move.w  0x10816e.l, d3                 | +41e  d3 = max Y
        move.w  0x108170.l, d4                 | +424  d4 = slope
        beq.w   .Lclamp_y                      | +42a  sin tramo diagonal
        bmi.w   .Lslope_neg                    | +42e
        move.w  0x108160.l, d0                 | +432  vel Y = -delta X
        neg.w   d0                             | +438
        bra.w   .Lslope_go                     | +43a
.Lslope_neg:                                   | $043C18
        move.w  0x108160.l, d0                 | +43e  vel Y = +delta X
.Lslope_go:                                    | $043C1E
        swap    d0                             | +444  a 16.16
        clr.w   d0                             | +446
        move.w  0x10816c.l, d5                 | +448  d5 = max X - pos X
        sub.w   0x106f50.l, d5                 | +44e  (distancia restante)
        muls.w  d5, d4                         | +454  d4 = slope * dist
        asr.l   #8, d4                         | +456  /256
        add.w   d4, d2                         | +458  desplaza la ventana Y
        add.w   d4, d3                         | +45a
.Lclamp_y:                                     | $043C36
        bsr.w   Scroll_ClampToRange_043E3A     | +45c  integra Y
        move.w  d0, 0x108162.l                 | +460  delta Y del frame
        move.l  d1, 0x106f58.l                 | +466  nueva pos Y
        move.b  0x108178.l, d0                 | +46c  d0 = modo de eje
        cmpi.b  #3, d0                         | +472
        bne.w   .Lnot_mode3                    | +476
        move.w  0x106f58.l, d1                 | +47a  modo 3: Y adelante
        sub.w   0x108176.l, d1                 | +480
        bra.w   .Lprogress                     | +486
.Lnot_mode3:                                   | $043C64
        cmpi.b  #2, d0                         | +48a
        bne.w   .Lnot_mode2                    | +48e
        move.w  0x108176.l, d1                 | +492  modo 2: Y atras
        sub.w   0x106f58.l, d1                 | +498
        bra.w   .Lprogress                     | +49e
.Lnot_mode2:                                   | $043C7C
        cmpi.b  #0, d0                         | +4a2  cmpi, NO tst (0C00)
        bne.w   .Lmode_else                    | +4a6
        move.w  0x108174.l, d1                 | +4aa  modo 0: X atras
        sub.w   0x106f50.l, d1                 | +4b0
        bra.w   .Lprogress                     | +4b6
.Lmode_else:                                   | $043C94
        move.w  0x106f50.l, d1                 | +4ba  resto: X adelante
        sub.w   0x108174.l, d1                 | +4c0
.Lprogress:                                    | $043CA0
        add.w   0x108172.l, d1                 | +4c6  progreso = base+avance
        cmp.w   0x106f5c.l, d1                 | +4cc
        bls.w   .Lsmooth                       | +4d2  no supera el maximo
        move.w  d1, 0x106f5c.l                 | +4d6  nuevo high-water
.Lsmooth:                                      | $043CB6
        bsr.w   Camera_SmoothingIntegrate_0434F8 | +4dc  paso de smoothing
        move.l  0x10817a.l, d0                 | +4e0
        add.l   0x106f58.l, d0                 | +4e6  pos Y + acumulador
        move.l  d0, 0x106f54.l                 | +4ec  pos Y suavizada
        move.w  0x108182.l, d0                 | +4f2
        add.w   d0, 0x108162.l                 | +4f8  delta Y += smoothing
        jsr     0x4cb88.l                      | +4fe  post-proceso
        bra.w   CameraApplyAll4_043D86         | +504  tail-call, no vuelve
        .size   SceneScriptVM_Frame_0437DA, .-SceneScriptVM_Frame_0437DA

| ----------------------------------------------------------------------------
|  SceneScript_EdgeArrivalTest_043CE2  @ $043CE2  (126 B)
| ----------------------------------------------------------------------------
|
        .globl  SceneScript_EdgeArrivalTest_043CE2
        .type   SceneScript_EdgeArrivalTest_043CE2, @function
        .section .text.SceneScript_EdgeArrivalTest_043CE2, "ax", @progbits
SceneScript_EdgeArrivalTest_043CE2:
        tst.b   0x108179.l                     | +00  chequeo deshabilitado?
        bne.w   SetC_043d66                    | +06  si: C=1 (isla)
        tst.b   0x10e39a.l                     | +0a  flag global armado?
        beq.w   SetC_043d66                    | +10  no: C=1 (isla)
        move.b  0x108178.l, d0                 | +14  d0 = modo de eje
        cmpi.b  #3, d0                         | +1a
        bne.w   .Lnot_m3                       | +1e
        move.w  0x106f58.l, d0                 | +22  modo 3: pos Y vs max Y
        cmp.w   0x10816e.l, d0                 | +28
        beq.w   SetC_043d66                    | +2e  llego: C=1
        bra.w   ClearC_043d60                  | +32  avanza: C=0
.Lnot_m3:                                      | $043D18
        cmpi.b  #2, d0                         | +36
        bne.w   .Lnot_m2                       | +3a
        move.w  0x106f58.l, d0                 | +3e  modo 2: pos Y vs min Y
        cmp.w   0x10816a.l, d0                 | +44
        beq.w   SetC_043d66                    | +4a
        bra.w   ClearC_043d60                  | +4e
.Lnot_m2:                                      | $043D34
        cmpi.b  #0, d0                         | +52  cmpi, NO tst (0C00)
        bne.w   .Lm_else                       | +56
        move.w  0x106f50.l, d0                 | +5a  modo 0: pos X vs min X
        cmp.w   0x108168.l, d0                 | +60
        beq.w   SetC_043d66                    | +66
        bra.w   ClearC_043d60                  | +6a
.Lm_else:                                      | $043D50
        move.w  0x106f50.l, d0                 | +6e  resto: pos X vs max X
        cmp.w   0x10816c.l, d0                 | +74
        beq.w   SetC_043d66                    | +7a  llego: C=1
                                               | fall-through a ClearC_043d60
        .size   SceneScript_EdgeArrivalTest_043CE2, .-SceneScript_EdgeArrivalTest_043CE2

| ----------------------------------------------------------------------------
|  Scroll_ClampToRange_043E3A  @ $043E3A  (78 B)
| ----------------------------------------------------------------------------
|
        .globl  Scroll_ClampToRange_043E3A
        .type   Scroll_ClampToRange_043E3A, @function
        .section .text.Scroll_ClampToRange_043E3A, "ax", @progbits
Scroll_ClampToRange_043E3A:
        swap    d2                             | +00  min a 16.16 (low=0)
        swap    d3                             | +02  max a 16.16...
        clr.w   d2                             | +04
        move.w  #0xffff, d3                    | +06  ...low=$FFFF (36 3C,
        tst.l   d0                             | +0a  NO moveq: solo la word)
        beq.w   ClrWD0_00043E88                | +0c  vel 0: isla, delta=0
        bmi.w   .Lvel_neg                      | +10
        cmp.l   d1, d3                         | +14  vel>0: pos ya >= max?
        bcs.w   ClrWD0_00043E88                | +16  si: saturado, delta=0
        move.l  d3, d2                         | +1a  d2 = limite activo (max)
        bra.w   .Lintegrate                    | +1c
.Lvel_neg:                                     | $043E5A
        cmp.l   d1, d2                         | +20  vel<0: pos ya < min?
        bcc.w   ClrWD0_00043E88                | +22  si: saturado, delta=0
.Lintegrate:                                   | $043E60  (d2 = min si vel<0)
        add.l   d1, d0                         | +26  d0 = pos nueva
        cmpi.l  #0xffffffff, d2                | +28  sin limite armado?
        beq.w   .Lcommit                       | +2e  (solo si min=-1)
        move.l  d2, d3                         | +32  cruce de limite:
        move.l  d2, d4                         | +34  signo(lim-nueva) XOR
        sub.l   d0, d3                         | +36  signo(lim-vieja)
        sub.l   d1, d4                         | +38
        eor.l   d3, d4                         | +3a
        bpl.w   .Lcommit                       | +3c  mismo lado: sin cruce
        move.l  d2, d0                         | +40  cruzo: pos = limite
.Lcommit:                                      | $043E7C
        move.l  d0, d2                         | +42  d2 = pos nueva
        swap    d0                             | +44  delta entero =
        swap    d1                             | +46  hi(nueva) - hi(vieja)
        sub.w   d1, d0                         | +48
        move.l  d2, d1                         | +4a  d1 = pos nueva (16.16)
        rts                                    | +4c
        .size   Scroll_ClampToRange_043E3A, .-Scroll_ClampToRange_043E3A

| ----------------------------------------------------------------------------
|  CollMap_RowToLocalY_043EEC  @ $043EEC  (22 B)
| ----------------------------------------------------------------------------
|
        .globl  CollMap_RowToLocalY_043EEC
        .type   CollMap_RowToLocalY_043EEC, @function
        .section .text.CollMap_RowToLocalY_043EEC, "ax", @progbits
CollMap_RowToLocalY_043EEC:
        lea.l   0x106f6c.l, a0                 | +00  a0 = ctx del mapa
        move.w  0x24(a0), d2                   | +06  d2 = tile_y_base
        add.w   d2, d0                         | +0a  fila absoluta
        lsl.w   #4, d0                         | +0c  *16 px por celda
        sub.w   0x8(a0), d0                    | +0e  - y_origin
        neg.w   d0                             | +12  eje Y invertido
        rts                                    | +14
        .size   CollMap_RowToLocalY_043EEC, .-CollMap_RowToLocalY_043EEC

| ----------------------------------------------------------------------------
|  CollMap_TestPoint_043F02  @ $043F02  (80 B)
| ----------------------------------------------------------------------------
|
        .globl  CollMap_TestPoint_043F02
        .type   CollMap_TestPoint_043F02, @function
        .section .text.CollMap_TestPoint_043F02, "ax", @progbits
CollMap_TestPoint_043F02:
        lea.l   0x106f6c.l, a0                 | +00  a0 = ctx del mapa
        add.w   0x4(a0), d0                    | +06  x += x_origin
        neg.w   d1                             | +0a  eje Y invertido
        add.w   0x8(a0), d1                    | +0c  y = y_origin - y
        move.w  0x22(a0), d2                   | +10  d2 = tile_x_base
        lsl.w   #4, d2                         | +14  *16
        add.w   d2, d0                         | +16  x absoluto en px
        move.w  d0, d3                         | +18  d3 = x & 7 (resto
        andi.w  #0x7, d3                       | +1a  sub-celda para caller)
        move.w  0x24(a0), d2                   | +1e  d2 = tile_y_base
        lsl.w   #4, d2                         | +22  *16
        add.w   d2, d1                         | +24  y absoluto en px
        move.w  d1, d4                         | +26  d4 = y & 7 (resto
        andi.w  #0x7, d4                       | +28  sub-celda para caller)
        move.w  d0, d2                         | +2c  empaquetado del indice:
        andi.w  #0x1f0, d0                     | +2e  x[8:4]
        andi.w  #0x8, d2                       | +32  x[3]
        andi.w  #0x1f8, d1                     | +36  y[8:3]
        ror.w   #2, d1                         | +3a  y[8:3] >> 2
        ror.w   #3, d2                         | +3c  x[3]   >> 3
        rol.w   #3, d0                         | +3e  x[8:4] << 3
        add.w   d1, d0                         | +40  idx 12 bits: bloques
        add.w   d2, d0                         | +42  de 2 cols x 64 filas
        lea.l   0x7c(a0), a0                   | +44  a0 = mapa 4 KB
        move.b  (a0, d0.w), d0                 | +48  lee la celda
        bne.w   SetC_043f58                    | +4c  solida: C=1 (isla)
                                               | fall-through a ClearC_043f52
        .size   CollMap_TestPoint_043F02, .-CollMap_TestPoint_043F02
