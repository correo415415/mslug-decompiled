| ============================================================================
|  Metal Slug 1 - asm/task_alloc.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #4
|
|  Task_AllocFromFreeList  @ $0004ae  (80 bytes, 8 callers)
|
|  Extrae un nodo de la free-list global de tasks (cabeza en $106e80.l),
|  lo enlaza detras del nodo activo (a6) en la lista doblemente enlazada
|  de "tasks vivas" y opcionalmente publica el nodo pasado en a1 como
|  contenido logico.
|
|  Layout del nodo Task (inferido de los offsets manipulados):
|      +$00  void *content       // payload logico (fijado desde a1 al asignar)
|      +$04  Task *prev          // link anterior en la lista viva
|      +$08  Task *next          // link posterior; en la free-list se
|                                // reutiliza como single-link (a0->next)
|      +$0c  Task *owner_entity  // entidad propietaria (a6 del caller)
|      +$10  u8    depth         // profundidad de anidamiento: owner.depth + 1
|
|  Flujo (ver secuencia exacta abajo):
|      1. a0 = *free_list_head  (leer cabeza).
|      2. Si cabeza == ENTITY_NIL ($FFFFFFFF), salta a Task_AllocFail_0506
|         (rama de "sin memoria libre") -- NO retorna por rts local.
|      3. Consumir nodo: free_list_head = a0->next  (leyendo a0[+8]).
|      4. Bracket movem.l d0-d7/a0-a6, -(a7)   ->   guardar TODOS los
|         registros alrededor de dos jsr:
|              jsr $5dc1c.l   (probable: init_task_slot)
|              jsr $5dc34.l   (probable: register_task_hook)
|         movem.l (a7)+, d0-d7/a0-a6   ->   restaurar todo.
|         Este bracket-save total es una firma inequivoca de asm a mano;
|         GCC nunca preserva a4/a5/a6 dentro de una funcion propia salvo
|         que las use, y aqui a4/a5 no se tocan.
|      5. a0->content    = a1               ; (a0) = a1
|         a0->owner_ent  = a6               ; $c(a0) = a6
|         a0->depth      = a6->depth + 1    ; carga byte, incrementa, guarda
|         a1 = a6->next  (lee campo $8 de a6)
|         a6->next = a0                     ; enlazar tras a6
|         a0->prev = a6                     ; back-pointer de a0
|         a0->next = a1                     ; forward-pointer de a0
|         a1->prev = a0                     ; back-pointer del sucesor
|      6. Fall-through al epilogo compartido en $0004fe
|         (registrado en Wave I como JsrAbsThunk_0004fe):
|              jsr $5dc34.l   ; rts
|         Es decir: la funcion no tiene rts propio, cede el rts al
|         thunk contiguo. Este solape estructural es la MISMA firma
|         que Sprite_InvokeBlit8Params (que cedia sus 8 B finales al
|         falso JsrAbsThunk_050248 detectado en la Wave I).
|
|  Entrada (registros absolutos, convencion no-C):
|      a6 : entidad propietaria (owner) del task que se asigna
|      a1 : contenido logico a publicar en el campo +$00 del nodo
|
|  Salida:
|      a0 : nodo Task recien enlazado (para el caller inmediato, aunque
|           la mayoria simplemente ignora el retorno y confia en la
|           actualizacion in-place de a6->next)
|      Modifica: $106e80.l (cabeza de free-list), a6->next, y los 5
|      campos del nodo nuevo.
|
|  Hallazgos forenses (asm a mano):
|    1. Fall-through al thunk contiguo $0004fe (mismo patron que S#1).
|    2. Bracket movem.l d0-d7/a0-a6 alrededor de dos jsr. GCC solo
|       preserva las callee-saved que efectivamente use.
|    3. Stream lineal de 5 move.l register-a-memoria con offsets
|       $0,$4,$8,$c inmediatos sobre a0/a6/a1: GCC-Os habria usado
|       lea + movem si el orden fuera consecutivo, o movep para pares
|       byte-word. La eleccion de move.l individuales es artesanal.
|    4. move.b $10(a6),d0 ; addq.b #1,d0 ; move.b d0,$10(a0)  --
|       increment-and-store sin extension del byte a word. Idioma
|       tipico de 68k ASM, no de GCC (que usaria addq.b directo a mem).
|  ============================================================================

        .text
        .globl  Task_AllocFromFreeList
        .type   Task_AllocFromFreeList, @function
        .section .text.Task_AllocFromFreeList, "ax", @progbits

Task_AllocFromFreeList:
        movea.l 0x106e80, a0            | +00  20 79 00 10 6e 80    a0 = free_list_head
        cmpa.l  #-1, a0                 | +06  b1 fc ff ff ff ff    lista vacia ?
        beq.w   Task_AllocFail_0506     | +0c  67 00 00 4a          si, ir al handler
        move.l  8(a0), 0x106e80         | +10  23 e8 00 08 00 10 6e 80   consumir nodo
        movem.l d0-d7/a0-a6, -(a7)      | +18  48 e7 ff fe          save all
        jsr     0x5dc1c                 | +1c  4e b9 00 05 dc 1c
        jsr     0x5dc34                 | +22  4e b9 00 05 dc 34
        movem.l (a7)+, d0-d7/a0-a6      | +28  4c df 7f ff          restore all
        move.l  a1, (a0)                | +2c  20 89                node->content   = a1
        move.l  a6, 0xc(a0)             | +2e  21 4e 00 0c          node->owner_ent = a6
        move.b  0x10(a6), d0            | +32  10 2e 00 10          d0 = owner.depth
        addq.b  #1, d0                  | +36  52 00                d0 += 1
        move.b  d0, 0x10(a0)            | +38  11 40 00 10          node->depth = d0
        movea.l 8(a6), a1               | +3c  22 6e 00 08          a1 = owner.next
        move.l  a0, 8(a6)               | +40  2d 48 00 08          owner.next = node
        move.l  a6, 4(a0)               | +44  21 4e 00 04          node->prev = a6
        move.l  a1, 8(a0)               | +48  21 49 00 08          node->next = a1
        move.l  a0, 4(a1)               | +4c  23 48 00 04          old_next.prev = node
                                        |                           <-- fall-through a $0004fe
                                        |                           (JsrAbsThunk_0004fe:
                                        |                             jsr $5dc34.l ; rts)
        .size   Task_AllocFromFreeList, .-Task_AllocFromFreeList
