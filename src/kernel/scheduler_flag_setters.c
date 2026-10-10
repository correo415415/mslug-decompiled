/*
 * Metal Slug 1 — Pequeños setters de flags + tail-call al scheduler
 * ==================================================================
 * Familia de funciones muy cortas que escriben un byte global y encadenan
 * inmediatamente con el scheduler común ($000518) mediante `jmp abs.l`.
 *
 * Patrón observado:
 *   move.b #$FF, ($ADDR).L ; jmp $000518 ; rts
 *   clr.b          ($ADDR).L ; jmp $000518 ; rts
 *
 * Semánticamente son setters/latches de estado global que delegan el resto
 * del trabajo al scheduler principal del task actual.
 *
 * Notas de codegen:
 *  - El store se expresa en C puro sobre un lvalue volatile absoluto.
 *  - El `jmp ... ; rts` final se fuerza con asm inline puntual porque GCC
 *    transformaría la llamada final en otra forma de tail-call no idéntica.
 */

#include "mslug.h"

extern void FUN_00000518(void);

__attribute__((section(".text.SetGlobalFlagFF_029588"), noreturn))
void SetGlobalFlagFF_029588(void)
{
    __asm__ volatile(
        "move.b #0xFF, 0x106F4A \n"
        "jmp FUN_00000518       \n"
        "rts                    \n"
        ::: "memory");
    __builtin_unreachable();
}

__attribute__((section(".text.ClearGlobalFlag_029598"), noreturn))
void ClearGlobalFlag_029598(void)
{
    __asm__ volatile(
        "clr.b 0x106F4A         \n"
        "jmp FUN_00000518       \n"
        "rts                    \n"
        ::: "memory", "cc");
    __builtin_unreachable();
}

__attribute__((section(".text.ClearGlobalFlag_0526aa"), noreturn))
void ClearGlobalFlag_0526aa(void)
{
    __asm__ volatile(
        "clr.b 0x10A2C8         \n"
        "jmp FUN_00000518       \n"
        "rts                    \n"
        ::: "memory", "cc");
    __builtin_unreachable();
}

__attribute__((section(".text.ClearGlobalFlag_08c7f2"), noreturn))
void ClearGlobalFlag_08c7f2(void)
{
    __asm__ volatile(
        "clr.b 0x10E2EE         \n"
        "jmp FUN_00000518       \n"
        "rts                    \n"
        ::: "memory", "cc");
    __builtin_unreachable();
}

__attribute__((section(".text.SetGlobalFlagFF_18db5a"), noreturn))
void SetGlobalFlagFF_18db5a(void)
{
    __asm__ volatile(
        "move.b #0xFF, 0x1081AE \n"
        "jmp FUN_00000518       \n"
        "rts                    \n"
        ::: "memory");
    __builtin_unreachable();
}

__attribute__((section(".text.ClearGlobalFlag_18db6a"), noreturn))
void ClearGlobalFlag_18db6a(void)
{
    __asm__ volatile(
        "clr.b 0x1081AE         \n"
        "jmp FUN_00000518       \n"
        "rts                    \n"
        ::: "memory", "cc");
    __builtin_unreachable();
}
