/*
 * Metal Slug 1 — Setters de la velocidad del smoothing de cámara-Y
 * ==================================================================
 * Wave WW. Pareja de presets que cargan una constante en el acumulador
 * de velocidad del smoothing de cámara-Y ($10817E.l, ver
 * Camera_SmoothingIntegrate_0434F8 en scene_loader_cluster_043xxx.s):
 *
 *   $0434D0: 23FC 0010 0000 0010 817E   move.l #$100000, $10817E  ; rápido
 *   $0434DA: 4E75                       rts                       ; (16 px)
 *   $0434DE: 23FC 0004 0000 0010 817E   move.l #$40000,  $10817E  ; suave
 *   $0434E8: 4E75                       rts                       ; (4 px)
 *
 * El integrador consumirá esa velocidad con damping asimétrico, produciendo
 * el "golpe de cámara" hacia arriba (rápido) o el reencuadre suave. GCC -Os
 * deriva ambos byte-exacto desde el store volatile absoluto.
 */

__attribute__((section(".text.Camera_PresetVelFast_0434D0")))
void Camera_PresetVelFast_0434D0(void)
{
    *(volatile unsigned long *)0x10817E = 0x100000; /* 16 px de impulso */
}

__attribute__((section(".text.Camera_PresetVelSlow_0434DE")))
void Camera_PresetVelSlow_0434DE(void)
{
    *(volatile unsigned long *)0x10817E = 0x40000;  /* 4 px de impulso */
}
