/*
 * Metal Slug 1 — Familia SetTaskB: setters `move.b d0, off(fp) ; rts`
 * =====================================================================
 * Funciones de 6 bytes que guardan el byte bajo de d0 en un offset
 * concreto de la entidad apuntada por fp (A6) y retornan. Son inlines
 * del compilador original del tipo:
 *
 *     void SetField_XX(u8 v) { fp->field_XX = v; }
 *
 * GCC 13 -Os reproduce el patrón exacto (1D40 00XX 4E75) desde el C
 * canónico `TASK_B(0x??) = (u8)_d0_w;`.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_task_setters_b.py.
 */

#include "mslug.h"

#define USE_D0
#include "mslug_regs.h"

__attribute__((section(".text.SetTaskB_029424")))
void SetTaskB_029424(void) { TASK_B(0x44) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_02aaba")))
void SetTaskB_02aaba(void) { TASK_B(0x90) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_032f82")))
void SetTaskB_032f82(void) { TASK_B(0x79) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_032fb4")))
void SetTaskB_032fb4(void) { TASK_B(0x79) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_03945a")))
void SetTaskB_03945a(void) { TASK_B(0x47) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_041c96")))
void SetTaskB_041c96(void) { TASK_B(0x76) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_045806")))
void SetTaskB_045806(void) { TASK_B(0x3B) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_04db6c")))
void SetTaskB_04db6c(void) { TASK_B(0x20) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_0550be")))
void SetTaskB_0550be(void) { TASK_B(0x74) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_059c0a")))
void SetTaskB_059c0a(void) { TASK_B(0x70) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_05e722")))
void SetTaskB_05e722(void) { TASK_B(0x59) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_07ed24")))
void SetTaskB_07ed24(void) { TASK_B(0x92) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_07ed44")))
void SetTaskB_07ed44(void) { TASK_B(0x92) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_085e0e")))
void SetTaskB_085e0e(void) { TASK_B(0x78) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_085e48")))
void SetTaskB_085e48(void) { TASK_B(0x78) = (u8)_d0_w; }

__attribute__((section(".text.SetTaskB_0999d8")))
void SetTaskB_0999d8(void) { TASK_B(0x6C) = (u8)_d0_w; }

