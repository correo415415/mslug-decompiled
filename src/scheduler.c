/*
 * Metal Slug 1 — Scheduler de tasks ($0518..$0655, 318 bytes, 7 rutinas)
 * =======================================================================
 * Núcleo del sistema de tasks del juego. Un `Task` es un nodo de una
 * lista doblemente enlazada con la siguiente estructura mínima
 * (deducida del análisis del scheduler):
 *
 *   struct Task {
 *      void (*slot0)(void);      // +$00 : handler actual del task
 *      struct Task *prev;        // +$04 : anterior en la lista
 *      struct Task *next;        // +$08 : siguiente en la lista
 *      ...
 *      u16  field16_prio;        // +$10 : prioridad (comparada con byte)
 *      u16  field18_flags;       // +$12 : bit 0 = "dead", bit 1 = ???
 *      ...
 *      u32  field48;             // +$48 : marcador aux ("id"/token)
 *      ...
 *   };
 *
 * Cabeceras/centinelas conocidas:
 *   $106E80 = puntero raíz de la free list (RAM global).
 *   Slot0 con valor centinela $00000400 = "task muerto/reservado".
 *   Slot0 con valor centinela $FFFFFFFF = "task vacío/libre".
 *
 * Las 7 rutinas cubiertas aquí forman el `runtime` del cooperative
 * scheduler. Las mini-thunks (JmpToScheduler_*, Jsr5B6ThenJmpScheduler_*)
 * ya matched en waves anteriores son variantes del punto de entrada
 * $518 usadas como continuación tras un tick de task.
 *
 * NOTA DE CODEGEN: idéntica a irq_handlers.c — funciones emitidas como
 * asm top-level con `.section` propia porque GCC 13/m68k añade `rts`
 * implícitos e ignora el atributo `naked`.
 */

#include "mslug.h"

extern void FUN_00013600(void);   /* $13600 — sanity/debug check llamado al inicio */

/* ---------------------------------------------------------------------
 * Scheduler_Main_0518 ($0518, 46 bytes)
 * ---------------------------------------------------------------------
 * Punto de entrada principal. Marca "no dirty" en flags, instala como
 * "punto de retorno" la siguiente instrucción ($52A), avanza al siguiente
 * task, compara prioridades y decide:
 *   - si nuevo.prio > actual.prio -> jmp $06CA (rearranca ejecución)
 *   - si no -> Task_UnlinkAlive($546) + jmp $06E2 (avanza main loop)
 *
 * Bytes originales:
 *   $0518: 4EB9 0001 3600     jsr    FUN_00013600
 *   $051E: 08AE 0001 0012     bclr   #1, 18(fp)
 *   $0524: 43FA 0004          lea    (pc+4).w, a1        ; a1 = &$52A
 *   $0528: 2C89               move.l a1, (fp)           ; fp->slot0 = a1
 *   $052A: 226E 0008          movea.l 8(fp), a1         ; a1 = fp->next
 *   $052E: 102E 0010          move.b 16(fp), d0
 *   $0532: B029 0010          cmp.b  16(a1), d0
 *   $0536: 6500 000A          bcs.w  $0542
 *   $053A: 4EBA 000A          jsr    (pc+0x0A).w        ; -> $546 UnlinkAlive
 *   $053E: 4EFA 01A2          jmp    (pc+$1A2).w        ; -> $06E2 (main loop)
 *   $0542: 4EFA 0186          jmp    (pc+$186).w        ; -> $06CA (rearranca)
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.Scheduler_Main_0518, \"ax\"                                 \n"
    ".globl   Scheduler_Main_0518                                               \n"
    ".globl   FUN_00000518                                                      \n"
    "Scheduler_Main_0518:                                                       \n"
    "FUN_00000518:                                                              \n"
    "    jsr     FUN_00013600                /* 4EB9 00013600  */              \n"
    "    bclr    #1, 18(%fp)                 /* 08AE 0001 0012 */              \n"
    "    lea     (4,%pc), %a1                /* 43FA 0004      */              \n"
    "    move.l  %a1, (%fp)                  /* 2C89           */              \n"
    "    movea.l 8(%fp), %a1                 /* 226E 0008      */              \n"
    "    move.b  16(%fp), %d0                /* 102E 0010      */              \n"
    "    cmp.b   16(%a1), %d0                /* B029 0010      */              \n"
    "    bcs.w   1f                          /* 6500 000A      */              \n"
    "    jsr     Task_UnlinkAlive_0546(%pc)  /* 4EBA 000A      */              \n"
    "    jmp     FUN_000006E2(%pc)           /* 4EFA 01A2      */              \n"
    "1:  jmp     FUN_000006CA(%pc)           /* 4EFA 0186      */              \n"
);

