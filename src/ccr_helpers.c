/*
 * Metal Slug 1 — Familia CCR helpers (macro-familia de 1029+ funciones)
 * ========================================================================
 * Mini-funciones de 6 bytes que manipulan el Condition Code Register (CCR)
 * y retornan. En el diseño del compilador original (Nazca / SN Systems)
 * eran el "canal de retorno booleano" — el llamador comprobaba el CCR
 * con bcc/bne inmediatamente tras el `jsr`:
 *
 *     jsr  ChequeoQueDevuelveXN
 *     bpl  todo_ok            ; bit N indica "no OK"
 *     ...  fallback
 *
 * Todas las mini-funciones caben en 6 bytes: una única instrucción CCR
 * (andi.b o ori.b sobre %ccr) + rts. Aparecen replicadas cientos de veces
 * en el ROM porque el compilador no las unificó — cada sitio tiene su
 * propia dirección para que las llamadas jsr abs.l apunten al lugar
 * exacto.
 *
 * En decompilación semántica final, todas se plegarán a una sola función
 * inline con nombre semántico; el compilador re-generaría las copias
 * conforme sea necesario.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_ccr_helpers.py — no editar
 * a mano. Regenerar tras cambios en el P ROM o en las heurísticas.
 */

#include "mslug.h"

