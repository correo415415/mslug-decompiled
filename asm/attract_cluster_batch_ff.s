| ============================================================================
|  Metal Slug 1 - asm/attract_cluster_batch_ff.s
|  ----------------------------------------------------------------------------
|  Wave FF batch 1 - cluster attract handlers, batch consolidado
|
|  Cierra el cluster attract/title $001744..$001AF7 iniciado en Wave EE.
|  Ocho handlers agrupados en un solo fichero (todos comparten firma
|  arquitectonica: son handlers registrados en la tabla de descriptores
|  $000BA2..$000E8A del subsistema attract).
|
|  Bloques cubiertos:
|      F2  Attract_InitBIOS_001744         @ $001744  132 B  (2 entradas)
|      F3  Attract_InitTaskAdd_3DBC8       @ $0017C8   30 B
|      F4  Attract_InitShow27_TaskAdd      @ $0017E6   44 B
|      F5  Attract_SetTimers2_And_Gate21   @ $001812   26 B  (fall a F6)
|      F6  Attract_TailChain_1CD4_1DA4     @ $00182C   12 B
|      F7  Attract_SoftReset_10FDAF        @ $001838   14 B  (thunk jmp $85E)
|      F8  Attract_DoubleCheck_400_Publish @ $001846   68 B
|      F9  Attract_WaitStateBackbone       @ $00188A   80 B  (backbone comun)
|      F12 Attract_PostStart_Cleanup       @ $001AB6   66 B
|
|  Cada bloque publica una firma C conceptual y notas forenses propias
|  al inicio de su seccion (dentro del mismo .s).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

| ============================================================================
|  F2: Attract_InitBIOS_001744  @ $001744  (132 bytes, 2 entradas)
|
|  Handler multi-entry. Referenciado desde tabla $000E52/$000E82 en su
|  primera entrada, y $000E56/$000E86/$120BA0 en la segunda ($00179A).
|
|  Firma C conceptual:
|
|      /* Entrada primaria: post-BIOS init de la fase attract. Ejecuta
|       * BIOS_FIX_CLEAR ($C004C2) + palette-fade ($52712), agrega dos
|       * task handlers ($46682 siempre, $59B6A solo si $106ED0 >= 6),
|       * corre init pesado $46AC6, sella tres flags con $FF y llama
|       * probe $1E0A + tail-scheduler.
|       *
|       * Entrada secundaria ($00179A): probe path. bsr Sub_00001DB8;
|       * si $106ED2 != 0 => rts inmediato ($0017C6). Si == 0, hace
|       * jsr $5B6 + lea $9B4(pc); jsr $2B58 + jsr $212E + bra $FE0.
|       * El rts final ($0017C6) NO es funcion aparte: es la salida
|       * de la rama "already-armed" del probe. */
|      void Attract_InitBIOS(void);      // entrada primaria
|      void Attract_InitBIOS_probe(void);// entrada secundaria ($00179A)
|
|  Notas forenses:
|    1. Dos entradas en la MISMA funcion son un patron hand-coded: la
|       tabla-macro apunta a offsets distintos dentro del mismo bloque
|       de code y la funcion no re-inicializa estado. GCC produce una
|       funcion por entry point.
|    2. `lea.l $9b4(pc), a0` en $0017B2 codificado con disp16 aunque
|       cabria en disp8 (distancia = -$D6B, needs 16). Consistente con
|       la ausencia sistematica de disp8 en el cluster.
|    3. El rts en $0017C6 corta la funcion en dos "sub-funciones"
|       logicas: probe (17c6) y init (arriba). Como escaner de callees
|       ninguna las trata como funciones distintas.
| ============================================================================

        .globl  Attract_InitBIOS_001744
        .type   Attract_InitBIOS_001744, @function
        .section .text.Attract_InitBIOS_001744, "ax", @progbits