/* ---------------------------------------------------------------------
 * Task_UnlinkAlive_0546 ($0546, 66 bytes)
 * ---------------------------------------------------------------------
 * Bifurcación por bit 0 de flags: si bit0==0 (task vivo), desengancha
 * este task de la lista principal y lo devuelve a la free list global
 * en $106E80.
 * Si bit0==1 (muerto), delega en Task_UnlinkDead_0588.
 *
 * Bytes originales:
 *   $0546: 102E 0012          move.b 18(fp), d0
 *   $054A: 0800 0000          btst   #0, d0
 *   $054E: 6600 0038          bne.w  $0588       -> UnlinkDead
 *   $0552: 4EB9 0001 3600     jsr    FUN_00013600
 *   $0558: 226E 0004          movea.l 4(fp), a1     ; a1 = fp->prev
 *   $055C: 246E 0008          movea.l 8(fp), a2     ; a2 = fp->next
 *   $0560: 234A 0008          move.l a2, 8(a1)      ; prev->next = next
 *   $0564: 2549 0004          move.l a1, 4(a2)      ; next->prev = prev
 *   $0568: 2CBC FFFF FFFF     move.l #-1, (fp)      ; fp->slot0 = -1
 *   $056E: 2D7C FFFF FFFF 0048 move.l #-1, 72(fp)   ; fp->field48 = -1
 *   $0576: 2D79 0010 6E80 0008 move.l ($106E80), 8(fp) ; fp->next = free_list_root
 *   $057E: 23CE 0010 6E80     move.l fp, ($106E80)   ; free_list_root = fp
 *   $0584: 2C49               movea.l a1, fp        ; fp = prev (continúa main loop)
 *   $0586: 4E75               rts
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.Task_UnlinkAlive_0546, \"ax\"                               \n"
    ".globl   Task_UnlinkAlive_0546                                             \n"
    "Task_UnlinkAlive_0546:                                                     \n"
    "    move.b  18(%fp), %d0                /* 102E 0012           */         \n"
    "    btst    #0, %d0                     /* 0800 0000           */         \n"
    "    bne.w   Task_UnlinkDead_0588        /* 6600 0038           */         \n"
    "    jsr     FUN_00013600                /* 4EB9 00013600       */         \n"
    "    movea.l 4(%fp), %a1                 /* 226E 0004           */         \n"
    "    movea.l 8(%fp), %a2                 /* 246E 0008           */         \n"
    "    move.l  %a2, 8(%a1)                 /* 234A 0008           */         \n"
    "    move.l  %a1, 4(%a2)                 /* 2549 0004           */         \n"
    "    move.l  #-1, (%fp)                  /* 2CBC FFFFFFFF       */         \n"
    "    move.l  #-1, 72(%fp)                /* 2D7C FFFFFFFF 0048  */         \n"
    "    move.l  0x106E80, 8(%fp)            /* 2D79 00106E80 0008  */         \n"
    "    move.l  %fp, 0x106E80               /* 23CE 00106E80       */         \n"
    "    movea.l %a1, %fp                    /* 2C49                */         \n"
    "    rts                                                                    \n"
);

