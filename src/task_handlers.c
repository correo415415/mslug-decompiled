/*
 * Metal Slug 1 — Familia SetTaskHandler_<addr>
 * ================================================
 * Mini-funciones de 8 bytes que instalan un handler cercano en el task
 * actual (fp->handler, offset 0) y retornan. Patrón exacto:
 *
 *     lea    pc+disp, a1        ; a1 = &handler
 *     move.l a1, (fp)            ; fp->handler = a1
 *     rts
 *
 * En el código original C:
 *
 *     void SetTaskHandler_N(void) {
 *         fp->handler = &TaskHandler_XXXXXX;
 *     }
 *
 * Cada handler destino es una rutina cercana (rango PC-rel 16-bit) que
 * se registra como símbolo extern TaskHandler_XXXXXX; el linker resuelve
 * su dirección exacta desde el defsym en symbols.py.
 *
 * COMPILAR CON `-mpcrel` (PER_FILE_CFLAGS) para que el `lea` salga corto
 * PC-relativo (4 B) en lugar de absolute long (6 B).
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_task_handlers.py.
 */

#include "mslug.h"

#define USE_A1
#include "mslug_regs.h"

/* Barrera de compilador que fuerza `move.l a1, (fp)` explícito
 * (sin optimizarlo como parte de otra expresión). */
#define STORE_A1_AT_FP()  __asm__ volatile("move.l %%a1, (%%fp)" ::: "memory")

extern void TaskHandler_000b90(void);
extern void TaskHandler_000ef0(void);
extern void TaskHandler_000f1a(void);
extern void TaskHandler_001b4c(void);
extern void TaskHandler_001b70(void);
extern void TaskHandler_001b80(void);
extern void TaskHandler_0257ec(void);
extern void TaskHandler_025882(void);
extern void TaskHandler_025ad8(void);
extern void TaskHandler_025b34(void);
extern void TaskHandler_025d5c(void);
extern void TaskHandler_025d64(void);
extern void TaskHandler_02b05e(void);
extern void TaskHandler_02b264(void);
extern void TaskHandler_02d02e(void);
extern void TaskHandler_02da38(void);
extern void TaskHandler_02ff86(void);
extern void TaskHandler_030bf6(void);
extern void TaskHandler_030d74(void);
extern void TaskHandler_0318ac(void);
extern void TaskHandler_0318d4(void);
extern void TaskHandler_0321bc(void);
extern void TaskHandler_036d64(void);
extern void TaskHandler_037b8e(void);
extern void TaskHandler_037c1a(void);
extern void TaskHandler_038cee(void);
extern void TaskHandler_038e4a(void);
extern void TaskHandler_0391aa(void);
extern void TaskHandler_03dc16(void);
extern void TaskHandler_03dc2c(void);
extern void TaskHandler_03dc74(void);
extern void TaskHandler_03dea8(void);
extern void TaskHandler_03debe(void);
extern void TaskHandler_03dee2(void);
extern void TaskHandler_03df32(void);
extern void TaskHandler_03df54(void);
extern void TaskHandler_03e084(void);
extern void TaskHandler_03e4e6(void);
extern void TaskHandler_03e50c(void);
extern void TaskHandler_03eaa2(void);
extern void TaskHandler_03eb82(void);
extern void TaskHandler_03ebf8(void);
extern void TaskHandler_03ec16(void);
extern void TaskHandler_03ec8c(void);
extern void TaskHandler_03ecaa(void);
extern void TaskHandler_03fcc0(void);
extern void TaskHandler_03fdd0(void);
extern void TaskHandler_03fe66(void);
extern void TaskHandler_04049c(void);
extern void TaskHandler_040d18(void);
extern void TaskHandler_040e54(void);
extern void TaskHandler_040ef2(void);
extern void TaskHandler_04155a(void);
extern void TaskHandler_04157e(void);
extern void TaskHandler_04181c(void);
extern void TaskHandler_042740(void);
extern void TaskHandler_042a44(void);
extern void TaskHandler_042a56(void);
extern void TaskHandler_042a6e(void);
extern void TaskHandler_042acc(void);
extern void TaskHandler_044c1a(void);
extern void TaskHandler_044df2(void);
extern void TaskHandler_044f8a(void);
extern void TaskHandler_044f9a(void);
extern void TaskHandler_045f2c(void);
extern void TaskHandler_0463ba(void);
extern void TaskHandler_0465de(void);
extern void TaskHandler_046664(void);
extern void TaskHandler_0466b4(void);
extern void TaskHandler_0466da(void);
extern void TaskHandler_04703a(void);
extern void TaskHandler_047050(void);
extern void TaskHandler_047146(void);
extern void TaskHandler_04718a(void);
extern void TaskHandler_047278(void);
extern void TaskHandler_04728e(void);
extern void TaskHandler_0472d2(void);
extern void TaskHandler_04731c(void);
extern void TaskHandler_047362(void);
extern void TaskHandler_048b1e(void);
extern void TaskHandler_048b26(void);
extern void TaskHandler_048ddc(void);
extern void TaskHandler_048dec(void);
extern void TaskHandler_04968a(void);
extern void TaskHandler_04a014(void);
extern void TaskHandler_04a024(void);
extern void TaskHandler_04a18c(void);
extern void TaskHandler_04ac32(void);
extern void TaskHandler_04bc48(void);
extern void TaskHandler_04c578(void);
extern void TaskHandler_04c58c(void);
extern void TaskHandler_04c606(void);
extern void TaskHandler_04c68a(void);
extern void TaskHandler_04c934(void);
extern void TaskHandler_04dc88(void);
extern void TaskHandler_04de32(void);
extern void TaskHandler_04eb94(void);
extern void TaskHandler_04f2a4(void);
extern void TaskHandler_050976(void);
extern void TaskHandler_051452(void);
extern void TaskHandler_05147e(void);
extern void TaskHandler_052514(void);
extern void TaskHandler_0526aa(void);
extern void TaskHandler_053c5c(void);
extern void TaskHandler_053c64(void);
extern void TaskHandler_056058(void);
extern void TaskHandler_056204(void);
extern void TaskHandler_056596(void);
extern void TaskHandler_057f4e(void);
extern void TaskHandler_058412(void);
extern void TaskHandler_058b1e(void);
extern void TaskHandler_058c8e(void);
extern void TaskHandler_058cce(void);
extern void TaskHandler_05943a(void);
extern void TaskHandler_05947a(void);
extern void TaskHandler_0594ba(void);
extern void TaskHandler_059722(void);
extern void TaskHandler_059756(void);
extern void TaskHandler_0597b0(void);
extern void TaskHandler_05980a(void);
extern void TaskHandler_059864(void);
extern void TaskHandler_0598ae(void);
extern void TaskHandler_05994a(void);
extern void TaskHandler_05996c(void);
extern void TaskHandler_059988(void);
extern void TaskHandler_0599aa(void);
extern void TaskHandler_0599c6(void);
extern void TaskHandler_0599f2(void);
extern void TaskHandler_059a1a(void);
extern void TaskHandler_059a40(void);
extern void TaskHandler_059a70(void);
extern void TaskHandler_059b86(void);
extern void TaskHandler_059bc6(void);
extern void TaskHandler_059c42(void);
extern void TaskHandler_059d62(void);
extern void TaskHandler_05a28a(void);
extern void TaskHandler_05a66e(void);
extern void TaskHandler_05a764(void);
extern void TaskHandler_05cbea(void);
extern void TaskHandler_05f00a(void);
extern void TaskHandler_05f0b0(void);
extern void TaskHandler_05f482(void);
extern void TaskHandler_05fa56(void);
extern void TaskHandler_05fb88(void);
extern void TaskHandler_05fbe6(void);
extern void TaskHandler_05fce6(void);
extern void TaskHandler_0606ee(void);
extern void TaskHandler_060bf6(void);
extern void TaskHandler_060c40(void);
extern void TaskHandler_060d3a(void);
extern void TaskHandler_060de8(void);
extern void TaskHandler_0620a0(void);
extern void TaskHandler_0620a8(void);
extern void TaskHandler_062f8c(void);
extern void TaskHandler_062f9a(void);
extern void TaskHandler_063942(void);
extern void TaskHandler_063952(void);
extern void TaskHandler_064222(void);
extern void TaskHandler_06422a(void);
extern void TaskHandler_064d7a(void);
extern void TaskHandler_064d8a(void);
extern void TaskHandler_06515e(void);
extern void TaskHandler_0667a4(void);
extern void TaskHandler_066a86(void);
extern void TaskHandler_066a8e(void);
extern void TaskHandler_067b7c(void);
extern void TaskHandler_067b84(void);
extern void TaskHandler_068310(void);
extern void TaskHandler_068346(void);
extern void TaskHandler_06850e(void);
extern void TaskHandler_068684(void);
extern void TaskHandler_0688f8(void);
extern void TaskHandler_06895c(void);
extern void TaskHandler_06986a(void);
extern void TaskHandler_069872(void);
extern void TaskHandler_06a41c(void);
extern void TaskHandler_06a452(void);
extern void TaskHandler_06a468(void);
extern void TaskHandler_06a93e(void);
extern void TaskHandler_06acaa(void);
extern void TaskHandler_06c4c6(void);
extern void TaskHandler_06c53e(void);
extern void TaskHandler_06c554(void);
extern void TaskHandler_06da7a(void);
extern void TaskHandler_06da90(void);
extern void TaskHandler_06e062(void);
extern void TaskHandler_06e80a(void);
extern void TaskHandler_06e818(void);
extern void TaskHandler_06e88c(void);
extern void TaskHandler_06e932(void);
extern void TaskHandler_06f340(void);
extern void TaskHandler_06f38c(void);
extern void TaskHandler_06f9ca(void);
extern void TaskHandler_06fa60(void);
extern void TaskHandler_06fb02(void);
extern void TaskHandler_0700a6(void);
extern void TaskHandler_070290(void);
extern void TaskHandler_0704f2(void);
extern void TaskHandler_070694(void);
extern void TaskHandler_07079e(void);
extern void TaskHandler_0707c8(void);
extern void TaskHandler_0713a2(void);
extern void TaskHandler_0716b6(void);
extern void TaskHandler_0716e2(void);
extern void TaskHandler_0716ea(void);
extern void TaskHandler_0716f2(void);
extern void TaskHandler_071b9a(void);
extern void TaskHandler_0724d4(void);
extern void TaskHandler_073454(void);
extern void TaskHandler_0734f4(void);
extern void TaskHandler_0734fc(void);
extern void TaskHandler_0735a6(void);
extern void TaskHandler_0738da(void);
extern void TaskHandler_073b62(void);
extern void TaskHandler_073f9a(void);
extern void TaskHandler_0751a6(void);
extern void TaskHandler_07525c(void);
extern void TaskHandler_076056(void);
extern void TaskHandler_0760a8(void);
extern void TaskHandler_0760c8(void);
extern void TaskHandler_076290(void);
extern void TaskHandler_07646e(void);
extern void TaskHandler_07692c(void);
extern void TaskHandler_076a90(void);
extern void TaskHandler_076b3a(void);
extern void TaskHandler_076bd0(void);
extern void TaskHandler_07726a(void);
extern void TaskHandler_077a8e(void);
extern void TaskHandler_079326(void);
extern void TaskHandler_079a8a(void);
extern void TaskHandler_079b42(void);
extern void TaskHandler_079c3c(void);
extern void TaskHandler_079c68(void);
extern void TaskHandler_079caa(void);
extern void TaskHandler_079d76(void);
extern void TaskHandler_079e68(void);
extern void TaskHandler_079e94(void);
extern void TaskHandler_079f6a(void);
extern void TaskHandler_07a00a(void);
extern void TaskHandler_07a1c0(void);
extern void TaskHandler_07a3e6(void);
extern void TaskHandler_07a3ee(void);
extern void TaskHandler_07aa78(void);
extern void TaskHandler_07aa94(void);
extern void TaskHandler_07abf4(void);
extern void TaskHandler_07ac4a(void);
extern void TaskHandler_07bace(void);
extern void TaskHandler_07bb1a(void);
extern void TaskHandler_07c106(void);
extern void TaskHandler_07c374(void);
extern void TaskHandler_07c424(void);
extern void TaskHandler_07c644(void);
extern void TaskHandler_07c65c(void);
extern void TaskHandler_07cf5c(void);
extern void TaskHandler_07d898(void);
extern void TaskHandler_07dba2(void);
extern void TaskHandler_07dba8(void);
extern void TaskHandler_07dcb6(void);
extern void TaskHandler_07e078(void);
extern void TaskHandler_07e08c(void);
extern void TaskHandler_07e674(void);
extern void TaskHandler_07e880(void);
extern void TaskHandler_07ebe6(void);
extern void TaskHandler_07f022(void);
extern void TaskHandler_07f186(void);
extern void TaskHandler_07f282(void);
extern void TaskHandler_07f2ca(void);
extern void TaskHandler_07f87c(void);
extern void TaskHandler_07fdb6(void);
extern void TaskHandler_07fdbc(void);
extern void TaskHandler_080382(void);
extern void TaskHandler_0803d2(void);
extern void TaskHandler_0803e8(void);
extern void TaskHandler_080454(void);
extern void TaskHandler_080508(void);
extern void TaskHandler_0805ee(void);
extern void TaskHandler_081018(void);
extern void TaskHandler_0811cc(void);
extern void TaskHandler_0811e4(void);
extern void TaskHandler_081214(void);
extern void TaskHandler_0812be(void);
extern void TaskHandler_081bee(void);
extern void TaskHandler_081d64(void);
extern void TaskHandler_081fac(void);
extern void TaskHandler_081ff0(void);
extern void TaskHandler_082388(void);
extern void TaskHandler_082456(void);
extern void TaskHandler_082464(void);
extern void TaskHandler_084410(void);
extern void TaskHandler_0844c0(void);
extern void TaskHandler_08450c(void);
extern void TaskHandler_08489a(void);
extern void TaskHandler_0848de(void);
extern void TaskHandler_084b24(void);
extern void TaskHandler_084b9a(void);
extern void TaskHandler_084bd2(void);
extern void TaskHandler_084c26(void);
extern void TaskHandler_084f5e(void);
extern void TaskHandler_084fca(void);
extern void TaskHandler_085134(void);
extern void TaskHandler_085484(void);
extern void TaskHandler_085a08(void);
extern void TaskHandler_0865be(void);
extern void TaskHandler_086854(void);
extern void TaskHandler_089398(void);
extern void TaskHandler_089504(void);
extern void TaskHandler_0895cc(void);
extern void TaskHandler_0898d4(void);
extern void TaskHandler_089960(void);
extern void TaskHandler_089a04(void);
extern void TaskHandler_08a31c(void);
extern void TaskHandler_08a44c(void);
extern void TaskHandler_08a516(void);
extern void TaskHandler_08a5c8(void);
extern void TaskHandler_08a9b0(void);
extern void TaskHandler_08aaf2(void);
extern void TaskHandler_08abd4(void);
extern void TaskHandler_08ac92(void);
extern void TaskHandler_08ae38(void);
extern void TaskHandler_08af68(void);
extern void TaskHandler_08b03c(void);
extern void TaskHandler_08b10a(void);
extern void TaskHandler_08bb84(void);
extern void TaskHandler_08c678(void);
extern void TaskHandler_08c8fa(void);
extern void TaskHandler_08cf22(void);
extern void TaskHandler_08cf6c(void);
extern void TaskHandler_08cfb6(void);
extern void TaskHandler_08d000(void);
extern void TaskHandler_08d04a(void);
extern void TaskHandler_08d094(void);
extern void TaskHandler_08d41c(void);
extern void TaskHandler_08d450(void);
extern void TaskHandler_08d472(void);
extern void TaskHandler_08d4c6(void);
extern void TaskHandler_08d4fe(void);
extern void TaskHandler_08d55c(void);
extern void TaskHandler_08d580(void);
extern void TaskHandler_08d5a8(void);
extern void TaskHandler_08d62e(void);
extern void TaskHandler_08d72a(void);
extern void TaskHandler_08d774(void);
extern void TaskHandler_08d7be(void);
extern void TaskHandler_08d8f4(void);
extern void TaskHandler_08db7a(void);
extern void TaskHandler_08dbe2(void);
extern void TaskHandler_08df72(void);
extern void TaskHandler_08dfb8(void);
extern void TaskHandler_08e288(void);
extern void TaskHandler_08e2f0(void);
extern void TaskHandler_08e4fe(void);
extern void TaskHandler_08e566(void);
extern void TaskHandler_08e622(void);
extern void TaskHandler_08e716(void);
extern void TaskHandler_08ea16(void);
extern void TaskHandler_08eb02(void);
extern void TaskHandler_08f96a(void);
extern void TaskHandler_08fcca(void);
extern void TaskHandler_08fd2e(void);
extern void TaskHandler_08fd68(void);
extern void TaskHandler_08fdaa(void);
extern void TaskHandler_08fdf8(void);
extern void TaskHandler_08fe32(void);
extern void TaskHandler_08fe9c(void);
extern void TaskHandler_08feb6(void);
extern void TaskHandler_08feca(void);
extern void TaskHandler_090098(void);
extern void TaskHandler_090e7e(void);
extern void TaskHandler_091338(void);
extern void TaskHandler_0913aa(void);
extern void TaskHandler_091514(void);
extern void TaskHandler_09152c(void);
extern void TaskHandler_091558(void);
extern void TaskHandler_0916c0(void);
extern void TaskHandler_097852(void);
extern void TaskHandler_09788c(void);
extern void TaskHandler_0978ac(void);
extern void TaskHandler_0978fa(void);
extern void TaskHandler_09792e(void);
extern void TaskHandler_09794a(void);
extern void TaskHandler_09806a(void);
extern void TaskHandler_098308(void);
extern void TaskHandler_0983f6(void);
extern void TaskHandler_098482(void);
extern void TaskHandler_098836(void);
extern void TaskHandler_098886(void);
extern void TaskHandler_09890c(void);
extern void TaskHandler_0989e0(void);
extern void TaskHandler_098afe(void);
extern void TaskHandler_098c00(void);
extern void TaskHandler_098dc6(void);
extern void TaskHandler_099004(void);
extern void TaskHandler_099180(void);
extern void TaskHandler_09921a(void);
extern void TaskHandler_0993a2(void);
extern void TaskHandler_09953e(void);
extern void TaskHandler_099610(void);
extern void TaskHandler_09976a(void);
extern void TaskHandler_099794(void);
extern void TaskHandler_099a64(void);
extern void TaskHandler_09a280(void);
extern void TaskHandler_09a2b8(void);
extern void TaskHandler_09b47c(void);
extern void TaskHandler_18d74e(void);

