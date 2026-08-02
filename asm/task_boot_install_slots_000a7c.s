| ============================================================================
|  Metal Slug 1 - asm/task_boot_install_slots_000a7c.s
|  ----------------------------------------------------------------------------
|  Wave SS#4 + SS#5 - tabla de pares (id, script) + instalador de tasks del
|  arranque.  Completa el mapa del bloque de boot $0009B4..$000B8A, que era
|  el ultimo hueco grande de la zona $000xxx tras Wave MM (scheduler
|  bootstrap + super-tabla dispatch).
|
|    ScriptSlotPairTable_0009B4   @ $0009B4  (200 B, datos-en-.text)
|    TaskSlots_BootInstall_000A7C @ $000A7C  (270 B, sin caller directo)
|
|  ---------- SS#4: ScriptSlotPairTable_0009B4 -------------------------------
|
|  Tabla de datos referenciada por `lea $9b4(pc), a0` desde TRES handlers
|  de la super-tabla dispatch ($0011F6, $0017B2, $001DAE), siempre seguida
|  de `jsr $2b58.l` (Sub_00002B58, aun no matcheado).  El applicator $2B58
|  interpreta la tabla como pares de words:
|
|      word 0:  id     - solo cuenta el byte bajo (id &= 0xFF); el byte
|                        alto es $08 en todas las entradas (tag de clase de
|                        comando para un dispatcher externo, hipotesis) o
|                        $00/$01 en las entradas especiales.
|                        id == $FF   -> salta el range-check (entrada especial)
|                        id  > $74   -> aborta el procesado de la tabla
|      word 1:  script - indice de registro de 64 B sobre la base ROM
|                        $1CE00 (script/anim data).  $FFFF = fin de tabla.
|
|      slot = $1082C8 + id*32          (array RAM de slots de 32 B)
|      slot->+2 (long) = $1CE00 + script*64
|      slot->+6 (word) = 1
|      slot->+0 (byte) = byte bajo del id;  luego |= 1
|
|  Interpretacion tentativa: asignacion masiva de scripts de animacion (o
|  de secuencia) a los slots del subsistema $1082C8 durante el arranque y
|  el attract.  DOS sub-tablas con terminador $FFFF cada una:
|      sub-tabla 1: ids $00..$0F + entrada especial ($FF, 0)   (16+1 pares)
|      sub-tabla 2: ids $00..$1F, scripts $0103..$011C + extras (32 pares)
|  Los consumidores pasan a0 = $9B4 (sub-tabla 1); la sub-tabla 2 se
|  procesa por continuacion natural del applicator, que rearma a0 tras el
|  primer $FFFF (mismo idioma auto-salto de la super-tabla dispatch MM#1),
|  o se referencia por a0 = $9FA que aun no hemos localizado.
|
|  ---------- SS#5: TaskSlots_BootInstall_000A7C -----------------------------
|
|  Handler de task de arranque.  SIN caller directo por jsr/bra en todo el
|  ROM: se alcanza via la tabla de pares (TCB, handler) en $178000, que
|  contiene la entrada  ($100120 -> $000A7C)  - es decir, se instala como
|  handler del TCB $100120 y el scheduler lo invoca con jmp (a0) (mismo
|  mecanismo threaded documentado en Wave MM#1).
|
|  Firma C conceptual:
|
|    /* Ejecutado una vez al arrancar el sistema de tasks.  Instala el
|       handler inicial de los 12 TCBs estaticos de $100xxx, arranca 3 de
|       ellos inmediatamente y enlaza los TCBs "partner" de los jugadores. */
|    void TaskSlots_BootInstall(void)
|    {
|        /* Task_InstallHandler_0000050E: TCB->handler = a1 (via $4C6) y
|           bset #0, TCB->+0x12 (flag "activo").  */
|        Task_InstallHandler($1008A0, RtsStub_0400);     // idle
|        Task_InstallHandler($100800, RtsStub_0400);     // idle
|        Task_InstallHandler($1006C0, JsrPcThunk_03240c);
|        Task_InstallHandler($100440, RtsStub_0400);
|        Task_InstallHandler($1004E0, RtsStub_0400);
|        Task_InstallHandler($100620, JsrPcThunk_032406);
|        Task_InstallHandler($100580, RtsStub_0400);
|        Task_InstallHandler($100760, JsrPcThunk_029582);
|        Task_InstallHandler($100300, RtsStub_0400);
|        Task_InstallHandler($1003A0, RtsStub_0400);
|        Task_InstallHandler($100260, RtsStub_0400);
|        Task_InstallHandler($1001C0, SchedulerBootstrap_Boot_000E8E);
|        Task_RunHandler($100440);                       // arranque inmediato
|        Task_RunHandler($1004E0);
|        Task_RunHandler($100580);
|        *(Task**)$10044C = (Task*)$100300;   // $100440->partner_0C
|        *(Task**)$1004EC = (Task*)$1003A0;   // $1004E0->partner_0C
|        // fall-through en SetTaskHandler_000b8a (matcheado, Wave H)
|    }
|
|  Notas forenses:
|    1. El unico handler instalado por PC-rel (`lea $e8e(pc), a1`) es el
|       SchedulerBootstrap_Boot_000E8E de Wave MM#1 - cierra el circulo:
|       ESTE es el codigo que lo instala en el TCB $1001C0.
|    2. Los enlaces $10044C/$1004EC = TCB->+0xC de $100440/$1004E0
|       apuntan a $100300/$1003A0: pares "task de jugador + partner".
|       Confirma +0xC como campo "partner/companion" del TCB.
|    3. Sin rts propio: cae por fall-through en SetTaskHandler_000b8a
|       (8 B, Wave H) - 9a aparicion del idioma fall-through, 2a hacia
|       una funcion matcheada de una wave C (tras MM#1).
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text

| ----------------------------------------------------------------------------
|  SS#4  ScriptSlotPairTable_0009B4  ($0009B4..$000A7B, 200 B, datos)
| ----------------------------------------------------------------------------
        .globl  ScriptSlotPairTable_0009B4
        .type   ScriptSlotPairTable_0009B4, @object
        .section .text.ScriptSlotPairTable_0009B4, "ax", @progbits

ScriptSlotPairTable_0009B4:
        | -------- sub-tabla 1: ids $00..$0F ---------------------------------
        .short  0x0800, 0x0193                  | slot $00 <- script $193
        .short  0x0801, 0x0000                  | slot $01 <- script $000
        .short  0x0802, 0x0190                  | slot $02 <- script $190
        .short  0x0803, 0x0191                  | slot $03 <- script $191
        .short  0x0804, 0x0192                  | slot $04 <- script $192
        .short  0x0805, 0x0194                  | slot $05 <- script $194
        .short  0x0806, 0x0195                  | slot $06 <- script $195
        .short  0x0807, 0x01a7                  | slot $07 <- script $1A7
        .short  0x0808, 0x0197                  | slot $08 <- script $197
        .short  0x0809, 0x0190                  | slot $09 <- script $190 (repite)
        .short  0x080a, 0x0000                  | slot $0A <- script $000
        .short  0x080b, 0x01a5                  | slot $0B <- script $1A5
        .short  0x080c, 0x01a6                  | slot $0C <- script $1A6
        .short  0x080d, 0x01a8                  | slot $0D <- script $1A8
        .short  0x080e, 0x01a9                  | slot $0E <- script $1A9
        .short  0x080f, 0x01aa                  | slot $0F <- script $1AA
        .short  0x00ff, 0x0000                  | entrada especial id=$FF
        .short  0xffff                          | fin sub-tabla 1
        | -------- sub-tabla 2: ids $00..$1F ---------------------------------
        .short  0x0800, 0x0103                  | slot $00 <- script $103
        .short  0x0801, 0x0104                  | slot $01 <- script $104
        .short  0x0802, 0x0105                  | slot $02 <- script $105
        .short  0x0803, 0x0106                  | slot $03 <- script $106
        .short  0x0804, 0x0107                  | slot $04 <- script $107
        .short  0x0805, 0x0108                  | slot $05 <- script $108
        .short  0x0806, 0x0109                  | slot $06 <- script $109
        .short  0x0807, 0x010a                  | slot $07 <- script $10A
        .short  0x0808, 0x010b                  | slot $08 <- script $10B
        .short  0x0809, 0x010c                  | slot $09 <- script $10C
        .short  0x080a, 0x010d                  | slot $0A <- script $10D
        .short  0x080b, 0x010e                  | slot $0B <- script $10E
        .short  0x080c, 0x010f                  | slot $0C <- script $10F
        .short  0x080d, 0x0110                  | slot $0D <- script $110
        .short  0x080e, 0x0111                  | slot $0E <- script $111
        .short  0x080f, 0x0112                  | slot $0F <- script $112
        .short  0x0810, 0x0113                  | slot $10 <- script $113
        .short  0x0811, 0x0114                  | slot $11 <- script $114
        .short  0x0812, 0x0115                  | slot $12 <- script $115
        .short  0x0813, 0x0116                  | slot $13 <- script $116
        .short  0x0814, 0x0117                  | slot $14 <- script $117
        .short  0x0815, 0x0118                  | slot $15 <- script $118
        .short  0x0816, 0x0119                  | slot $16 <- script $119
        .short  0x0817, 0x011a                  | slot $17 <- script $11A
        .short  0x0818, 0x011b                  | slot $18 <- script $11B
        .short  0x0819, 0x011c                  | slot $19 <- script $11C
        .short  0x081a, 0x018c                  | slot $1A <- script $18C
        .short  0x081b, 0x01bb                  | slot $1B <- script $1BB
        .short  0x081c, 0x01bc                  | slot $1C <- script $1BC
        .short  0x081d, 0x0001                  | slot $1D <- script $001
        .short  0x081e, 0x01d3                  | slot $1E <- script $1D3
        .short  0x081f, 0x01a0                  | slot $1F <- script $1A0
        .short  0xffff                          | fin sub-tabla 2

| ----------------------------------------------------------------------------
|  SS#5  TaskSlots_BootInstall_000A7C  ($000A7C..$000B89, 270 B)
|  Fall-through final en SetTaskHandler_000b8a (Wave H, matcheado).
| ----------------------------------------------------------------------------
        .globl  TaskSlots_BootInstall_000A7C
        .type   TaskSlots_BootInstall_000A7C, @function
        .section .text.TaskSlots_BootInstall_000A7C, "ax", @progbits

TaskSlots_BootInstall_000A7C:
        lea     0x1008a0.l, a0                  | +000  TCB $1008A0
        lea     RtsStub_0400, a1                | +006  handler idle (rts)
        jsr     Task_InstallHandler_0000050E    | +00c
        lea     0x100800.l, a0                  | +012  TCB $100800
        lea     RtsStub_0400, a1                | +018
        jsr     Task_InstallHandler_0000050E    | +01e
        lea     0x1006c0.l, a0                  | +024  TCB $1006C0
        lea     JsrPcThunk_03240c, a1           | +02a  handler real
        jsr     Task_InstallHandler_0000050E    | +030
        lea     0x100440.l, a0                  | +036  TCB $100440 (player 1)
        lea     RtsStub_0400, a1                | +03c
        jsr     Task_InstallHandler_0000050E    | +042
        lea     0x1004e0.l, a0                  | +048  TCB $1004E0 (player 2)
        lea     RtsStub_0400, a1                | +04e
        jsr     Task_InstallHandler_0000050E    | +054
        lea     0x100620.l, a0                  | +05a  TCB $100620
        lea     JsrPcThunk_032406, a1           | +060  handler real
        jsr     Task_InstallHandler_0000050E    | +066
        lea     0x100580.l, a0                  | +06c  TCB $100580
        lea     RtsStub_0400, a1                | +072
        jsr     Task_InstallHandler_0000050E    | +078
        lea     0x100760.l, a0                  | +07e  TCB $100760
        lea     JsrPcThunk_029582, a1           | +084  handler real
        jsr     Task_InstallHandler_0000050E    | +08a
        lea     0x100300.l, a0                  | +090  TCB $100300 (partner P1)
        lea     RtsStub_0400, a1                | +096
        jsr     Task_InstallHandler_0000050E    | +09c
        lea     0x1003a0.l, a0                  | +0a2  TCB $1003A0 (partner P2)
        lea     RtsStub_0400, a1                | +0a8
        jsr     Task_InstallHandler_0000050E    | +0ae
        lea     0x100260.l, a0                  | +0b4  TCB $100260
        lea     RtsStub_0400, a1                | +0ba
        jsr     Task_InstallHandler_0000050E    | +0c0
        lea     0x1001c0.l, a0                  | +0c6  TCB $1001C0 (scheduler!)
        lea     SchedulerBootstrap_Boot_000E8E(pc), a1  | +0cc  PC-rel (43fa 0344)
        jsr     Task_InstallHandler_0000050E    | +0d0
        lea     0x100440.l, a0                  | +0d6  arranque inmediato de
        jsr     Task_RunHandler_05FE            | +0dc  los tasks de jugador
        lea     0x1004e0.l, a0                  | +0e2
        jsr     Task_RunHandler_05FE            | +0e8
        lea     0x100580.l, a0                  | +0ee
        jsr     Task_RunHandler_05FE            | +0f4
        move.l  #0x100300, 0x10044c.l           | +0fa  $100440->partner_0C
        move.l  #0x1003a0, 0x1004ec.l           | +104  $1004E0->partner_0C
        | fall-through en SetTaskHandler_000b8a ($000B8A, Wave H)      | +10e
