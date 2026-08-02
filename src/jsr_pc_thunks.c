/*
 * Metal Slug 1 — Familia JsrPcThunk (trampolines PC-relativos, 6 B)
 * Auto-generado por gen_jsr_pc_thunks.py. Requiere -mpcrel.
 */
#include "mslug.h"

__attribute__((section(".text.JsrPcThunk_001096")))
void JsrPcThunk_001096(void) {
    extern void PcThunkTarget_001af8(void);
    __asm__ volatile("jsr PcThunkTarget_001af8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0010ec")))
void JsrPcThunk_0010ec(void) {
    extern void PcThunkTarget_001af8(void);
    __asm__ volatile("jsr PcThunkTarget_001af8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

/* JsrPcThunk_0012ee ABSORBIDO por Init_ModeToggle_001260 (Wave EE batch 1 - FP #28)
 * $0012EE..$0012F3 (6 B: jsr $1af8(pc); rts) es la cola compartida de la rama
 * .Ltail_pending de Init_ModeToggle_001260, no un thunk independiente. */

__attribute__((section(".text.JsrPcThunk_00134e")))
void JsrPcThunk_00134e(void) {
    extern void PcThunkTarget_001af8(void);
    __asm__ volatile("jsr PcThunkTarget_001af8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

/* JsrPcThunk_01394c absorbido por Scratch_Alloc_01390E (Wave JJ#2).
 * FP #41 del proyecto: los 6 bytes en $01394C..$013951 son la cola
 * `jsr $1399C(pc); rts` (SpriteRange_DisableAll) del particionado de
 * pools de sprites, no un thunk independiente. */

__attribute__((section(".text.JsrPcThunk_013ac2")))
void JsrPcThunk_013ac2(void) {
    extern void PcThunkTarget_01399c(void);
    __asm__ volatile("jsr PcThunkTarget_01399c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_013ad8")))
void JsrPcThunk_013ad8(void) {
    extern void PcThunkTarget_01399c(void);
    __asm__ volatile("jsr PcThunkTarget_01399c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_025dcc")))
void JsrPcThunk_025dcc(void) {
    extern void PcThunkTarget_025e74(void);
    __asm__ volatile("jsr PcThunkTarget_025e74(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02698c")))
void JsrPcThunk_02698c(void) {
    extern void PcThunkTarget_0281c8(void);
    __asm__ volatile("jsr PcThunkTarget_0281c8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

/* JsrPcThunk_027886 ABSORBIDO por Entity_Probe_Scratch_02785C (Wave Z#4).
 * Los 6 B en $027886..$02788B (`jsr $28108(pc); rts`) son la cola del
 * helper probe/scratch. 11° falso positivo del proyecto (mismo patron
 * que los absorbidos en W#16, Y#8, Y#11).
 * $028108 = Entity_ApplyFadeShade_028108 (promovido en Wave QQ#2).
 */
#if 0
__attribute__((section(".text.JsrPcThunk_027886")))
void JsrPcThunk_027886(void) {
    extern void Entity_ApplyFadeShade_028108(void);
    __asm__ volatile("jsr Entity_ApplyFadeShade_028108(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}
#endif

__attribute__((section(".text.JsrPcThunk_0294aa")))
void JsrPcThunk_0294aa(void) {
    extern void EntitySetSpriteMap(void);
    __asm__ volatile("jsr EntitySetSpriteMap(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_029582")))
void JsrPcThunk_029582(void) {
    extern void PcThunkTarget_02a46c(void);
    __asm__ volatile("jsr PcThunkTarget_02a46c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02aae4")))
void JsrPcThunk_02aae4(void) {
    extern void PcThunkTarget_02ab86(void);
    __asm__ volatile("jsr PcThunkTarget_02ab86(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02ac40")))
void JsrPcThunk_02ac40(void) {
    extern void PcThunkTarget_02ac4c(void);
    __asm__ volatile("jsr PcThunkTarget_02ac4c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02bcfa")))
void JsrPcThunk_02bcfa(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02db4c")))
void JsrPcThunk_02db4c(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02dc56")))
void JsrPcThunk_02dc56(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02dca4")))
void JsrPcThunk_02dca4(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02dd1a")))
void JsrPcThunk_02dd1a(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02f1fc")))
void JsrPcThunk_02f1fc(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02fe64")))
void JsrPcThunk_02fe64(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02fed4")))
void JsrPcThunk_02fed4(void) {
    extern void PcThunkTarget_02ac80(void);
    __asm__ volatile("jsr PcThunkTarget_02ac80(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_02ff1c")))
void JsrPcThunk_02ff1c(void) {
    extern void PcThunkTarget_02fada(void);
    __asm__ volatile("jsr PcThunkTarget_02fada(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_032406")))
void JsrPcThunk_032406(void) {
    extern void PcThunkTarget_032ea4(void);
    __asm__ volatile("jsr PcThunkTarget_032ea4(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03240c")))
void JsrPcThunk_03240c(void) {
    extern void PcThunkTarget_032eba(void);
    __asm__ volatile("jsr PcThunkTarget_032eba(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_033010")))
void JsrPcThunk_033010(void) {
    extern void PcThunkTarget_032f88(void);
    __asm__ volatile("jsr PcThunkTarget_032f88(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03302e")))
void JsrPcThunk_03302e(void) {
    extern void PcThunkTarget_032f3c(void);
    __asm__ volatile("jsr PcThunkTarget_032f3c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_033868")))
void JsrPcThunk_033868(void) {
    extern void PcThunkTarget_0334a2(void);
    __asm__ volatile("jsr PcThunkTarget_0334a2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0338f4")))
void JsrPcThunk_0338f4(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03394c")))
void JsrPcThunk_03394c(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_033a58")))
void JsrPcThunk_033a58(void) {
    extern void PcThunkTarget_0334a2(void);
    __asm__ volatile("jsr PcThunkTarget_0334a2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_034d2c")))
void JsrPcThunk_034d2c(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0354ec")))
void JsrPcThunk_0354ec(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_036632")))
void JsrPcThunk_036632(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0366f8")))
void JsrPcThunk_0366f8(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0368da")))
void JsrPcThunk_0368da(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_036c86")))
void JsrPcThunk_036c86(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_036fbc")))
void JsrPcThunk_036fbc(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_037012")))
void JsrPcThunk_037012(void) {
    extern void PcThunkTarget_036dca(void);
    __asm__ volatile("jsr PcThunkTarget_036dca(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_037054")))
void JsrPcThunk_037054(void) {
    extern void PcThunkTarget_036dca(void);
    __asm__ volatile("jsr PcThunkTarget_036dca(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_037db4")))
void JsrPcThunk_037db4(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_037ebc")))
void JsrPcThunk_037ebc(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03803e")))
void JsrPcThunk_03803e(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03819c")))
void JsrPcThunk_03819c(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_038554")))
void JsrPcThunk_038554(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_038682")))
void JsrPcThunk_038682(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_038a22")))
void JsrPcThunk_038a22(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_038ae0")))
void JsrPcThunk_038ae0(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_038bde")))
void JsrPcThunk_038bde(void) {
    extern void PcThunkTarget_033522(void);
    __asm__ volatile("jsr PcThunkTarget_033522(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_039616")))
void JsrPcThunk_039616(void) {
    extern void PcThunkTarget_039416(void);
    __asm__ volatile("jsr PcThunkTarget_039416(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0396aa")))
void JsrPcThunk_0396aa(void) {
    extern void PcThunkTarget_039416(void);
    __asm__ volatile("jsr PcThunkTarget_039416(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03973e")))
void JsrPcThunk_03973e(void) {
    extern void PcThunkTarget_039416(void);
    __asm__ volatile("jsr PcThunkTarget_039416(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0397d6")))
void JsrPcThunk_0397d6(void) {
    extern void PcThunkTarget_039416(void);
    __asm__ volatile("jsr PcThunkTarget_039416(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03983c")))
void JsrPcThunk_03983c(void) {
    extern void PcThunkTarget_039416(void);
    __asm__ volatile("jsr PcThunkTarget_039416(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0398a2")))
void JsrPcThunk_0398a2(void) {
    extern void PcThunkTarget_039416(void);
    __asm__ volatile("jsr PcThunkTarget_039416(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03e6d4")))
void JsrPcThunk_03e6d4(void) {
    extern void PcThunkTarget_03e7a6(void);
    __asm__ volatile("jsr PcThunkTarget_03e7a6(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_03e7ba")))
void JsrPcThunk_03e7ba(void) {
    extern void PcThunkTarget_03e84c(void);
    __asm__ volatile("jsr PcThunkTarget_03e84c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0404e8")))
void JsrPcThunk_0404e8(void) {
    extern void PcThunkTarget_041e02(void);
    __asm__ volatile("jsr PcThunkTarget_041e02(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_040526")))
void JsrPcThunk_040526(void) {
    extern void PcThunkTarget_041e02(void);
    __asm__ volatile("jsr PcThunkTarget_041e02(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_040e08")))
void JsrPcThunk_040e08(void) {
    extern void PcThunkTarget_041ddc(void);
    __asm__ volatile("jsr PcThunkTarget_041ddc(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04138a")))
void JsrPcThunk_04138a(void) {
    extern void PcThunkTarget_041ff6(void);
    __asm__ volatile("jsr PcThunkTarget_041ff6(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_041402")))
void JsrPcThunk_041402(void) {
    extern void PcThunkTarget_041ff6(void);
    __asm__ volatile("jsr PcThunkTarget_041ff6(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_041cda")))
void JsrPcThunk_041cda(void) {
    extern void PcThunkTarget_041c1a(void);
    __asm__ volatile("jsr PcThunkTarget_041c1a(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_042182")))
