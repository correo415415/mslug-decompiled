/*
 * Metal Slug 1 — Familia JsrAbsThunk (trampolines jsr abs.l ; rts)
 * ====================================================================
 * Mini-funciones de 8 bytes que llaman a otra rutina y retornan:
 *
 *     jsr    RealFunction.L        ; 4EB9 XXXXXXXX  (6 B)
 *     rts                          ; 4E75           (2 B)
 *
 * En C original:
 *     void Thunk_XXXXXX(void) { RealFunction_YYYYYY(); }
 *
 * GCC prefiere convertir en tail-call (`jmp abs.l`, 6 B), pero con una
 * barrera `__asm__ volatile("" ::: "memory")` tras la llamada mantiene el
 * `rts` explícito y emite los 8 bytes exactos del ROM.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_jsr_abs_thunks.py.
 */

#include "mslug.h"


__attribute__((section(".text.JsrAbsThunk_000408")))
void JsrAbsThunk_000408(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0004fe")))
void JsrAbsThunk_0004fe(void) {
    extern void ThunkTarget_05dc34(void);
    ThunkTarget_05dc34();
    __asm__ volatile("" ::: "memory");
}

/*
 * JsrAbsThunk_000762 ELIMINADO (falso positivo Wave I).
 *
 * Los 8 bytes 4eb90005dc34 4e75 en $000762 no son un thunk independiente,
 * sino los ultimos 8 B del helper Entity_AllocFromFreeList_0006FE (W#16),
 * que emite exactamente `jsr Entity_InitFields_05DC34; rts` como cierre
 * tras la insercion en la linked list del padre (segunda pasada de init).
 *
 * SEPTIMO falso positivo por reuso de epilogos detectado, tras:
 *   1. ex-JsrAbsThunk_050248  (absorbido por Sprite_InvokeBlit8Params, S)
 *   2. ex-JsrAbsThunk_051804  (absorbido por Entity_CopyField68AndCall_0517FE, V#3)
 *   3. ex-SetTaskHandler_049fea (absorbido por Entity_ProbeAndInstallHandler_049FD0, V#8)
 *   4. ex-ClearXN_09a7c6      (absorbido por Entity_ProbeMoveX_09A7AA, W#8)
 *   5. ex-ClearXN_0282d2      (absorbido por Entity_SwapProbeCommit_028292, W#12)
 *   6. este                    (absorbido por Entity_AllocFromFreeList_0006FE, W#16)
 *
 * Ver asm/entity_alloc_from_freelist_0006fe.s para el cuerpo real.
 */

/* JsrAbsThunk_001aae ABSORBIDO por Dispatcher_ModeTable_001922 (Wave EE batch 1 - FP #29)
 * $001AAE..$001AB5 (8 B: jsr $981FC.l; rts) es la rama .Ltwo_path de
 * Dispatcher_ModeTable_001922, no un thunk independiente. */

/* JsrAbsThunk_001c2c ABSORBIDO por Handler_TimerAndReplace_001BCC (Wave Z batch 2 #13).
 * Los 8 B en $001C2C..$001C33 (`jsr $47482.l; rts`) son la cola tail-call
 * del handler de timer. 21 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.JsrAbsThunk_001c2c")))
void JsrAbsThunk_001c2c(void) {
    extern void ThunkTarget_047482(void);
    ThunkTarget_047482();
    __asm__ volatile("" ::: "memory");
}
#endif

__attribute__((section(".text.JsrAbsThunk_001d34")))
void JsrAbsThunk_001d34(void) {
    extern void FUN_000005B6(void);
    FUN_000005B6();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_001d9c")))
void JsrAbsThunk_001d9c(void) {
    extern void FUN_000005B6(void);
    FUN_000005B6();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_001e02")))
void JsrAbsThunk_001e02(void) {
    extern void ThunkTarget_0004ae(void);
    ThunkTarget_0004ae();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_00211e ABSORBIDO por Init_MasterSubsystems_0020E2 (Wave Y#8).
 * Los 8 bytes en $00211E..$002125 (`jsr $ffe.l; rts`) NO son un thunk
 * independiente: son las dos ultimas instrucciones del inicializador
 * master del arranque post-BIOS. Octavo falso positivo del proyecto,
 * mismo patron que JsrAbsThunk_000762 (Entity_AllocFromFreeList_0006FE,
 * Wave W#16). Se retira del registro; la entrada canonica cubre ahora
 * los 68 bytes completos de $0020E2..$002125.
 */
#if 0
__attribute__((section(".text.JsrAbsThunk_00211e")))
void JsrAbsThunk_00211e(void) {
    extern void ThunkTarget_000ffe(void);
    ThunkTarget_000ffe();
    __asm__ volatile("" ::: "memory");
}
#endif

__attribute__((section(".text.JsrAbsThunk_024e6e")))
void JsrAbsThunk_024e6e(void) {
    extern void FUN_00099afc(void);
    FUN_00099afc();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0257e4")))
void JsrAbsThunk_0257e4(void) {
    extern void ThunkTarget_05180c(void);
    ThunkTarget_05180c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_028702")))
void JsrAbsThunk_028702(void) {
    extern void ThunkTarget_051914(void);
    ThunkTarget_051914();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a242")))
void JsrAbsThunk_02a242(void) {
    extern void ThunkTarget_03060a(void);
    ThunkTarget_03060a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a5cc")))
void JsrAbsThunk_02a5cc(void) {
    extern void ThunkTarget_05d00e(void);
    ThunkTarget_05d00e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a5f2")))
void JsrAbsThunk_02a5f2(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a5fe")))
void JsrAbsThunk_02a5fe(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a624")))
void JsrAbsThunk_02a624(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a630")))
void JsrAbsThunk_02a630(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a65c")))
void JsrAbsThunk_02a65c(void) {
    extern void ThunkTarget_0004ae(void);
    ThunkTarget_0004ae();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a758")))
void JsrAbsThunk_02a758(void) {
    extern void ThunkTarget_02785c(void);
    ThunkTarget_02785c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02a870")))
void JsrAbsThunk_02a870(void) {
    extern void ThunkTarget_02788c(void);
    ThunkTarget_02788c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02abb2")))
void JsrAbsThunk_02abb2(void) {
    extern void ThunkTarget_05cdfc(void);
    ThunkTarget_05cdfc();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02b7d2")))
void JsrAbsThunk_02b7d2(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02b8c6")))
void JsrAbsThunk_02b8c6(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02ba2c")))
void JsrAbsThunk_02ba2c(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02bb92")))
void JsrAbsThunk_02bb92(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02be2a")))
void JsrAbsThunk_02be2a(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02bf5c")))
void JsrAbsThunk_02bf5c(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c072")))
void JsrAbsThunk_02c072(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c188")))
void JsrAbsThunk_02c188(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c42a")))
void JsrAbsThunk_02c42a(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c54c")))
void JsrAbsThunk_02c54c(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c640")))
void JsrAbsThunk_02c640(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c716")))
void JsrAbsThunk_02c716(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c8f8")))
void JsrAbsThunk_02c8f8(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02c9a8")))
void JsrAbsThunk_02c9a8(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02ce14")))
void JsrAbsThunk_02ce14(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02cff2")))
void JsrAbsThunk_02cff2(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02dcb4")))
void JsrAbsThunk_02dcb4(void) {
    extern void ThunkTarget_0005fe(void);
    ThunkTarget_0005fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02e3a8")))
void JsrAbsThunk_02e3a8(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02e48e")))
void JsrAbsThunk_02e48e(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02e57a")))
void JsrAbsThunk_02e57a(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02e7f2")))
void JsrAbsThunk_02e7f2(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02e91c")))
void JsrAbsThunk_02e91c(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02eb0c")))
void JsrAbsThunk_02eb0c(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02ec9e")))
void JsrAbsThunk_02ec9e(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02ee70")))
void JsrAbsThunk_02ee70(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02efa2")))
void JsrAbsThunk_02efa2(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02f0d4")))
void JsrAbsThunk_02f0d4(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02f30a")))
void JsrAbsThunk_02f30a(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02f41a")))
void JsrAbsThunk_02f41a(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02f4f4")))
void JsrAbsThunk_02f4f4(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02f5ce")))
void JsrAbsThunk_02f5ce(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_02ff42")))
void JsrAbsThunk_02ff42(void) {
    extern void ThunkTarget_0818aa(void);
    ThunkTarget_0818aa();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_030060")))
void JsrAbsThunk_030060(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0301e8")))
void JsrAbsThunk_0301e8(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03038a")))
void JsrAbsThunk_03038a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_030602")))
void JsrAbsThunk_030602(void) {
    extern void ThunkTarget_05d6c2(void);
    ThunkTarget_05d6c2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03181e")))
void JsrAbsThunk_03181e(void) {
    extern void ThunkTarget_0517fe(void);
    ThunkTarget_0517fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_031846")))
void JsrAbsThunk_031846(void) {
    extern void ThunkTarget_05e4b2(void);
    ThunkTarget_05e4b2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03186e")))
void JsrAbsThunk_03186e(void) {
    extern void ThunkTarget_05e4b2(void);
    ThunkTarget_05e4b2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_031dba")))
void JsrAbsThunk_031dba(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_032aa0")))
void JsrAbsThunk_032aa0(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_032c76")))
void JsrAbsThunk_032c76(void) {
    extern void ThunkTarget_09b9f6(void);
    ThunkTarget_09b9f6();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03336e")))
void JsrAbsThunk_03336e(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_033382")))
void JsrAbsThunk_033382(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03356a")))
void JsrAbsThunk_03356a(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_037c12")))
void JsrAbsThunk_037c12(void) {
    extern void ThunkTarget_0005fe(void);
    ThunkTarget_0005fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_037c6c")))
void JsrAbsThunk_037c6c(void) {
    extern void ThunkTarget_0005fe(void);
    ThunkTarget_0005fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0393c6")))
void JsrAbsThunk_0393c6(void) {
    extern void ThunkTarget_032d00(void);
    ThunkTarget_032d00();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_039448")))
void JsrAbsThunk_039448(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03a602")))
void JsrAbsThunk_03a602(void) {
    extern void ThunkTarget_0283ca(void);
    ThunkTarget_0283ca();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03c62a")))
void JsrAbsThunk_03c62a(void) {
    extern void ThunkTarget_0283ca(void);
    ThunkTarget_0283ca();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03c8d0")))
void JsrAbsThunk_03c8d0(void) {
    extern void ThunkTarget_0517fe(void);
    ThunkTarget_0517fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03da98")))
void JsrAbsThunk_03da98(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e11e")))
void JsrAbsThunk_03e11e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e176")))
void JsrAbsThunk_03e176(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e198")))
void JsrAbsThunk_03e198(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e200")))
void JsrAbsThunk_03e200(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e2ac")))
void JsrAbsThunk_03e2ac(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e2d4")))
void JsrAbsThunk_03e2d4(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e2f6")))
void JsrAbsThunk_03e2f6(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e402")))
void JsrAbsThunk_03e402(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e504")))
void JsrAbsThunk_03e504(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e50c")))
void JsrAbsThunk_03e50c(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e844")))
void JsrAbsThunk_03e844(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e9d4")))
void JsrAbsThunk_03e9d4(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03e9f2")))
void JsrAbsThunk_03e9f2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03fa06")))
void JsrAbsThunk_03fa06(void) {
    extern void ThunkTarget_0004ae(void);
    ThunkTarget_0004ae();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_03ff0c")))
void JsrAbsThunk_03ff0c(void) {
    extern void ThunkTarget_028292(void);
    ThunkTarget_028292();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_041552")))
void JsrAbsThunk_041552(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_041eaa")))
void JsrAbsThunk_041eaa(void) {
    extern void ThunkTarget_0799de(void);
    ThunkTarget_0799de();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_041ee6")))
void JsrAbsThunk_041ee6(void) {
    extern void ThunkTarget_0799de(void);
    ThunkTarget_0799de();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_041f14")))
void JsrAbsThunk_041f14(void) {
    extern void ThunkTarget_0799de(void);
    ThunkTarget_0799de();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_041f32")))
void JsrAbsThunk_041f32(void) {
    extern void ThunkTarget_0799de(void);
    ThunkTarget_0799de();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_042904")))
void JsrAbsThunk_042904(void) {
    extern void ThunkTarget_028292(void);
    ThunkTarget_028292();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_0436d6 absorbido por SceneLoader_Main_043568 (Wave HH#1).
 * FP #30 del proyecto: los 8 bytes en $0436D6..$0436DD son el `jsr $4CB5C.l;
 * rts` final de SceneLoader_Main, no un thunk independiente. */

/* JsrAbsThunk_043dec absorbido por CameraApplyOne_043DAA (Wave JJ#1).
 * FP #40 del proyecto: los 8 bytes en $043DEC..$043DF3 son la cola
 * `jsr $51F30.l; rts` (Transform_Commit) de la aplicacion de camara, y
 * ademas el destino del beq.w del segundo test de flags. No es un thunk. */

__attribute__((section(".text.JsrAbsThunk_044a24")))
void JsrAbsThunk_044a24(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_046926")))
void JsrAbsThunk_046926(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_046a24")))
void JsrAbsThunk_046a24(void) {
    extern void ThunkTarget_047888(void);
    ThunkTarget_047888();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_046b18 absorbido por FixLayer_QuadBatch_046AC6 (Wave HH#3).
 * FP #32 del proyecto: los 8 bytes en $046B18..$046B1F son el `jsr $5DA9C.l;
 * rts` final del batch #4 (columna derecha), no un thunk independiente. */

__attribute__((section(".text.JsrAbsThunk_047096")))
void JsrAbsThunk_047096(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0470b6")))
void JsrAbsThunk_0470b6(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_047682")))
void JsrAbsThunk_047682(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0477f4")))
void JsrAbsThunk_0477f4(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_047844")))
void JsrAbsThunk_047844(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_047880")))
void JsrAbsThunk_047880(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_049a18")))
void JsrAbsThunk_049a18(void) {
    extern void ThunkTarget_027cee(void);
    ThunkTarget_027cee();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_049afe")))
void JsrAbsThunk_049afe(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04a1be")))
void JsrAbsThunk_04a1be(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04a260")))
void JsrAbsThunk_04a260(void) {
    extern void ThunkTarget_027a92(void);
    ThunkTarget_027a92();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04aaca")))
void JsrAbsThunk_04aaca(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04abb8")))
void JsrAbsThunk_04abb8(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04c9a6")))
void JsrAbsThunk_04c9a6(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04c9ce")))
void JsrAbsThunk_04c9ce(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04ca50")))
void JsrAbsThunk_04ca50(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04caa0")))
void JsrAbsThunk_04caa0(void) {
    extern void ThunkTarget_09a7aa(void);
    ThunkTarget_09a7aa();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04cbb0")))
void JsrAbsThunk_04cbb0(void) {
    extern void ThunkTarget_05a9d6(void);
    ThunkTarget_05a9d6();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04d3d8")))
void JsrAbsThunk_04d3d8(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04d44c")))
void JsrAbsThunk_04d44c(void) {
    extern void ThunkTarget_027cee(void);
    ThunkTarget_027cee();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04d582")))
void JsrAbsThunk_04d582(void) {
    extern void ThunkTarget_05e9e4(void);
    ThunkTarget_05e9e4();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04e2a8")))
void JsrAbsThunk_04e2a8(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04e578")))
void JsrAbsThunk_04e578(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04ebb4")))
void JsrAbsThunk_04ebb4(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04fa50")))
void JsrAbsThunk_04fa50(void) {
    extern void ThunkTarget_002c26(void);
    ThunkTarget_002c26();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_04fa68")))
void JsrAbsThunk_04fa68(void) {
    extern void ThunkTarget_002c26(void);
    ThunkTarget_002c26();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_050248 ELIMINADO (era falso positivo de la Wave I).
 * Sus 8 bytes 0x4EB9 0x0005 0x1DE2 0x4E75 son el epilogo interno
 * de Sprite_InvokeBlit8Params @ $05022A (jsr $51de2.l ; rts final),
 * no una funcion independiente. Ver decomp/asm/sprite_invoke.s. */

__attribute__((section(".text.JsrAbsThunk_05080e")))
void JsrAbsThunk_05080e(void) {
    extern void ThunkTarget_0004ae(void);
    ThunkTarget_0004ae();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05081a")))
void JsrAbsThunk_05081a(void) {
    extern void ThunkTarget_0004ae(void);
    ThunkTarget_0004ae();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_051476")))
void JsrAbsThunk_051476(void) {
    extern void ThunkTarget_0283ca(void);
    ThunkTarget_0283ca();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05168e")))
void JsrAbsThunk_05168e(void) {
    extern void ThunkTarget_0004ae(void);
    ThunkTarget_0004ae();
    __asm__ volatile("" ::: "memory");
}

/*
 * JsrAbsThunk_051804 ELIMINADO (falso positivo Wave I).
 *
 * Los 8 bytes 4eb90005ccc84e75 en $051804 no son un thunk independiente,
 * sino el epilogo compartido del helper Entity_CopyField68AndCall_0517FE
 * (Wave V#3) que empieza 6 B antes en $0517FE con move.b $68(a6),$68(a0).
 * Mismo patron forense que Sprite_InvokeBlit8Params absorbiendo el
 * ex-JsrAbsThunk_050248 en la transicion Wave I -> Wave S: MSLUG1
 * reutiliza epilogos entre helper y "thunk", cosa que ningun compilador
 * GCC produce.  Ver asm/entity_copy_field68_call_0517fe.s.
 */

/* ABSORBIDO por Player_IncCounterAt7_051A86 (Wave AA batch 1 #4).
 * La cola `jsr $5170C.l; rts` era la salida propia de #4, contabilizada
 * erroneamente por el escaner de Wave I como thunk independiente.
 * 26o falso positivo del proyecto (mismo idioma que los 25 anteriores).
 *
 * __attribute__((section(".text.JsrAbsThunk_051a9c")))
 * void JsrAbsThunk_051a9c(void) {
 *     extern void ThunkTarget_05170c(void);
 *     ThunkTarget_05170c();
 *     __asm__ volatile("" ::: "memory");
 * }
 */

__attribute__((section(".text.JsrAbsThunk_0522fe")))
void JsrAbsThunk_0522fe(void) {
    extern void ThunkTarget_05dad8(void);
    ThunkTarget_05dad8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_052314")))
void JsrAbsThunk_052314(void) {
    extern void ThunkTarget_05dad8(void);
    ThunkTarget_05dad8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_052764")))
void JsrAbsThunk_052764(void) {
    extern void ThunkTarget_002c26(void);
    ThunkTarget_002c26();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05276e")))
void JsrAbsThunk_05276e(void) {
    extern void ThunkTarget_05239e(void);
    ThunkTarget_05239e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_052778")))
void JsrAbsThunk_052778(void) {
    extern void ThunkTarget_0523b2(void);
    ThunkTarget_0523b2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_052780")))
void JsrAbsThunk_052780(void) {
    extern void ThunkTarget_05dd2a(void);
    ThunkTarget_05dd2a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05278e")))
void JsrAbsThunk_05278e(void) {
    extern void ThunkTarget_0004ae(void);
    ThunkTarget_0004ae();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_052796")))
void JsrAbsThunk_052796(void) {
    extern void ThunkTarget_05026c(void);
    ThunkTarget_05026c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_053dc2")))
void JsrAbsThunk_053dc2(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_053e70")))
void JsrAbsThunk_053e70(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_053e94")))
void JsrAbsThunk_053e94(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_053eb2")))
void JsrAbsThunk_053eb2(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_053eda")))
void JsrAbsThunk_053eda(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_055184")))
void JsrAbsThunk_055184(void) {
    extern void ThunkTarget_002c30(void);
    ThunkTarget_002c30();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0551c8")))
void JsrAbsThunk_0551c8(void) {
    extern void ThunkTarget_002c30(void);
    ThunkTarget_002c30();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05520c")))
void JsrAbsThunk_05520c(void) {
    extern void ThunkTarget_002c30(void);
    ThunkTarget_002c30();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_055250")))
void JsrAbsThunk_055250(void) {
    extern void ThunkTarget_002c30(void);
    ThunkTarget_002c30();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_056172")))
void JsrAbsThunk_056172(void) {
    extern void ThunkTarget_08f308(void);
    ThunkTarget_08f308();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_057438")))
void JsrAbsThunk_057438(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05965e")))
void JsrAbsThunk_05965e(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_059b52")))
void JsrAbsThunk_059b52(void) {
    extern void ThunkTarget_05ca2a(void);
    ThunkTarget_05ca2a();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_05dbd4 absorbido por ListCursor_Reinit_05DBC2 (Wave II#2).
 * FP #38 del proyecto: los 8 bytes en $05DBD4..$05DBDB son la cola
 * `jsr $5DA56.l; rts` del reinit de cursor, no un thunk independiente. */

/* JsrAbsThunk_05dbf8 absorbido por ListCursor_ReinitClipped_05DBDC (Wave II#2).
 * FP #39 del proyecto: los 8 bytes en $05DBF8..$05DBFF son la cola
 * `jsr $5DA9C.l; rts` del borrado de lista, no un thunk independiente. */

__attribute__((section(".text.JsrAbsThunk_05e53c")))
void JsrAbsThunk_05e53c(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05e744")))
void JsrAbsThunk_05e744(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05e75e")))
void JsrAbsThunk_05e75e(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05e7fc")))
void JsrAbsThunk_05e7fc(void) {
    extern void ThunkTarget_05d6c2(void);
    ThunkTarget_05d6c2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05e8ee")))
void JsrAbsThunk_05e8ee(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05e982")))
void JsrAbsThunk_05e982(void) {
    extern void ThunkTarget_05d6c2(void);
    ThunkTarget_05d6c2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05eb3c")))
void JsrAbsThunk_05eb3c(void) {
    extern void ThunkTarget_002c26(void);
    ThunkTarget_002c26();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05efa6")))
void JsrAbsThunk_05efa6(void) {
    extern void ThunkTarget_05d6c2(void);
    ThunkTarget_05d6c2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05f302")))
void JsrAbsThunk_05f302(void) {
    extern void ThunkTarget_05d6c2(void);
    ThunkTarget_05d6c2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05f932")))
void JsrAbsThunk_05f932(void) {
    extern void ThunkTarget_099812(void);
    ThunkTarget_099812();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05fdae")))
void JsrAbsThunk_05fdae(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_05ff6e")))
void JsrAbsThunk_05ff6e(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06005a")))
void JsrAbsThunk_06005a(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_060182")))
void JsrAbsThunk_060182(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06075c")))
void JsrAbsThunk_06075c(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0609cc")))
void JsrAbsThunk_0609cc(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_060e12")))
void JsrAbsThunk_060e12(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_060e20")))
void JsrAbsThunk_060e20(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_060e2e")))
void JsrAbsThunk_060e2e(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_060e3c")))
void JsrAbsThunk_060e3c(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0614de")))
void JsrAbsThunk_0614de(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_061578")))
void JsrAbsThunk_061578(void) {
    extern void ThunkTarget_05a9e2(void);
    ThunkTarget_05a9e2();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_061644")))
void JsrAbsThunk_061644(void) {
    extern void ThunkTarget_027cee(void);
    ThunkTarget_027cee();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0616c2")))
void JsrAbsThunk_0616c2(void) {
    extern void ThunkTarget_027cee(void);
    ThunkTarget_027cee();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_061742")))
void JsrAbsThunk_061742(void) {
    extern void ThunkTarget_027cee(void);
    ThunkTarget_027cee();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06276c")))
void JsrAbsThunk_06276c(void) {
    extern void ThunkTarget_05e5a8(void);
    ThunkTarget_05e5a8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0627c2")))
void JsrAbsThunk_0627c2(void) {
    extern void ThunkTarget_0283d8(void);
    ThunkTarget_0283d8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_062826")))
void JsrAbsThunk_062826(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_062894")))
void JsrAbsThunk_062894(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0628a4")))
void JsrAbsThunk_0628a4(void) {
    extern void ThunkTarget_09a7aa(void);
    ThunkTarget_09a7aa();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06291a")))
void JsrAbsThunk_06291a(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_062a46")))
void JsrAbsThunk_062a46(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0635fa")))
void JsrAbsThunk_0635fa(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_063d66")))
void JsrAbsThunk_063d66(void) {
    extern void ThunkTarget_099812(void);
    ThunkTarget_099812();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_063e8a")))
void JsrAbsThunk_063e8a(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06452c")))
void JsrAbsThunk_06452c(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_065dce")))
void JsrAbsThunk_065dce(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06668a")))
void JsrAbsThunk_06668a(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0676b2")))
void JsrAbsThunk_0676b2(void) {
    extern void ThunkTarget_049fd0(void);
    ThunkTarget_049fd0();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_067f72")))
void JsrAbsThunk_067f72(void) {
    extern void ThunkTarget_027c8c(void);
    ThunkTarget_027c8c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0681fe")))
void JsrAbsThunk_0681fe(void) {
    extern void ThunkTarget_028998(void);
    ThunkTarget_028998();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_068a68")))
void JsrAbsThunk_068a68(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_068ac2")))
void JsrAbsThunk_068ac2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_068b3e")))
void JsrAbsThunk_068b3e(void) {
    extern void ThunkTarget_099812(void);
    ThunkTarget_099812();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_068b5c")))
void JsrAbsThunk_068b5c(void) {
    extern void ThunkTarget_099812(void);
    ThunkTarget_099812();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_068b9a")))
void JsrAbsThunk_068b9a(void) {
    extern void ThunkTarget_099812(void);
    ThunkTarget_099812();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0698a2")))
void JsrAbsThunk_0698a2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06991c")))
void JsrAbsThunk_06991c(void) {
    extern void ThunkTarget_0519be(void);
    ThunkTarget_0519be();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06995e")))
void JsrAbsThunk_06995e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_069d6e")))
void JsrAbsThunk_069d6e(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_069f60")))
void JsrAbsThunk_069f60(void) {
    extern void ThunkTarget_09a7aa(void);
    ThunkTarget_09a7aa();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06a03c")))
void JsrAbsThunk_06a03c(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06a048")))
void JsrAbsThunk_06a048(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06a05a")))
void JsrAbsThunk_06a05a(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06af46")))
void JsrAbsThunk_06af46(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06af60")))
void JsrAbsThunk_06af60(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06b272")))
void JsrAbsThunk_06b272(void) {
    extern void ThunkTarget_027afc(void);
    ThunkTarget_027afc();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06b96c")))
void JsrAbsThunk_06b96c(void) {
    extern void ThunkTarget_0138fe(void);
    ThunkTarget_0138fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06c736")))
void JsrAbsThunk_06c736(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06cd5a")))
void JsrAbsThunk_06cd5a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06d594")))
void JsrAbsThunk_06d594(void) {
    extern void ThunkTarget_0138fe(void);
    ThunkTarget_0138fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06e38c")))
void JsrAbsThunk_06e38c(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06e4a0")))
void JsrAbsThunk_06e4a0(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06e9b8")))
void JsrAbsThunk_06e9b8(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06ef06")))
void JsrAbsThunk_06ef06(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06f0be")))
void JsrAbsThunk_06f0be(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06f0cc")))
void JsrAbsThunk_06f0cc(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06f0da")))
void JsrAbsThunk_06f0da(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06f0e8")))
void JsrAbsThunk_06f0e8(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06f0f6")))
void JsrAbsThunk_06f0f6(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_06f104")))
void JsrAbsThunk_06f104(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0704c4")))
void JsrAbsThunk_0704c4(void) {
    extern void ThunkTarget_049fd0(void);
    ThunkTarget_049fd0();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_070eea")))
void JsrAbsThunk_070eea(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_070efc")))
void JsrAbsThunk_070efc(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_071012")))
void JsrAbsThunk_071012(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07177e")))
void JsrAbsThunk_07177e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_071bde")))
void JsrAbsThunk_071bde(void) {
    extern void SetTaskHandler_04a166(void);
    SetTaskHandler_04a166();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072bc6")))
void JsrAbsThunk_072bc6(void) {
    extern void ThunkTarget_0138fe(void);
    ThunkTarget_0138fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072d70")))
void JsrAbsThunk_072d70(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072d9c")))
void JsrAbsThunk_072d9c(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072db0")))
void JsrAbsThunk_072db0(void) {
    extern void ThunkTarget_06e412(void);
    ThunkTarget_06e412();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072e0c")))
void JsrAbsThunk_072e0c(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072e78")))
void JsrAbsThunk_072e78(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072ee0")))
void JsrAbsThunk_072ee0(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_072f38")))
void JsrAbsThunk_072f38(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_073402")))
void JsrAbsThunk_073402(void) {
    extern void ThunkTarget_0283d8(void);
    ThunkTarget_0283d8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_074224")))
void JsrAbsThunk_074224(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0743a4")))
void JsrAbsThunk_0743a4(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0743d6")))
void JsrAbsThunk_0743d6(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_074400")))
void JsrAbsThunk_074400(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07440e")))
void JsrAbsThunk_07440e(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0745a8")))
void JsrAbsThunk_0745a8(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0745b6")))
void JsrAbsThunk_0745b6(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07685a")))
void JsrAbsThunk_07685a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_076952")))
void JsrAbsThunk_076952(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_076b9e")))
void JsrAbsThunk_076b9e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_076c6c")))
void JsrAbsThunk_076c6c(void) {
    extern void ThunkTarget_05ca2a(void);
    ThunkTarget_05ca2a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_076d0e")))
void JsrAbsThunk_076d0e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0775d2")))
void JsrAbsThunk_0775d2(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0775e4")))
void JsrAbsThunk_0775e4(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_078210")))
void JsrAbsThunk_078210(void) {
    extern void ThunkTarget_0283d8(void);
    ThunkTarget_0283d8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_078dee")))
void JsrAbsThunk_078dee(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07a7e0")))
void JsrAbsThunk_07a7e0(void) {
    extern void ThunkTarget_05dad8(void);
    ThunkTarget_05dad8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07c860")))
void JsrAbsThunk_07c860(void) {
    extern void ThunkTarget_0283d8(void);
    ThunkTarget_0283d8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07ef42")))
void JsrAbsThunk_07ef42(void) {
    extern void ThunkTarget_049fd0(void);
    ThunkTarget_049fd0();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07f272")))
void JsrAbsThunk_07f272(void) {
    extern void SetTaskHandler_04a166(void);
    SetTaskHandler_04a166();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_07f664")))
void JsrAbsThunk_07f664(void) {
    extern void SetTaskHandler_04a166(void);
    SetTaskHandler_04a166();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_080082")))
void JsrAbsThunk_080082(void) {
    extern void ThunkTarget_049fd0(void);
    ThunkTarget_049fd0();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0809c4")))
void JsrAbsThunk_0809c4(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_080a38")))
void JsrAbsThunk_080a38(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_080aa2")))
void JsrAbsThunk_080aa2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0817e4")))
void JsrAbsThunk_0817e4(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0817f2")))
void JsrAbsThunk_0817f2(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_081800")))
void JsrAbsThunk_081800(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08180e")))
void JsrAbsThunk_08180e(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08181c")))
void JsrAbsThunk_08181c(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_081832")))
void JsrAbsThunk_081832(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_081840")))
void JsrAbsThunk_081840(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_081856")))
void JsrAbsThunk_081856(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_081864")))
void JsrAbsThunk_081864(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0818a2")))
void JsrAbsThunk_0818a2(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_0818e4 ABSORBIDO por Entity_Build4FromTemplates_0818AA (Wave Y#11).
 * Los 8 bytes en $0818E4..$0818EB (`jsr $5dd02.l; rts`) NO son un thunk
 * independiente: son las dos ultimas instrucciones del constructor batch
 * de 4 entities. Noveno falso positivo del proyecto, mismo patron que
 * JsrAbsThunk_000762 (W#16) y JsrAbsThunk_00211e (Y#8).
 */
#if 0
__attribute__((section(".text.JsrAbsThunk_0818e4")))
void JsrAbsThunk_0818e4(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}
#endif

__attribute__((section(".text.JsrAbsThunk_081da8")))
void JsrAbsThunk_081da8(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_082022")))
void JsrAbsThunk_082022(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0823fa")))
void JsrAbsThunk_0823fa(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08325a")))
void JsrAbsThunk_08325a(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_083268")))
void JsrAbsThunk_083268(void) {
    extern void ThunkTarget_077c7e(void);
    ThunkTarget_077c7e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0832ae")))
void JsrAbsThunk_0832ae(void) {
    extern void Stub_000434CE(void);
    Stub_000434CE();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0832c8")))
void JsrAbsThunk_0832c8(void) {
    extern void ThunkTarget_0283ca(void);
    ThunkTarget_0283ca();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_083bda")))
void JsrAbsThunk_083bda(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08492a")))
void JsrAbsThunk_08492a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_084b5a")))
void JsrAbsThunk_084b5a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_084c9c")))
void JsrAbsThunk_084c9c(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0851e8")))
void JsrAbsThunk_0851e8(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085268")))
void JsrAbsThunk_085268(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085330")))
void JsrAbsThunk_085330(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08540a")))
void JsrAbsThunk_08540a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085bd0")))
void JsrAbsThunk_085bd0(void) {
    extern void ThunkTarget_002c26(void);
    ThunkTarget_002c26();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085c2a")))
void JsrAbsThunk_085c2a(void) {
    extern void ThunkTarget_002c26(void);
    ThunkTarget_002c26();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085cfc")))
void JsrAbsThunk_085cfc(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085d22")))
void JsrAbsThunk_085d22(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085d2a")))
void JsrAbsThunk_085d2a(void) {
    extern void ThunkTarget_08b558(void);
    ThunkTarget_08b558();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_085f58")))
void JsrAbsThunk_085f58(void) {
    extern void ThunkTarget_099812(void);
    ThunkTarget_099812();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_086034")))
void JsrAbsThunk_086034(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_086048")))
void JsrAbsThunk_086048(void) {
    extern void InputGuardCall219c(void);
    InputGuardCall219c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08606e")))
void JsrAbsThunk_08606e(void) {
    extern void ThunkTarget_027cee(void);
    ThunkTarget_027cee();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0863ce")))
void JsrAbsThunk_0863ce(void) {
    extern void ThunkTarget_0283d8(void);
    ThunkTarget_0283d8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0864c8")))
void JsrAbsThunk_0864c8(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0864e2")))
void JsrAbsThunk_0864e2(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0864fc")))
void JsrAbsThunk_0864fc(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_086516")))
void JsrAbsThunk_086516(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_086530")))
void JsrAbsThunk_086530(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08654a")))
void JsrAbsThunk_08654a(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08783e")))
void JsrAbsThunk_08783e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0895f6")))
void JsrAbsThunk_0895f6(void) {
    extern void ThunkTarget_0283d8(void);
    ThunkTarget_0283d8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0896d6")))
void JsrAbsThunk_0896d6(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_089832")))
void JsrAbsThunk_089832(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_089894")))
void JsrAbsThunk_089894(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_089ac0")))
void JsrAbsThunk_089ac0(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ae60")))
void JsrAbsThunk_08ae60(void) {
    extern void ThunkTarget_02783a(void);
    ThunkTarget_02783a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08b06c")))
void JsrAbsThunk_08b06c(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_08b586 ABSORBIDO por Handler_ConditionalHitCounter_08B558 (Wave Z batch 2 #14).
 * Los 8 B en $08B586..$08B58D (`jsr $6E224.l; rts`) son la cola tail-call
 * del handler condicional. 22 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.JsrAbsThunk_08b586")))
void JsrAbsThunk_08b586(void) {
    extern void ThunkTarget_06e224(void);
    ThunkTarget_06e224();
    __asm__ volatile("" ::: "memory");
}
#endif

__attribute__((section(".text.JsrAbsThunk_08b770")))
void JsrAbsThunk_08b770(void) {
    extern void ThunkTarget_027cee(void);
    ThunkTarget_027cee();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08b906")))
void JsrAbsThunk_08b906(void) {
    extern void ThunkTarget_043fac(void);
    ThunkTarget_043fac();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08b920")))
void JsrAbsThunk_08b920(void) {
    extern void StateMachineRun(void);
    StateMachineRun();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ba04")))
void JsrAbsThunk_08ba04(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ba4a")))
void JsrAbsThunk_08ba4a(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ba96")))
void JsrAbsThunk_08ba96(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08bb56")))
void JsrAbsThunk_08bb56(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08bb7c")))
void JsrAbsThunk_08bb7c(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

/* JsrAbsThunk_08c156/1a4/1e2/232/28e ABSORBIDOS por Anim_State_F1..F5 (Wave GG batch 2 - FPs #32-#36)
 * Colas `jsr $28d70.l; rts` (8 B c/u) que forman la salida tail-common de
 * cada handler F1..F5 del cluster $08Cxxx. No son thunks independientes. */

/* JsrAbsThunk_08c2b0 ABSORBIDO por Anim_State_F6_08C296 (Wave GG batch 2 - FP #37)
 * Cola `jsr $436de.l; rts` (8 B) del handler F6 (tail solo con 2 jsr, no 3
 * como F1..F5). $08C2B0..$08C2B7 es el final de F6. */

__attribute__((section(".text.JsrAbsThunk_08c31a")))
void JsrAbsThunk_08c31a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08c376")))
void JsrAbsThunk_08c376(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08c3d2")))
void JsrAbsThunk_08c3d2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08c42e")))
void JsrAbsThunk_08c42e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08c4c2")))
void JsrAbsThunk_08c4c2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08c4f2")))
void JsrAbsThunk_08c4f2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08c5aa")))
void JsrAbsThunk_08c5aa(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ccbe")))
void JsrAbsThunk_08ccbe(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08cd98")))
void JsrAbsThunk_08cd98(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08cdf4")))
void JsrAbsThunk_08cdf4(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ce16")))
void JsrAbsThunk_08ce16(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ce5c")))
void JsrAbsThunk_08ce5c(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d0a0")))
void JsrAbsThunk_08d0a0(void) {
    extern void ThunkTarget_096a80(void);
    ThunkTarget_096a80();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d17a")))
void JsrAbsThunk_08d17a(void) {
    extern void ThunkTarget_096a80(void);
    ThunkTarget_096a80();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d578")))
void JsrAbsThunk_08d578(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d6e4")))
void JsrAbsThunk_08d6e4(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d7da")))
void JsrAbsThunk_08d7da(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d7fc")))
void JsrAbsThunk_08d7fc(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d892")))
void JsrAbsThunk_08d892(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d8be")))
void JsrAbsThunk_08d8be(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d930")))
void JsrAbsThunk_08d930(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d97e")))
void JsrAbsThunk_08d97e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d98c")))
void JsrAbsThunk_08d98c(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08d9da")))
void JsrAbsThunk_08d9da(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08da28")))
void JsrAbsThunk_08da28(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08da96")))
void JsrAbsThunk_08da96(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08dada")))
void JsrAbsThunk_08dada(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08db48")))
void JsrAbsThunk_08db48(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ded8")))
void JsrAbsThunk_08ded8(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08df42")))
void JsrAbsThunk_08df42(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08df8c")))
void JsrAbsThunk_08df8c(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e0b2")))
void JsrAbsThunk_08e0b2(void) {
    extern void ThunkTarget_000772(void);
    ThunkTarget_000772();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e104")))
void JsrAbsThunk_08e104(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e194")))
void JsrAbsThunk_08e194(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e1f2")))
void JsrAbsThunk_08e1f2(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e3bc")))
void JsrAbsThunk_08e3bc(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e406")))
void JsrAbsThunk_08e406(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e49a")))
void JsrAbsThunk_08e49a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e654")))
void JsrAbsThunk_08e654(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e6b0")))
void JsrAbsThunk_08e6b0(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e6d8")))
void JsrAbsThunk_08e6d8(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08e730")))
void JsrAbsThunk_08e730(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08edbe")))
void JsrAbsThunk_08edbe(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ee0e")))
void JsrAbsThunk_08ee0e(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ee72")))
void JsrAbsThunk_08ee72(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ef38")))
void JsrAbsThunk_08ef38(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08f07c")))
void JsrAbsThunk_08f07c(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08f0c8")))
void JsrAbsThunk_08f0c8(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08f100")))
void JsrAbsThunk_08f100(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_08ffee")))
void JsrAbsThunk_08ffee(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0900dc")))
void JsrAbsThunk_0900dc(void) {
    extern void ThunkTarget_002c30(void);
    ThunkTarget_002c30();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_090164")))
void JsrAbsThunk_090164(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0902f6")))
void JsrAbsThunk_0902f6(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0904dc")))
void JsrAbsThunk_0904dc(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09058a")))
void JsrAbsThunk_09058a(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0905ba")))
void JsrAbsThunk_0905ba(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_090628")))
void JsrAbsThunk_090628(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_090684")))
void JsrAbsThunk_090684(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0906e0")))
void JsrAbsThunk_0906e0(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_090c0a")))
void JsrAbsThunk_090c0a(void) {
    extern void ThunkTarget_05dd02(void);
    ThunkTarget_05dd02();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_090ef8")))
void JsrAbsThunk_090ef8(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_091128")))
void JsrAbsThunk_091128(void) {
    extern void ThunkTarget_0006fe(void);
    ThunkTarget_0006fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09150c")))
void JsrAbsThunk_09150c(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0915d4")))
void JsrAbsThunk_0915d4(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_091628")))
void JsrAbsThunk_091628(void) {
    extern void ThunkTarget_05dad8(void);
    ThunkTarget_05dad8();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_0916c0")))
void JsrAbsThunk_0916c0(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

/*
 * JsrAbsThunk_096b1c ELIMINADO (falso positivo Wave I).
 *
 * Los 8 bytes 4eb90005d6c2 4e75 en $096B1C no son un thunk independiente,
 * sino el cierre `jsr Sprite_HexFormat4_05D6C2; rts` del helper de HUD
 * debug Debug_DrawHUDVars_096A80 (Wave X#1) que empieza 156 B antes.
 *
 * OCTAVO falso positivo por reuso de epilogos detectado, tras:
 *   1. ex-JsrAbsThunk_050248    (S: Sprite_InvokeBlit8Params)
 *   2. ex-JsrAbsThunk_051804    (V#3: Entity_CopyField68AndCall_0517FE)
 *   3. ex-SetTaskHandler_049fea (V#8: Entity_ProbeAndInstallHandler_049FD0)
 *   4. ex-ClearXN_09a7c6        (W#8: Entity_ProbeMoveX_09A7AA)
 *   5. ex-ClearXN_0282d2        (W#12: Entity_SwapProbeCommit_028292)
 *   6. ex-JsrAbsThunk_000762    (W#16: Entity_AllocFromFreeList_0006FE)
 *   7. este                      (X#1: Debug_DrawHUDVars_096A80)
 *
 * Ver asm/debug_draw_hud_vars_096a80.s para el cuerpo real.
 */

/* JsrAbsThunk_096b76 absorbido por DebugTriggers_TwoBits_096B24 (Wave HH#2).
 * FP #31 del proyecto: los 8 bytes en $096B76..$096B7D son el `jsr $4AE.l;
 * rts` final del segundo bloque debug-trigger, no un thunk independiente. */

__attribute__((section(".text.JsrAbsThunk_0979f0")))
void JsrAbsThunk_0979f0(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_097a40")))
void JsrAbsThunk_097a40(void) {
    extern void ThunkTarget_05db1a(void);
    ThunkTarget_05db1a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_097bfa")))
void JsrAbsThunk_097bfa(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_097cbc")))
void JsrAbsThunk_097cbc(void) {
    extern void ThunkTarget_05da56(void);
    ThunkTarget_05da56();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09813c")))
void JsrAbsThunk_09813c(void) {
    extern void ThunkTarget_05da9c(void);
    ThunkTarget_05da9c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_098982")))
void JsrAbsThunk_098982(void) {
    extern void ThunkTarget_002c30(void);
    ThunkTarget_002c30();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_099abe")))
void JsrAbsThunk_099abe(void) {
    extern void ThunkTarget_05ca2a(void);
    ThunkTarget_05ca2a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_099fea")))
void JsrAbsThunk_099fea(void) {
    extern void ThunkTarget_04784c(void);
    ThunkTarget_04784c();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09a0b4")))
void JsrAbsThunk_09a0b4(void) {
    extern void ThunkTarget_0477fc(void);
    ThunkTarget_0477fc();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09a876")))
void JsrAbsThunk_09a876(void) {
    extern void ThunkTarget_032cba(void);
    ThunkTarget_032cba();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09a8b6")))
void JsrAbsThunk_09a8b6(void) {
    extern void EntitySetSpriteMap(void);
    EntitySetSpriteMap();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09b902")))
void JsrAbsThunk_09b902(void) {
    extern void ThunkTarget_099812(void);
    ThunkTarget_099812();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09c44c")))
void JsrAbsThunk_09c44c(void) {
    extern void ThunkTarget_05ca2a(void);
    ThunkTarget_05ca2a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09c4b0")))
void JsrAbsThunk_09c4b0(void) {
    extern void ThunkTarget_05ca2a(void);
    ThunkTarget_05ca2a();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_09c600")))
void JsrAbsThunk_09c600(void) {
    extern void ThunkTarget_032b36(void);
    ThunkTarget_032b36();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_18d56c")))
void JsrAbsThunk_18d56c(void) {
    extern void ThunkTarget_0517fe(void);
    ThunkTarget_0517fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_18d57e")))
void JsrAbsThunk_18d57e(void) {
    extern void ThunkTarget_0517fe(void);
    ThunkTarget_0517fe();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_18d746")))
void JsrAbsThunk_18d746(void) {
    extern void ThunkTarget_0283ca(void);
    ThunkTarget_0283ca();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_18d766")))
void JsrAbsThunk_18d766(void) {
    extern void ThunkTarget_028d70(void);
    ThunkTarget_028d70();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_18d9d4")))
void JsrAbsThunk_18d9d4(void) {
    extern void ThunkTarget_032afa(void);
    ThunkTarget_032afa();
    __asm__ volatile("" ::: "memory");
}

__attribute__((section(".text.JsrAbsThunk_19c95a")))
void JsrAbsThunk_19c95a(void) {
    extern void ThunkTarget_00236e(void);
    ThunkTarget_00236e();
    __asm__ volatile("" ::: "memory");
}

