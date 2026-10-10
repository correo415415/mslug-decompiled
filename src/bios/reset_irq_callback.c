/*
 * Metal Slug 1 — ResetIrqCallback ($0009A8, 12 bytes)
 * =====================================================
 * Reinstala el callback de VBlank del juego, apuntándolo al stub por
 * defecto que vive en $0008F2 (un simple `rts`). Se llama desde
 * GameFrame al inicio de cada frame — así, si alguien había instalado
 * un callback temporal (p. ej. la rutina de fade), queda desactivado
 * automáticamente cuando ese consumidor termina de usarlo.
 *
 * Bytes originales:
 *   $0009A8: 41FA FF48           lea    (pc+(-0xB8)).w, a0   ; a0 = $0008F2
 *   $0009AC: 23C8 0010 6EA8      move.l a0, ($106EA8).L      ; g_vblank_cb = a0
 *   $0009B2: 4E75                rts
 *
 * Detalles de codegen (para replicar la elección del compilador original):
 *  - El compilador original (SN Systems SDK) usa `lea pc+disp,a0` + `move.l a0,abs.l`
 *    en vez de `move.l #imm,abs.l`. Para forzar esa forma en GCC hay que:
 *      (a) hacer que el puntero pase por un An intermedio -> alias register
 *          global `_a0_ptr` en A0 (declarado en mslug.h);
 *      (b) compilar CON `-mpcrel` para que el `lea` de un símbolo cercano
 *          use el modo PC-relativo corto (4 B) en lugar del absoluto largo (6 B).
 *
 * `-mpcrel` NO es apto para todo el proyecto porque forzaría PC-rel 16-bit
 * incluso para llamadas fuera de rango (BIOS_FIX_CLEAR @ $C004C2 explota).
 * Por eso lo activamos SOLO en este archivo — el matcher (match_batch.py /
 * match_c.py) lo aplica vía PER_FILE_CFLAGS.
 *
 * En C:  g_vblank_callback = &VBlankCallbackDefault;   (via A0)
 */

#include "mslug.h"

#define USE_A0
#include "mslug_regs.h"

void ResetIrqCallback(void)
{
    _a0_ptr = (void (*)(void)) &VBlankCallbackDefault;
    g_vblank_callback = _a0_ptr;
}
