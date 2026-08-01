| ============================================================================
|  Metal Slug 1 - asm/dispatcher_mode_table_001922.s
|  ----------------------------------------------------------------------------
|  Wave EE batch 1 - #3
|
|  Dispatcher_ModeTable_001922  @ $001922  (402 bytes)
|
|  Dispatcher-tabla de 11 estados (0..A) del subsistema "attract/title mode".
|  Cada estado se codifica como puntero long en la tabla de descriptores
|  $000C8E..$000E12 (fila "next" del descriptor). El scheduler invoca a
|  este handler cuando el nodo activo tiene su ptr apuntando aqui; el
|  handler publica el opcode de estado en $106ECE, arma el puntero del
|  handler concreto en $100260, dispara el trigger via jsr $5FE y hace
|  bra.w al backbone comun $00188A ("wait state").
|
|  Firma C conceptual:
|
|      /* Handler de un slot del scheduler. Trece labels internas:
|       *
|       *   $001922: state=0 -> publish handler $967FE
|       *   $001940: state=1 -> publish handler $96840
|       *   $00195E: state=2 -> publish handler $96882
|       *   $00197C: state=3 -> publish handler $968C4
|       *   $00199A: state=4 -> publish handler $96906
|       *   $0019B8: state=5 -> publish handler $96948
|       *   $0019D6: state=6 -> scheduler_add($1B4C) + publish $96840
|       *   $0019FE: state=7 -> publish handler $9698A
|       *   $001A1C: state=8 -> publish handler $96906  (mismo que #4)
|       *   $001A3A: state=9 -> publish handler $96906  (mismo que #4)
|       *   $001A58: state=A -> solo bra.w al backbone $188A
|       *
|       * Cada estado publica $106ECE = <opcode> antes de armar el
|       * handler. El estado 6 lleva un scheduler_add() extra que
|       * inserta $1B4C (TaskHandler_001B4C) en la cola antes de la
|       * publicacion habitual.
|       *
|       * Todos los estados terminan con bra.w $00188A - Attract_WaitStateBackbone_00188A
|       * es la cabecera "wait state" comun (target reciproco de todos
|       * los bra.w del cluster), que hace tail-call al scheduler tras
|       * publicar el handler.
|       *
|       * El bloque $001A64..$001AB5 es la rama "post-Start" del
|       * cluster: setup de timers ($44=$2, $45=$2), snapshot dual de
|       * $10FDB6/B7 en d0/d1, y branch table por combinaciones (0/1/2):
|       *   - si algun operando == 2 -> jsr $981FC.l + rts
|       *   - si algun operando == 1 -> jsr $981FC.l + fall-through al
|       *                                bloque post-rts $001AB6 (que es
|       *                                otra funcion del cluster)
|       *   - default (ambos == 0)   -> bsr Sub_00001E0A + publish
|       *                                handler ptr $E42 en $70(a6) +
|       *                                tail-jump $FC6.
|       */
|      void Dispatcher_ModeTable(void);
|
|  Layout del bloque estado i (30 B, para i in [0..8]):
|      move.b  #i, $106ECE.l
|      lea.l   $100260.l, a0            | a0 = &HANDLER_PTR_SLOT
|      move.l  #<HandlerAddr>, (a0)     | *a0 = HandlerAddr
|      jsr     $5FE.l                   | scheduler_publish(a0)
|      bra.w   Attract_WaitStateBackbone_00188A             | tail al backbone comun
|
|  Excepcion state=6 ($0019D6): 44 B total, con scheduler_add($1B4C).
|  Excepcion state=A ($001A58): 12 B, solo publica $106ECE = $A y bra.
|
|  Handlers registrados (todos parecen ser mini-scripts en zona $9xxx):
|      $967FE, $96840, $96882, $968C4, $96906, $96948,
|      $9698A                                (states 0..7)
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Los 11 estados son estructuralmente identicos pero cada uno se
|       codifica *literalmente*, sin bucle ni tabla de handlers. Es
|       imposible que GCC genere 30 B x 11 sin factorizar en subrutina
|       tabla. El codigo original claramente proviene de una macro de
|       ensamblador expandida (algo tipo STATE_PUBLISH i, addr).
|    2. `move.b #i, $106ECE.l` con `13FC 00i 00 106ECE` (10 B) por
|       estado. GCC habria usado `move.b i, d0; move.b d0, $106ECE.l`
|       si i variara, o bien un solo move.b si estuviera factorizado.
|    3. Los tres estados #4, #8, #9 apuntan al mismo handler $96906.
|       GCC habria producido una unica etiqueta compartida y branch
|       table. Aqui se duplica el bloque completo.
|    4. El estado #6 anade un `lea $1b4c(pc); jsr $4ae` *antes* del
|       bloque comun, en lugar de al final. Delata que la macro de
|       ensamblador toma un parametro opcional "aux_handler".
|    5. En el bloque post-Start ($001A64+) el uso de `lea.l $10FDB6.l, a4`
|       + `move.b (a4)+, d0; move.b (a4), d1` sirve para leer dos
|       bytes contiguos con auto-incremento sin recalcular la direccion.
|       GCC no emite este patron (usaria dos moves absolutos).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Dispatcher_ModeTable_001922
        .type   Dispatcher_ModeTable_001922, @function
        .section .text.Dispatcher_ModeTable_001922, "ax", @progbits

Dispatcher_ModeTable_001922:
                                              | ---- state=0 ($001922) ----
        move.b  #0x0, 0x106ece.l               | +00  opcode = 0
        lea.l   0x100260.l, a0                 | +08
        move.l  #0x967fe, (a0)                 | +0e  handler = $967FE
        jsr     0x5fe.l                        | +14  scheduler_publish
        bra.w   Attract_WaitStateBackbone_00188A                   | +1a  tail al backbone

                                              | ---- state=1 ($001940) ----