void JsrPcThunk_042182(void) {
    extern void PcThunkTarget_042040(void);
    __asm__ volatile("jsr PcThunkTarget_042040(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0469dc")))
void JsrPcThunk_0469dc(void) {
    extern void PcThunkTarget_04698c(void);
    __asm__ volatile("jsr PcThunkTarget_04698c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04711e")))
void JsrPcThunk_04711e(void) {
    extern void PcThunkTarget_04707e(void);
    __asm__ volatile("jsr PcThunkTarget_04707e(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04df2a")))
void JsrPcThunk_04df2a(void) {
    extern void PcThunkTarget_04faf8(void);
    __asm__ volatile("jsr PcThunkTarget_04faf8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04df92")))
void JsrPcThunk_04df92(void) {
    extern void PcThunkTarget_04faf8(void);
    __asm__ volatile("jsr PcThunkTarget_04faf8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fa9a")))
void JsrPcThunk_04fa9a(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fab0")))
void JsrPcThunk_04fab0(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fac6")))
void JsrPcThunk_04fac6(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fadc")))
void JsrPcThunk_04fadc(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04faf2")))
void JsrPcThunk_04faf2(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fb08")))
void JsrPcThunk_04fb08(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fb1e")))
void JsrPcThunk_04fb1e(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fb34")))
void JsrPcThunk_04fb34(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fb6e")))
void JsrPcThunk_04fb6e(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_04fb84")))
void JsrPcThunk_04fb84(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_050224")))
void JsrPcThunk_050224(void) {
    extern void StateMachineRun(void);
    __asm__ volatile("jsr StateMachineRun(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_053a3c")))
void JsrPcThunk_053a3c(void) {
    extern void PcThunkTarget_053dca(void);
    __asm__ volatile("jsr PcThunkTarget_053dca(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_055108")))
void JsrPcThunk_055108(void) {
    extern void PcThunkTarget_055214(void);
    __asm__ volatile("jsr PcThunkTarget_055214(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_05511c")))
void JsrPcThunk_05511c(void) {
    extern void PcThunkTarget_055148(void);
    __asm__ volatile("jsr PcThunkTarget_055148(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_055142")))
void JsrPcThunk_055142(void) {
    extern void PcThunkTarget_055148(void);
    __asm__ volatile("jsr PcThunkTarget_055148(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0577b6")))
void JsrPcThunk_0577b6(void) {
    extern void PcThunkTarget_056e1e(void);
    __asm__ volatile("jsr PcThunkTarget_056e1e(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_057838")))
void JsrPcThunk_057838(void) {
    extern void PcThunkTarget_056e1e(void);
    __asm__ volatile("jsr PcThunkTarget_056e1e(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_05d30a")))
void JsrPcThunk_05d30a(void) {
    extern void PcThunkTarget_05cf6c(void);
    __asm__ volatile("jsr PcThunkTarget_05cf6c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_05d6aa")))
void JsrPcThunk_05d6aa(void) {
    extern void ThunkTarget_05d6c2(void);
    __asm__ volatile("jsr ThunkTarget_05d6c2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

/* JsrPcThunk_05db64 absorbido por InstallListPubHead_05DB58 (Wave II#1).
 * FP #35 del proyecto: los 6 bytes en $05DB64..$05DB69 son la cola
 * `jsr $5DBC2(pc); rts` de la instalacion de lista, no un thunk propio. */

__attribute__((section(".text.JsrPcThunk_05e080")))
void JsrPcThunk_05e080(void) {
    extern void PcThunkTarget_05e018(void);
    __asm__ volatile("jsr PcThunkTarget_05e018(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_05e54c")))
void JsrPcThunk_05e54c(void) {
    extern void PcThunkTarget_05e530(void);
    __asm__ volatile("jsr PcThunkTarget_05e530(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_05e8ce")))
void JsrPcThunk_05e8ce(void) {
    extern void PcThunkTarget_05dd5c(void);
    __asm__ volatile("jsr PcThunkTarget_05dd5c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_062b7e")))
void JsrPcThunk_062b7e(void) {
    extern void PcThunkTarget_063336(void);
    __asm__ volatile("jsr PcThunkTarget_063336(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0634f0")))
