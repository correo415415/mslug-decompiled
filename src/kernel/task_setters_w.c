/*
 * Metal Slug 1 — Familia SetTaskW: setters `move.w d0, off(fp) ; rts`
 * =====================================================================
 * Funciones de 6 bytes que guardan el word en d0 en un offset concreto
 * de la entidad apuntada por fp (A6) y retornan. En el compilador original
 * son inlines de un solo statement:
 *
 *     void SetField_XX(u16 v) { fp->field_XX = v; }
 *
 * GCC 13 -Os reproduce este patrón exacto (3D40 00XX 4E75) desde el C
 * canónico `TASK_W(0x??) = _d0_w;`. Cada sitio se emite en su propia
 * sección .text.SetTaskW_<addr> para que el linker las coloque en su
 * dirección de ROM correspondiente.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_task_setters_w.py — no
 * editar a mano; regenerar tras cambios en el P ROM o el escáner.
 */

#include "mslug.h"

#define USE_D0
#include "mslug_regs.h"

__attribute__((section(".text.SetTaskW_026830")))
void SetTaskW_026830(void) { TASK_W(0x2A) = _d0_w; }

__attribute__((section(".text.SetTaskW_02aa1e")))
void SetTaskW_02aa1e(void) { TASK_W(0x94) = _d0_w; }

__attribute__((section(".text.SetTaskW_0422e4")))
void SetTaskW_0422e4(void) { TASK_W(0x76) = _d0_w; }

__attribute__((section(".text.SetTaskW_042476")))
void SetTaskW_042476(void) { TASK_W(0x7E) = _d0_w; }

__attribute__((section(".text.SetTaskW_04485a")))
void SetTaskW_04485a(void) { TASK_W(0x80) = _d0_w; }

__attribute__((section(".text.SetTaskW_04a1fc")))
void SetTaskW_04a1fc(void) { TASK_W(0x28) = _d0_w; }

__attribute__((section(".text.SetTaskW_04d3f8")))
void SetTaskW_04d3f8(void) { TASK_W(0x24) = _d0_w; }

__attribute__((section(".text.SetTaskW_056e18")))
void SetTaskW_056e18(void) { TASK_W(0x8A) = _d0_w; }

__attribute__((section(".text.SetTaskW_056f9a")))
void SetTaskW_056f9a(void) { TASK_W(0x28) = _d0_w; }

__attribute__((section(".text.SetTaskW_057174")))
void SetTaskW_057174(void) { TASK_W(0x90) = _d0_w; }

__attribute__((section(".text.SetTaskW_05dcb0")))
void SetTaskW_05dcb0(void) { TASK_W(0x28) = _d0_w; }

__attribute__((section(".text.SetTaskW_05dcc8")))
void SetTaskW_05dcc8(void) { TASK_W(0x28) = _d0_w; }

__attribute__((section(".text.SetTaskW_0614b0")))
void SetTaskW_0614b0(void) { TASK_W(0x22) = _d0_w; }

__attribute__((section(".text.SetTaskW_06190a")))
void SetTaskW_06190a(void) { TASK_W(0x78) = _d0_w; }

__attribute__((section(".text.SetTaskW_061976")))
void SetTaskW_061976(void) { TASK_W(0x84) = _d0_w; }

__attribute__((section(".text.SetTaskW_0626b2")))
void SetTaskW_0626b2(void) { TASK_W(0x38) = _d0_w; }

__attribute__((section(".text.SetTaskW_065dae")))
void SetTaskW_065dae(void) { TASK_W(0x2C) = _d0_w; }

__attribute__((section(".text.SetTaskW_0665e2")))
void SetTaskW_0665e2(void) { TASK_W(0x14) = _d0_w; }

__attribute__((section(".text.SetTaskW_067f40")))
void SetTaskW_067f40(void) { TASK_W(0x72) = _d0_w; }

__attribute__((section(".text.SetTaskW_06d1d6")))
void SetTaskW_06d1d6(void) { TASK_W(0x8A) = _d0_w; }

__attribute__((section(".text.SetTaskW_06f0b2")))
void SetTaskW_06f0b2(void) { TASK_W(0x36) = _d0_w; }

__attribute__((section(".text.SetTaskW_06f11e")))
void SetTaskW_06f11e(void) { TASK_W(0x2C) = _d0_w; }

__attribute__((section(".text.SetTaskW_06f136")))
void SetTaskW_06f136(void) { TASK_W(0x2C) = _d0_w; }

__attribute__((section(".text.SetTaskW_075ff6")))
void SetTaskW_075ff6(void) { TASK_W(0x70) = _d0_w; }

__attribute__((section(".text.SetTaskW_077dde")))
void SetTaskW_077dde(void) { TASK_W(0x24) = _d0_w; }

__attribute__((section(".text.SetTaskW_07924a")))
void SetTaskW_07924a(void) { TASK_W(0x22) = _d0_w; }

__attribute__((section(".text.SetTaskW_07dc14")))
void SetTaskW_07dc14(void) { TASK_W(0x38) = _d0_w; }

__attribute__((section(".text.SetTaskW_07dc38")))
void SetTaskW_07dc38(void) { TASK_W(0x38) = _d0_w; }

__attribute__((section(".text.SetTaskW_07ed64")))
void SetTaskW_07ed64(void) { TASK_W(0x90) = _d0_w; }

__attribute__((section(".text.SetTaskW_07ed76")))
void SetTaskW_07ed76(void) { TASK_W(0x90) = _d0_w; }

__attribute__((section(".text.SetTaskW_07fee2")))
void SetTaskW_07fee2(void) { TASK_W(0x76) = _d0_w; }

__attribute__((section(".text.SetTaskW_08002e")))
void SetTaskW_08002e(void) { TASK_W(0x7C) = _d0_w; }

__attribute__((section(".text.SetTaskW_0817ca")))
void SetTaskW_0817ca(void) { TASK_W(0x76) = _d0_w; }

__attribute__((section(".text.SetTaskW_08187e")))
void SetTaskW_08187e(void) { TASK_W(0x2C) = _d0_w; }

__attribute__((section(".text.SetTaskW_081896")))
void SetTaskW_081896(void) { TASK_W(0x2C) = _d0_w; }

__attribute__((section(".text.SetTaskW_085602")))
void SetTaskW_085602(void) { TASK_W(0x72) = _d0_w; }

__attribute__((section(".text.SetTaskW_0856a4")))
void SetTaskW_0856a4(void) { TASK_W(0x72) = _d0_w; }

__attribute__((section(".text.SetTaskW_088432")))
void SetTaskW_088432(void) { TASK_W(0x14) = _d0_w; }

__attribute__((section(".text.SetTaskW_08cc3e")))
void SetTaskW_08cc3e(void) { TASK_W(0x90) = _d0_w; }

__attribute__((section(".text.SetTaskW_08d2ce")))
void SetTaskW_08d2ce(void) { TASK_W(0x22) = _d0_w; }

__attribute__((section(".text.SetTaskW_08d2f2")))
void SetTaskW_08d2f2(void) { TASK_W(0x24) = _d0_w; }

__attribute__((section(".text.SetTaskW_08d348")))
void SetTaskW_08d348(void) { TASK_W(0x24) = _d0_w; }

__attribute__((section(".text.SetTaskW_09bb36")))
void SetTaskW_09bb36(void) { TASK_W(0x14) = _d0_w; }

__attribute__((section(".text.SetTaskW_19cb64")))
void SetTaskW_19cb64(void) { TASK_W(0x28) = _d0_w; }