__attribute__((section(".text.ClearXN_0007c0")))
void ClearXN_0007c0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0007c6")))
void SetXN_0007c6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_001cc8")))
void SetXN_001cc8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_001cce")))
void ClearXN_001cce(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_001e40")))
void SetC_001e40(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_001e46")))
void ClearC_001e46(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0138f2")))
void ClearXN_0138f2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0138f8")))
void SetXN_0138f8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_013ca4")))
void SetC_013ca4(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_013d06")))
void SetC_013d06(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_013d0c")))
void ClearC_013d0c(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_024e10")))
void ClearXN_024e10(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_024e16")))
void SetXN_024e16(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_024e2c")))
void ClearXN_024e2c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_024e32")))
void SetXN_024e32(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_024fdc")))
void SetC_024fdc(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_024fe6")))
void ClearC_024fe6(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_025420")))
void SetC_025420(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_02542c")))
void ClearC_02542c(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_02543a")))
void ClearC_02543a(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_025450")))
void ClearXN_025450(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_025456")))
void SetXN_025456(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_025eca")))
void ClearXN_025eca(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_025ed0")))
void SetXN_025ed0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_025ff6")))
void NopCCR_025ff6(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_026128")))
void NopCCR_026128(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_026148")))
void NopCCR_026148(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_026308")))
void NopCCR_026308(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_0264de")))
void NopCCR_0264de(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_0266c6")))
void NopCCR_0266c6(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0266ea")))
void ClearXN_0266ea(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0266f0")))
void SetXN_0266f0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_026706")))
void ClearXN_026706(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02670c")))
void SetXN_02670c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_026722")))
void ClearXN_026722(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_026728")))
void SetXN_026728(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02673e")))
void ClearXN_02673e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_026744")))
void SetXN_026744(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0267ba")))
void SetXN_0267ba(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0267c0")))
void ClearXN_0267c0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0267d6")))
void ClearXN_0267d6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0267dc")))
void SetXN_0267dc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_026d96")))
void SetXN_026d96(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_026dbc")))
void ClearXN_026dbc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02727a")))
void SetXN_02727a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0272a2")))
void ClearXN_0272a2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0273d6")))
void SetXN_0273d6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0273f6")))
void ClearXN_0273f6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027424")))
void SetXN_027424(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027458")))
void SetXN_027458(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02745e")))
void ClearXN_02745e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0274ac")))
void SetXN_0274ac(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0274b2")))
void ClearXN_0274b2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0275b2")))
void ClearXN_0275b2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0275f0")))
void SetXN_0275f0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027726")))
void SetXN_027726(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027736")))
void ClearXN_027736(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027830")))
void SetXN_027830(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

/* SetXN_0278d4 ABSORBIDO por Entity_ProbeRevertCcr_02788C (Wave Z#5).
 * Los 6 B en $0278D4..$0278D9 (`ori.b #$11, ccr; rts`) son la cola de la
 * rama "colision SI" del helper probe/revert. 12° falso positivo del
 * proyecto (mismo patron que los absorbidos en W#16, Y#8, Y#11).
 */
#if 0
__attribute__((section(".text.SetXN_0278d4")))
void SetXN_0278d4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }
#endif

/* ClearXN_0278fc ABSORBIDO por Entity_ProbeRevertCcr_02788C (Wave Z#5).
 * Los 6 B en $0278FC..$027901 (`andi.b #$EE, ccr; rts`) son la cola de la
 * rama "colision NO" del helper probe/revert. 14° falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.ClearXN_0278fc")))
void ClearXN_0278fc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }
#endif

__attribute__((section(".text.ClearXN_027944")))
void ClearXN_027944(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027966")))
void SetXN_027966(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0279b4")))
void SetXN_0279b4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0279dc")))
void ClearXN_0279dc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027a68")))
void SetXN_027a68(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027a8c")))
void ClearXN_027a8c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

/* ClearXN_027ad4 ABSORBIDO por Entity_ProbeRevertCcr_027A92 (Wave Z#6).
 * Los 6 B en $027AD4..$027AD9 (`andi.b #$EE, ccr; rts`) son la cola de la
 * rama "colision NO" del helper probe/revert gemelo. 13° falso positivo
 * del proyecto.
 */
#if 0
__attribute__((section(".text.ClearXN_027ad4")))
void ClearXN_027ad4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }
#endif

/* SetXN_027af6 ABSORBIDO por Entity_ProbeRevertCcr_027A92 (Wave Z#6).
 * Los 6 B en $027AF6..$027AFB (`ori.b #$11, ccr; rts`) son la cola de la
 * rama "colision SI" del helper probe/revert gemelo. 15° falso positivo
 * del proyecto.
 */
#if 0
__attribute__((section(".text.SetXN_027af6")))
void SetXN_027af6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }
#endif

/* ClearXN_027b3e ABSORBIDO por Entity_ProbeRevertCcr_027AFC (Wave Z batch 2 #5).
 * Los 6 B en $027B3E..$027B43 (`andi.b #$EE, ccr; rts`) son la cola de la
 * rama "colision NO" del helper probe/revert. 16 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.ClearXN_027b3e")))
void ClearXN_027b3e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }
#endif

/* SetXN_027b60 ABSORBIDO por Entity_ProbeRevertCcr_027AFC (Wave Z batch 2 #5).
 * Los 6 B en $027B60..$027B65 (`ori.b #$11, ccr; rts`) son la cola de la
 * rama "colision SI" del helper probe/revert. 17 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.SetXN_027b60")))
void SetXN_027b60(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }
#endif

__attribute__((section(".text.ClearXN_027ba4")))
void ClearXN_027ba4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027bc2")))
void SetXN_027bc2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

/*
 * ClearXN_027c06 ABSORBIDO por Wave T#11 (Entity_ProbeTransformFreeCcr_027bc8).
 * SetXN_027c24   ABSORBIDO por Wave T#12 (Entity_RestoreTransformSetC_027c0c).
 * ClearXN_027c68 ABSORBIDO por Wave T#13 (Entity_ProbeTransformFreeCcr_027c2a).
 * SetXN_027c86   ABSORBIDO por Wave T#14 (Entity_RestoreTransformSetC_027c6e).
 * Los cuatro eran epilogos compartidos, 0 callers externos.
 */

/*
 * ClearXN_027cca ABSORBIDO por Wave T#9: era el epilogo compartido de
 * Entity_ProbeTransformFreeCcr_027c8c @ $027c8c (0 callers externos).
 * Ver asm/entity_probe_transform_027c8c.s.
 *
 * SetXN_027ce8 ABSORBIDO por Wave T#10: era el epilogo compartido de
 * Entity_RestoreTransformSetC_027cd0 @ $027cd0 (0 callers externos).
 * Ver asm/entity_restore_transform_027cd0.s.
 */

/*
 * ClearXN_027d2c ABSORBIDO por Wave T#7: era el epilogo compartido
 * de Entity_ProbeTransformFreeCcr @ $027cee (0 callers externos desde
 * codigo matcheado). Ver asm/entity_probe_transform_027cee.s.
 */

/*
 * SetXN_027d4a ABSORBIDO por Wave T#8: era el epilogo compartido de
 * Entity_RestoreTransformSetC_027d32 @ $027d32 (0 callers externos desde
 * codigo matcheado). Ver asm/entity_restore_transform_027d32.s.
 */

/*
 * ClearXN_027d8e ABSORBIDO por Wave T#15 (Entity_ProbeTransformFreeCcr_027d50).
 * SetXN_027dac   ABSORBIDO por Wave T#16 (Entity_RestoreTransformSetC_027d94).
 * Cierran el cluster probe/revert completo $027bc8..$027db1 (5 pares).
 */

__attribute__((section(".text.SetXN_027e56")))
void SetXN_027e56(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027e5c")))
void ClearXN_027e5c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027e72")))
void SetXN_027e72(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027e78")))
void ClearXN_027e78(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027e90")))
void SetXN_027e90(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027e96")))
void ClearXN_027e96(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027eae")))
void SetXN_027eae(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027eb4")))
void ClearXN_027eb4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027efc")))
void ClearXN_027efc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027f02")))
void SetXN_027f02(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027f52")))
void ClearXN_027f52(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027f5a")))
void SetXN_027f5a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_027fcc")))
void ClearXN_027fcc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_027fd2")))
void SetXN_027fd2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028000")))
void ClearXN_028000(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028008")))
void SetXN_028008(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028068")))
void SetXN_028068(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02806e")))
void ClearXN_02806e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0280b2")))
void ClearXN_0280b2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0280c0")))
void SetXN_0280c0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0280fc")))
void SetXN_0280fc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028102")))
void ClearXN_028102(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_028282")))
void ClearC_028282(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_02828c")))
void SetC_02828c(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

/*
 * ClearXN_0282d2 ELIMINADO (falso positivo Wave F).
 *
 * Los 6 bytes 023c00ee 4e75 en $0282D2 no son un CCR helper independiente,
 * sino el epilogo `andi.b #$EE, ccr; rts` del brazo .Lcommit del helper
 * Entity_SwapProbeCommit_028292 (Wave W#12) que empieza 64 B antes.
 *
 * SEXTO falso positivo por reuso de epilogos detectado, tras:
 *   1. ex-JsrAbsThunk_050248  (absorbido por Sprite_InvokeBlit8Params, Wave S)
 *   2. ex-JsrAbsThunk_051804  (absorbido por Entity_CopyField68AndCall_0517FE, V#3)
 *   3. ex-SetTaskHandler_049fea (absorbido por Entity_ProbeAndInstallHandler_049FD0, V#8)
 *   4. ex-ClearXN_09a7c6      (absorbido por Entity_ProbeMoveX_09A7AA, W#8)
 *   5. este                    (absorbido por Entity_SwapProbeCommit_028292, W#12)
 *
 * Patron consolidado: los CCR-helpers de la Wave F que emiten SOLO el
 * epilogo `andi.b #$EE, ccr; rts` o `ori.b #$11, ccr; rts` deben
 * revalidarse contra los helpers de dispatch que retornan por CCR:
 * son sospechosos de ser epilogos compartidos.
 *
 * Ver asm/entity_swap_probe_028292.s para el cuerpo real.
 */

__attribute__((section(".text.ClearXN_028318")))
void ClearXN_028318(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02835e")))
void ClearXN_02835e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0283a4")))
void ClearXN_0283a4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0283be")))
void ClearXN_0283be(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0283c4")))
void SetXN_0283c4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02868a")))
void ClearXN_02868a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0286be")))
void SetXN_0286be(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028716")))
void ClearXN_028716(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028752")))
void SetXN_028752(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028762")))
void ClearXN_028762(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028768")))
void SetXN_028768(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0287d6")))
void ClearXN_0287d6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02885e")))
void ClearXN_02885e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028880")))
void SetXN_028880(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0288c6")))
void ClearXN_0288c6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028912")))
void ClearXN_028912(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028920")))
void SetXN_028920(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_028986")))
void ClearC_028986(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_02898c")))
void SetC_02898c(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028992")))
void ClearXN_028992(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

/* SetXN_0289ea ABSORBIDO por Player_Dispatch3Slots_028998 (Wave Z batch 2 #2).
 * Los 6 B en $0289EA..$0289EF (`ori.b #$11, ccr; rts`) son la cola del path
 * "exito" del dispatcher de 3 slots de jugador. 18 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.SetXN_0289ea")))
void SetXN_0289ea(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }
#endif

/* ClearXN_0289f0 ABSORBIDO por Player_Dispatch3Slots_028998 (Wave Z batch 2 #2).
 * Los 6 B en $0289F0..$0289F5 (`andi.b #$EE, ccr; rts`) son la cola del path
 * "default" del dispatcher de 3 slots. 24 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.ClearXN_0289f0")))
void ClearXN_0289f0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }
#endif

__attribute__((section(".text.ClearXN_028b08")))
void ClearXN_028b08(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028b0e")))
void SetXN_028b0e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028b7c")))
void ClearXN_028b7c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028c14")))
void ClearXN_028c14(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028c1a")))
void SetXN_028c1a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028c60")))
void ClearXN_028c60(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028cac")))
void ClearXN_028cac(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028cb2")))
void SetXN_028cb2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_028cc8")))
void ClearXN_028cc8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_028cce")))
void SetXN_028cce(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_029020")))
void ClearXN_029020(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_029026")))
void SetXN_029026(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02955a")))
void ClearXN_02955a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_029560")))
void SetXN_029560(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_029576")))
void ClearXN_029576(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02957c")))
void SetXN_02957c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a270")))
void ClearXN_02a270(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a288")))
void SetXN_02a288(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a2f2")))
void SetXN_02a2f2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a3ca")))
void SetXN_02a3ca(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a3d0")))
void ClearXN_02a3d0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a3e8")))
void SetXN_02a3e8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a3ee")))
void ClearXN_02a3ee(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a412")))
void SetXN_02a412(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a418")))
void ClearXN_02a418(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a4bc")))
void SetXN_02a4bc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a4e6")))
void ClearXN_02a4e6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a4fc")))
void ClearXN_02a4fc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a5a2")))
void SetXN_02a5a2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a5a8")))
void ClearXN_02a5a8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a684")))
void ClearXN_02a684(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a6b8")))
void ClearXN_02a6b8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a706")))
void SetXN_02a706(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a70c")))
void ClearXN_02a70c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a71a")))
void ClearXN_02a71a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a7a0")))
void SetXN_02a7a0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a7a6")))
void ClearXN_02a7a6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a7cc")))
void SetXN_02a7cc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a7d2")))
void ClearXN_02a7d2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a818")))
void SetXN_02a818(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a81e")))
void ClearXN_02a81e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a84e")))
void SetXN_02a84e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a854")))
void ClearXN_02a854(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a8ae")))
void ClearXN_02a8ae(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a8ba")))
void SetXN_02a8ba(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02a92e")))
void SetXN_02a92e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02a952")))
void ClearXN_02a952(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02aade")))
void ClearXN_02aade(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ab28")))
void ClearXN_02ab28(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ab36")))
void SetXN_02ab36(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ab72")))
void ClearXN_02ab72(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ab80")))
void SetXN_02ab80(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02abba")))
void ClearXN_02abba(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02abc0")))
void ClearXN_02abc0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02abcc")))
void ClearXN_02abcc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02abea")))
void SetXN_02abea(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ac08")))
void SetXN_02ac08(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ac46")))
void ClearXN_02ac46(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ac56")))
void ClearXN_02ac56(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ac64")))
void SetXN_02ac64(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ac74")))
void ClearXN_02ac74(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ac7a")))
void SetXN_02ac7a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ac9c")))
void ClearXN_02ac9c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02acf0")))
void ClearXN_02acf0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02acf6")))
void SetXN_02acf6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ad2a")))
void SetXN_02ad2a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ad30")))
void ClearXN_02ad30(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ad5e")))
void SetXN_02ad5e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ad6a")))
void ClearXN_02ad6a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02f87a")))
void ClearXN_02f87a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02f880")))
void SetXN_02f880(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02faf6")))
void ClearXN_02faf6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02fb68")))
void ClearXN_02fb68(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02fb8c")))
void SetXN_02fb8c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02ffda")))
void SetXN_02ffda(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02ffe0")))
void ClearXN_02ffe0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_02fff6")))
void ClearXN_02fff6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_02fffc")))
void SetXN_02fffc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0303e2")))
void ClearC_0303e2(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_0303e8")))
void SetC_0303e8(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03075e")))
void SetXN_03075e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_030764")))
void ClearXN_030764(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0307f8")))
void ClearXN_0307f8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0307fe")))
void SetXN_0307fe(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0323fa")))
void ClearXN_0323fa(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_032400")))
void SetXN_032400(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_032abc")))
void ClearXN_032abc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_032ac2")))
void SetXN_032ac2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_032d60")))
void SetXN_032d60(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_032d66")))
void ClearXN_032d66(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_032dd4")))
void SetXN_032dd4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_032dda")))
void ClearXN_032dda(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_032dfc")))
void SetXN_032dfc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_032e02")))
void ClearXN_032e02(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_032e1a")))
void ClearXN_032e1a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_032e98")))
void SetXN_032e98(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_032e9e")))
void ClearXN_032e9e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_032fe2")))
void ClearXN_032fe2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_032fea")))
void SetXN_032fea(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_03306c")))
void ClearXN_03306c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03307a")))
void SetXN_03307a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0330b6")))
void ClearXN_0330b6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0330c4")))
void SetXN_0330c4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_033166")))
void ClearXN_033166(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03316c")))
void SetXN_03316c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0331c8")))
void ClearXN_0331c8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_03324a")))
void ClearXN_03324a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_033250")))
void SetXN_033250(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0334c0")))
void ClearXN_0334c0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_039244")))
void ClearXN_039244(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03924a")))
void SetXN_03924a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_039366")))
void SetXN_039366(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_039376")))
void ClearXN_039376(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_039404")))
void SetXN_039404(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_03940a")))
void ClearXN_03940a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_03dbbc")))
void ClearXN_03dbbc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03dbc2")))
void SetXN_03dbc2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03edee")))
void SetXN_03edee(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_03ee06")))
void ClearXN_03ee06(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03ee16")))
void SetXN_03ee16(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_03ee2e")))
void ClearXN_03ee2e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_03ee34")))
void SetXN_03ee34(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_03ee52")))
void ClearC_03ee52(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_03eeac")))
void ClearC_03eeac(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_03eec6")))
void SetC_03eec6(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_03eed6")))
void ClearC_03eed6(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_03ef08")))
void ClearC_03ef08(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_03ef0e")))
void SetC_03ef0e(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_03ef1e")))
void ClearC_03ef1e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_03ef50")))
void ClearC_03ef50(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_03ef56")))
void SetC_03ef56(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_03ef94")))
void ClearC_03ef94(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0403d8")))
void ClearXN_0403d8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0403de")))
void SetXN_0403de(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_041d58")))
void ClearXN_041d58(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_041d66")))
void SetXN_041d66(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_041d98")))
void SetXN_041d98(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_041daa")))
void SetXN_041daa(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_041db0")))
void ClearXN_041db0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_041e3c")))
void ClearXN_041e3c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_041e48")))
void SetXN_041e48(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_041f78")))
void ClearXN_041f78(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_041f7e")))
void SetXN_041f7e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0423a0")))
void ClearXN_0423a0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0423a6")))
void SetXN_0423a6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0423b8")))
void ClearC_0423b8(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_0423da")))
void SetC_0423da(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0423e4")))
void ClearC_0423e4(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0434c2")))
void ClearXN_0434c2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0434c8")))
void SetXN_0434c8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_043d60")))
void ClearC_043d60(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_043d66")))
void SetC_043d66(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_043f52")))
void ClearC_043f52(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_043f58")))
void SetC_043f58(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_044256")))
void ClearXN_044256(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04425c")))
void SetXN_04425c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0449b6")))
void ClearXN_0449b6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0449bc")))
void SetXN_0449bc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_045700")))
void ClearXN_045700(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_045710")))
void SetXN_045710(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04577e")))
void ClearXN_04577e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_045784")))
void SetXN_045784(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04579a")))
void ClearXN_04579a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0457a0")))
void SetXN_0457a0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046118")))
void ClearXN_046118(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04611e")))
void SetXN_04611e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046134")))
void ClearXN_046134(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04613a")))
void SetXN_04613a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046528")))
void ClearXN_046528(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04652e")))
void SetXN_04652e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0465fc")))
void ClearXN_0465fc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_046602")))
void SetXN_046602(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046956")))
void ClearXN_046956(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046966")))
void ClearXN_046966(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04697a")))
void SetXN_04697a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046984")))
void ClearXN_046984(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046998")))
void ClearXN_046998(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0469a8")))
void ClearXN_0469a8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0469bc")))
void SetXN_0469bc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0469c6")))
void ClearXN_0469c6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_046a3c")))
void ClearXN_046a3c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_046a42")))
void SetXN_046a42(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_046e6c")))
void NopCCR_046e6c(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_046fd0")))
void NopCCR_046fd0(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0478f0")))
void ClearXN_0478f0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0478f6")))
void SetXN_0478f6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_048f4e")))
void ClearXN_048f4e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_048fc6")))
void ClearXN_048fc6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049048")))
void SetXN_049048(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04904e")))
void ClearXN_04904e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0490dc")))
void SetXN_0490dc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0490f4")))
void ClearXN_0490f4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04911c")))
void SetXN_04911c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049122")))
void ClearXN_049122(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049166")))
void SetXN_049166(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04916c")))
void ClearXN_04916c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04918a")))
void ClearXN_04918a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049190")))
void SetXN_049190(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0491d2")))
void SetXN_0491d2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0491d8")))
void ClearXN_0491d8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049212")))
void SetXN_049212(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049218")))
void ClearXN_049218(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04924a")))
void SetXN_04924a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049250")))
void ClearXN_049250(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049298")))
void SetXN_049298(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04929e")))
void ClearXN_04929e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0492c2")))
void SetXN_0492c2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0492c8")))
void ClearXN_0492c8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0492ec")))
void SetXN_0492ec(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0492f2")))
void ClearXN_0492f2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049320")))
void ClearXN_049320(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049326")))
void SetXN_049326(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049336")))
void SetXN_049336(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049340")))
void ClearXN_049340(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04935e")))
void SetXN_04935e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049368")))
void ClearXN_049368(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0493b4")))
void ClearXN_0493b4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0493ba")))
void SetXN_0493ba(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0493d8")))
void ClearXN_0493d8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0493de")))
void SetXN_0493de(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04953e")))
void ClearXN_04953e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049544")))
void SetXN_049544(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049b6c")))
void ClearXN_049b6c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049b72")))
void SetXN_049b72(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049b82")))
void ClearXN_049b82(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049b88")))
void SetXN_049b88(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049b9e")))
void ClearXN_049b9e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049ba4")))
void SetXN_049ba4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_049d7e")))
void ClearXN_049d7e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_049d84")))
void SetXN_049d84(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_049fc4")))
void SetC_049fc4(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04bb8e")))
void ClearXN_04bb8e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04bb94")))
void SetXN_04bb94(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04c76a")))
void SetXN_04c76a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04c770")))
void ClearXN_04c770(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04c9e2")))
void SetXN_04c9e2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04c9e8")))
void ClearXN_04c9e8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04ca02")))
void SetXN_04ca02(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04ca08")))
void ClearXN_04ca08(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04cab8")))
void ClearXN_04cab8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04cabe")))
void SetXN_04cabe(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04cbc8")))
void ClearXN_04cbc8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04cbce")))
void SetXN_04cbce(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04d440")))
void SetXN_04d440(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04d446")))
void ClearXN_04d446(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04d468")))
void SetXN_04d468(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04d46e")))
void ClearXN_04d46e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04d4a0")))
void ClearXN_04d4a0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04d4aa")))
void SetXN_04d4aa(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_04d5bc")))
void SetC_04d5bc(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_04d5c2")))
void ClearC_04d5c2(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_04d5d8")))
void SetC_04d5d8(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_04d5de")))
void ClearC_04d5de(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_04d69c")))
void SetC_04d69c(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_04d6a6")))
void ClearC_04d6a6(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_04d6e0")))
void ClearXN_04d6e0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_04d6e6")))
void SetXN_04d6e6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_04fa7e")))
void SetC_04fa7e(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_04fa84")))
void ClearC_04fa84(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_050260")))
void ClearXN_050260(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_050266")))
void SetXN_050266(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0504ea")))
void ClearC_0504ea(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_050514")))
void SetC_050514(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05051a")))
void ClearC_05051a(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_050540")))
void ClearC_050540(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_050546")))
void SetC_050546(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_050594")))
void ClearC_050594(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05059a")))
void SetC_05059a(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_0505e4")))
void SetC_0505e4(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0505ea")))
void ClearC_0505ea(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_050632")))
void SetC_050632(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_050638")))
void ClearC_050638(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0506ea")))
void ClearC_0506ea(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_0506f0")))
void SetC_0506f0(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05071a")))
void ClearC_05071a(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_050720")))
void SetC_050720(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05160c")))
void ClearXN_05160c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_051612")))
void SetXN_051612(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0516ae")))
void ClearC_0516ae(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_0516b4")))
void SetC_0516b4(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_051782")))
void SetC_051782(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_051788")))
void ClearC_051788(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05179e")))
void ClearXN_05179e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0517a4")))
void SetXN_0517a4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

/* SetC_051c7a absorbido por Collision_ProbeRange_051C08 (Wave KK#2).
 * FP #42: los 6 B en $051C7A son la rama "colision" ori.b #$1, ccr; rts. */

/* SetC_051cea absorbido por Collision_ProbeX_051C82 (Wave KK#2).
 * FP #43: rama "colision" del probe X. */

/* ClearC_051cf0 absorbido por Collision_ProbeX_051C82 (Wave KK#2).
 * FP #44: rama "sin colision" del probe X (andi.b #$FE, ccr; rts). */

/* SetC_051d78 absorbido por Collision_ProbeY_051CF6 (Wave KK#2).
 * FP #45: rama "colision" del probe Y. */

/* ClearC_051d7e absorbido por Collision_ProbeY_051CF6 (Wave KK#2).
 * FP #46: rama "sin colision" del probe Y. */

__attribute__((section(".text.SetC_051dd6")))
void SetC_051dd6(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_051ddc")))
void ClearC_051ddc(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

/* SetV_05202c absorbido por TileMap_HandlerInline_051F94 (Wave KK#2).
 * FP #47: los 6 B en $05202C son `addi.w #$800, d3` dentro del bucle
 * del handler MMIO, no un helper CCR (Wave N los clasifico mal). */

__attribute__((section(".text.ClearXN_052042")))
void ClearXN_052042(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_052048")))
void SetXN_052048(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_0522a8")))
void NopCCR_0522a8(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_052392")))
void ClearXN_052392(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_052398")))
void SetXN_052398(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05274a")))
void ClearXN_05274a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_052750")))
void SetXN_052750(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0527ae")))
void ClearXN_0527ae(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0527b4")))
void SetXN_0527b4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_053f8a")))
void ClearXN_053f8a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_053f90")))
void SetXN_053f90(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_055ba6")))
void ClearXN_055ba6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_055bac")))
void SetXN_055bac(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_055bc2")))
void ClearXN_055bc2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_055bc8")))
void SetXN_055bc8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_056a48")))
void ClearXN_056a48(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_056a4e")))
void SetXN_056a4e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_056a64")))
void ClearXN_056a64(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_056a6a")))
void SetXN_056a6a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_056a80")))
void ClearXN_056a80(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_056a86")))
void SetXN_056a86(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_056a9c")))
void ClearXN_056a9c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_056aa2")))
void SetXN_056aa2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_056ab8")))
void ClearXN_056ab8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_056abe")))
void SetXN_056abe(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_056e3e")))
void ClearC_056e3e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_056e44")))
void SetC_056e44(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_056ec6")))
void SetC_056ec6(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_056ecc")))
void ClearC_056ecc(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_056f04")))
void SetC_056f04(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_056f0a")))
void ClearC_056f0a(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_056f58")))
void SetC_056f58(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_056f5e")))
void ClearC_056f5e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_056f7e")))
void ClearC_056f7e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_056f84")))
void SetC_056f84(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_056fe0")))
void SetC_056fe0(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_056fe6")))
void ClearC_056fe6(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_057520")))
void SetC_057520(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_057526")))
void ClearC_057526(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_059342")))
void ClearXN_059342(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_059348")))
void SetXN_059348(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_059be2")))
void ClearXN_059be2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_059be8")))
void SetXN_059be8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_059e44")))
void ClearXN_059e44(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_059e4a")))
void SetXN_059e4a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05a818")))
void ClearXN_05a818(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05a81e")))
void SetXN_05a81e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05caf0")))
void ClearXN_05caf0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05caf6")))
void SetXN_05caf6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05cbfc")))
void ClearXN_05cbfc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05cc02")))
void SetXN_05cc02(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

/*
 * ClearXN_05cfc2 ABSORBIDO por Wave U (InputMask_CheckChannelAvail_05cfa8).
 * ClearXN_05d002 ABSORBIDO por Wave U (InputMask_TestChannelBit_05cff8, brazo C=0).
 * SetXN_05d008   ABSORBIDO por Wave U (InputMask_TestChannelBit_05cff8, brazo C=1).
 * Los tres eran epilogos compartidos, 0 callers externos.
 */

/*
 * ClearXN_05d04c ABSORBIDO por Wave U v2 (InputMask_CheckChannelAvail_05d032).
 * ClearXN_05d08e ABSORBIDO por Wave U v2 (InputMask_TestChannelAllBits_05d082, brazo C=0).
 * SetXN_05d094   ABSORBIDO por Wave U v2 (InputMask_TestChannelAllBits_05d082, brazo C=1).
 * Los tres eran epilogos compartidos, 0 callers externos.
 */

/*
 * ClearXN_05d0ec ABSORBIDO por Wave U v3 (InputMask_TestChannelBit_05d0e2, brazo C=0).
 * SetXN_05d0f2   ABSORBIDO por Wave U v3 (InputMask_TestChannelBit_05d0e2, brazo C=1).
 * ClearXN_05d18a ABSORBIDO por Wave U v3 (InputMask_CheckChannelAvail_05d170, epilogo).
 * ClearXN_05d1ce ABSORBIDO por Wave U v3 (InputMask_TestChannelNibbleXor_05d1c0, brazo C=0).
 * SetXN_05d1d4   ABSORBIDO por Wave U v3 (InputMask_TestChannelNibbleXor_05d1c0, brazo C=1).
 * Los cinco eran epilogos compartidos, 0 callers externos.
 */

__attribute__((section(".text.ClearXN_05d204")))
void ClearXN_05d204(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05d20a")))
void SetXN_05d20a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05d234")))
void ClearXN_05d234(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05d23a")))
void SetXN_05d23a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05d310")))
void ClearXN_05d310(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

/* ClearXN_05db4c y SetXN_05db52 absorbidos por CompareField10_CCR_05DB3C
 * (Wave II#1). FP #33 y #34 del proyecto: los 6 bytes en $05DB4C..$05DB51
 * son la rama "greater-equal" (andi.b #$EE, ccr; rts) y los 6 bytes en
 * $05DB52..$05DB57 son la rama "less-than" (ori.b #$11, ccr; rts) del
 * probe CCR bilateral, no helpers CCR independientes. */

__attribute__((section(".text.ClearXNV_05dba0")))
void ClearXNV_05dba0(void) { __asm__ volatile("andi.b #0x0E, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05dbbc")))
void SetC_05dbbc(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

/* ClearXN_05dc10 absorbido por CompareField10_CCR_05DC00 (Wave II#2).
 * FP #36 del proyecto: los 6 bytes en $05DC10..$05DC15 son la rama
 * "greater-equal" del probe CCR bilateral clon, no un helper propio. */

/* SetXN_05dc16 absorbido por CompareField10_CCR_05DC00 (Wave II#2).
 * FP #37 del proyecto: los 6 bytes en $05DC16..$05DC1B son la rama
 * "less-than" del probe CCR bilateral clon, no un helper propio. */

__attribute__((section(".text.ClearC_05ddb2")))
void ClearC_05ddb2(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05ddb8")))
void SetC_05ddb8(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05ddec")))
void ClearC_05ddec(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05de12")))
void ClearC_05de12(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e11c")))
void ClearXN_05e11c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e124")))
void ClearXN_05e124(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e130")))
void SetXN_05e130(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e15a")))
void ClearXN_05e15a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e160")))
void SetXN_05e160(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e19e")))
void ClearXN_05e19e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e1a4")))
void SetXN_05e1a4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e1de")))
void ClearXN_05e1de(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e1e4")))
void SetXN_05e1e4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e21e")))
void ClearXN_05e21e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e224")))
void SetXN_05e224(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e2cc")))
void ClearXN_05e2cc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e2d2")))
void SetXN_05e2d2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e35a")))
void ClearXN_05e35a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e360")))
void SetXN_05e360(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e37a")))
void ClearXN_05e37a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e396")))
void ClearXN_05e396(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e39c")))
void SetXN_05e39c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e3e2")))
void SetXN_05e3e2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e3ee")))
void ClearXN_05e3ee(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e41e")))
void ClearXN_05e41e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e424")))
void SetXN_05e424(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e446")))
void SetXN_05e446(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e44c")))
void ClearXN_05e44c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e47c")))
void ClearXN_05e47c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e482")))
void SetXN_05e482(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e4a6")))
void SetXN_05e4a6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e4ac")))
void ClearXN_05e4ac(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

/* ClearXN_05e5ba ABSORBIDO por ProbeTwoAttemptsCcr_05E5A8 (Wave Z batch 2 #1).
 * Los 6 B en $05E5BA..$05E5BF (`andi.b #$EE, ccr; rts`) son la cola del path
 * "exito primer intento" del probe-2-attempts. 19 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.ClearXN_05e5ba")))
void ClearXN_05e5ba(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }
#endif

/* ClearXN_05e5d4 ABSORBIDO por ProbeTwoAttemptsCcr_05E5A8 (Wave Z batch 2 #1).
 * Los 6 B en $05E5D4..$05E5D9 (`andi.b #$EE, ccr; rts`) son la cola del path
 * "exito segundo intento" del probe-2-attempts. 25 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.ClearXN_05e5d4")))
void ClearXN_05e5d4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }
#endif

/* SetXN_05e5da ABSORBIDO por ProbeTwoAttemptsCcr_05E5A8 (Wave Z batch 2 #1).
 * Los 6 B en $05E5DA..$05E5DF (`ori.b #$11, ccr; rts`) son la cola del path
 * "fallo total" del probe-2-attempts. 20 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.SetXN_05e5da")))
void SetXN_05e5da(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }
#endif

__attribute__((section(".text.ClearXN_05e5f2")))
void ClearXN_05e5f2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e60c")))
void ClearXN_05e60c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e612")))
void SetXN_05e612(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e632")))
void ClearXN_05e632(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e640")))
void ClearXN_05e640(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e646")))
void SetXN_05e646(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e70a")))
void SetXN_05e70a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e714")))
void ClearXN_05e714(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e7d8")))
void SetXN_05e7d8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e7ea")))
void ClearXN_05e7ea(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e88a")))
void ClearXN_05e88a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e890")))
void SetXN_05e890(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e8d4")))
void SetXN_05e8d4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05e906")))
void ClearXN_05e906(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05e90c")))
void SetXN_05e90c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05ea62")))
void ClearXN_05ea62(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05ea68")))
void SetXN_05ea68(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05ead8")))
void ClearXN_05ead8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05eade")))
void SetXN_05eade(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05efbe")))
void ClearXN_05efbe(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05efc4")))
void SetXN_05efc4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05f378")))
void ClearXN_05f378(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05f37e")))
void SetXN_05f37e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05f984")))
void SetC_05f984(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05f98a")))
void ClearC_05f98a(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05f9a0")))
void SetC_05f9a0(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05f9a6")))
void ClearC_05f9a6(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05f9bc")))
void SetC_05f9bc(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05f9c2")))
void ClearC_05f9c2(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05f9d8")))
void SetC_05f9d8(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05f9de")))
void ClearC_05f9de(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05f9f4")))
void ClearXN_05f9f4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05f9fa")))
void SetXN_05f9fa(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05fd34")))
void ClearC_05fd34(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05fd3a")))
void SetC_05fd3a(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_05fd50")))
void SetC_05fd50(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_05fd56")))
void ClearC_05fd56(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_05fd6c")))
void ClearXN_05fd6c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_05fd72")))
void SetXN_05fd72(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_06038c")))
void SetC_06038c(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_060392")))
void ClearC_060392(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_0603a8")))
void SetC_0603a8(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0603ae")))
void ClearC_0603ae(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_060414")))
void ClearXN_060414(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06041a")))
void SetXN_06041a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_06054e")))
void SetC_06054e(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_060554")))
void ClearC_060554(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06056a")))
void ClearXN_06056a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_060570")))
void SetXN_060570(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_060cc8")))
void ClearXN_060cc8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_060cce")))
void SetXN_060cce(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_060e00")))
void SetC_060e00(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_060e06")))
void ClearC_060e06(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_060e56")))
void ClearXN_060e56(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_060e5c")))
void SetXN_060e5c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_061768")))
void SetXN_061768(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06176e")))
void ClearXN_06176e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_061792")))
void SetXN_061792(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_061798")))
void ClearXN_061798(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0619a2")))
void ClearXN_0619a2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0619a8")))
void SetXN_0619a8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0619be")))
void ClearXN_0619be(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0619c4")))
void SetXN_0619c4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0626c4")))
void SetXN_0626c4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0626ca")))
void ClearXN_0626ca(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0626e4")))
void ClearXN_0626e4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0626ea")))
void SetXN_0626ea(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_062726")))
void SetXN_062726(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06272c")))
void ClearXN_06272c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_062766")))
void ClearXN_062766(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_062932")))
void ClearXN_062932(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_062938")))
void SetXN_062938(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0633d4")))
void ClearXN_0633d4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0633da")))
void SetXN_0633da(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063402")))
void SetXN_063402(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063408")))
void ClearXN_063408(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063422")))
void ClearXN_063422(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063428")))
void SetXN_063428(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063612")))
void ClearXN_063612(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063618")))
void SetXN_063618(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063c18")))
void SetXN_063c18(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063c1e")))
void ClearXN_063c1e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063c38")))
void ClearXN_063c38(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063c3e")))
void SetXN_063c3e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063c6a")))
void ClearXN_063c6a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063c70")))
void SetXN_063c70(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063cc8")))
void SetXN_063cc8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063cce")))
void ClearXN_063cce(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_063eba")))
void ClearXN_063eba(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_063ec0")))
void SetXN_063ec0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0643e8")))
void SetXN_0643e8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0643ee")))
void ClearXN_0643ee(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_064410")))
void ClearXN_064410(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06441c")))
void SetXN_06441c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_064430")))
void ClearXN_064430(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_064436")))
void SetXN_064436(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06447a")))
void SetXN_06447a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_064480")))
void ClearXN_064480(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_064492")))
void SetXN_064492(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_064498")))
void ClearXN_064498(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_064544")))
void ClearXN_064544(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06454a")))
void SetXN_06454a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_065860")))
void SetXN_065860(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_065882")))
void ClearXN_065882(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_065b2a")))
void SetXN_065b2a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_065b30")))
void ClearXN_065b30(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_065b9e")))
void ClearXN_065b9e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_065baa")))
void SetXN_065baa(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_065c24")))
void SetXN_065c24(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_065c2a")))
void ClearXN_065c2a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_065d3e")))
void SetXN_065d3e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_065d44")))
void ClearXN_065d44(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_065dd6")))
void ClearXN_065dd6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_065f34")))
void ClearXN_065f34(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_065f3a")))
void SetXN_065f3a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_066616")))
void SetXN_066616(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06661c")))
void ClearXN_06661c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_066674")))
void SetXN_066674(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06667a")))
void ClearXN_06667a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0666a2")))
void ClearXN_0666a2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0666a8")))
void SetXN_0666a8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_066c00")))
void SetXN_066c00(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_066c0c")))
void ClearXN_066c0c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_066c34")))
void SetXN_066c34(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_066c3a")))
void ClearXN_066c3a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0670c6")))
void ClearXN_0670c6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0670cc")))
void SetXN_0670cc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_067f5c")))
void ClearXN_067f5c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_067f80")))
void ClearXN_067f80(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_067fe0")))
void SetXN_067fe0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_067fe6")))
void ClearXN_067fe6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_068014")))
void ClearXN_068014(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06801a")))
void SetXN_06801a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_068064")))
void SetXN_068064(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06806a")))
void ClearXN_06806a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06809e")))
void SetXN_06809e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0680dc")))
void SetXN_0680dc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0680e2")))
void ClearXN_0680e2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0680fe")))
void ClearXN_0680fe(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_068104")))
void SetXN_068104(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_068124")))
void ClearXN_068124(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06812a")))
void SetXN_06812a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06814e")))
void ClearXN_06814e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_068154")))
void SetXN_068154(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06817a")))
void SetXN_06817a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_068180")))
void ClearXN_068180(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0681ba")))
void SetXN_0681ba(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0681c0")))
void ClearXN_0681c0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0681de")))
void SetXN_0681de(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0681e8")))
void ClearXN_0681e8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0682c4")))
void ClearXN_0682c4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0682ca")))
void SetXN_0682ca(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0689da")))
void SetXN_0689da(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0689e0")))
void ClearXN_0689e0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_068a4e")))
void SetXN_068a4e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_068a54")))
void ClearXN_068a54(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_068aca")))
void ClearXN_068aca(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_068b08")))
void SetXN_068b08(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_068b0e")))
void ClearXN_068b0e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_068c12")))
void ClearXN_068c12(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_068c18")))
void SetXN_068c18(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_069dc8")))
void SetXN_069dc8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_069dce")))
void ClearXN_069dce(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_069de8")))
void ClearXN_069de8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_069dee")))
void SetXN_069dee(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_069e2a")))
void SetXN_069e2a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_069e30")))
void ClearXN_069e30(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_069ea4")))
void SetXN_069ea4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06a072")))
void ClearXN_06a072(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06a078")))
void SetXN_06a078(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b202")))
void ClearXN_06b202(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b208")))
void SetXN_06b208(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b23c")))
void ClearXN_06b23c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b2d0")))
void ClearXN_06b2d0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b2d6")))
void SetXN_06b2d6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b32a")))
void ClearXN_06b32a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b330")))
void SetXN_06b330(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b35c")))
void ClearXN_06b35c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b362")))
void SetXN_06b362(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b37e")))
void ClearXN_06b37e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b384")))
void SetXN_06b384(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b44e")))
void SetXN_06b44e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b454")))
void ClearXN_06b454(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b472")))
void SetXN_06b472(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b478")))
void ClearXN_06b478(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b4e6")))
void SetXN_06b4e6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b4ec")))
void ClearXN_06b4ec(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b52e")))
void SetXN_06b52e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b534")))
void ClearXN_06b534(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b544")))
void SetXN_06b544(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b54a")))
void ClearXN_06b54a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b66c")))
void ClearXN_06b66c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b672")))
void SetXN_06b672(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b6bc")))
void SetXN_06b6bc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b6c6")))
void ClearXN_06b6c6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b6e6")))
void SetXN_06b6e6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b6ec")))
void ClearXN_06b6ec(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b8de")))
void SetXN_06b8de(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b8e4")))
void ClearXN_06b8e4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06b93c")))
void SetXN_06b93c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06b946")))
void ClearXN_06b946(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06ba20")))
void ClearXN_06ba20(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06ba26")))
void SetXN_06ba26(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d07c")))
void ClearXN_06d07c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d082")))
void SetXN_06d082(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d0a8")))
void SetXN_06d0a8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d0ae")))
void ClearXN_06d0ae(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d136")))
void ClearXN_06d136(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d166")))
void ClearXN_06d166(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d16c")))
void SetXN_06d16c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d198")))
void ClearXN_06d198(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d19e")))
void SetXN_06d19e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d1f6")))
void ClearXN_06d1f6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d200")))
void SetXN_06d200(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d21c")))
void ClearXN_06d21c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d222")))
void SetXN_06d222(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d290")))
void SetXN_06d290(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d296")))
void ClearXN_06d296(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d3dc")))
void ClearXN_06d3dc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d3e2")))
void SetXN_06d3e2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d436")))
void SetXN_06d436(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d440")))
void ClearXN_06d440(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06d648")))
void ClearXN_06d648(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06d64e")))
void SetXN_06d64e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06e218")))
void SetXN_06e218(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06e21e")))
void ClearXN_06e21e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06e312")))
void ClearXN_06e312(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06e318")))
void SetXN_06e318(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06e51e")))
void ClearXN_06e51e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06e524")))
void SetXN_06e524(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06ef74")))
void ClearXN_06ef74(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06ef7a")))
void SetXN_06ef7a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06efc4")))
void SetXN_06efc4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06efca")))
void ClearXN_06efca(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06efe4")))
void ClearXN_06efe4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06efea")))
void SetXN_06efea(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_06f172")))
void ClearXN_06f172(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_06f178")))
void SetXN_06f178(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_070ac6")))
void ClearXN_070ac6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_070ad2")))
void SetXN_070ad2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_070b60")))
void ClearXN_070b60(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_070b6a")))
void SetXN_070b6a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_070b80")))
void ClearXN_070b80(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_070bbc")))
void ClearXN_070bbc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_070bfc")))
void ClearXN_070bfc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_070c40")))
void SetXN_070c40(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_070c48")))
void ClearXN_070c48(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_070e00")))
void SetXN_070e00(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_070e06")))
void ClearXN_070e06(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07105c")))
void ClearXN_07105c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_071062")))
void SetXN_071062(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0726f6")))
void ClearXN_0726f6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0726fc")))
void SetXN_0726fc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_072744")))
void SetXN_072744(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07274a")))
void ClearXN_07274a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_072776")))
void SetXN_072776(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07277c")))
void ClearXN_07277c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07278e")))
void SetXN_07278e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_072794")))
void ClearXN_072794(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_072b8a")))
void SetXN_072b8a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_072b90")))
void ClearXN_072b90(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_072c32")))
void SetXN_072c32(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_072c3e")))
void ClearXN_072c3e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_072fbc")))
void ClearXN_072fbc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_072fc2")))
void SetXN_072fc2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07411c")))
void SetXN_07411c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07415a")))
void ClearXN_07415a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_074160")))
void SetXN_074160(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07418a")))
void ClearXN_07418a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_074190")))
void SetXN_074190(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0741b4")))
void SetXN_0741b4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0741ba")))
void ClearXN_0741ba(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0742e8")))
void SetXN_0742e8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_074300")))
void ClearXN_074300(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0745ce")))
void ClearXN_0745ce(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0745d4")))
void SetXN_0745d4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_074624")))
void SetC_074624(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_074644")))
void ClearC_074644(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_07464c")))
void ClearC_07464c(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_076fde")))
void ClearXN_076fde(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_076fe4")))
void SetXN_076fe4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0775bc")))
void ClearXN_0775bc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0775c2")))
void SetXN_0775c2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_077bec")))
void SetXN_077bec(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_077c06")))
void ClearXN_077c06(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_077c72")))
void ClearXN_077c72(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_077c78")))
void SetXN_077c78(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_078a78")))
void ClearXN_078a78(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_078a7e")))
void SetXN_078a7e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_078f7e")))
void ClearXN_078f7e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_078f84")))
void SetXN_078f84(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07900e")))
void SetXN_07900e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079100")))
void ClearXN_079100(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079174")))
void ClearXN_079174(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079260")))
void ClearXN_079260(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_079266")))
void SetXN_079266(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0796e4")))
void SetXN_0796e4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0796f0")))
void ClearXN_0796f0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079838")))
void ClearXN_079838(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07983e")))
void SetXN_07983e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079998")))
void ClearXN_079998(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07999e")))
void SetXN_07999e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079a60")))
void ClearXN_079a60(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_079a66")))
void SetXN_079a66(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_079b58")))
void SetC_079b58(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_079b5e")))
void ClearC_079b5e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079c80")))
void ClearXN_079c80(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_079c86")))
void SetXN_079c86(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_079d8e")))
void SetC_079d8e(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_079d94")))
void ClearC_079d94(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_079eac")))
void ClearXN_079eac(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_079eb2")))
void SetXN_079eb2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07a44a")))
void ClearXN_07a44a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07a450")))
void SetXN_07a450(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07a964")))
void ClearXN_07a964(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07a96a")))
void SetXN_07a96a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07b998")))
void ClearXN_07b998(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07b99e")))
void SetXN_07b99e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07b9ae")))
void SetXN_07b9ae(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07b9e4")))
void SetXN_07b9e4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07ba28")))
void ClearXN_07ba28(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07ba70")))
void ClearXN_07ba70(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07ba76")))
void SetXN_07ba76(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_07c612")))
void ClearC_07c612(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_07c622")))
void SetC_07c622(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_07c638")))
void SetC_07c638(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_07c63e")))
void ClearC_07c63e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07cef6")))
void ClearXN_07cef6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07cefc")))
void SetXN_07cefc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_07df9c")))
void SetC_07df9c(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_07dfa2")))
void ClearC_07dfa2(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_07dfb8")))
void ClearXN_07dfb8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_07dfbe")))
void SetXN_07dfbe(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_07fdd4")))
void SetC_07fdd4(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_07fdda")))
void ClearC_07fdda(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_07fdee")))
void SetC_07fdee(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_07fdf4")))
void ClearC_07fdf4(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_080048")))
void SetC_080048(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08004e")))
void ClearC_08004e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08009a")))
void ClearXN_08009a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0800a0")))
void SetXN_0800a0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_080926")))
void ClearXN_080926(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08092c")))
void SetXN_08092c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08093e")))
void ClearXN_08093e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_080944")))
void SetXN_080944(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_080958")))
void ClearXN_080958(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08097c")))
void ClearXN_08097c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_080990")))
void SetXN_080990(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_080f26")))
void ClearC_080f26(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_080f2c")))
void SetC_080f2c(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0818fc")))
void ClearXN_0818fc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_081902")))
void SetXN_081902(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_082ca6")))
void ClearXN_082ca6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_082cac")))
void SetXN_082cac(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_082de8")))
void ClearXN_082de8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_082df2")))
void SetXN_082df2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_082dfe")))
void SetXN_082dfe(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_082e6e")))
void SetXN_082e6e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_082e74")))
void ClearXN_082e74(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_082ed4")))
void SetXN_082ed4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_082eda")))
void ClearXN_082eda(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_082eec")))
void ClearXN_082eec(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_082ef2")))
void SetXN_082ef2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_082f78")))
void SetXN_082f78(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_082f8a")))
void ClearXN_082f8a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0832e0")))
void ClearXN_0832e0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0832e6")))
void SetXN_0832e6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_085b5c")))
void SetXN_085b5c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_085b62")))
void ClearXN_085b62(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_085b72")))
void SetXN_085b72(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_085b78")))
void ClearXN_085b78(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_085d9c")))
void SetXN_085d9c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_085da2")))
void ClearXN_085da2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_085eb0")))
void SetXN_085eb0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_085eb6")))
void ClearXN_085eb6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_085f92")))
void SetXN_085f92(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_085f98")))
void ClearXN_085f98(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_085fa4")))
void ClearXN_085fa4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_085faa")))
void SetXN_085faa(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_086562")))
void ClearXN_086562(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_086568")))
void SetXN_086568(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_088f68")))
void ClearXN_088f68(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_088f6e")))
void SetXN_088f6e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08b61a")))
void SetXN_08b61a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08b620")))
void ClearXN_08b620(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08b7e0")))
void SetXN_08b7e0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08b7ea")))
void ClearXN_08b7ea(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08b820")))
void SetXN_08b820(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08b826")))
void ClearXN_08b826(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08b938")))
void ClearXN_08b938(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08b93e")))
void SetXN_08b93e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08bc68")))
void ClearXN_08bc68(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08bc6e")))
void SetXN_08bc6e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08bce0")))
void SetXN_08bce0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08be22")))
void SetXN_08be22(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08bf90")))
void SetXN_08bf90(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08c002")))
void SetXN_08c002(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08c7d6")))
void ClearXN_08c7d6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08c7dc")))
void SetXN_08c7dc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08c94a")))
void ClearXN_08c94a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08c950")))
void SetXN_08c950(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08d1a2")))
void ClearXN_08d1a2(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08d1a8")))
void SetXN_08d1a8(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08d1dc")))
void ClearC_08d1dc(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08d1e2")))
void ClearC_08d1e2(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08d206")))
void ClearXN_08d206(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08d20c")))
void SetXN_08d20c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08d240")))
void ClearC_08d240(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08d246")))
void ClearC_08d246(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08d26a")))
void ClearXN_08d26a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08d270")))
void SetXN_08d270(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08d2a4")))
void ClearC_08d2a4(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08d2aa")))
void ClearC_08d2aa(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08eff6")))
void ClearXN_08eff6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08effc")))
void SetXN_08effc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08f04a")))
void ClearXN_08f04a(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08f05e")))
void SetXN_08f05e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08f068")))
void ClearXN_08f068(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08f12c")))
void ClearXN_08f12c(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08f132")))
void SetXN_08f132(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f2ee")))
void SetC_08f2ee(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f302")))
void ClearC_08f302(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f38c")))
void SetC_08f38c(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f3a0")))
void ClearC_08f3a0(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f4dc")))
void ClearC_08f4dc(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f51a")))
void SetC_08f51a(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f574")))
void ClearC_08f574(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f598")))
void SetC_08f598(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f5d6")))
void SetC_08f5d6(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f60e")))
void ClearC_08f60e(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f656")))
void SetC_08f656(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f696")))
void SetC_08f696(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08f6c6")))
void ClearXN_08f6c6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08f6cc")))
void SetXN_08f6cc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08f72c")))
void SetXN_08f72c(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08f732")))
void ClearXN_08f732(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f76e")))
void SetC_08f76e(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f774")))
void ClearC_08f774(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f790")))
void SetC_08f790(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f796")))
void ClearC_08f796(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f856")))
void ClearC_08f856(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_08f87e")))
void SetC_08f87e(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_08f8bc")))
void ClearC_08f8bc(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_08f90e")))
void ClearXN_08f90e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_08f914")))
void SetXN_08f914(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0910b0")))
void SetXN_0910b0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0910b6")))
void ClearXN_0910b6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0910ca")))
void SetXN_0910ca(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0910d0")))
void ClearXN_0910d0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0910e4")))
void SetXN_0910e4(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0910ea")))
void ClearXN_0910ea(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0910fe")))
void SetXN_0910fe(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_091104")))
void ClearXN_091104(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_091118")))
void SetXN_091118(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09111e")))
void ClearXN_09111e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0967b4")))
void ClearXN_0967b4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0967ba")))
void SetXN_0967ba(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_097730")))
void ClearXN_097730(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_097736")))
void SetXN_097736(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_097b64")))
void SetXN_097b64(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_097b6e")))
void ClearXN_097b6e(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0981dc")))
void ClearXN_0981dc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0981e2")))
void SetXN_0981e2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_09827c")))
void ClearC_09827c(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_098282")))
void SetC_098282(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0985ba")))
void SetXN_0985ba(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0985c4")))
void ClearXN_0985c4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09865a")))
void SetXN_09865a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_098664")))
void ClearXN_098664(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_098714")))
void ClearXN_098714(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09871a")))
void SetXN_09871a(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0989b0")))
void ClearXN_0989b0(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0989b6")))
void SetXN_0989b6(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_0997ac")))
void ClearXN_0997ac(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_0997b2")))
void SetXN_0997b2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearC_0998e8")))
void ClearC_0998e8(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_099934")))
void SetC_099934(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_0999b4")))
void SetC_0999b4(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

/* ABSORBIDO por Clipping_Test_0999DE (Wave DD #1).
 * 27o falso positivo Wave F del proyecto: el `andi.b #$FE, ccr; rts` en
 * $0999F0..$0999F5 forma el camino rapido "accepted" de la funcion
 * semantica Clipping_Test, no un helper CCR independiente.
 *
 * __attribute__((section(".text.ClearC_0999f0")))
 * void ClearC_0999f0(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }
 */

__attribute__((section(".text.ClearC_099a1c")))
void ClearC_099a1c(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetC_099a44")))
void SetC_099a44(void) { __asm__ volatile("ori.b  #0x01, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_099ad6")))
void ClearXN_099ad6(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_099adc")))
void SetXN_099adc(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_099f0a")))
void NopCCR_099f0a(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.NopCCR_099f34")))
void NopCCR_099f34(void) { __asm__ volatile("ori.b  #0x00, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09a0cc")))
void ClearXN_09a0cc(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09a0d2")))
void SetXN_09a0d2(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09a5e8")))
void ClearXN_09a5e8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09a5ee")))
void SetXN_09a5ee(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

/*
 * ClearXN_09a7c6 ELIMINADO (falso positivo Wave F).
 *
 * Los 6 bytes 023c00ee 4e75 en $09A7C6 no son un CCR helper independiente,
 * sino el epilogo `andi.b #$EE, ccr; rts` del helper Entity_ProbeMoveX_09A7AA
 * (Wave W#8) que empieza 28 B antes en $09A7AA con jsr $9A7CC(pc).
 *
 * QUINTO falso positivo por reuso de epilogos detectado, tras:
 *   1. ex-JsrAbsThunk_050248 (absorbido por Sprite_InvokeBlit8Params, Wave S)
 *   2. ex-JsrAbsThunk_051804 (absorbido por Entity_CopyField68AndCall_0517FE, Wave V#3)
 *   3. ex-SetTaskHandler_049fea (absorbido por Entity_ProbeAndInstallHandler_049FD0, Wave V#8)
 *   4. este                    (absorbido por Entity_ProbeMoveX_09A7AA, Wave W#8)
 *
 * En todos los casos las Waves batch F/H/I detectan el sufijo comun sin
 * darse cuenta de que forma parte del cuerpo de una funcion contigua.
 * Ver asm/entity_probe_move_x_09a7aa.s para el cuerpo real.
 */

__attribute__((section(".text.ClearC_09a842")))
void ClearC_09a842(void) { __asm__ volatile("andi.b #0xFE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09b8e8")))
void ClearXN_09b8e8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09b9ae")))
void SetXN_09b9ae(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09b9b4")))
void ClearXN_09b9b4(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09bdba")))
void ClearXN_09bdba(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09bdc0")))
void SetXN_09bdc0(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09c228")))
void ClearXN_09c228(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09c22e")))
void SetXN_09c22e(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

__attribute__((section(".text.ClearXN_09c4c8")))
void ClearXN_09c4c8(void) { __asm__ volatile("andi.b #0xEE, %%ccr" ::: "cc"); }

__attribute__((section(".text.SetXN_09c4ce")))
void SetXN_09c4ce(void) { __asm__ volatile("ori.b  #0x11, %%ccr" ::: "cc"); }