void JsrPcThunk_0634f0(void) {
    extern void PcThunkTarget_0634f6(void);
    __asm__ volatile("jsr PcThunkTarget_0634f6(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_065eee")))
void JsrPcThunk_065eee(void) {
    extern void EntityGroup_SpawnLinkedFromTemplateList_065C94(void);
    __asm__ volatile("jsr EntityGroup_SpawnLinkedFromTemplateList_065C94(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_065efa")))
void JsrPcThunk_065efa(void) {
    extern void EntityGroup_SpawnLinkedFromTemplateList_065C94(void);
    __asm__ volatile("jsr EntityGroup_SpawnLinkedFromTemplateList_065C94(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_065f06")))
void JsrPcThunk_065f06(void) {
    extern void EntityGroup_SpawnLinkedFromTemplateList_065C94(void);
    __asm__ volatile("jsr EntityGroup_SpawnLinkedFromTemplateList_065C94(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_065f12")))
void JsrPcThunk_065f12(void) {
    extern void EntityGroup_SpawnLinkedFromTemplateList_065C94(void);
    __asm__ volatile("jsr EntityGroup_SpawnLinkedFromTemplateList_065C94(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_065f1e")))
void JsrPcThunk_065f1e(void) {
    extern void EntityGroup_SpawnLinkedFromTemplateList_065C94(void);
    __asm__ volatile("jsr EntityGroup_SpawnLinkedFromTemplateList_065C94(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_068398")))