Attract_InitBIOS_001744:
        jsr     0xc004c2.l                     | +00  BIOS_FIX_CLEAR
        jsr     0x52712.l                      | +06  ThunkTarget_052712 (palette fade)
        lea.l   0x46682.l, a1                  | +0c  a1 = TaskHandler_00046682
        jsr     0x4ae.l                        | +12  scheduler_add(a1)
        cmpi.b  #0x6, 0x106ed0.l               | +18  if (state_gate < 6)
        bcs.w   .Lff2_skip_aux                 | +20    skip second task-add
        lea.l   0x59b6a.l, a1                  | +24  a1 = TaskHandler_00059B6A
        jsr     0x4ae.l                        | +2a  scheduler_add(a1)
.Lff2_skip_aux:                                | $001774
        jsr     0x46ac6.l                      | +30  Sub_00046AC6 (init pesado)
        move.b  #0xff, 0x106ece.l              | +36  seal opcode
        move.b  #0xff, 0x106ecf.l              | +3e  seal opcode2
        move.b  #0xff, 0x106ed2.l              | +46  seal pending flag
        bsr.w   Sub_00001E0A                   | +4e  probe/setup
        bra.w   Sub_00000FE0                   | +52  tail al scheduler

                                              | ---- entrada secundaria ($00179A) ----
.Lff2_probe_entry:                             | $00179A
        bsr.w   Sub_00001DB8                   | +56  probe/setup
        tst.b   0x106ed2.l                     | +5a  if (pending == 0)
        beq.w   .Lff2_do_init                  | +60    goto init
        bra.w   .Lff2_rts                      | +64  else rts inmediato
.Lff2_do_init:                                 | $0017AC
        jsr     0x5b6.l                        | +68  FUN_000005B6
        lea.l   .Lff2_tpl_9b4(pc), a0          | +6e  a0 = template ptr $000009B4
        jsr     0x2b58.l                       | +72  Sub_00002B58(a0)
        jsr     0x212e.l                       | +78  InputQueue_InitAndPushOp4
        bra.w   Sub_00000FE0                   | +7e  tail al scheduler
.Lff2_rts:                                     | $0017C6
        rts                                    | +82

        .equ    .Lff2_tpl_9b4, ScriptSlotPairTable_0009B4

        .size   Attract_InitBIOS_001744, .-Attract_InitBIOS_001744


| ============================================================================
|  F3: Attract_InitTaskAdd_3DBC8_0017C8  @ $0017C8  (30 bytes)
|
|  Handler minimo: seala $106ED2=$FF, agrega task $3DBC8, corre init
|  pesado $46AC6, y tail-call al scheduler. Referenciado 12 veces desde
|  la tabla de descriptores (uno de los handlers mas usados del cluster).
|
|      void Attract_InitTaskAdd_3DBC8(void);
| ============================================================================

        .globl  Attract_InitTaskAdd_3DBC8_0017C8
        .type   Attract_InitTaskAdd_3DBC8_0017C8, @function
        .section .text.Attract_InitTaskAdd_3DBC8_0017C8, "ax", @progbits

Attract_InitTaskAdd_3DBC8_0017C8:
        move.b  #0xff, 0x106ed2.l              | +00  seal pending flag
        lea.l   0x3dbc8.l, a1                  | +08  a1 = TaskHandler_0003DBC8
        jsr     0x4ae.l                        | +0e  scheduler_add(a1)
        jsr     0x46ac6.l                      | +14  Sub_00046AC6 (init pesado)
        bra.w   Sub_00000FE0                   | +1a  tail al scheduler

        .size   Attract_InitTaskAdd_3DBC8_0017C8, .-Attract_InitTaskAdd_3DBC8_0017C8