__attribute__((section(".text.SetTaskHandler_000b8a")))
void SetTaskHandler_000b8a(void) {
    _a1_ptr = &TaskHandler_000b90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_00116a")))
void SetTaskHandler_00116a(void) {
    _a1_ptr = &TaskHandler_000f1a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_001b14")))
void SetTaskHandler_001b14(void) {
    _a1_ptr = &TaskHandler_000ef0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_001b68")))
void SetTaskHandler_001b68(void) {
    _a1_ptr = &TaskHandler_001b70;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_001b78")))
void SetTaskHandler_001b78(void) {
    _a1_ptr = &TaskHandler_001b80;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_001c3c")))
void SetTaskHandler_001c3c(void) {
    _a1_ptr = &TaskHandler_001b4c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_025878")))
void SetTaskHandler_025878(void) {
    _a1_ptr = &TaskHandler_025882;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_02591a")))
void SetTaskHandler_02591a(void) {
    _a1_ptr = &TaskHandler_0257ec;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_025998")))
void SetTaskHandler_025998(void) {
    _a1_ptr = &TaskHandler_025b34;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_025a06")))
void SetTaskHandler_025a06(void) {
    _a1_ptr = &TaskHandler_025ad8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_025cbe")))
void SetTaskHandler_025cbe(void) {
    _a1_ptr = &TaskHandler_025d64;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_025d54")))
void SetTaskHandler_025d54(void) {
    _a1_ptr = &TaskHandler_025d5c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_025e40")))
void SetTaskHandler_025e40(void) {
    _a1_ptr = &TaskHandler_0257ec;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_02ae36")))
void SetTaskHandler_02ae36(void) {
    _a1_ptr = &TaskHandler_02d02e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_02b056")))
void SetTaskHandler_02b056(void) {
    _a1_ptr = &TaskHandler_02b05e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_02b25c")))
void SetTaskHandler_02b25c(void) {
    _a1_ptr = &TaskHandler_02b264;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_02d72e")))
void SetTaskHandler_02d72e(void) {
    _a1_ptr = &TaskHandler_02da38;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_02ff7e")))
void SetTaskHandler_02ff7e(void) {
    _a1_ptr = &TaskHandler_02ff86;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_030bae")))
void SetTaskHandler_030bae(void) {
    _a1_ptr = &TaskHandler_030bf6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_030bce")))
void SetTaskHandler_030bce(void) {
    _a1_ptr = &TaskHandler_030bf6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_030bee")))
void SetTaskHandler_030bee(void) {
    _a1_ptr = &TaskHandler_030bf6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_030cfc")))
void SetTaskHandler_030cfc(void) {
    _a1_ptr = &TaskHandler_030d74;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_030d54")))
void SetTaskHandler_030d54(void) {
    _a1_ptr = &TaskHandler_030d74;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_030d6c")))
void SetTaskHandler_030d6c(void) {
    _a1_ptr = &TaskHandler_030d74;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_031682")))
void SetTaskHandler_031682(void) {
    _a1_ptr = &TaskHandler_0318d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0317ca")))
void SetTaskHandler_0317ca(void) {
    _a1_ptr = &TaskHandler_0318d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0318a4")))
void SetTaskHandler_0318a4(void) {
    _a1_ptr = &TaskHandler_0318ac;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0318cc")))
void SetTaskHandler_0318cc(void) {
    _a1_ptr = &TaskHandler_0318d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0319c8")))
void SetTaskHandler_0319c8(void) {
    _a1_ptr = &TaskHandler_0318d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_032068")))
void SetTaskHandler_032068(void) {
    _a1_ptr = &TaskHandler_0321bc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0320cc")))
void SetTaskHandler_0320cc(void) {
    _a1_ptr = &TaskHandler_0321bc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0321b4")))
void SetTaskHandler_0321b4(void) {
    _a1_ptr = &TaskHandler_0321bc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_033630")))
void SetTaskHandler_033630(void) {
    _a1_ptr = &TaskHandler_036d64;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0342bc")))
void SetTaskHandler_0342bc(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_034b30")))
void SetTaskHandler_034b30(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_037652")))
void SetTaskHandler_037652(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_037770")))
void SetTaskHandler_037770(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03786c")))
void SetTaskHandler_03786c(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0378be")))
void SetTaskHandler_0378be(void) {
    _a1_ptr = &TaskHandler_037c1a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0379da")))
void SetTaskHandler_0379da(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_037a2c")))
void SetTaskHandler_037a2c(void) {
    _a1_ptr = &TaskHandler_037c1a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_038424")))
void SetTaskHandler_038424(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0388e8")))
void SetTaskHandler_0388e8(void) {
    _a1_ptr = &TaskHandler_037b8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_038c68")))
void SetTaskHandler_038c68(void) {
    _a1_ptr = &TaskHandler_038cee;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_038ce6")))
void SetTaskHandler_038ce6(void) {
    _a1_ptr = &TaskHandler_038cee;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_038d80")))
void SetTaskHandler_038d80(void) {
    _a1_ptr = &TaskHandler_038e4a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_038e08")))
void SetTaskHandler_038e08(void) {
    _a1_ptr = &TaskHandler_038e4a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0391a2")))
void SetTaskHandler_0391a2(void) {
    _a1_ptr = &TaskHandler_0391aa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03dc0e")))
void SetTaskHandler_03dc0e(void) {
    _a1_ptr = &TaskHandler_03dc16;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03dc24")))
void SetTaskHandler_03dc24(void) {
    _a1_ptr = &TaskHandler_03dc2c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03dc6c")))
void SetTaskHandler_03dc6c(void) {
    _a1_ptr = &TaskHandler_03dc74;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03dd20")))
void SetTaskHandler_03dd20(void) {
    _a1_ptr = &TaskHandler_03debe;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03dea0")))
void SetTaskHandler_03dea0(void) {
    _a1_ptr = &TaskHandler_03dea8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03deb6")))
void SetTaskHandler_03deb6(void) {
    _a1_ptr = &TaskHandler_03debe;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03deda")))
void SetTaskHandler_03deda(void) {
    _a1_ptr = &TaskHandler_03dee2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03df2a")))
void SetTaskHandler_03df2a(void) {
    _a1_ptr = &TaskHandler_03df32;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03df4c")))
void SetTaskHandler_03df4c(void) {
    _a1_ptr = &TaskHandler_03df54;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03e07c")))
void SetTaskHandler_03e07c(void) {
    _a1_ptr = &TaskHandler_03e084;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03e430")))
void SetTaskHandler_03e430(void) {
    _a1_ptr = &TaskHandler_03e4e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03e472")))
void SetTaskHandler_03e472(void) {
    _a1_ptr = &TaskHandler_03e4e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03e488")))
void SetTaskHandler_03e488(void) {
    _a1_ptr = &TaskHandler_03e50c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03e4aa")))
void SetTaskHandler_03e4aa(void) {
    _a1_ptr = &TaskHandler_03e4e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03ea5e")))
void SetTaskHandler_03ea5e(void) {
    _a1_ptr = &TaskHandler_03eaa2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03eb7c")))
void SetTaskHandler_03eb7c(void) {
    _a1_ptr = &TaskHandler_03eb82;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03ebb4")))
void SetTaskHandler_03ebb4(void) {
    _a1_ptr = &TaskHandler_03ebf8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03ec10")))
void SetTaskHandler_03ec10(void) {
    _a1_ptr = &TaskHandler_03ec16;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03ec48")))
void SetTaskHandler_03ec48(void) {
    _a1_ptr = &TaskHandler_03ec8c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03eca4")))
void SetTaskHandler_03eca4(void) {
    _a1_ptr = &TaskHandler_03ecaa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03fcb8")))
void SetTaskHandler_03fcb8(void) {
    _a1_ptr = &TaskHandler_03fcc0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03fdc8")))
void SetTaskHandler_03fdc8(void) {
    _a1_ptr = &TaskHandler_03fdd0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_03fe5e")))
void SetTaskHandler_03fe5e(void) {
    _a1_ptr = &TaskHandler_03fe66;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_040494")))
void SetTaskHandler_040494(void) {
    _a1_ptr = &TaskHandler_04049c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_040d10")))
void SetTaskHandler_040d10(void) {
    _a1_ptr = &TaskHandler_040d18;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_040e4c")))
void SetTaskHandler_040e4c(void) {
    _a1_ptr = &TaskHandler_040e54;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_040eb2")))
void SetTaskHandler_040eb2(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041320")))
void SetTaskHandler_041320(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041482")))
void SetTaskHandler_041482(void) {
    _a1_ptr = &TaskHandler_04155a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041576")))
void SetTaskHandler_041576(void) {
    _a1_ptr = &TaskHandler_04157e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04161e")))
void SetTaskHandler_04161e(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04175a")))
void SetTaskHandler_04175a(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041814")))
void SetTaskHandler_041814(void) {
    _a1_ptr = &TaskHandler_04181c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041848")))
void SetTaskHandler_041848(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041912")))
void SetTaskHandler_041912(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0419b6")))
void SetTaskHandler_0419b6(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041aac")))
void SetTaskHandler_041aac(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_041c12")))
void SetTaskHandler_041c12(void) {
    _a1_ptr = &TaskHandler_040ef2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0424a2")))
void SetTaskHandler_0424a2(void) {
    _a1_ptr = &TaskHandler_042740;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0429fc")))
void SetTaskHandler_0429fc(void) {
    _a1_ptr = &TaskHandler_042acc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_042a0c")))
void SetTaskHandler_042a0c(void) {
    _a1_ptr = &TaskHandler_042a6e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_042a1a")))
void SetTaskHandler_042a1a(void) {
    _a1_ptr = &TaskHandler_042a56;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_042a22")))
void SetTaskHandler_042a22(void) {
    _a1_ptr = &TaskHandler_042a44;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_044af6")))
void SetTaskHandler_044af6(void) {
    _a1_ptr = &TaskHandler_044c1a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_044d28")))
void SetTaskHandler_044d28(void) {
    _a1_ptr = &TaskHandler_044c1a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_044dea")))
void SetTaskHandler_044dea(void) {
    _a1_ptr = &TaskHandler_044df2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_044ee6")))
void SetTaskHandler_044ee6(void) {
    _a1_ptr = &TaskHandler_044f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_044f82")))
void SetTaskHandler_044f82(void) {
    _a1_ptr = &TaskHandler_044f8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_045f24")))
void SetTaskHandler_045f24(void) {
    _a1_ptr = &TaskHandler_045f2c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_046258")))
void SetTaskHandler_046258(void) {
    _a1_ptr = &TaskHandler_0463ba;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0463b2")))
void SetTaskHandler_0463b2(void) {
    _a1_ptr = &TaskHandler_0463ba;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0465d6")))
void SetTaskHandler_0465d6(void) {
    _a1_ptr = &TaskHandler_0465de;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04665c")))
void SetTaskHandler_04665c(void) {
    _a1_ptr = &TaskHandler_046664;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0466ac")))
void SetTaskHandler_0466ac(void) {
    _a1_ptr = &TaskHandler_0466b4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0466d2")))
void SetTaskHandler_0466d2(void) {
    _a1_ptr = &TaskHandler_0466da;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_047032")))
void SetTaskHandler_047032(void) {
    _a1_ptr = &TaskHandler_04703a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_047048")))
void SetTaskHandler_047048(void) {
    _a1_ptr = &TaskHandler_047050;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04713e")))
void SetTaskHandler_04713e(void) {
    _a1_ptr = &TaskHandler_047146;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_047182")))
void SetTaskHandler_047182(void) {
    _a1_ptr = &TaskHandler_04718a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0471d2")))
void SetTaskHandler_0471d2(void) {
    _a1_ptr = &TaskHandler_047146;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_047270")))
void SetTaskHandler_047270(void) {
    _a1_ptr = &TaskHandler_047278;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_047286")))
void SetTaskHandler_047286(void) {
    _a1_ptr = &TaskHandler_04731c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0472ca")))
void SetTaskHandler_0472ca(void) {
    _a1_ptr = &TaskHandler_0472d2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_047314")))
void SetTaskHandler_047314(void) {
    _a1_ptr = &TaskHandler_04728e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04735a")))
void SetTaskHandler_04735a(void) {
    _a1_ptr = &TaskHandler_047362;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048a3c")))
void SetTaskHandler_048a3c(void) {
    _a1_ptr = &TaskHandler_048b1e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048a88")))
void SetTaskHandler_048a88(void) {
    _a1_ptr = &TaskHandler_048b1e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048b16")))
void SetTaskHandler_048b16(void) {
    _a1_ptr = &TaskHandler_048b1e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048b98")))
void SetTaskHandler_048b98(void) {
    _a1_ptr = &TaskHandler_048b26;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048c9c")))
void SetTaskHandler_048c9c(void) {
    _a1_ptr = &TaskHandler_048b26;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048d06")))
void SetTaskHandler_048d06(void) {
    _a1_ptr = &TaskHandler_048b26;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048e24")))
void SetTaskHandler_048e24(void) {
    _a1_ptr = &TaskHandler_048b26;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048e3a")))
void SetTaskHandler_048e3a(void) {
    _a1_ptr = &TaskHandler_048ddc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048e4c")))
void SetTaskHandler_048e4c(void) {
    _a1_ptr = &TaskHandler_048dec;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_048e9e")))
void SetTaskHandler_048e9e(void) {
    _a1_ptr = &TaskHandler_048b26;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_049b50")))
void SetTaskHandler_049b50(void) {
    _a1_ptr = &TaskHandler_04968a;
    STORE_A1_AT_FP();
}

/*
 * SetTaskHandler_049fea ELIMINADO (falso positivo Wave H).
 *
 * Los 8 bytes 43fa00282c894e75 en $049FEA no son un setter independiente,
 * sino el brazo .Linstall_channel_b del helper de dispatch triple
 * Entity_ProbeAndInstallHandler_049FD0 (Wave V#8) que termina justo aqui
 * con lea Handler_0004A014(pc),a1 + move.l a1,(a6) + rts.
 *
 * Es el TERCER falso positivo por reuso de epilogos detectado, tras
 * ex-JsrAbsThunk_050248 (absorbido por Sprite_InvokeBlit8Params, Wave S)
 * y ex-JsrAbsThunk_051804 (absorbido por Entity_CopyField68AndCall_0517FE,
 * Wave V#3). En los tres casos el patron es el mismo: la Wave batch H/I
 * detecta el sufijo comun sin darse cuenta de que forma parte del cuerpo
 * de una funcion anterior contigua.
 *
 * Ver asm/entity_probe_install_handler_049fd0.s para el cuerpo real.
 */

__attribute__((section(".text.SetTaskHandler_04a00c")))
void SetTaskHandler_04a00c(void) {
    _a1_ptr = &TaskHandler_04a024;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04a166")))
void SetTaskHandler_04a166(void) {
    _a1_ptr = &TaskHandler_04a18c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04ac2a")))
void SetTaskHandler_04ac2a(void) {
    _a1_ptr = &TaskHandler_04ac32;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04bc40")))
void SetTaskHandler_04bc40(void) {
    _a1_ptr = &TaskHandler_04bc48;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04bd56")))
void SetTaskHandler_04bd56(void) {
    _a1_ptr = &TaskHandler_04c578;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04bdfc")))
void SetTaskHandler_04bdfc(void) {
    _a1_ptr = &TaskHandler_04c578;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04be6a")))
void SetTaskHandler_04be6a(void) {
    _a1_ptr = &TaskHandler_04c578;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04bed8")))
void SetTaskHandler_04bed8(void) {
    _a1_ptr = &TaskHandler_04c578;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04bf50")))
void SetTaskHandler_04bf50(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04bfe8")))
void SetTaskHandler_04bfe8(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c080")))
void SetTaskHandler_04c080(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c120")))
void SetTaskHandler_04c120(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c1a8")))
void SetTaskHandler_04c1a8(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c26e")))
void SetTaskHandler_04c26e(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c2dc")))
void SetTaskHandler_04c2dc(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c442")))
void SetTaskHandler_04c442(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c570")))
void SetTaskHandler_04c570(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c5fe")))
void SetTaskHandler_04c5fe(void) {
    _a1_ptr = &TaskHandler_04c68a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c682")))
void SetTaskHandler_04c682(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c6cc")))
void SetTaskHandler_04c6cc(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c6e2")))
void SetTaskHandler_04c6e2(void) {
    _a1_ptr = &TaskHandler_04c606;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c82a")))
void SetTaskHandler_04c82a(void) {
    _a1_ptr = &TaskHandler_04c934;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c908")))
void SetTaskHandler_04c908(void) {
    _a1_ptr = &TaskHandler_04c934;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04c950")))
void SetTaskHandler_04c950(void) {
    _a1_ptr = &TaskHandler_04c58c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04daca")))
void SetTaskHandler_04daca(void) {
    _a1_ptr = &TaskHandler_04f2a4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04dc42")))
void SetTaskHandler_04dc42(void) {
    _a1_ptr = &TaskHandler_04dc88;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04dc8e")))
void SetTaskHandler_04dc8e(void) {
    _a1_ptr = &TaskHandler_04f2a4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04dde4")))
void SetTaskHandler_04dde4(void) {
    _a1_ptr = &TaskHandler_04de32;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04de38")))
void SetTaskHandler_04de38(void) {
    _a1_ptr = &TaskHandler_04f2a4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04eb8c")))
void SetTaskHandler_04eb8c(void) {
    _a1_ptr = &TaskHandler_04eb94;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_04f3a6")))
void SetTaskHandler_04f3a6(void) {
    _a1_ptr = &TaskHandler_04f2a4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05096e")))
void SetTaskHandler_05096e(void) {
    _a1_ptr = &TaskHandler_050976;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05138a")))
void SetTaskHandler_05138a(void) {
    _a1_ptr = &TaskHandler_05147e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05144a")))
void SetTaskHandler_05144a(void) {
    _a1_ptr = &TaskHandler_051452;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05250c")))
void SetTaskHandler_05250c(void) {
    _a1_ptr = &TaskHandler_052514;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0526a2")))
void SetTaskHandler_0526a2(void) {
    _a1_ptr = &TaskHandler_0526aa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_053c54")))
void SetTaskHandler_053c54(void) {
    _a1_ptr = &TaskHandler_053c5c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_053c5c")))
void SetTaskHandler_053c5c(void) {
    _a1_ptr = &TaskHandler_053c64;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_055fc2")))
void SetTaskHandler_055fc2(void) {
    _a1_ptr = &TaskHandler_056058;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0561fc")))
void SetTaskHandler_0561fc(void) {
    _a1_ptr = &TaskHandler_056204;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05658e")))
void SetTaskHandler_05658e(void) {
    _a1_ptr = &TaskHandler_056596;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_057022")))
void SetTaskHandler_057022(void) {
    _a1_ptr = &TaskHandler_058b1e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05703c")))
void SetTaskHandler_05703c(void) {
    _a1_ptr = &TaskHandler_058c8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0573d6")))
void SetTaskHandler_0573d6(void) {
    _a1_ptr = &TaskHandler_058b1e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_057406")))
void SetTaskHandler_057406(void) {
    _a1_ptr = &TaskHandler_058cce;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_057550")))
void SetTaskHandler_057550(void) {
    _a1_ptr = &TaskHandler_058412;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_057cb8")))
void SetTaskHandler_057cb8(void) {
    _a1_ptr = &TaskHandler_057f4e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_057cfc")))
void SetTaskHandler_057cfc(void) {
    _a1_ptr = &TaskHandler_057f4e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059432")))
void SetTaskHandler_059432(void) {
    _a1_ptr = &TaskHandler_05943a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059472")))
void SetTaskHandler_059472(void) {
    _a1_ptr = &TaskHandler_05947a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0594b2")))
void SetTaskHandler_0594b2(void) {
    _a1_ptr = &TaskHandler_0594ba;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05971a")))
void SetTaskHandler_05971a(void) {
    _a1_ptr = &TaskHandler_059722;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05974e")))
void SetTaskHandler_05974e(void) {
    _a1_ptr = &TaskHandler_059756;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0597a8")))
void SetTaskHandler_0597a8(void) {
    _a1_ptr = &TaskHandler_0597b0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059802")))
void SetTaskHandler_059802(void) {
    _a1_ptr = &TaskHandler_05980a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05985c")))
void SetTaskHandler_05985c(void) {
    _a1_ptr = &TaskHandler_059864;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0598a6")))
void SetTaskHandler_0598a6(void) {
    _a1_ptr = &TaskHandler_0598ae;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059942")))
void SetTaskHandler_059942(void) {
    _a1_ptr = &TaskHandler_05994a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059964")))
void SetTaskHandler_059964(void) {
    _a1_ptr = &TaskHandler_05996c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059980")))
void SetTaskHandler_059980(void) {
    _a1_ptr = &TaskHandler_059988;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0599a2")))
void SetTaskHandler_0599a2(void) {
    _a1_ptr = &TaskHandler_0599aa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0599be")))
void SetTaskHandler_0599be(void) {
    _a1_ptr = &TaskHandler_0599c6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0599ea")))
void SetTaskHandler_0599ea(void) {
    _a1_ptr = &TaskHandler_0599f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059a12")))
void SetTaskHandler_059a12(void) {
    _a1_ptr = &TaskHandler_059a1a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059a38")))
void SetTaskHandler_059a38(void) {
    _a1_ptr = &TaskHandler_059a40;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059a68")))
void SetTaskHandler_059a68(void) {
    _a1_ptr = &TaskHandler_059a70;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059b7e")))
void SetTaskHandler_059b7e(void) {
    _a1_ptr = &TaskHandler_059b86;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059bbe")))
void SetTaskHandler_059bbe(void) {
    _a1_ptr = &TaskHandler_059bc6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059c3a")))
void SetTaskHandler_059c3a(void) {
    _a1_ptr = &TaskHandler_059c42;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_059d5a")))
void SetTaskHandler_059d5a(void) {
    _a1_ptr = &TaskHandler_059d62;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05a282")))
void SetTaskHandler_05a282(void) {
    _a1_ptr = &TaskHandler_05a28a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05a666")))
void SetTaskHandler_05a666(void) {
    _a1_ptr = &TaskHandler_05a66e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05a75c")))
void SetTaskHandler_05a75c(void) {
    _a1_ptr = &TaskHandler_05a764;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05a7e8")))
void SetTaskHandler_05a7e8(void) {
    _a1_ptr = &TaskHandler_05a764;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05cbe4")))
void SetTaskHandler_05cbe4(void) {
    _a1_ptr = &TaskHandler_05cbea;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05f03e")))
void SetTaskHandler_05f03e(void) {
    _a1_ptr = &TaskHandler_05f0b0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05f0a8")))
void SetTaskHandler_05f0a8(void) {
    _a1_ptr = &TaskHandler_05f00a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05f112")))
void SetTaskHandler_05f112(void) {
    _a1_ptr = &TaskHandler_05f00a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05f3ea")))
void SetTaskHandler_05f3ea(void) {
    _a1_ptr = &TaskHandler_05f482;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fa4e")))
void SetTaskHandler_05fa4e(void) {
    _a1_ptr = &TaskHandler_05fa56;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fa9e")))
void SetTaskHandler_05fa9e(void) {
    _a1_ptr = &TaskHandler_05fb88;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fb1c")))
void SetTaskHandler_05fb1c(void) {
    _a1_ptr = &TaskHandler_05fb88;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fb80")))
void SetTaskHandler_05fb80(void) {
    _a1_ptr = &TaskHandler_05fb88;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fbde")))
void SetTaskHandler_05fbde(void) {
    _a1_ptr = &TaskHandler_05fbe6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fc1e")))
void SetTaskHandler_05fc1e(void) {
    _a1_ptr = &TaskHandler_05fce6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fc70")))
void SetTaskHandler_05fc70(void) {
    _a1_ptr = &TaskHandler_05fce6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_05fcde")))
void SetTaskHandler_05fcde(void) {
    _a1_ptr = &TaskHandler_05fce6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0606e6")))
void SetTaskHandler_0606e6(void) {
    _a1_ptr = &TaskHandler_0606ee;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_060bee")))
void SetTaskHandler_060bee(void) {
    _a1_ptr = &TaskHandler_060bf6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_060c92")))
void SetTaskHandler_060c92(void) {
    _a1_ptr = &TaskHandler_060c40;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_060d32")))
void SetTaskHandler_060d32(void) {
    _a1_ptr = &TaskHandler_060d3a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_060d98")))
void SetTaskHandler_060d98(void) {
    _a1_ptr = &TaskHandler_060de8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_060de0")))
void SetTaskHandler_060de0(void) {
    _a1_ptr = &TaskHandler_060de8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_061f96")))
void SetTaskHandler_061f96(void) {
    _a1_ptr = &TaskHandler_0620a0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062000")))
void SetTaskHandler_062000(void) {
    _a1_ptr = &TaskHandler_0620a0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06200c")))
void SetTaskHandler_06200c(void) {
    _a1_ptr = &TaskHandler_0620a0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06207c")))
void SetTaskHandler_06207c(void) {
    _a1_ptr = &TaskHandler_0620a0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062122")))
void SetTaskHandler_062122(void) {
    _a1_ptr = &TaskHandler_0620a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06231e")))
void SetTaskHandler_06231e(void) {
    _a1_ptr = &TaskHandler_0620a0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062352")))
void SetTaskHandler_062352(void) {
    _a1_ptr = &TaskHandler_0620a0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062414")))
void SetTaskHandler_062414(void) {
    _a1_ptr = &TaskHandler_0620a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0624ce")))
void SetTaskHandler_0624ce(void) {
    _a1_ptr = &TaskHandler_0620a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06252e")))
void SetTaskHandler_06252e(void) {
    _a1_ptr = &TaskHandler_0620a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062632")))
void SetTaskHandler_062632(void) {
    _a1_ptr = &TaskHandler_0620a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06267c")))
void SetTaskHandler_06267c(void) {
    _a1_ptr = &TaskHandler_0620a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062a00")))
void SetTaskHandler_062a00(void) {
    _a1_ptr = &TaskHandler_062f8c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062dc4")))
void SetTaskHandler_062dc4(void) {
    _a1_ptr = &TaskHandler_062f8c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062e3c")))
void SetTaskHandler_062e3c(void) {
    _a1_ptr = &TaskHandler_062f8c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062e82")))
void SetTaskHandler_062e82(void) {
    _a1_ptr = &TaskHandler_062f8c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062ed8")))
void SetTaskHandler_062ed8(void) {
    _a1_ptr = &TaskHandler_062f8c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_062f84")))
void SetTaskHandler_062f84(void) {
    _a1_ptr = &TaskHandler_062f8c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_063040")))
void SetTaskHandler_063040(void) {
    _a1_ptr = &TaskHandler_062f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0630b4")))
void SetTaskHandler_0630b4(void) {
    _a1_ptr = &TaskHandler_062f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0630fe")))
void SetTaskHandler_0630fe(void) {
    _a1_ptr = &TaskHandler_062f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_063134")))
void SetTaskHandler_063134(void) {
    _a1_ptr = &TaskHandler_062f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0631c8")))
void SetTaskHandler_0631c8(void) {
    _a1_ptr = &TaskHandler_062f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06321e")))
void SetTaskHandler_06321e(void) {
    _a1_ptr = &TaskHandler_062f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_063312")))
void SetTaskHandler_063312(void) {
    _a1_ptr = &TaskHandler_062f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_063882")))
void SetTaskHandler_063882(void) {
    _a1_ptr = &TaskHandler_063942;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0638fc")))
void SetTaskHandler_0638fc(void) {
    _a1_ptr = &TaskHandler_063942;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_063b30")))
void SetTaskHandler_063b30(void) {
    _a1_ptr = &TaskHandler_063952;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_063bae")))
void SetTaskHandler_063bae(void) {
    _a1_ptr = &TaskHandler_063952;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_063c04")))
void SetTaskHandler_063c04(void) {
    _a1_ptr = &TaskHandler_063952;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0641ca")))
void SetTaskHandler_0641ca(void) {
    _a1_ptr = &TaskHandler_064222;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06421a")))
void SetTaskHandler_06421a(void) {
    _a1_ptr = &TaskHandler_064222;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_064372")))
void SetTaskHandler_064372(void) {
    _a1_ptr = &TaskHandler_06422a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0643d2")))
void SetTaskHandler_0643d2(void) {
    _a1_ptr = &TaskHandler_06422a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0646f8")))
void SetTaskHandler_0646f8(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_064a78")))
void SetTaskHandler_064a78(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_064ad0")))
void SetTaskHandler_064ad0(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_064b10")))
void SetTaskHandler_064b10(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_064ce6")))
void SetTaskHandler_064ce6(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_064d72")))
void SetTaskHandler_064d72(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065034")))
void SetTaskHandler_065034(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065046")))
void SetTaskHandler_065046(void) {
    _a1_ptr = &TaskHandler_064d7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0650b0")))
void SetTaskHandler_0650b0(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0650f6")))
void SetTaskHandler_0650f6(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065156")))
void SetTaskHandler_065156(void) {
    _a1_ptr = &TaskHandler_06515e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065274")))
void SetTaskHandler_065274(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0652ec")))
void SetTaskHandler_0652ec(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065372")))
void SetTaskHandler_065372(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0653b6")))
void SetTaskHandler_0653b6(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06540c")))
void SetTaskHandler_06540c(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065462")))
void SetTaskHandler_065462(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0655ec")))
void SetTaskHandler_0655ec(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065660")))
void SetTaskHandler_065660(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065764")))
void SetTaskHandler_065764(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065a9c")))
void SetTaskHandler_065a9c(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_065aec")))
void SetTaskHandler_065aec(void) {
    _a1_ptr = &TaskHandler_064d8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_066794")))
void SetTaskHandler_066794(void) {
    _a1_ptr = &TaskHandler_0667a4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06679c")))
void SetTaskHandler_06679c(void) {
    _a1_ptr = &TaskHandler_0667a4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_066a7e")))
void SetTaskHandler_066a7e(void) {
    _a1_ptr = &TaskHandler_066a86;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_066b64")))
void SetTaskHandler_066b64(void) {
    _a1_ptr = &TaskHandler_066a8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_066bc2")))
void SetTaskHandler_066bc2(void) {
    _a1_ptr = &TaskHandler_066a8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0671d4")))
void SetTaskHandler_0671d4(void) {
    _a1_ptr = &TaskHandler_067b7c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06764e")))
void SetTaskHandler_06764e(void) {
    _a1_ptr = &TaskHandler_067b7c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_067778")))
void SetTaskHandler_067778(void) {
    _a1_ptr = &TaskHandler_067b7c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_067b74")))
void SetTaskHandler_067b74(void) {
    _a1_ptr = &TaskHandler_067b7c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_067c74")))
void SetTaskHandler_067c74(void) {
    _a1_ptr = &TaskHandler_067b84;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_067ce0")))
void SetTaskHandler_067ce0(void) {
    _a1_ptr = &TaskHandler_067b84;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_067e12")))
void SetTaskHandler_067e12(void) {
    _a1_ptr = &TaskHandler_067b84;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_067e72")))
void SetTaskHandler_067e72(void) {
    _a1_ptr = &TaskHandler_067b84;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_068308")))
void SetTaskHandler_068308(void) {
    _a1_ptr = &TaskHandler_068310;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06833e")))
void SetTaskHandler_06833e(void) {
    _a1_ptr = &TaskHandler_068346;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0685d0")))
void SetTaskHandler_0685d0(void) {
    _a1_ptr = &TaskHandler_068684;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06861a")))
void SetTaskHandler_06861a(void) {
    _a1_ptr = &TaskHandler_068684;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06867c")))
void SetTaskHandler_06867c(void) {
    _a1_ptr = &TaskHandler_06850e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0686c6")))
void SetTaskHandler_0686c6(void) {
    _a1_ptr = &TaskHandler_06850e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_068942")))
void SetTaskHandler_068942(void) {
    _a1_ptr = &TaskHandler_06895c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_068954")))
void SetTaskHandler_068954(void) {
    _a1_ptr = &TaskHandler_0688f8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0697c6")))
void SetTaskHandler_0697c6(void) {
    _a1_ptr = &TaskHandler_06986a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_069862")))
void SetTaskHandler_069862(void) {
    _a1_ptr = &TaskHandler_06986a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0699aa")))
void SetTaskHandler_0699aa(void) {
    _a1_ptr = &TaskHandler_069872;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_069ae0")))
void SetTaskHandler_069ae0(void) {
    _a1_ptr = &TaskHandler_069872;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_069ba2")))
void SetTaskHandler_069ba2(void) {
    _a1_ptr = &TaskHandler_069872;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_069cc8")))
void SetTaskHandler_069cc8(void) {
    _a1_ptr = &TaskHandler_069872;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06a3ce")))
void SetTaskHandler_06a3ce(void) {
    _a1_ptr = &TaskHandler_06a41c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06a414")))
void SetTaskHandler_06a414(void) {
    _a1_ptr = &TaskHandler_06a41c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06a44a")))
void SetTaskHandler_06a44a(void) {
    _a1_ptr = &TaskHandler_06a452;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06a7b4")))
void SetTaskHandler_06a7b4(void) {
    _a1_ptr = &TaskHandler_06a41c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06a964")))
void SetTaskHandler_06a964(void) {
    _a1_ptr = &TaskHandler_06a41c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06a9fc")))
void SetTaskHandler_06a9fc(void) {
    _a1_ptr = &TaskHandler_06a93e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06abda")))
void SetTaskHandler_06abda(void) {
    _a1_ptr = &TaskHandler_06a41c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06ac54")))
void SetTaskHandler_06ac54(void) {
    _a1_ptr = &TaskHandler_06a41c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06aca2")))
void SetTaskHandler_06aca2(void) {
    _a1_ptr = &TaskHandler_06acaa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06afee")))
void SetTaskHandler_06afee(void) {
    _a1_ptr = &TaskHandler_06a468;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06b00c")))
void SetTaskHandler_06b00c(void) {
    _a1_ptr = &TaskHandler_06a468;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06b1cc")))
void SetTaskHandler_06b1cc(void) {
    _a1_ptr = &TaskHandler_06a468;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06c460")))
void SetTaskHandler_06c460(void) {
    _a1_ptr = &TaskHandler_06c4c6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06c4be")))
void SetTaskHandler_06c4be(void) {
    _a1_ptr = &TaskHandler_06c4c6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06c536")))
void SetTaskHandler_06c536(void) {
    _a1_ptr = &TaskHandler_06c53e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06ce40")))
void SetTaskHandler_06ce40(void) {
    _a1_ptr = &TaskHandler_06c554;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06ce5e")))
void SetTaskHandler_06ce5e(void) {
    _a1_ptr = &TaskHandler_06c554;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06d046")))
void SetTaskHandler_06d046(void) {
    _a1_ptr = &TaskHandler_06c554;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06d9e4")))
void SetTaskHandler_06d9e4(void) {
    _a1_ptr = &TaskHandler_06da7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06da44")))
void SetTaskHandler_06da44(void) {
    _a1_ptr = &TaskHandler_06da7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06dbcc")))
void SetTaskHandler_06dbcc(void) {
    _a1_ptr = &TaskHandler_06da90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06dc56")))
void SetTaskHandler_06dc56(void) {
    _a1_ptr = &TaskHandler_06da90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06dc88")))
void SetTaskHandler_06dc88(void) {
    _a1_ptr = &TaskHandler_06da90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06dcd8")))
void SetTaskHandler_06dcd8(void) {
    _a1_ptr = &TaskHandler_06da90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06defe")))
void SetTaskHandler_06defe(void) {
    _a1_ptr = &TaskHandler_06da90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e05a")))
void SetTaskHandler_06e05a(void) {
    _a1_ptr = &TaskHandler_06e062;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e156")))
void SetTaskHandler_06e156(void) {
    _a1_ptr = &TaskHandler_06da90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e16e")))
void SetTaskHandler_06e16e(void) {
    _a1_ptr = &TaskHandler_06da90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e664")))
void SetTaskHandler_06e664(void) {
    _a1_ptr = &TaskHandler_06e80a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e74a")))
void SetTaskHandler_06e74a(void) {
    _a1_ptr = &TaskHandler_06e80a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e884")))
void SetTaskHandler_06e884(void) {
    _a1_ptr = &TaskHandler_06e88c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e92a")))
void SetTaskHandler_06e92a(void) {
    _a1_ptr = &TaskHandler_06e932;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06e95e")))
void SetTaskHandler_06e95e(void) {
    _a1_ptr = &TaskHandler_06e818;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06ea8e")))
void SetTaskHandler_06ea8e(void) {
    _a1_ptr = &TaskHandler_06e818;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06eb7c")))
void SetTaskHandler_06eb7c(void) {
    _a1_ptr = &TaskHandler_06e818;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06ed3a")))
void SetTaskHandler_06ed3a(void) {
    _a1_ptr = &TaskHandler_06e818;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06ee7a")))
void SetTaskHandler_06ee7a(void) {
    _a1_ptr = &TaskHandler_06e818;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06f1ee")))
void SetTaskHandler_06f1ee(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06f338")))
void SetTaskHandler_06f338(void) {
    _a1_ptr = &TaskHandler_06f340;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06f384")))
void SetTaskHandler_06f384(void) {
    _a1_ptr = &TaskHandler_06f38c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06fa58")))
void SetTaskHandler_06fa58(void) {
    _a1_ptr = &TaskHandler_06fa60;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06faf2")))
void SetTaskHandler_06faf2(void) {
    _a1_ptr = &TaskHandler_06f9ca;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06fb98")))
void SetTaskHandler_06fb98(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06fc48")))
void SetTaskHandler_06fc48(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_06fee8")))
void SetTaskHandler_06fee8(void) {
    _a1_ptr = &TaskHandler_0700a6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070106")))
void SetTaskHandler_070106(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070166")))
void SetTaskHandler_070166(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070206")))
void SetTaskHandler_070206(void) {
    _a1_ptr = &TaskHandler_0700a6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070288")))
void SetTaskHandler_070288(void) {
    _a1_ptr = &TaskHandler_070290;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0702a0")))
void SetTaskHandler_0702a0(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0702dc")))
void SetTaskHandler_0702dc(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0704ea")))
void SetTaskHandler_0704ea(void) {
    _a1_ptr = &TaskHandler_0704f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07068c")))
void SetTaskHandler_07068c(void) {
    _a1_ptr = &TaskHandler_070694;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070734")))
void SetTaskHandler_070734(void) {
    _a1_ptr = &TaskHandler_07079e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070796")))
void SetTaskHandler_070796(void) {
    _a1_ptr = &TaskHandler_07079e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0707c0")))
void SetTaskHandler_0707c0(void) {
    _a1_ptr = &TaskHandler_0707c8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07080e")))
void SetTaskHandler_07080e(void) {
    _a1_ptr = &TaskHandler_070694;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070854")))
void SetTaskHandler_070854(void) {
    _a1_ptr = &TaskHandler_070694;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0708aa")))
void SetTaskHandler_0708aa(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0708ec")))
void SetTaskHandler_0708ec(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07091a")))
void SetTaskHandler_07091a(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07094e")))
void SetTaskHandler_07094e(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_070a2a")))
void SetTaskHandler_070a2a(void) {
    _a1_ptr = &TaskHandler_06fb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07139a")))
void SetTaskHandler_07139a(void) {
    _a1_ptr = &TaskHandler_0713a2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0715f2")))
void SetTaskHandler_0715f2(void) {
    _a1_ptr = &TaskHandler_0716b6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0716ae")))
void SetTaskHandler_0716ae(void) {
    _a1_ptr = &TaskHandler_0716b6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0716da")))
void SetTaskHandler_0716da(void) {
    _a1_ptr = &TaskHandler_0716e2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071866")))
void SetTaskHandler_071866(void) {
    _a1_ptr = &TaskHandler_0716ea;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071a46")))
void SetTaskHandler_071a46(void) {
    _a1_ptr = &TaskHandler_071b9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071afc")))
void SetTaskHandler_071afc(void) {
    _a1_ptr = &TaskHandler_071b9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071b4e")))
void SetTaskHandler_071b4e(void) {
    _a1_ptr = &TaskHandler_071b9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071b92")))
void SetTaskHandler_071b92(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071c7c")))
void SetTaskHandler_071c7c(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071db2")))
void SetTaskHandler_071db2(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071e48")))
void SetTaskHandler_071e48(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071eae")))
void SetTaskHandler_071eae(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_071f6a")))
void SetTaskHandler_071f6a(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_072012")))
void SetTaskHandler_072012(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07209e")))
void SetTaskHandler_07209e(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07213c")))
void SetTaskHandler_07213c(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0721e2")))
void SetTaskHandler_0721e2(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_072270")))
void SetTaskHandler_072270(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0722fe")))
void SetTaskHandler_0722fe(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0723ca")))
void SetTaskHandler_0723ca(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07242e")))
void SetTaskHandler_07242e(void) {
    _a1_ptr = &TaskHandler_0724d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_072482")))
void SetTaskHandler_072482(void) {
    _a1_ptr = &TaskHandler_0724d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0724cc")))
void SetTaskHandler_0724cc(void) {
    _a1_ptr = &TaskHandler_0724d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0726cc")))
void SetTaskHandler_0726cc(void) {
    _a1_ptr = &TaskHandler_0716f2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07344c")))
void SetTaskHandler_07344c(void) {
    _a1_ptr = &TaskHandler_073454;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07359e")))
void SetTaskHandler_07359e(void) {
    _a1_ptr = &TaskHandler_0735a6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073680")))
void SetTaskHandler_073680(void) {
    _a1_ptr = &TaskHandler_0734f4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073754")))
void SetTaskHandler_073754(void) {
    _a1_ptr = &TaskHandler_0734f4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0737fc")))
void SetTaskHandler_0737fc(void) {
    _a1_ptr = &TaskHandler_0738da;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07394a")))
void SetTaskHandler_07394a(void) {
    _a1_ptr = &TaskHandler_0734f4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073a2a")))
void SetTaskHandler_073a2a(void) {
    _a1_ptr = &TaskHandler_0734f4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073b5a")))
void SetTaskHandler_073b5a(void) {
    _a1_ptr = &TaskHandler_073b62;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073b8a")))
void SetTaskHandler_073b8a(void) {
    _a1_ptr = &TaskHandler_0734f4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073c20")))
void SetTaskHandler_073c20(void) {
    _a1_ptr = &TaskHandler_0734fc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073e54")))
void SetTaskHandler_073e54(void) {
    _a1_ptr = &TaskHandler_0734fc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073ec8")))
void SetTaskHandler_073ec8(void) {
    _a1_ptr = &TaskHandler_073f9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_073fc2")))
void SetTaskHandler_073fc2(void) {
    _a1_ptr = &TaskHandler_0734fc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_074106")))
void SetTaskHandler_074106(void) {
    _a1_ptr = &TaskHandler_0734fc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07519e")))
void SetTaskHandler_07519e(void) {
    _a1_ptr = &TaskHandler_0751a6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_075254")))
void SetTaskHandler_075254(void) {
    _a1_ptr = &TaskHandler_07525c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07600a")))
void SetTaskHandler_07600a(void) {
    _a1_ptr = &TaskHandler_07646e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07604e")))
void SetTaskHandler_07604e(void) {
    _a1_ptr = &TaskHandler_076056;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0760a0")))
void SetTaskHandler_0760a0(void) {
    _a1_ptr = &TaskHandler_0760a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0760c0")))
void SetTaskHandler_0760c0(void) {
    _a1_ptr = &TaskHandler_0760c8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0760ec")))
void SetTaskHandler_0760ec(void) {
    _a1_ptr = &TaskHandler_076290;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_076924")))
void SetTaskHandler_076924(void) {
    _a1_ptr = &TaskHandler_07692c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_076a88")))
void SetTaskHandler_076a88(void) {
    _a1_ptr = &TaskHandler_076a90;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_076b32")))
void SetTaskHandler_076b32(void) {
    _a1_ptr = &TaskHandler_076b3a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_076bc8")))
void SetTaskHandler_076bc8(void) {
    _a1_ptr = &TaskHandler_076bd0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_077262")))
void SetTaskHandler_077262(void) {
    _a1_ptr = &TaskHandler_07726a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07789a")))
void SetTaskHandler_07789a(void) {
    _a1_ptr = &TaskHandler_077a8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0778f4")))
void SetTaskHandler_0778f4(void) {
    _a1_ptr = &TaskHandler_077a8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_077a0e")))
void SetTaskHandler_077a0e(void) {
    _a1_ptr = &TaskHandler_077a8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_077a86")))
void SetTaskHandler_077a86(void) {
    _a1_ptr = &TaskHandler_077a8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_077b60")))
void SetTaskHandler_077b60(void) {
    _a1_ptr = &TaskHandler_077a8e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07943c")))
void SetTaskHandler_07943c(void) {
    _a1_ptr = &TaskHandler_079326;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079a82")))
void SetTaskHandler_079a82(void) {
    _a1_ptr = &TaskHandler_079a8a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079adc")))
void SetTaskHandler_079adc(void) {
    _a1_ptr = &TaskHandler_079b42;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079b1a")))
void SetTaskHandler_079b1a(void) {
    _a1_ptr = &TaskHandler_079b42;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079c0a")))
void SetTaskHandler_079c0a(void) {
    _a1_ptr = &TaskHandler_079c68;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079c34")))
void SetTaskHandler_079c34(void) {
    _a1_ptr = &TaskHandler_079c3c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079ca2")))
void SetTaskHandler_079ca2(void) {
    _a1_ptr = &TaskHandler_079caa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079cfc")))
void SetTaskHandler_079cfc(void) {
    _a1_ptr = &TaskHandler_079d76;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079d4e")))
void SetTaskHandler_079d4e(void) {
    _a1_ptr = &TaskHandler_079d76;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079e22")))
void SetTaskHandler_079e22(void) {
    _a1_ptr = &TaskHandler_079e94;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079e60")))
void SetTaskHandler_079e60(void) {
    _a1_ptr = &TaskHandler_079e68;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079f62")))
void SetTaskHandler_079f62(void) {
    _a1_ptr = &TaskHandler_079f6a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_079fe0")))
void SetTaskHandler_079fe0(void) {
    _a1_ptr = &TaskHandler_07a3e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a002")))
void SetTaskHandler_07a002(void) {
    _a1_ptr = &TaskHandler_07a00a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a0a8")))
void SetTaskHandler_07a0a8(void) {
    _a1_ptr = &TaskHandler_07a3e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a12a")))
void SetTaskHandler_07a12a(void) {
    _a1_ptr = &TaskHandler_07a3e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a196")))
void SetTaskHandler_07a196(void) {
    _a1_ptr = &TaskHandler_07a3e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a1b8")))
void SetTaskHandler_07a1b8(void) {
    _a1_ptr = &TaskHandler_07a1c0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a274")))
void SetTaskHandler_07a274(void) {
    _a1_ptr = &TaskHandler_07a3ee;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a2fe")))
void SetTaskHandler_07a2fe(void) {
    _a1_ptr = &TaskHandler_07a3ee;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a3de")))
void SetTaskHandler_07a3de(void) {
    _a1_ptr = &TaskHandler_07a3e6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07a9e8")))
void SetTaskHandler_07a9e8(void) {
    _a1_ptr = &TaskHandler_07aa78;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07aa70")))
void SetTaskHandler_07aa70(void) {
    _a1_ptr = &TaskHandler_07aa78;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07aa8c")))
void SetTaskHandler_07aa8c(void) {
    _a1_ptr = &TaskHandler_07aa94;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07abec")))
void SetTaskHandler_07abec(void) {
    _a1_ptr = &TaskHandler_07abf4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ac42")))
void SetTaskHandler_07ac42(void) {
    _a1_ptr = &TaskHandler_07ac4a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07bac6")))
void SetTaskHandler_07bac6(void) {
    _a1_ptr = &TaskHandler_07bace;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07bb12")))
void SetTaskHandler_07bb12(void) {
    _a1_ptr = &TaskHandler_07bb1a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07bc86")))
void SetTaskHandler_07bc86(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07bd1a")))
void SetTaskHandler_07bd1a(void) {
    _a1_ptr = &TaskHandler_07c374;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07bdc4")))
void SetTaskHandler_07bdc4(void) {
    _a1_ptr = &TaskHandler_07c374;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07be74")))
void SetTaskHandler_07be74(void) {
    _a1_ptr = &TaskHandler_07c374;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07beec")))
void SetTaskHandler_07beec(void) {
    _a1_ptr = &TaskHandler_07c374;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c068")))
void SetTaskHandler_07c068(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c0fe")))
void SetTaskHandler_07c0fe(void) {
    _a1_ptr = &TaskHandler_07c106;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c1ae")))
void SetTaskHandler_07c1ae(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c36c")))
void SetTaskHandler_07c36c(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c41c")))
void SetTaskHandler_07c41c(void) {
    _a1_ptr = &TaskHandler_07c424;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c472")))
void SetTaskHandler_07c472(void) {
    _a1_ptr = &TaskHandler_07c644;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c50a")))
void SetTaskHandler_07c50a(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c74c")))
void SetTaskHandler_07c74c(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c8aa")))
void SetTaskHandler_07c8aa(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07c910")))
void SetTaskHandler_07c910(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ca58")))
void SetTaskHandler_07ca58(void) {
    _a1_ptr = &TaskHandler_07c65c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07cf54")))
void SetTaskHandler_07cf54(void) {
    _a1_ptr = &TaskHandler_07cf5c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d04e")))
void SetTaskHandler_07d04e(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d170")))
void SetTaskHandler_07d170(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d226")))
void SetTaskHandler_07d226(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d32a")))
void SetTaskHandler_07d32a(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d3a8")))
void SetTaskHandler_07d3a8(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d4a2")))
void SetTaskHandler_07d4a2(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d5b8")))
void SetTaskHandler_07d5b8(void) {
    _a1_ptr = &TaskHandler_07dba8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d630")))
void SetTaskHandler_07d630(void) {
    _a1_ptr = &TaskHandler_07dba8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d69e")))
void SetTaskHandler_07d69e(void) {
    _a1_ptr = &TaskHandler_07dba8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d766")))
void SetTaskHandler_07d766(void) {
    _a1_ptr = &TaskHandler_07dba8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07d890")))
void SetTaskHandler_07d890(void) {
    _a1_ptr = &TaskHandler_07d898;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07da3e")))
void SetTaskHandler_07da3e(void) {
    _a1_ptr = &TaskHandler_07dba8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07dcae")))
void SetTaskHandler_07dcae(void) {
    _a1_ptr = &TaskHandler_07dcb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ddd8")))
void SetTaskHandler_07ddd8(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07de86")))
void SetTaskHandler_07de86(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07df64")))
void SetTaskHandler_07df64(void) {
    _a1_ptr = &TaskHandler_07dba2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07dff8")))
void SetTaskHandler_07dff8(void) {
    _a1_ptr = &TaskHandler_07e078;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e034")))
void SetTaskHandler_07e034(void) {
    _a1_ptr = &TaskHandler_07e08c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e070")))
void SetTaskHandler_07e070(void) {
    _a1_ptr = &TaskHandler_07e078;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e25c")))
void SetTaskHandler_07e25c(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e292")))
void SetTaskHandler_07e292(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e2ca")))
void SetTaskHandler_07e2ca(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e2ea")))
void SetTaskHandler_07e2ea(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e404")))
void SetTaskHandler_07e404(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e4fc")))
void SetTaskHandler_07e4fc(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e5fa")))
void SetTaskHandler_07e5fa(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e66c")))
void SetTaskHandler_07e66c(void) {
    _a1_ptr = &TaskHandler_07e674;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e726")))
void SetTaskHandler_07e726(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e84c")))
void SetTaskHandler_07e84c(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e862")))
void SetTaskHandler_07e862(void) {
    _a1_ptr = &TaskHandler_07e880;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e878")))
void SetTaskHandler_07e878(void) {
    _a1_ptr = &TaskHandler_07e880;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e97e")))
void SetTaskHandler_07e97e(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07e9d2")))
void SetTaskHandler_07e9d2(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ea26")))
void SetTaskHandler_07ea26(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ea78")))
void SetTaskHandler_07ea78(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07eaba")))
void SetTaskHandler_07eaba(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07eb22")))
void SetTaskHandler_07eb22(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07eb6e")))
void SetTaskHandler_07eb6e(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ebde")))
void SetTaskHandler_07ebde(void) {
    _a1_ptr = &TaskHandler_07ebe6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ecc8")))
void SetTaskHandler_07ecc8(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ecf8")))
void SetTaskHandler_07ecf8(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07edec")))
void SetTaskHandler_07edec(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ee34")))
void SetTaskHandler_07ee34(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ee72")))
void SetTaskHandler_07ee72(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ef0a")))
void SetTaskHandler_07ef0a(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07ef80")))
void SetTaskHandler_07ef80(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07efc4")))
void SetTaskHandler_07efc4(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f012")))
void SetTaskHandler_07f012(void) {
    _a1_ptr = &TaskHandler_07f186;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f01a")))
void SetTaskHandler_07f01a(void) {
    _a1_ptr = &TaskHandler_07f022;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f058")))
void SetTaskHandler_07f058(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f0f4")))
void SetTaskHandler_07f0f4(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f140")))
void SetTaskHandler_07f140(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f17e")))
void SetTaskHandler_07f17e(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f222")))
void SetTaskHandler_07f222(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f27a")))
void SetTaskHandler_07f27a(void) {
    _a1_ptr = &TaskHandler_07f2ca;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f2c2")))
void SetTaskHandler_07f2c2(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f3e0")))
void SetTaskHandler_07f3e0(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f45c")))
void SetTaskHandler_07f45c(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f4be")))
void SetTaskHandler_07f4be(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f52a")))
void SetTaskHandler_07f52a(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f59c")))
void SetTaskHandler_07f59c(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f5e2")))
void SetTaskHandler_07f5e2(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f634")))
void SetTaskHandler_07f634(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f66c")))
void SetTaskHandler_07f66c(void) {
    _a1_ptr = &TaskHandler_07f282;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f760")))
void SetTaskHandler_07f760(void) {
    _a1_ptr = &TaskHandler_07fdb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f7e8")))
void SetTaskHandler_07f7e8(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f842")))
void SetTaskHandler_07f842(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f874")))
void SetTaskHandler_07f874(void) {
    _a1_ptr = &TaskHandler_07f87c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f8d4")))
void SetTaskHandler_07f8d4(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f980")))
void SetTaskHandler_07f980(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07f9c8")))
void SetTaskHandler_07f9c8(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fa18")))
void SetTaskHandler_07fa18(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fa8a")))
void SetTaskHandler_07fa8a(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fac4")))
void SetTaskHandler_07fac4(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fb20")))
void SetTaskHandler_07fb20(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fb66")))
void SetTaskHandler_07fb66(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fbca")))
void SetTaskHandler_07fbca(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fc12")))
void SetTaskHandler_07fc12(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fd40")))
void SetTaskHandler_07fd40(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_07fda0")))
void SetTaskHandler_07fda0(void) {
    _a1_ptr = &TaskHandler_07fdbc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0800c4")))
void SetTaskHandler_0800c4(void) {
    _a1_ptr = &TaskHandler_0803e8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080108")))
void SetTaskHandler_080108(void) {
    _a1_ptr = &TaskHandler_0803e8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0801e6")))
void SetTaskHandler_0801e6(void) {
    _a1_ptr = &TaskHandler_0803d2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080226")))
void SetTaskHandler_080226(void) {
    _a1_ptr = &TaskHandler_0803d2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0802d8")))
void SetTaskHandler_0802d8(void) {
    _a1_ptr = &TaskHandler_0803d2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08037a")))
void SetTaskHandler_08037a(void) {
    _a1_ptr = &TaskHandler_080382;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0803ca")))
void SetTaskHandler_0803ca(void) {
    _a1_ptr = &TaskHandler_0803d2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080500")))
void SetTaskHandler_080500(void) {
    _a1_ptr = &TaskHandler_080508;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080538")))
void SetTaskHandler_080538(void) {
    _a1_ptr = &TaskHandler_080454;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0805e6")))
void SetTaskHandler_0805e6(void) {
    _a1_ptr = &TaskHandler_0805ee;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080676")))
void SetTaskHandler_080676(void) {
    _a1_ptr = &TaskHandler_0811e4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08072e")))
void SetTaskHandler_08072e(void) {
    _a1_ptr = &TaskHandler_081214;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080898")))
void SetTaskHandler_080898(void) {
    _a1_ptr = &TaskHandler_081214;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080bce")))
void SetTaskHandler_080bce(void) {
    _a1_ptr = &TaskHandler_0811cc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080cd4")))
void SetTaskHandler_080cd4(void) {
    _a1_ptr = &TaskHandler_0811cc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080d8a")))
void SetTaskHandler_080d8a(void) {
    _a1_ptr = &TaskHandler_0811cc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080e40")))
void SetTaskHandler_080e40(void) {
    _a1_ptr = &TaskHandler_0811cc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_080ef2")))
void SetTaskHandler_080ef2(void) {
    _a1_ptr = &TaskHandler_0811cc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0812b6")))
void SetTaskHandler_0812b6(void) {
    _a1_ptr = &TaskHandler_0812be;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_081344")))
void SetTaskHandler_081344(void) {
    _a1_ptr = &TaskHandler_081018;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_081632")))
void SetTaskHandler_081632(void) {
    _a1_ptr = &TaskHandler_081214;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_081666")))
void SetTaskHandler_081666(void) {
    _a1_ptr = &TaskHandler_081214;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_081cce")))
void SetTaskHandler_081cce(void) {
    _a1_ptr = &TaskHandler_081bee;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_081d5c")))
void SetTaskHandler_081d5c(void) {
    _a1_ptr = &TaskHandler_081d64;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_081fe8")))
void SetTaskHandler_081fe8(void) {
    _a1_ptr = &TaskHandler_081ff0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08204a")))
void SetTaskHandler_08204a(void) {
    _a1_ptr = &TaskHandler_081fac;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08244e")))
void SetTaskHandler_08244e(void) {
    _a1_ptr = &TaskHandler_082388;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08250c")))
void SetTaskHandler_08250c(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08255a")))
void SetTaskHandler_08255a(void) {
    _a1_ptr = &TaskHandler_082464;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08261e")))
void SetTaskHandler_08261e(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_082674")))
void SetTaskHandler_082674(void) {
    _a1_ptr = &TaskHandler_082464;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_082834")))
void SetTaskHandler_082834(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_082924")))
void SetTaskHandler_082924(void) {
    _a1_ptr = &TaskHandler_082464;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0829a4")))
void SetTaskHandler_0829a4(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_082a5e")))
void SetTaskHandler_082a5e(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_082b3e")))
void SetTaskHandler_082b3e(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_082bb0")))
void SetTaskHandler_082bb0(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_082c00")))
void SetTaskHandler_082c00(void) {
    _a1_ptr = &TaskHandler_082456;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084408")))
void SetTaskHandler_084408(void) {
    _a1_ptr = &TaskHandler_084410;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0844b8")))
void SetTaskHandler_0844b8(void) {
    _a1_ptr = &TaskHandler_0844c0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084504")))
void SetTaskHandler_084504(void) {
    _a1_ptr = &TaskHandler_08450c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084892")))
void SetTaskHandler_084892(void) {
    _a1_ptr = &TaskHandler_08489a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0848d6")))
void SetTaskHandler_0848d6(void) {
    _a1_ptr = &TaskHandler_0848de;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084b1c")))
void SetTaskHandler_084b1c(void) {
    _a1_ptr = &TaskHandler_084b24;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084b92")))
void SetTaskHandler_084b92(void) {
    _a1_ptr = &TaskHandler_084b9a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084bca")))
void SetTaskHandler_084bca(void) {
    _a1_ptr = &TaskHandler_084bd2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084c1e")))
void SetTaskHandler_084c1e(void) {
    _a1_ptr = &TaskHandler_084c26;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084f4e")))
void SetTaskHandler_084f4e(void) {
    _a1_ptr = &TaskHandler_084fca;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_084f56")))
void SetTaskHandler_084f56(void) {
    _a1_ptr = &TaskHandler_084f5e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08512c")))
void SetTaskHandler_08512c(void) {
    _a1_ptr = &TaskHandler_085134;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08547c")))
void SetTaskHandler_08547c(void) {
    _a1_ptr = &TaskHandler_085484;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_085ac6")))
void SetTaskHandler_085ac6(void) {
    _a1_ptr = &TaskHandler_085a08;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0865b6")))
void SetTaskHandler_0865b6(void) {
    _a1_ptr = &TaskHandler_0865be;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08684c")))
void SetTaskHandler_08684c(void) {
    _a1_ptr = &TaskHandler_086854;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_089390")))
void SetTaskHandler_089390(void) {
    _a1_ptr = &TaskHandler_089398;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0894fc")))
void SetTaskHandler_0894fc(void) {
    _a1_ptr = &TaskHandler_089504;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0895c4")))
void SetTaskHandler_0895c4(void) {
    _a1_ptr = &TaskHandler_0895cc;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_089958")))
void SetTaskHandler_089958(void) {
    _a1_ptr = &TaskHandler_089960;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0899c8")))
void SetTaskHandler_0899c8(void) {
    _a1_ptr = &TaskHandler_089a04;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0899fc")))
void SetTaskHandler_0899fc(void) {
    _a1_ptr = &TaskHandler_0898d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_089a28")))
void SetTaskHandler_089a28(void) {
    _a1_ptr = &TaskHandler_0898d4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08a2e4")))
void SetTaskHandler_08a2e4(void) {
    _a1_ptr = &TaskHandler_08a5c8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08a314")))
void SetTaskHandler_08a314(void) {
    _a1_ptr = &TaskHandler_08a31c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08a444")))
void SetTaskHandler_08a444(void) {
    _a1_ptr = &TaskHandler_08a44c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08a50e")))
void SetTaskHandler_08a50e(void) {
    _a1_ptr = &TaskHandler_08a516;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08a978")))
void SetTaskHandler_08a978(void) {
    _a1_ptr = &TaskHandler_08ac92;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08a9a8")))
void SetTaskHandler_08a9a8(void) {
    _a1_ptr = &TaskHandler_08a9b0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08aaea")))
void SetTaskHandler_08aaea(void) {
    _a1_ptr = &TaskHandler_08aaf2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08abcc")))
void SetTaskHandler_08abcc(void) {
    _a1_ptr = &TaskHandler_08abd4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08ae30")))
void SetTaskHandler_08ae30(void) {
    _a1_ptr = &TaskHandler_08ae38;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08af60")))
void SetTaskHandler_08af60(void) {
    _a1_ptr = &TaskHandler_08af68;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08b02c")))
void SetTaskHandler_08b02c(void) {
    _a1_ptr = &TaskHandler_08b03c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08b102")))
void SetTaskHandler_08b102(void) {
    _a1_ptr = &TaskHandler_08b10a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08bb22")))
void SetTaskHandler_08bb22(void) {
    _a1_ptr = &TaskHandler_08bb84;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08c670")))
void SetTaskHandler_08c670(void) {
    _a1_ptr = &TaskHandler_08c678;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08c932")))
void SetTaskHandler_08c932(void) {
    _a1_ptr = &TaskHandler_08c8fa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08cf1a")))
void SetTaskHandler_08cf1a(void) {
    _a1_ptr = &TaskHandler_08cf22;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08cf64")))
void SetTaskHandler_08cf64(void) {
    _a1_ptr = &TaskHandler_08cf6c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08cfae")))
void SetTaskHandler_08cfae(void) {
    _a1_ptr = &TaskHandler_08cfb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08cff8")))
void SetTaskHandler_08cff8(void) {
    _a1_ptr = &TaskHandler_08d000;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d042")))
void SetTaskHandler_08d042(void) {
    _a1_ptr = &TaskHandler_08d04a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d08c")))
void SetTaskHandler_08d08c(void) {
    _a1_ptr = &TaskHandler_08d094;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d414")))
void SetTaskHandler_08d414(void) {
    _a1_ptr = &TaskHandler_08d41c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d448")))
void SetTaskHandler_08d448(void) {
    _a1_ptr = &TaskHandler_08d450;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d46c")))
void SetTaskHandler_08d46c(void) {
    _a1_ptr = &TaskHandler_08d472;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d4be")))
void SetTaskHandler_08d4be(void) {
    _a1_ptr = &TaskHandler_08d4c6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d4f6")))
void SetTaskHandler_08d4f6(void) {
    _a1_ptr = &TaskHandler_08d4fe;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d530")))
void SetTaskHandler_08d530(void) {
    _a1_ptr = &TaskHandler_08d5a8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d554")))
void SetTaskHandler_08d554(void) {
    _a1_ptr = &TaskHandler_08d55c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d626")))
void SetTaskHandler_08d626(void) {
    _a1_ptr = &TaskHandler_08d62e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d668")))
void SetTaskHandler_08d668(void) {
    _a1_ptr = &TaskHandler_08d580;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d722")))
void SetTaskHandler_08d722(void) {
    _a1_ptr = &TaskHandler_08d72a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d7b6")))
void SetTaskHandler_08d7b6(void) {
    _a1_ptr = &TaskHandler_08d7be;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d83a")))
void SetTaskHandler_08d83a(void) {
    _a1_ptr = &TaskHandler_08d774;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08d8ec")))
void SetTaskHandler_08d8ec(void) {
    _a1_ptr = &TaskHandler_08d8f4;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08dbc0")))
void SetTaskHandler_08dbc0(void) {
    _a1_ptr = &TaskHandler_08dbe2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08dc28")))
void SetTaskHandler_08dc28(void) {
    _a1_ptr = &TaskHandler_08db7a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08df6a")))
void SetTaskHandler_08df6a(void) {
    _a1_ptr = &TaskHandler_08df72;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08dfb0")))
void SetTaskHandler_08dfb0(void) {
    _a1_ptr = &TaskHandler_08dfb8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08e280")))
void SetTaskHandler_08e280(void) {
    _a1_ptr = &TaskHandler_08e288;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08e2e8")))
void SetTaskHandler_08e2e8(void) {
    _a1_ptr = &TaskHandler_08e2f0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08e544")))
void SetTaskHandler_08e544(void) {
    _a1_ptr = &TaskHandler_08e566;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08e5ac")))
void SetTaskHandler_08e5ac(void) {
    _a1_ptr = &TaskHandler_08e4fe;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08e61a")))
void SetTaskHandler_08e61a(void) {
    _a1_ptr = &TaskHandler_08e622;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08e70e")))
void SetTaskHandler_08e70e(void) {
    _a1_ptr = &TaskHandler_08e716;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08ea6c")))
void SetTaskHandler_08ea6c(void) {
    _a1_ptr = &TaskHandler_08ea16;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08eafa")))
void SetTaskHandler_08eafa(void) {
    _a1_ptr = &TaskHandler_08eb02;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08f962")))
void SetTaskHandler_08f962(void) {
    _a1_ptr = &TaskHandler_08f96a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fca2")))
void SetTaskHandler_08fca2(void) {
    _a1_ptr = &TaskHandler_08fcca;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fd26")))
void SetTaskHandler_08fd26(void) {
    _a1_ptr = &TaskHandler_08fd2e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fd60")))
void SetTaskHandler_08fd60(void) {
    _a1_ptr = &TaskHandler_08fd68;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fda2")))
void SetTaskHandler_08fda2(void) {
    _a1_ptr = &TaskHandler_08fdaa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fdf0")))
void SetTaskHandler_08fdf0(void) {
    _a1_ptr = &TaskHandler_08fdf8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fe2a")))
void SetTaskHandler_08fe2a(void) {
    _a1_ptr = &TaskHandler_08fe32;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fe94")))
void SetTaskHandler_08fe94(void) {
    _a1_ptr = &TaskHandler_08fe9c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08feae")))
void SetTaskHandler_08feae(void) {
    _a1_ptr = &TaskHandler_08feb6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_08fec4")))
void SetTaskHandler_08fec4(void) {
    _a1_ptr = &TaskHandler_08feca;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_090044")))
void SetTaskHandler_090044(void) {
    _a1_ptr = &TaskHandler_090098;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_090090")))
void SetTaskHandler_090090(void) {
    _a1_ptr = &TaskHandler_090098;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_090e76")))
void SetTaskHandler_090e76(void) {
    _a1_ptr = &TaskHandler_090e7e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09108e")))
void SetTaskHandler_09108e(void) {
    _a1_ptr = &TaskHandler_090e7e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_091330")))
void SetTaskHandler_091330(void) {
    _a1_ptr = &TaskHandler_091338;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0913a4")))
void SetTaskHandler_0913a4(void) {
    _a1_ptr = &TaskHandler_0913aa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_091550")))
void SetTaskHandler_091550(void) {
    _a1_ptr = &TaskHandler_091558;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_091570")))
void SetTaskHandler_091570(void) {
    _a1_ptr = &TaskHandler_09152c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_091662")))
void SetTaskHandler_091662(void) {
    _a1_ptr = &TaskHandler_091514;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0916b8")))
void SetTaskHandler_0916b8(void) {
    _a1_ptr = &TaskHandler_0916c0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09784a")))
void SetTaskHandler_09784a(void) {
    _a1_ptr = &TaskHandler_097852;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09787c")))
void SetTaskHandler_09787c(void) {
    _a1_ptr = &TaskHandler_09788c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_097884")))
void SetTaskHandler_097884(void) {
    _a1_ptr = &TaskHandler_0978fa;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0978a4")))
void SetTaskHandler_0978a4(void) {
    _a1_ptr = &TaskHandler_0978ac;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_097926")))
void SetTaskHandler_097926(void) {
    _a1_ptr = &TaskHandler_09792e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_097944")))
void SetTaskHandler_097944(void) {
    _a1_ptr = &TaskHandler_09794a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09804a")))
void SetTaskHandler_09804a(void) {
    _a1_ptr = &TaskHandler_09806a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098300")))
void SetTaskHandler_098300(void) {
    _a1_ptr = &TaskHandler_098308;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0983ee")))
void SetTaskHandler_0983ee(void) {
    _a1_ptr = &TaskHandler_0983f6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09847a")))
void SetTaskHandler_09847a(void) {
    _a1_ptr = &TaskHandler_098482;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098830")))
void SetTaskHandler_098830(void) {
    _a1_ptr = &TaskHandler_098836;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09887e")))
void SetTaskHandler_09887e(void) {
    _a1_ptr = &TaskHandler_098886;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098904")))
void SetTaskHandler_098904(void) {
    _a1_ptr = &TaskHandler_09890c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0989d8")))
void SetTaskHandler_0989d8(void) {
    _a1_ptr = &TaskHandler_0989e0;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098a80")))
void SetTaskHandler_098a80(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098acc")))
void SetTaskHandler_098acc(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098af6")))
void SetTaskHandler_098af6(void) {
    _a1_ptr = &TaskHandler_098afe;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098b5e")))
void SetTaskHandler_098b5e(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098baa")))
void SetTaskHandler_098baa(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098bf8")))
void SetTaskHandler_098bf8(void) {
    _a1_ptr = &TaskHandler_098c00;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098d1a")))
void SetTaskHandler_098d1a(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098d84")))
void SetTaskHandler_098d84(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098dbe")))
void SetTaskHandler_098dbe(void) {
    _a1_ptr = &TaskHandler_098dc6;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098e46")))
void SetTaskHandler_098e46(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098e92")))
void SetTaskHandler_098e92(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098f38")))
void SetTaskHandler_098f38(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098f9a")))
void SetTaskHandler_098f9a(void) {
    _a1_ptr = &TaskHandler_099004;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_098ffc")))
void SetTaskHandler_098ffc(void) {
    _a1_ptr = &TaskHandler_099004;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099068")))
void SetTaskHandler_099068(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0990b4")))
void SetTaskHandler_0990b4(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099116")))
void SetTaskHandler_099116(void) {
    _a1_ptr = &TaskHandler_099180;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099178")))
void SetTaskHandler_099178(void) {
    _a1_ptr = &TaskHandler_099180;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0991d2")))
void SetTaskHandler_0991d2(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099212")))
void SetTaskHandler_099212(void) {
    _a1_ptr = &TaskHandler_09921a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099280")))
void SetTaskHandler_099280(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0992d0")))
void SetTaskHandler_0992d0(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09933e")))
void SetTaskHandler_09933e(void) {
    _a1_ptr = &TaskHandler_09976a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09939a")))
void SetTaskHandler_09939a(void) {
    _a1_ptr = &TaskHandler_0993a2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0993f2")))
void SetTaskHandler_0993f2(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09943e")))
void SetTaskHandler_09943e(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09949a")))
void SetTaskHandler_09949a(void) {
    _a1_ptr = &TaskHandler_0993a2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0994f6")))
void SetTaskHandler_0994f6(void) {
    _a1_ptr = &TaskHandler_0993a2;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099536")))
void SetTaskHandler_099536(void) {
    _a1_ptr = &TaskHandler_09953e;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099588")))
void SetTaskHandler_099588(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0995c4")))
void SetTaskHandler_0995c4(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099608")))
void SetTaskHandler_099608(void) {
    _a1_ptr = &TaskHandler_099610;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099668")))
void SetTaskHandler_099668(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_0996ae")))
void SetTaskHandler_0996ae(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099762")))
void SetTaskHandler_099762(void) {
    _a1_ptr = &TaskHandler_09976a;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09978c")))
void SetTaskHandler_09978c(void) {
    _a1_ptr = &TaskHandler_099794;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_099a5c")))
void SetTaskHandler_099a5c(void) {
    _a1_ptr = &TaskHandler_099a64;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09a2b0")))
void SetTaskHandler_09a2b0(void) {
    _a1_ptr = &TaskHandler_09a2b8;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09a2f8")))
void SetTaskHandler_09a2f8(void) {
    _a1_ptr = &TaskHandler_09a280;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09b662")))
void SetTaskHandler_09b662(void) {
    _a1_ptr = &TaskHandler_09b47c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09b752")))
void SetTaskHandler_09b752(void) {
    _a1_ptr = &TaskHandler_09b47c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09b7b4")))
void SetTaskHandler_09b7b4(void) {
    _a1_ptr = &TaskHandler_09b47c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09b7f0")))
void SetTaskHandler_09b7f0(void) {
    _a1_ptr = &TaskHandler_09b47c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_09b824")))
void SetTaskHandler_09b824(void) {
    _a1_ptr = &TaskHandler_09b47c;
    STORE_A1_AT_FP();
}

__attribute__((section(".text.SetTaskHandler_18d6f0")))
void SetTaskHandler_18d6f0(void) {
    _a1_ptr = &TaskHandler_18d74e;
    STORE_A1_AT_FP();
}