void JsrPcThunk_068398(void) {
    extern void Camera0_RelinkAndWrapScroll_06896A(void);
    __asm__ volatile("jsr Camera0_RelinkAndWrapScroll_06896A(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0683ae")))
void JsrPcThunk_0683ae(void) {
    extern void Camera0_RelinkAndWrapScroll_06896A(void);
    __asm__ volatile("jsr Camera0_RelinkAndWrapScroll_06896A(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0683ea")))
void JsrPcThunk_0683ea(void) {
    extern void Camera0_RelinkAndWrapScroll_06896A(void);
    __asm__ volatile("jsr Camera0_RelinkAndWrapScroll_06896A(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_068724")))
void JsrPcThunk_068724(void) {
    extern void PcThunkTarget_068ab8(void);
    __asm__ volatile("jsr PcThunkTarget_068ab8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_068758")))
void JsrPcThunk_068758(void) {
    extern void PcThunkTarget_068ab8(void);
    __asm__ volatile("jsr PcThunkTarget_068ab8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0687ae")))
void JsrPcThunk_0687ae(void) {
    extern void PcThunkTarget_068ab8(void);
    __asm__ volatile("jsr PcThunkTarget_068ab8(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_06d130")))
void JsrPcThunk_06d130(void) {
    extern void PcThunkTarget_06d13c(void);
    __asm__ volatile("jsr PcThunkTarget_06d13c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

/* JsrPcThunk_06e244 ABSORBIDO por Entity_SpawnAndTag_06E224 (Wave Z batch 2 #7).
 * Los 6 B en $06E244..$06E249 (`jsr Entity_CopyAnimFromLeader_06E2BC(pc); rts`) son la
 * cola del spawner con tag. 23 falso positivo del proyecto.
 */
#if 0
__attribute__((section(".text.JsrPcThunk_06e244")))
void JsrPcThunk_06e244(void) {
    extern void Entity_CopyAnimFromLeader_06E2BC(void);
    __asm__ volatile("jsr Entity_CopyAnimFromLeader_06E2BC(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}
#endif

__attribute__((section(".text.JsrPcThunk_06e26a")))
void JsrPcThunk_06e26a(void) {
    extern void Entity_CopyAnimFromLeader_06E2BC(void);
    __asm__ volatile("jsr Entity_CopyAnimFromLeader_06E2BC(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_06e290")))
void JsrPcThunk_06e290(void) {
    extern void Entity_CopyAnimFromLeader_06E2BC(void);
    __asm__ volatile("jsr Entity_CopyAnimFromLeader_06E2BC(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_06e2b6")))
void JsrPcThunk_06e2b6(void) {
    extern void Entity_CopyAnimFromLeader_06E2BC(void);
    __asm__ volatile("jsr Entity_CopyAnimFromLeader_06E2BC(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_06fab4")))
void JsrPcThunk_06fab4(void) {
    extern void PcThunkTarget_070ab0(void);
    __asm__ volatile("jsr PcThunkTarget_070ab0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_072826")))
