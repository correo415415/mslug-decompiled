| ============================================================================
|  Metal Slug 1 - asm/entity_probe_install_handler_049fd0.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity/Sprite helpers) - funcion #8
|
|  Entity_ProbeAndInstallHandler_049FD0  @ $049FD0  (34 bytes, 4 callers)
|
|  Sonda doble con dispatch a handler por CCR:
|      bsr.b Sub_049FBA              -- primer probe local (retorna CCR)
|      si C=0 (probe pasa)   -> rts
|      si C=1                -> jsr Sub_0002_7EBA (probe global)
|                               si C=0 -> instala handler $4A014 en (a6)
|                               si C=1 -> instala handler $4A034 en (a6)
|                               rts
|
|  Los handlers $4A014 y $4A034 son cuerpos contiguos que se distinguen
|  por moveq #0/#1 vs moveq #2/#3 en d7 (probablemente identifica el
|  "canal" o "slot logico" en el que la entidad quedo instalada). Ambos
|  hacen jsr $27F60.l + scc.b $70(a6) + fall-through al epilogo comun
|  en $4A050.
|
|  Firma C conceptual (dispatch triple, no rederivable por GCC 1:1):
|
|      /* Prueba doble; en caso de fallo instala uno de dos handlers
|       * segun cual probe fallo. La entidad se identifica siempre por a6. */
|      void Entity_ProbeAndInstallHandler(struct Entity *a6);
|
|  Notas forenses:
|    - bsr.b (2 B) a $49FBA seguido de bcc.w a $49FF0 es el patron canonico
|      de "probe local que retorna condicion por CCR". GCC nunca emite
|      esto: llamaria a la funcion, guardaria el retorno en d0 y probaria
|      con tst.b. El uso directo de CCR es evidencia dura de asm a mano.
|    - lea.l pc+d,a1 + move.l a1,(a6) es "instalar handler PC-relativo
|      en el primer slot del entity" (offset 0 = puntero al script/handler
|      activo). Este idioma ya lo vimos en Entity_InstallHandlerAndCopyXf
|      (Wave T#6) instalando el handler literal $77C98.
|    - La eleccion entre $4A014 y $4A034 depende del CCR devuelto por
|      Sub_0002_7EBA: probablemente $4A014 = "canal principal" y $4A034
|      = "canal secundario/alternativo".
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ProbeAndInstallHandler_049FD0
        .type   Entity_ProbeAndInstallHandler_049FD0, @function
        .section .text.Entity_ProbeAndInstallHandler_049FD0, "ax", @progbits

Entity_ProbeAndInstallHandler_049FD0:
        bsr.b   Sub_00049FBA            | +00  probe local (retorna CCR)
        bcc.w   .Ldone                  | +02  C=0 -> exito, ir a rts
        jsr     Sub_00027EBA            | +06  probe global
        bcc.w   .Linstall_channel_b     | +0c  C=0 -> instalar handler B ($4A014)
        lea     Handler_0004A034(pc), a1 | +10 C=1 -> instalar handler A ($4A034)
        move.l  a1, (a6)                | +14  entity->script_ptr = handler
        bra.w   .Ldone                  | +16
.Linstall_channel_b:
        lea     Handler_0004A014(pc), a1 | +1a
        move.l  a1, (a6)                | +1e  entity->script_ptr = handler
.Ldone:
        rts                             | +20
        .size   Entity_ProbeAndInstallHandler_049FD0, .-Entity_ProbeAndInstallHandler_049FD0