Label_001940:
        move.b  #0x1, 0x106ece.l               | +1e  opcode = 1
        lea.l   0x100260.l, a0                 | +26
        move.l  #0x96840, (a0)                 | +2c
        jsr     0x5fe.l                        | +32
        bra.w   Attract_WaitStateBackbone_00188A                   | +38

                                              | ---- state=2 ($00195E) ----
        move.b  #0x2, 0x106ece.l               | +3c
        lea.l   0x100260.l, a0                 | +44
        move.l  #0x96882, (a0)                 | +4a
        jsr     0x5fe.l                        | +50
        bra.w   Attract_WaitStateBackbone_00188A                   | +56

                                              | ---- state=3 ($00197C) ----
        move.b  #0x3, 0x106ece.l               | +5a
        lea.l   0x100260.l, a0                 | +62
        move.l  #0x968c4, (a0)                 | +68
        jsr     0x5fe.l                        | +6e
        bra.w   Attract_WaitStateBackbone_00188A                   | +74

                                              | ---- state=4 ($00199A) ----
Label_00199A:
        move.b  #0x4, 0x106ece.l               | +78
        lea.l   0x100260.l, a0                 | +80
        move.l  #0x96906, (a0)                 | +86
        jsr     0x5fe.l                        | +8c
        bra.w   Attract_WaitStateBackbone_00188A                   | +92

                                              | ---- state=5 ($0019B8) ----
        move.b  #0x5, 0x106ece.l               | +96
        lea.l   0x100260.l, a0                 | +9e
        move.l  #0x96948, (a0)                 | +a4
        jsr     0x5fe.l                        | +aa
        bra.w   Attract_WaitStateBackbone_00188A                   | +b0

                                              | ---- state=6 ($0019D6) - con scheduler_add extra ----
        move.b  #0x6, 0x106ece.l               | +b4
        lea.l   TaskHandler_001b4c(pc), a1     | +bc  aux handler: $1B4C
        jsr     0x4ae.l                        | +c0  scheduler_add(a1)
        lea.l   0x100260.l, a0                 | +c6
        move.l  #0x96840, (a0)                 | +cc
        jsr     0x5fe.l                        | +d2
        bra.w   Attract_WaitStateBackbone_00188A                   | +d8

                                              | ---- state=7 ($0019FE) ----
        move.b  #0x7, 0x106ece.l               | +dc
        lea.l   0x100260.l, a0                 | +e4
        move.l  #0x9698a, (a0)                 | +ea
        jsr     0x5fe.l                        | +f0
        bra.w   Attract_WaitStateBackbone_00188A                   | +f6

                                              | ---- state=8 ($001A1C) ----
        move.b  #0x8, 0x106ece.l               | +fa
        lea.l   0x100260.l, a0                 | +102
        move.l  #0x96906, (a0)                 | +108
        jsr     0x5fe.l                        | +10e
        bra.w   Attract_WaitStateBackbone_00188A                   | +114

                                              | ---- state=9 ($001A3A) ----
        move.b  #0x9, 0x106ece.l               | +118
        lea.l   0x100260.l, a0                 | +120
        move.l  #0x96906, (a0)                 | +126
        jsr     0x5fe.l                        | +12c
        bra.w   Attract_WaitStateBackbone_00188A                   | +132

                                              | ---- state=A ($001A58) - solo publica opcode ----
        move.b  #0xa, 0x106ece.l               | +136
        bra.w   Attract_WaitStateBackbone_00188A                   | +13e

                                              | ---- rama post-Start ($001A64) ----
                                              | Alcanzada por la tabla $000C9A / $000CBA / ...
                                              | como puntero long "$001A64".
        move.b  #0x2, 0x45(a6)                 | +142  timer_b = 2
        move.b  #0x2, 0x44(a6)                 | +148  timer_a = 2
        bsr.w   Sub_00001DB8                   | +14e  probe/setup
        lea.l   0x10fdb6.l, a4                 | +152  a4 = &INPUT_MASK
        move.b  (a4)+, d0                      | +158  d0 = mask_p1
        move.b  (a4),  d1                      | +15a  d1 = mask_p2
        cmpi.b  #0x2, d0                       | +15c  if (mask_p1 == 2)
        beq.w   .Ltwo_path                     | +160    goto rts-then-fall
        cmpi.b  #0x2, d1                       | +164  if (mask_p2 == 2)
        beq.w   .Ltwo_path                     | +168
        cmpi.b  #0x1, d0                       | +16c  if (mask_p1 == 1)
        beq.w   .Lone_path                     | +170    goto jsr+fall
        cmpi.b  #0x1, d1                       | +174  if (mask_p2 == 1)
        beq.w   .Lone_path                     | +178
                                              |
                                              | ---- default: ambos == 0 ----
        bsr.w   Sub_00001E0A                   | +17c
        move.l  #0xe42, 0x70(a6)               | +180  publish handler ptr $E42
        bra.w   Sub_00000FC6                   | +188  tail al scheduler

.Ltwo_path:                                    | $001AAE
        jsr     0x981fc.l                      | +18c  Copy2Bytes_10FDB6to10E3A0
        rts                                    | +192  (fin del dispatcher aqui)

.Lone_path:                                    | $001AB6 (etiqueta pero NO
                                              |          rts propio: cae en
                                              |          la siguiente funcion
                                              |          $001AB6 del cluster,
                                              |          que EE-batch2 cubre)

        .size   Dispatcher_ModeTable_001922, .-Dispatcher_ModeTable_001922