void JsrPcThunk_072826(void) {
    extern void PcThunkTarget_072a94(void);
    __asm__ volatile("jsr PcThunkTarget_072a94(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_072c92")))
void JsrPcThunk_072c92(void) {
    extern void Entity_CheckActiveBoxOverlap_072C98(void);
    __asm__ volatile("jsr Entity_CheckActiveBoxOverlap_072C98(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_074116")))
void JsrPcThunk_074116(void) {
    extern void PcThunkTarget_074166(void);
    __asm__ volatile("jsr PcThunkTarget_074166(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_07526a")))
void JsrPcThunk_07526a(void) {
    extern void PcThunkTarget_0745e2(void);
    __asm__ volatile("jsr PcThunkTarget_0745e2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0798a6")))
void JsrPcThunk_0798a6(void) {
    extern void Entity_CheckBoxOverlapWithSelector_0798AC(void);
    __asm__ volatile("jsr Entity_CheckBoxOverlapWithSelector_0798AC(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_086dae")))
void JsrPcThunk_086dae(void) {
    extern void PcThunkTarget_088438(void);
    __asm__ volatile("jsr PcThunkTarget_088438(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0877ce")))
void JsrPcThunk_0877ce(void) {
    extern void PcThunkTarget_08846a(void);
    __asm__ volatile("jsr PcThunkTarget_08846a(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08b712")))
void JsrPcThunk_08b712(void) {
    extern void PcThunkTarget_08b82c(void);
    __asm__ volatile("jsr PcThunkTarget_08b82c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08d5a2")))
void JsrPcThunk_08d5a2(void) {
    extern void PcThunkTarget_08d804(void);
    __asm__ volatile("jsr PcThunkTarget_08d804(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08d5c6")))
void JsrPcThunk_08d5c6(void) {
    extern void PcThunkTarget_08d804(void);
    __asm__ volatile("jsr PcThunkTarget_08d804(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08dbdc")))