/* ---------------------------------------------------------------------
 * Task_UnlinkDead_0588 ($0588, 46 bytes)
 * ---------------------------------------------------------------------
 * Variante de UnlinkAlive para tasks marcados como muertos (bit0 del
 * campo flags). Diferencias respecto a UnlinkAlive:
 *   - fp->slot0 recibe $00000400 (centinela "muerto reciclable")
 *     en vez de $FFFFFFFF.
 *   - Vuelve a marcar bit0 de flags como set con `bset #0, 18(fp)`.
 *   - NO reengancha en la free list global ($106E80): el task queda
 *     "aparcado" pero con slot0=$400.
 *
 * Bytes originales:
 *   $0588: 4EB9 0001 3600     jsr    FUN_00013600
 *   $058E: 226E 0004          movea.l 4(fp), a1
 *   $0592: 246E 0008          movea.l 8(fp), a2
 *   $0596: 234A 0008          move.l a2, 8(a1)
 *   $059A: 2549 0004          move.l a1, 4(a2)
 *   $059E: 2CBC 0000 0400     move.l #$400, (fp)
 *   $05A4: 2D7C FFFF FFFF 0048 move.l #-1, 72(fp)
 *   $05AC: 08EE 0000 0012     bset   #0, 18(fp)
 *   $05B2: 2C49               movea.l a1, fp
 *   $05B4: 4E75               rts
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.Task_UnlinkDead_0588, \"ax\"                                \n"
    ".globl   Task_UnlinkDead_0588                                              \n"
    "Task_UnlinkDead_0588:                                                      \n"
    "    jsr     FUN_00013600                /* 4EB9 00013600       */         \n"
    "    movea.l 4(%fp), %a1                 /* 226E 0004           */         \n"
    "    movea.l 8(%fp), %a2                 /* 246E 0008           */         \n"
    "    move.l  %a2, 8(%a1)                 /* 234A 0008           */         \n"
    "    move.l  %a1, 4(%a2)                 /* 2549 0004           */         \n"
    "    move.l  #0x400, (%fp)               /* 2CBC 00000400       */         \n"
    "    move.l  #-1, 72(%fp)                /* 2D7C FFFFFFFF 0048  */         \n"
    "    bset    #0, 18(%fp)                 /* 08EE 0000 0012      */         \n"
    "    movea.l %a1, %fp                    /* 2C49                */         \n"
    "    rts                                                                    \n"
);

/* ---------------------------------------------------------------------
 * Task_WalkList_05B6 ($05B6, 36 bytes)
 * ---------------------------------------------------------------------
 * Salva fp en pila, coge la prioridad de fp (en d0.b), y avanza por la
 * lista `next` mientras encuentre tasks con prioridad menor a d0. Cuando
 * encuentre uno de prioridad >= d0, hace UnlinkAlive del task apuntado
 * originalmente y sigue. Es el "reordenador" que empuja el task actual
 * a su posición correcta en la lista ordenada por prioridad.
 *
 * Bytes originales:
 *   $05B6: 48E7 0002          movem.l fp, -(sp)     ; salva fp
 *   $05BA: 102E 0010          move.b 16(fp), d0     ; d0 = fp->prio
 *   $05BE: 2C6E 0008          movea.l 8(fp), fp     ; fp = fp->next
 *   $05C2: B02E 0010          cmp.b  16(fp), d0
 *   $05C6: 6400 000C          bcc.w  $05D4          ; si prio(fp) <= d0 -> salir
 *   $05CA: 3F00               move.w d0, -(sp)      ; salva d0 antes de jsr
 *   $05CC: 6100 FF78          bsr.w  $0546          ; UnlinkAlive
 *   $05D0: 301F               move.w (sp)+, d0      ; restaura d0
 *   $05D2: 60EA               bra.b  $05BE          ; loop
 *   $05D4: 4CDF 4000          movem.l (sp)+, fp
 *   $05D8: 4E75               rts
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.Task_WalkList_05B6, \"ax\"                                  \n"
    ".globl   Task_WalkList_05B6                                                \n"
    "Task_WalkList_05B6:                                                        \n"
    "    movem.l %fp, -(%sp)                 /* 48E7 0002      */              \n"
    "    move.b  16(%fp), %d0                /* 102E 0010      */              \n"
    "1:  movea.l 8(%fp), %fp                 /* 2C6E 0008      */              \n"
    "    cmp.b   16(%fp), %d0                /* B02E 0010      */              \n"
    "    bcc.w   2f                          /* 6400 000C      */              \n"
    "    move.w  %d0, -(%sp)                 /* 3F00           */              \n"
    "    bsr.w   Task_UnlinkAlive_0546       /* 6100 FF78      */              \n"
    "    move.w  (%sp)+, %d0                 /* 301F           */              \n"
    "    bra.b   1b                          /* 60EA           */              \n"
    "2:  movem.l (%sp)+, %fp                 /* 4CDF 4000      */              \n"
    "    rts                                                                    \n"
);

