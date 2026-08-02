/*
 * Metal Slug 1 — Entity setters (funciones cortas que fijan un campo)
 * =====================================================================
 * Rutinas muy pequeñas que asignan un valor a un offset concreto de la
 * entidad apuntada por fp (A6) y opcionalmente encadenan con otra función
 * via tail-call. En el compilador original SN Systems/Nazca aparecen como
 * inlines del tipo:
 *
 *     void SetField38AndUpdate(Entity *e, u16 v) {
 *         e->field_38 = v;
 *         UpdateField38Bits(e);
 *     }
 *
 * donde el compilador transforma la última llamada en `jmp` PC-relativo.
 *
 * Convención de paso de parámetros (Nazca):
 *   - fp (A6) apunta al task/entity actual
 *   - d0     lleva el argumento primario (16 o 32 bits según el caller)
 */

#include "mslug.h"

#define USE_A0
#define USE_D0
#include "mslug_regs.h"

/* ---------------------------------------------------------------------
 * EntitySetField38AndUpdate  ($028134, 8 bytes)
 * ---------------------------------------------------------------------
 * Guarda el word en d0 en fp->field_38 y encadena con
 * Entity_ApplyFadeShade_028108 (Wave QQ#2: computa el "fade shade" de
 * los bits altos de +$38 a partir del contador +$24).
 *
 * Bytes originales:
 *   $028134: 3D40 0038         move.w  d0, 56(fp)          ; fp->field_38 = d0
 *   $028138: 4EFA FFCE         jmp     (pc+(-0x32)).w      ; -> $028108
 *
 * En C (semántico):
 *     void EntitySetField38AndUpdate(u16 value_d0) {
 *         fp->field_38 = value_d0;
 *         Entity_ApplyFadeShade_028108();  // tail-call -> jmp PC-rel
 *     }
 * -------------------------------------------------------------------- */
void EntitySetField38AndUpdate(void)
{
    TASK_W(0x38) = _d0_w;
    /* Tail-call PC-relativo corto (4EFA dddd, 4 B). El linker resuelve
     * el desplazamiento a $028108 = -0x32 desde $02813A (PC tras opcode). */
    __asm__ volatile("jmp Entity_ApplyFadeShade_028108(%%pc)" ::: "memory");
    __builtin_unreachable();
}

/* ---------------------------------------------------------------------
 * EntitySetSpriteMap  ($028CD4, 28 bytes)
 * ---------------------------------------------------------------------
 * Instala un puntero a "mapa de sprites" (recibido en a0) en dos campos
 * de la entidad (+$3C = current, +$40 = base), limpia tres contadores de
 * animación (+$3B anim_frame_lo, +$46/+$47 counters) y marca el bit 5 de
 * fp->flags_69 como "sprite_map dirty" para que el renderer sepa que hay
 * que refrescar.
 *
 * Bytes originales:
 *   $028CD4: 2D48 003C          move.l a0, 60(fp)   ; sprite_map_current = a0
 *   $028CD8: 2D48 0040          move.l a0, 64(fp)   ; sprite_map_base    = a0
 *   $028CDC: 422E 003B          clr.b  59(fp)       ; anim_frame_lo    = 0
 *   $028CE0: 422E 0046          clr.b  70(fp)       ; anim_counter_46  = 0
 *   $028CE4: 422E 0047          clr.b  71(fp)       ; anim_counter_47  = 0
 *   $028CE8: 08EE 0005 0069     bset   #5, 105(fp)  ; flags_69 |= (1<<5)
 *   $028CEE: 4E75               rts
 *
 * En C:
 *     void EntitySetSpriteMap(void *sprite_map) {   // sprite_map en a0
 *         fp->sprite_map_current = sprite_map;
 *         fp->sprite_map_base    = sprite_map;
 *         fp->anim_frame_lo      = 0;
 *         fp->anim_counter_46    = 0;
 *         fp->anim_counter_47    = 0;
 *         fp->flags_69 |= (1u << 5);
 *     }
 *
 * Necesitamos TASK_BARRIER() entre stores porque GCC fusiona con `clr.w`
 * los tres `clr.b` contiguos si no ponemos barreras. `bset #imm, d16(An)`
 * no lo emite GCC desde `|=` (usa `or.b #mask`), por eso la microinstrucción
 * final va en asm inline puntual.
 * -------------------------------------------------------------------- */
void EntitySetSpriteMap(void)
{
    TASK_L(0x3C) = (unsigned int)_a0_ptr;   TASK_BARRIER();
    TASK_L(0x40) = (unsigned int)_a0_ptr;   TASK_BARRIER();
    TASK_B(0x3B) = 0;                        TASK_BARRIER();
    TASK_B(0x46) = 0;                        TASK_BARRIER();
    TASK_B(0x47) = 0;                        TASK_BARRIER();
    __asm__ volatile("bset #5, 0x69(%%fp)" ::: "cc", "memory");
}