void JsrPcThunk_08dbdc(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08dc44")))
void JsrPcThunk_08dc44(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08dc8e")))
void JsrPcThunk_08dc8e(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08dcf4")))
void JsrPcThunk_08dcf4(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08dd20")))
void JsrPcThunk_08dd20(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08dd78")))
void JsrPcThunk_08dd78(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08ddaa")))
void JsrPcThunk_08ddaa(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08ddd4")))
void JsrPcThunk_08ddd4(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08de24")))
void JsrPcThunk_08de24(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08de5e")))
void JsrPcThunk_08de5e(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08debc")))
void JsrPcThunk_08debc(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e2a6")))
void JsrPcThunk_08e2a6(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e560")))
void JsrPcThunk_08e560(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e5c8")))
void JsrPcThunk_08e5c8(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e7d8")))
void JsrPcThunk_08e7d8(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e81c")))
void JsrPcThunk_08e81c(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e866")))
void JsrPcThunk_08e866(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e8aa")))
void JsrPcThunk_08e8aa(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e8f4")))
void JsrPcThunk_08e8f4(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e938")))
void JsrPcThunk_08e938(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e982")))
void JsrPcThunk_08e982(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e9ce")))
void JsrPcThunk_08e9ce(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08e9f6")))
void JsrPcThunk_08e9f6(void) {
    extern void PcThunkTarget_08ea50(void);
    __asm__ volatile("jsr PcThunkTarget_08ea50(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08ec4a")))
void JsrPcThunk_08ec4a(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08ecd4")))
void JsrPcThunk_08ecd4(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_08ed7c")))
void JsrPcThunk_08ed7c(void) {
    extern void PcThunkTarget_08efb0(void);
    __asm__ volatile("jsr PcThunkTarget_08efb0(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_097a4e")))
void JsrPcThunk_097a4e(void) {
    extern void PcThunkTarget_097a60(void);
    __asm__ volatile("jsr PcThunkTarget_097a60(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_097a5a")))
void JsrPcThunk_097a5a(void) {
    extern void PcThunkTarget_097a72(void);
    __asm__ volatile("jsr PcThunkTarget_097a72(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_0980e6")))
void JsrPcThunk_0980e6(void) {
    extern void PcThunkTarget_097c5c(void);
    __asm__ volatile("jsr PcThunkTarget_097c5c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099b7e")))
void JsrPcThunk_099b7e(void) {
    extern void PcThunkTarget_099ee4(void);
    __asm__ volatile("jsr PcThunkTarget_099ee4(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099b9e")))
void JsrPcThunk_099b9e(void) {
    extern void PcThunkTarget_099ee4(void);
    __asm__ volatile("jsr PcThunkTarget_099ee4(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099d62")))
void JsrPcThunk_099d62(void) {
    extern void PcThunkTarget_099de4(void);
    __asm__ volatile("jsr PcThunkTarget_099de4(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099d72")))
void JsrPcThunk_099d72(void) {
    extern void PcThunkTarget_099e14(void);
    __asm__ volatile("jsr PcThunkTarget_099e14(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099d8e")))
void JsrPcThunk_099d8e(void) {
    extern void PcThunkTarget_099e9c(void);
    __asm__ volatile("jsr PcThunkTarget_099e9c(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099db8")))
void JsrPcThunk_099db8(void) {
    extern void FixGlyph16_DrawCursorA_099F3A(void);
    __asm__ volatile("jsr FixGlyph16_DrawCursorA_099F3A(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099ddc")))
void JsrPcThunk_099ddc(void) {
    extern void FixGlyph16_DrawCursorA_099F3A(void);
    __asm__ volatile("jsr FixGlyph16_DrawCursorA_099F3A(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099df6")))
void JsrPcThunk_099df6(void) {
    extern void FixGlyphRun_Draw2F61F0_099FD2(void);
    __asm__ volatile("jsr FixGlyphRun_Draw2F61F0_099FD2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099e0e")))
void JsrPcThunk_099e0e(void) {
    extern void FixGlyphRun_Draw2F61F0_099FD2(void);
    __asm__ volatile("jsr FixGlyphRun_Draw2F61F0_099FD2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099e30")))
void JsrPcThunk_099e30(void) {
    extern void FixGlyph16_DrawDigit72EF_099FF2(void);
    __asm__ volatile("jsr FixGlyph16_DrawDigit72EF_099FF2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099e52")))
void JsrPcThunk_099e52(void) {
    extern void FixGlyph16_DrawDigit72EF_099FF2(void);
    __asm__ volatile("jsr FixGlyph16_DrawDigit72EF_099FF2(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099e74")))
void JsrPcThunk_099e74(void) {
    extern void FixGlyph16_DrawDigit72F3_09A03C(void);
    __asm__ volatile("jsr FixGlyph16_DrawDigit72F3_09A03C(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099e96")))
void JsrPcThunk_099e96(void) {
    extern void FixGlyph16_DrawDigit72F3_09A03C(void);
    __asm__ volatile("jsr FixGlyph16_DrawDigit72F3_09A03C(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099eb8")))
void JsrPcThunk_099eb8(void) {
    extern void FixGlyphRun_DrawPad2P_09A086(void);
    __asm__ volatile("jsr FixGlyphRun_DrawPad2P_09A086(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

__attribute__((section(".text.JsrPcThunk_099ecc")))
void JsrPcThunk_099ecc(void) {
    extern void FixGlyphRun_DrawPad2P_09A086(void);
    __asm__ volatile("jsr FixGlyphRun_DrawPad2P_09A086(%%pc)" ::: "memory","cc","d0","d1","a0","a1");
}