/* ---------------------------------------------------------------------
 * Task_ChangeHandler_05DA ($05DA, 36 bytes)
 * ---------------------------------------------------------------------
 * Guardián + delegación:
 *   if (a0 == -1)  return;
 *   if (a0 == $400) return;
 *   push fp; fp = a0; Task_WalkList_05B6(); pop fp;
 *
 * Salta si el puntero es un centinela ($FFFFFFFF o $00000400), o si es
 * un task real, se aparca como fp y se le hace WalkList (reordena por
 * prioridad).
 *
 * Bytes originales:
 *   $05DA: B1FC FFFF FFFF     cmpa.l #-1, a0
 *   $05E0: 6700 001A          beq.w  $05FC
 *   $05E4: B1FC 0000 0400     cmpa.l #$400, a0
 *   $05EA: 6700 0010          beq.w  $05FC
 *   $05EE: 48E7 0002          movem.l fp, -(sp)
 *   $05F2: 2C48               movea.l a0, fp
 *   $05F4: 4EBA FFC0          jsr    (pc+FFC0).w      ; -> $05B6 WalkList
 *   $05F8: 4CDF 4000          movem.l (sp)+, fp
 *   $05FC: 4E75               rts
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.Task_ChangeHandler_05DA, \"ax\"                             \n"
    ".globl   Task_ChangeHandler_05DA                                           \n"
    "Task_ChangeHandler_05DA:                                                   \n"
    "    cmpa.l  #-1, %a0                    /* B1FC FFFFFFFF  */              \n"
    "    beq.w   1f                          /* 6700 001A      */              \n"
    "    cmpa.l  #0x400, %a0                 /* B1FC 00000400  */              \n"
    "    beq.w   1f                          /* 6700 0010      */              \n"
    "    movem.l %fp, -(%sp)                 /* 48E7 0002      */              \n"
    "    movea.l %a0, %fp                    /* 2C48           */              \n"
    "    jsr     Task_WalkList_05B6(%pc)     /* 4EBA FFC0      */              \n"
    "    movem.l (%sp)+, %fp                 /* 4CDF 4000      */              \n"
    "1:  rts                                                                    \n"
);

/* ---------------------------------------------------------------------
 * Task_RunHandler_05FE ($05FE, 40 bytes)
 * ---------------------------------------------------------------------
 * Ejecuta las rutinas de "actualización de estado" ($5DC1C y $5DC34) con
 * todos los registros salvados, y aplica el resultado como nueva
 * prioridad al task apuntado por a0, marcándolo activo (bit 0 clear +
 * set) y sin flags residuales.
 *
 * Bytes originales:
 *   $05FE: 3028 0010          move.w 16(a0), d0    ; d0 = a0->prio
 *   $0602: 48E7 FFFE          movem.l d0-fp, -(sp)
 *   $0606: 4EB9 0005 DC1C     jsr    $05DC1C
 *   $060C: 4EB9 0005 DC34     jsr    $05DC34
 *   $0612: 4CDF 7FFF          movem.l (sp)+, d0-fp
 *   $0616: 3140 0010          move.w d0, 16(a0)   ; a0->prio = d0
 *   $061A: 4268 0012          clr.w  18(a0)      ; a0->flags = 0
 *   $061E: 08E8 0000 0012     bset   #0, 18(a0)  ; a0->flags |= 1
 *   $0624: 4E75               rts
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.Task_RunHandler_05FE, \"ax\"                                \n"
    ".globl   Task_RunHandler_05FE                                              \n"
    "Task_RunHandler_05FE:                                                      \n"
    "    move.w  16(%a0), %d0                /* 3028 0010      */              \n"
    "    movem.l %d0-%fp, -(%sp)             /* 48E7 FFFE      */              \n"
    "    jsr     0x5DC1C                     /* 4EB9 0005DC1C  */              \n"
    "    jsr     0x5DC34                     /* 4EB9 0005DC34  */              \n"
    "    movem.l (%sp)+, %d0-%fp             /* 4CDF 7FFF      */              \n"
    "    move.w  %d0, 16(%a0)                /* 3140 0010      */              \n"
    "    clr.w   18(%a0)                     /* 4268 0012      */              \n"
    "    bset    #0, 18(%a0)                 /* 08E8 0000 0012 */              \n"
    "    rts                                                                    \n"
);