| ============================================================================
|  F4: Attract_InitShow27_TaskAdd_0017E6  @ $0017E6  (44 bytes)
|
|  Handler que dispara "guarded input show" (opcode $27 en $2352 =
|  InputGuardCall219c, Wave A#4), agrega task $46608, marca flag $21(a6)
|  con $FF y tail-call al scheduler.
|
|      void Attract_InitShow27_TaskAdd(void);
|
|  Nota forense: el `move.w #$27, d0` seguido de `jsr $2352` fija arg=0x27
|  como codigo de operacion. En Wave A ya se sabia que $2352 lee d0 como
|  offset con clamp; aqui vemos su primer caller no-thunk del proyecto.
| ============================================================================

        .globl  Attract_InitShow27_TaskAdd_0017E6
        .type   Attract_InitShow27_TaskAdd_0017E6, @function
        .section .text.Attract_InitShow27_TaskAdd_0017E6, "ax", @progbits

Attract_InitShow27_TaskAdd_0017E6:
        bsr.w   Sub_00001E0A                   | +00  probe/setup
        move.b  #0xff, 0x106ed2.l              | +04  seal pending flag
        move.w  #0x27, d0                      | +0c  d0 = opcode 0x27
        jsr     0x2352.l                       | +10  InputGuardCall219c(d0)
        lea.l   0x46608.l, a1                  | +16  a1 = TaskHandler_00046608
        jsr     0x4ae.l                        | +1c  scheduler_add(a1)
        move.b  #0xff, 0x21(a6)                | +22  self->flag_21 = $FF
        bra.w   Sub_00000FE0                   | +28  tail al scheduler

        .size   Attract_InitShow27_TaskAdd_0017E6, .-Attract_InitShow27_TaskAdd_0017E6


| ============================================================================
|  F5: Attract_SetTimers2_And_Gate21_001812  @ $001812  (26 bytes)
|
|  Handler compacto: probe + arma timers ($45=$2, $44=$2) + gate por
|  flag $21(a6). Si $21(a6) != 0 => rts inmediato. Si == 0 => fall-through
|  a F6 (Attract_TailChain_1CD4_1DA4_00182C).
|
|      void Attract_SetTimers2_And_Gate21(void);
|
|  Nota forense: el `rts` en $00182A no cierra la funcion: es la salida
|  short-circuit del gate. La funcion continua en F6 por fall-through.
|  Ninguna referencia externa apunta a $00182C (verificado en Wave EE).
| ============================================================================

        .globl  Attract_SetTimers2_And_Gate21_001812
        .type   Attract_SetTimers2_And_Gate21_001812, @function
        .section .text.Attract_SetTimers2_And_Gate21_001812, "ax", @progbits

Attract_SetTimers2_And_Gate21_001812:
        bsr.w   Sub_00001DB8                   | +00  probe/setup
        move.b  #0x2, 0x45(a6)                 | +04  timer_b = 2
        move.b  #0x2, 0x44(a6)                 | +0a  timer_a = 2
        tst.b   0x21(a6)                       | +10  if (self->flag_21 == 0)
        beq.w   .Lff5_fall                     | +14    fall-through a F6
        rts                                    | +18  else rts inmediato
.Lff5_fall:                                    | $00182C - fall-through a F6

        .size   Attract_SetTimers2_And_Gate21_001812, .-Attract_SetTimers2_And_Gate21_001812


| ============================================================================
|  F6: Attract_TailChain_1CD4_1DA4_00182C  @ $00182C  (12 bytes)
|
|  Continuacion por fall-through de F5. Encadena dos subrutinas
|  ($1CD4 y $1DA4, ambas PC-rel) y tail-call al scheduler.
|
|      void Attract_TailChain_1CD4_1DA4(void);
|
|  Nota: aunque F6 no tiene entrada externa registrada, se emite como
|  simbolo global independiente porque el registro maestro necesita
|  cubrir estos 12 B tambien byte-a-byte, y separar F6 permite que el
|  matcher los contabilice como funcion aparte (evitando extender F5).
| ============================================================================

        .globl  Attract_TailChain_1CD4_1DA4_00182C
        .type   Attract_TailChain_1CD4_1DA4_00182C, @function
        .section .text.Attract_TailChain_1CD4_1DA4_00182C, "ax", @progbits

Attract_TailChain_1CD4_1DA4_00182C:
        bsr.w   TaskList_ChangeAndRunEight_001CD4           | +00  jsr $1CD4 (probe)
        bsr.w   Sub_00001DA4                   | +04  bsr Sub_00001DA4
        bra.w   Sub_00000FE0                   | +08  tail al scheduler

        .size   Attract_TailChain_1CD4_1DA4_00182C, .-Attract_TailChain_1CD4_1DA4_00182C


| ============================================================================
|  F7: Attract_SoftReset_10FDAF_001838  @ $001838  (14 bytes)
|
|  Thunk tail-call: publica $01 en flag $10FDAF, hace `jmp $85E.l`
|  (SoftReset del BIOS). Sin rts propio.
|
|      void Attract_SoftReset_10FDAF(void);
|
|  Referenciado desde tabla $000BB6, $000E6A, $000E8A. Tercera vez que
|  vemos el patron "publish + jmp abs.l" en el cluster (habia dos en
|  Wave EE tail-jumps).
| ============================================================================

        .globl  Attract_SoftReset_10FDAF_001838
        .type   Attract_SoftReset_10FDAF_001838, @function
        .section .text.Attract_SoftReset_10FDAF_001838, "ax", @progbits

Attract_SoftReset_10FDAF_001838:
        move.b  #0x1, 0x10fdaf.l               | +00  publish reboot flag
        jmp     SoftReset_085E                 | +08  tail-call SoftReset ($85E)

        .size   Attract_SoftReset_10FDAF_001838, .-Attract_SoftReset_10FDAF_001838


| ============================================================================
|  F8: Attract_DoubleCheck_400_Publish_001846  @ $001846  (68 bytes)
|
|  Handler mas referenciado del cluster (12 entradas de tabla). Verifica
|  dos slots ($100300 y $1003A0), y por cada uno cuyo valor sea $400
|  publica un handler distinto ($2575C o $25766) via jsr $5FE.
|
|      void Attract_DoubleCheck_400_Publish(void);
|
|  Firma:
|      if (*(long*)$100300 == $400) {
|          *(long*)$100300 = $2575C;
|          scheduler_publish();       // jsr $5FE
|      }
|      if (*(long*)$1003A0 == $400) {
|          *(long*)$1003A0 = $25766;
|          scheduler_publish();
|      }
|      bra.w scheduler_tail;
|
|  Nota forense: los `cmpi.l #$400, addr.l` son 10 B cada uno (0CB9 + 4B
|  imm + 4B addr). Patron hand-coded: GCC habria usado `movea.l addr, a0;
|  cmp.l #$400, (a0)` (mas corto y mas rapido en 68000).
| ============================================================================

        .globl  Attract_DoubleCheck_400_Publish_001846
        .type   Attract_DoubleCheck_400_Publish_001846, @function
        .section .text.Attract_DoubleCheck_400_Publish_001846, "ax", @progbits

Attract_DoubleCheck_400_Publish_001846:
        cmpi.l  #0x400, 0x100300.l             | +00  if (slot_P1 == $400)
        bne.w   .Lff8_p2                       | +0a    skip P1 publish
        lea.l   0x100300.l, a0                 | +0e
        move.l  #0x2575c, (a0)                 | +14  slot_P1 = handler $2575C
        jsr     0x5fe.l                        | +1a  scheduler_publish
.Lff8_p2:                                      | $001866
        cmpi.l  #0x400, 0x1003a0.l             | +20  if (slot_P2 == $400)
        bne.w   .Lff8_tail                     | +2a    skip P2 publish
        lea.l   0x1003a0.l, a0                 | +2e
        move.l  #0x25766, (a0)                 | +34  slot_P2 = handler $25766
        jsr     0x5fe.l                        | +3a  scheduler_publish
.Lff8_tail:                                    | $001886
        bra.w   Sub_00000FE0                   | +40  tail al scheduler

        .size   Attract_DoubleCheck_400_Publish_001846, .-Attract_DoubleCheck_400_Publish_001846


| ============================================================================
|  F9: Attract_WaitStateBackbone_00188A  @ $00188A  (80 bytes efectivos)
|
|  BACKBONE COMUN del cluster attract. Target de todos los `bra.w $188A`
|  emitidos desde los 11 estados de Dispatcher_ModeTable_001922 (EE#3),
|  y desde el path init de F2. Ejecuta el "wait state loop":
|
|      1. jsr $212E             InputQueue_InitAndPushOp4
|      2. move.b $106ED0 -> $106ECF   (state_snapshot = current_state)
|      3. jsr $52712            palette fade (ThunkTarget_052712)
|      4. move.w #$1, d0
|      5. jsr $523B2            ThunkTarget_0523b2(1)  publish
|      6. lea $1B4C(pc); jsr $4AE   scheduler_add(TaskHandler_001B4C)
|      7. jsr $1DCC(pc)         PcThunkTarget_001DCC
|      8. seala $106ED2 y $106ED3 con $FF
|      9. lea $1C44(pc); jsr $4AE   scheduler_add(handler_001C44)
|     10. jsr $C004C2            BIOS_FIX_CLEAR
|     11. jsr $46AC6.l           Sub_00046AC6 (init pesado)  <-- ultimo op
|
|  La funcion NO tiene rts propio: cae exactamente en Init_EntitySpawn_0018DA
|  (Wave EE#2) por fall-through. Es decir, el ultimo `jsr $46AC6.l` (6 B)
|  termina en $0018DE, y el bsr.w de EE#2 empieza ahi mismo. Esto convierte
|  EE#2 en la "continuacion natural" de F9 cuando se llega por fall-through
|  desde el backbone, aunque la tabla de descriptores lo apunte como funcion
|  independiente.
|
|      void Attract_WaitStateBackbone(void); // no rts, fall-through a EE#2
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Fall-through a EE#2 sin rts significa que el escaner de fronteras
|       tiene que reconocer que la ultima instruccion del backbone es un
|       jsr abs.l y NO un bra/jmp/rts. Idioma hand-coded para ahorrar los
|       4 bytes del bra.w a $18DA.
|    2. Dos `lea (pc,d16), a1` (a $1B4C y a $1C44) en la MISMA funcion,
|       ambos con desplazamientos positivos grandes. GCC habria emitido
|       una direccion absoluta larga si -mpcrel estuviera desactivado,
|       o ambos PC-rel si activo. Aqui coexisten porque el codigo original
|       usa la forma corta cuando cabe (16-bit signed disp).
|    3. La secuencia `move.b addr1.l, addr2.l` en $001890 es un memory-to-
|       memory move (10 B: 13F9 + 4B src + 4B dst) que GCC no emite jamas
|       (siempre pasa por dX). Idioma tipico de codigo escrito a mano en
|       68000.
| ============================================================================

        .globl  Attract_WaitStateBackbone_00188A
        .type   Attract_WaitStateBackbone_00188A, @function
        .section .text.Attract_WaitStateBackbone_00188A, "ax", @progbits

Attract_WaitStateBackbone_00188A:
        jsr     0x212e.l                       | +00  InputQueue_InitAndPushOp4
        move.b  0x106ed0.l, 0x106ecf.l         | +06  snapshot state
        jsr     0x52712.l                      | +10  palette fade
        move.w  #0x1, d0                       | +16  d0 = 1
        jsr     0x523b2.l                      | +1a  ThunkTarget_0523b2(1)
        lea.l   TaskHandler_001b4c(pc), a1     | +20  a1 = TaskHandler_001B4C
        jsr     0x4ae.l                        | +24  scheduler_add(a1)
        jsr     PcThunkTarget_001DCC(pc)       | +2a  PcThunkTarget_001DCC
        move.b  #0xff, 0x106ed2.l              | +2e  seal
        move.b  #0xff, 0x106ed3.l              | +36  seal
        lea.l   .Lff9_tpl_1c44(pc), a1         | +3e  a1 = handler $1C44
        jsr     0x4ae.l                        | +42  scheduler_add(a1)
        jsr     0xc004c2.l                     | +48  BIOS_FIX_CLEAR
                                              |
                                              | ---- opcode literal del ultimo jsr ----
                                              | El jsr $46AC6.l final del backbone son 6 B
                                              | fisicos ($0018D8..$0018DD = 4EB9 0004 6AC6),
                                              | pero por SOLAPE de instruccion con EE#2
                                              | (Init_EntitySpawn_0018DA arranca en $0018DA
                                              | segun descriptor), solo emitimos aqui los
                                              | 2 bytes del OPCODE. Los 4 bytes del OPERANDO
                                              | ($0018DA..$0018DD = 0004 6AC6) se emiten
                                              | como .byte al inicio de EE#2 y forman
                                              | juntos la instruccion completa en runtime.
                                              | Fall-through: $0018DE = bsr.w del EE#2 (+4).
        .byte   0x4e, 0xb9                     | +4e  opcode jsr abs.l (sin operando)

        .equ    .Lff9_tpl_1c44, TaskHandler_001C44

        .size   Attract_WaitStateBackbone_00188A, .-Attract_WaitStateBackbone_00188A


| ============================================================================
|  F12: Attract_PostStart_Cleanup_001AB6  @ $001AB6  (66 bytes)
|
|  Continuacion de la rama .Lone_path de Dispatcher_ModeTable_001922
|  (EE#3), alcanzada por fall-through cuando P1 o P2 pulsaron Start con
|  mask = 1. Ejecuta cleanup post-Start:
|
|      1. jsr $981FC             Copy2Bytes_10FDB6to10E3A0
|      2. probe bit-0 de $100001
|         si == 0 => skip
|         si != 0 => jsr $5D288; si C=1 => lea $F76(pc), a1; move.l a1, (a6)
|                                (repatch del handler del task actual)
|      3. si $106ED6 != 0 => clear $106ED6; rts
|      4. si $106ED2 == 0 => tail-call al scheduler ($FE0)
|      5. else => clr $106ED6; rts
|
|  Referenciada por fall-through desde .Lone_path (EE#3, $001AB6 no esta
|  en la tabla de descriptores).
|
|      void Attract_PostStart_Cleanup(void);
|
|  Notas forenses:
|    1. `move.l a1, (a6)` en $001AD6 sobreescribe DIRECTAMENTE el handler
|       del task node actual (a6 apunta al task en el scheduler). GCC
|       nunca emite este patron - requiere conocimiento explicito de la
|       convencion "a6 = current_task_ptr" del scheduler. Confirma la
|       hipotesis arquitectural.
|    2. `beq.w $FE0` con offset $F4F2 (=-$0B0E) en $001AEC es un tail-jump
|       muy largo (fuera del rango de bra.s), pero cabe en disp16. GCC
|       elegiria automaticamente jmp abs.l si es mas eficiente (aqui no
|       lo es, quedan 4 B iguales).
|    3. `move.l a1, (a6)` intercalado con `bra.w $1ADC` (+2 B) sugiere
|       que el codigo original tenia un `bra.b .+2` (2 B, no 4) que el
|       ensamblador convirtio a bra.w por politica de sizing (o el autor
|       lo dejo asi para evitar recompilar el resto del bloque).
| ============================================================================

        .globl  Attract_PostStart_Cleanup_001AB6
        .type   Attract_PostStart_Cleanup_001AB6, @function
        .section .text.Attract_PostStart_Cleanup_001AB6, "ax", @progbits

Attract_PostStart_Cleanup_001AB6:
        jsr     0x981fc.l                      | +00  Copy2Bytes_10FDB6to10E3A0
        btst.b  #0x0, 0x100001.l               | +06  test bit-0 de $100001
        beq.w   .Lff12_check_ed6               | +0e    if (bit-0 == 0) skip probe
        jsr     0x5d288.l                      | +12  Sub_0005D288 (probe)
        bcc.w   .Lff12_bra_short               | +18    if (C == 0) skip repatch
        lea.l   .Lff12_pc_f76(pc), a1          | +1c  a1 = handler $F76 (PC-rel)
        move.l  a1, (a6)                       | +20  self->handler = a1
.Lff12_bra_short:                              | $001AD8
        bra.w   .Lff12_check_ed6               | +22  (bra.w a +2 B, patron)
.Lff12_check_ed6:                              | $001ADC
        tst.b   0x106ed6.l                     | +26  if ($106ED6 != 0)
        bne.w   .Lff12_clr_and_rts             | +2c    goto clear+rts
        tst.b   0x106ed2.l                     | +30  if ($106ED2 == 0)
        beq.w   Sub_00000FE0                   | +36    tail al scheduler
.Lff12_clr_and_rts:                            | $001AF0
        clr.b   0x106ed6.l                     | +3a  $106ED6 = 0
        rts                                    | +40

        .equ    .Lff12_pc_f76, PcThunkTarget_000F76

        .size   Attract_PostStart_Cleanup_001AB6, .-Attract_PostStart_Cleanup_001AB6