/* ---------------------------------------------------------------------
 * Task_ChangeAndRun_0626 ($0626, 48 bytes)
 * ---------------------------------------------------------------------
 * Combinación de ChangeHandler + instalación de "handler stub $400" +
 * RunHandler. Se usa para arrancar un task nuevo:
 *   if (a0 == -1)   return;
 *   if (a0 == $400) return;
 *   push fp; fp = a0;
 *   Task_WalkList_05B6();
 *   a1 = (Task*)$400;      // stub genérico de "task recién creado"
 *   fp->slot0 = a1;
 *   a0 = fp;
 *   Task_RunHandler_05FE();
 *   pop fp;
 *   return;
 *
 * Bytes originales:
 *   $0626: B1FC FFFF FFFF     cmpa.l #-1, a0
 *   $062C: 6700 0026          beq.w  $0654
 *   $0630: B1FC 0000 0400     cmpa.l #$400, a0
 *   $0636: 6700 001C          beq.w  $0654
 *   $063A: 48E7 0002          movem.l fp, -(sp)
 *   $063E: 2C48               movea.l a0, fp
 *   $0640: 4EBA FF74          jsr    (pc+FF74).w    ; -> $05B6 WalkList
 *   $0644: 43FA FDBA          lea    (pc-$0246).w, a1 ; a1 = $0400
 *   $0648: 2C89               move.l a1, (fp)      ; fp->slot0 = $400
 *   $064A: 204E               movea.l fp, a0
 *   $064C: 4EBA FFB0          jsr    (pc+FFB0).w    ; -> $05FE RunHandler
 *   $0650: 4CDF 4000          movem.l (sp)+, fp
 *   $0654: 4E75               rts
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.Task_ChangeAndRun_0626, \"ax\"                              \n"
    ".globl   Task_ChangeAndRun_0626                                            \n"
    "Task_ChangeAndRun_0626:                                                    \n"
    "    cmpa.l  #-1, %a0                    /* B1FC FFFFFFFF  */              \n"
    "    beq.w   1f                          /* 6700 0026      */              \n"
    "    cmpa.l  #0x400, %a0                 /* B1FC 00000400  */              \n"
    "    beq.w   1f                          /* 6700 001C      */              \n"
    "    movem.l %fp, -(%sp)                 /* 48E7 0002      */              \n"
    "    movea.l %a0, %fp                    /* 2C48           */              \n"
    "    jsr     Task_WalkList_05B6(%pc)     /* 4EBA FF74      */              \n"
    "    lea     RtsStub_0400(%pc), %a1      /* 43FA FDBA      */              \n"
    "    move.l  %a1, (%fp)                  /* 2C89           */              \n"
    "    movea.l %fp, %a0                    /* 204E           */              \n"
    "    jsr     Task_RunHandler_05FE(%pc)   /* 4EBA FFB0      */              \n"
    "    movem.l (%sp)+, %fp                 /* 4CDF 4000      */              \n"
    "1:  rts                                                                    \n"
);
