| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching — DATOS
|  Wave VVVVV
|  Región: $1F8000..$200000  (32,768 B en 1 hueco(s), 4 entradas)
| ============================================================================
|
|  BORRADOR generado por tools/gen_data_region.py — volcado estructurado:
|  .fill para rachas de $00, .dc.l Simbolo para punteros a entradas
|  conocidas, .dc.w para el resto. Cada entrada empieza en una dirección
|  referenciada desde el código (o frontera de racha de ceros).
|
        .text

| ----------------------------------------------------------------------------
|  Data_1f8000  @ $1F8000  (2,244 B)
| ----------------------------------------------------------------------------
        .section .text.Data_1f8000, "ax", @progbits
        .global Data_1f8000
Data_1f8000:
        .dc.w   0xfff8,0x0200,0x0024,0x3658,0xffff,0x0020                   | +00000
        .dc.l   Data_180200                   | +0000c  -> $180200
        .dc.w   0x0024,0x3662,0xffff,0x0020,0xfff8,0x1d00,0x0416,0x0050     | +00010
        .dc.w   0x0204,0x0000,0x0000,0x0000,0x0020,0xfff8                   | +00020
        .dc.l   Data_180200                   | +0002c  -> $180200
        .dc.w   0x0024,0x363a,0xffff,0x0000,0x0000,0x0200,0x0024,0x3644     | +00030
        .dc.w   0xffff,0x0000                                               | +00040
        .dc.l   Data_180200                   | +00044  -> $180200
        .dc.w   0x0024,0x364e,0xffff                                        | +00048
        .dc.l   Data_00fff8                   | +0004e  -> $00FFF8
        .dc.w   0x0200,0x0024,0x3658,0xffff,0x0020                          | +00052
        .dc.l   Data_180200                   | +0005c  -> $180200
        .dc.w   0x0024,0x3662,0xffff,0x0020,0xfff8,0x1d00,0xffff,0xffff     | +00060
        .dc.w   0x0316,0x0014,0x0204,0x0000,0x0000,0x0008,0x0038,0xfff8     | +00070
        .dc.w   0x0028,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +00080
        .dc.w   0x0024,0x3644,0xffff,0x0008,0x0028,0x0200,0x0024,0x364e     | +00090
        .dc.w   0xffff,0x0008,0xfff8,0x0200,0x0024,0x3658,0xffff,0x0038     | +000a0
        .dc.w   0x0028,0x0200,0x0024,0x3662,0xffff,0x0038,0xfff8,0x1d00     | +000b0
        .dc.w   0x0416,0x0050,0x0204,0x0000,0x0000,0x0008,0x0038,0xfff8     | +000c0
        .dc.w   0x0028,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +000d0
        .dc.w   0x0024,0x3644,0xffff,0x0008,0x0028,0x0200,0x0024,0x364e     | +000e0
        .dc.w   0xffff,0x0008,0xfff8,0x0200,0x0024,0x3658,0xffff,0x0038     | +000f0
        .dc.w   0x0028,0x0200,0x0024,0x3662,0xffff,0x0038,0xfff8,0x1d00     | +00100
        .dc.w   0xffff,0xffff,0x0316,0x0014,0x0204,0x0000,0x0000,0x0018     | +00110
        .dc.w   0x0068,0xfff0,0x0038,0x0200,0x0024,0x363a,0xffff,0x0000     | +00120
        .dc.w   0x0000,0x0200,0x0024,0x3644,0xffff                          | +00130
        .dc.l   Data_180038                   | +0013a  -> $180038
        .dc.w   0x0200,0x0024,0x364e,0xffff                                 | +0013e
        .dc.l   Data_18fff0                   | +00146  -> $18FFF0
        .dc.w   0x0200,0x0024,0x3658,0xffff,0x0068,0x0038,0x0200,0x0024     | +0014a
        .dc.w   0x3662,0xffff,0x0068,0xfff0,0x1d00,0x0416,0x0050,0x0204     | +0015a
        .dc.w   0x0000,0x0000,0x0018,0x0068,0xfff0,0x0038,0x0200,0x0024     | +0016a
        .dc.w   0x363a,0xffff,0x0000,0x0000,0x0200,0x0024,0x3644,0xffff     | +0017a
        .dc.l   Data_180038                   | +0018a  -> $180038
        .dc.w   0x0200,0x0024,0x364e,0xffff                                 | +0018e
        .dc.l   Data_18fff0                   | +00196  -> $18FFF0
        .dc.w   0x0200,0x0024,0x3658,0xffff,0x0068,0x0038,0x0200,0x0024     | +0019a
        .dc.w   0x3662,0xffff,0x0068,0xfff0,0x1d00,0xffff,0xffff,0x0316     | +001aa
        .dc.w   0x0014,0x0204,0x0000                                        | +001ba
        .dc.l   Data_00fff0                   | +001c0  -> $00FFF0
        .dc.l   Data_100000                   | +001c4  -> $100000
        .dc.w   0x0020,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +001c8
        .dc.w   0x0024,0x3644,0xffff,0xfff0,0x0020,0x0200,0x0024,0x364e     | +001d8
        .dc.w   0xffff,0xfff0,0x0000,0x0200,0x0024,0x3658,0xffff            | +001e8
        .dc.l   Data_100020                   | +001f6  -> $100020
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +001fa
        .dc.l   Data_100000                   | +00202  -> $100000
        .dc.w   0x1d00,0x0416,0x0050,0x0204,0x0000                          | +00206
        .dc.l   Data_00fff0                   | +00210  -> $00FFF0
        .dc.l   Data_100000                   | +00214  -> $100000
        .dc.w   0x0020,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +00218
        .dc.w   0x0024,0x3644,0xffff,0xfff0,0x0020,0x0200,0x0024,0x364e     | +00228
        .dc.w   0xffff,0xfff0,0x0000,0x0200,0x0024,0x3658,0xffff            | +00238
        .dc.l   Data_100020                   | +00246  -> $100020
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +0024a
        .dc.l   Data_100000                   | +00252  -> $100000
        .dc.w   0x1d00,0xffff,0xffff,0x0316,0x0014,0x0204,0x0000            | +00256
        .dc.l   Data_00fff0                   | +00264  -> $00FFF0
        .dc.l   Data_100000                   | +00268  -> $100000
        .dc.w   0x0040,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +0026c
        .dc.w   0x0024,0x3644,0xffff,0xfff0,0x0040,0x0200,0x0024,0x364e     | +0027c
        .dc.w   0xffff,0xfff0,0x0000,0x0200,0x0024,0x3658,0xffff            | +0028c
        .dc.l   Str_100040                    | +0029a  -> $100040
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +0029e
        .dc.l   Data_100000                   | +002a6  -> $100000
        .dc.w   0x1d00,0x0416,0x0050,0x0204,0x0000                          | +002aa
        .dc.l   Data_00fff0                   | +002b4  -> $00FFF0
        .dc.l   Data_100000                   | +002b8  -> $100000
        .dc.w   0x0040,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +002bc
        .dc.w   0x0024,0x3644,0xffff,0xfff0,0x0040,0x0200,0x0024,0x364e     | +002cc
        .dc.w   0xffff,0xfff0,0x0000,0x0200,0x0024,0x3658,0xffff            | +002dc
        .dc.l   Str_100040                    | +002ea  -> $100040
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +002ee
        .dc.l   Data_100000                   | +002f6  -> $100000
        .dc.w   0x1d00,0xffff,0xffff,0x0316,0x0014,0x0204,0x0000            | +002fa
        .dc.l   Data_00fff0                   | +00308  -> $00FFF0
        .dc.l   Data_100000                   | +0030c  -> $100000
        .dc.w   0x0060,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +00310
        .dc.w   0x0024,0x3644,0xffff,0xfff0,0x0060,0x0200,0x0024,0x364e     | +00320
        .dc.w   0xffff,0xfff0,0x0000,0x0200,0x0024,0x3658,0xffff            | +00330
        .dc.l   Str_100060                    | +0033e  -> $100060
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +00342
        .dc.l   Data_100000                   | +0034a  -> $100000
        .dc.w   0x1d00,0x0416,0x0050,0x0204,0x0000                          | +0034e
        .dc.l   Data_00fff0                   | +00358  -> $00FFF0
        .dc.l   Data_100000                   | +0035c  -> $100000
        .dc.w   0x0060,0x0200,0x0024,0x363a,0xffff,0x0000,0x0000,0x0200     | +00360
        .dc.w   0x0024,0x3644,0xffff,0xfff0,0x0060,0x0200,0x0024,0x364e     | +00370
        .dc.w   0xffff,0xfff0,0x0000,0x0200,0x0024,0x3658,0xffff            | +00380
        .dc.l   Str_100060                    | +0038e  -> $100060
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +00392
        .dc.l   Data_100000                   | +0039a  -> $100000
        .dc.w   0x1d00,0xffff,0xffff,0x0700,0x0026,0x1e1a,0x0070            | +0039e
        .dc.l   Data_010208                   | +003ac  -> $010208
        .dc.w   0x0026,0x1cd4,0xffff,0x0700,0x0026,0x1e26,0x0070            | +003b0
        .dc.l   Data_010208                   | +003be  -> $010208
        .dc.w   0x0026,0x1ce0,0xffff,0x0700,0x0026,0x1e36,0x0070            | +003c2
        .dc.l   Data_010208                   | +003d0  -> $010208
        .dc.w   0x0026,0x1cf0,0xffff,0x0700,0x0026,0x1e46,0x0070            | +003d4
        .dc.l   Data_010208                   | +003e2  -> $010208
        .dc.w   0x0026,0x1d04,0xffff,0x0700,0x0026,0x1e5e,0x0070            | +003e6
        .dc.l   Data_010208                   | +003f4  -> $010208
        .dc.w   0x0026,0x1d20,0xffff,0x0700,0x0026,0x1e72,0x0070            | +003f8
        .dc.l   Data_010208                   | +00406  -> $010208
        .dc.w   0x0026,0x1d34,0xffff,0x0302,0x002f,0x83da,0x0a00,0xffff     | +0040a
        .dc.w   0xffff,0x0700,0x0026,0x1e86,0x0070                          | +0041a
        .dc.l   Data_010208                   | +00424  -> $010208
        .dc.w   0x0026,0x1d48,0xffff,0x0700,0x0026,0x1e9a,0x0070            | +00428
        .dc.l   Data_010208                   | +00436  -> $010208
        .dc.w   0x0026,0x1d5c,0xffff,0x0700,0x0026,0x1eaa,0x0070            | +0043a
        .dc.l   Data_010208                   | +00448  -> $010208
        .dc.w   0x0026,0x1d70,0xffff,0x0700,0x0026,0x1eba,0x0070            | +0044c
        .dc.l   Data_010208                   | +0045a  -> $010208
        .dc.w   0x0026,0x1d84,0xffff,0x0700,0x0026,0x1eca,0x0070            | +0045e
        .dc.l   Data_010208                   | +0046c  -> $010208
        .dc.w   0x0026,0x1d98,0xffff,0x0700,0x0026,0x1eda,0x0070            | +00470
        .dc.l   Data_010208                   | +0047e  -> $010208
        .dc.w   0x0026,0x1db8,0xffff,0x0700,0x0026,0x1eea,0x0070            | +00482
        .dc.l   Data_010208                   | +00490  -> $010208
        .dc.w   0x0026,0x1dd8,0xffff,0x0700,0x0026,0x1efa,0x0070            | +00494
        .dc.l   Data_010208                   | +004a2  -> $010208
        .dc.w   0x0026,0x1df8,0xffff,0x1600,0x0700,0x0026,0x2052,0x0070     | +004a6
        .dc.l   Data_010208                   | +004b6  -> $010208
        .dc.w   0x0026,0x1f0a,0xffff,0x0700,0x0026,0x205e,0x0070            | +004ba
        .dc.l   Data_010208                   | +004c8  -> $010208
        .dc.w   0x0026,0x1f16,0xffff,0x0700,0x0026,0x206e,0x0070            | +004cc
        .dc.l   Data_010208                   | +004da  -> $010208
        .dc.w   0x0026,0x1f26,0xffff,0x0700,0x0026,0x207e,0x0070            | +004de
        .dc.l   Data_010208                   | +004ec  -> $010208
        .dc.w   0x0026,0x1f3a,0xffff,0x0700,0x0026,0x2096,0x0070            | +004f0
        .dc.l   Data_010208                   | +004fe  -> $010208
        .dc.w   0x0026,0x1f56,0xffff,0x0700,0x0026,0x20ae,0x0070            | +00502
        .dc.l   Data_010208                   | +00510  -> $010208
        .dc.w   0x0026,0x1f6a,0xffff,0x0302,0x002f,0x84e4,0x0a00,0xffff     | +00514
        .dc.w   0xffff,0x0700,0x0026,0x20c2,0x0070                          | +00524
        .dc.l   Data_010208                   | +0052e  -> $010208
        .dc.w   0x0026,0x1f7e,0xffff,0x0700,0x0026,0x20d6,0x0070            | +00532
        .dc.l   Data_010208                   | +00540  -> $010208
        .dc.w   0x0026,0x1f92,0xffff,0x0700,0x0026,0x2750,0x0070            | +00544
        .dc.l   Data_010208                   | +00552  -> $010208
        .dc.w   0x0026,0x1fa6,0xffff,0x0700,0x0026,0x2760,0x0070            | +00556
        .dc.l   Data_010208                   | +00564  -> $010208
        .dc.w   0x0026,0x1fba,0xffff,0x0700,0x0026,0x2770,0x0070            | +00568
        .dc.l   Data_010208                   | +00576  -> $010208
        .dc.w   0x0026,0x1fce,0xffff,0x0700,0x0026,0x2780,0x0070            | +0057a
        .dc.l   Data_010208                   | +00588  -> $010208
        .dc.w   0x0026,0x1ff0,0xffff,0x0700,0x0026,0x2790,0x0070            | +0058c
        .dc.l   Data_010208                   | +0059a  -> $010208
        .dc.w   0x0026,0x2012,0xffff,0x0700,0x0026,0x27a0,0x0070            | +0059e
        .dc.l   Data_010208                   | +005ac  -> $010208
        .dc.w   0x0026,0x2034,0xffff,0x1600,0x0314                          | +005b0
        .dc.l   Data_0a0404                   | +005ba  -> $0A0404
        .dc.w   0x0000                                                      | +005be
        .dc.l   Data_00fff0                   | +005c0  -> $00FFF0
        .dc.l   Str_10fff0                    | +005c4  -> $10FFF0
        .dc.l   Data_100200                   | +005c8  -> $100200
        .dc.w   0x0024,0x363a,0xffff,0x0000,0x0000,0x0200,0x0024,0x3644     | +005cc
        .dc.w   0xffff,0xfff0                                               | +005dc
        .dc.l   Data_100200                   | +005e0  -> $100200
        .dc.w   0x0024,0x364e,0xffff,0xfff0,0xfff0,0x0200,0x0024,0x3658     | +005e4
        .dc.w   0xffff                                                      | +005f4
        .dc.l   Data_100010                   | +005f6  -> $100010
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +005fa
        .dc.l   Str_10fff0                    | +00602  -> $10FFF0
        .dc.w   0x1d00,0x0414                                               | +00606
        .dc.l   Data_0a0404                   | +0060a  -> $0A0404
        .dc.w   0x0000                                                      | +0060e
        .dc.l   Data_00fff0                   | +00610  -> $00FFF0
        .dc.l   Str_10fff0                    | +00614  -> $10FFF0
        .dc.l   Data_100200                   | +00618  -> $100200
        .dc.w   0x0024,0x363a,0xffff,0x0000,0x0000,0x0200,0x0024,0x3644     | +0061c
        .dc.w   0xffff,0xfff0                                               | +0062c
        .dc.l   Data_100200                   | +00630  -> $100200
        .dc.w   0x0024,0x364e,0xffff,0xfff0,0xfff0,0x0200,0x0024,0x3658     | +00634
        .dc.w   0xffff                                                      | +00644
        .dc.l   Data_100010                   | +00646  -> $100010
        .dc.w   0x0200,0x0024,0x3662,0xffff                                 | +0064a
        .dc.l   Str_10fff0                    | +00652  -> $10FFF0
        .dc.w   0x1d00,0xffff,0xffff,0x0a00,0x002f,0x877c                   | +00656
        .dc.l   Data_010208                   | +00662  -> $010208
        .dc.w   0x0025,0xc3de,0xffff,0x0a00,0xffff,0xffff                   | +00666
        .dc.l   Data_010208                   | +00672  -> $010208
        .dc.w   0x0025,0xc416,0xffff                                        | +00676
        .dc.l   Data_010208                   | +0067c  -> $010208
        .dc.w   0x0025,0xc44e,0xffff                                        | +00680
        .dc.l   Data_010208                   | +00686  -> $010208
        .dc.w   0x0025,0xc48c,0xffff                                        | +0068a
        .dc.l   Data_010208                   | +00690  -> $010208
        .dc.w   0x0025,0xc4ca,0xffff                                        | +00694
        .dc.l   Data_010208                   | +0069a  -> $010208
        .dc.w   0x0025,0xc506,0xffff                                        | +0069e
        .dc.l   Data_010208                   | +006a4  -> $010208
        .dc.w   0x0025,0xc540,0xffff                                        | +006a8
        .dc.l   Data_010208                   | +006ae  -> $010208
        .dc.w   0x0025,0xc576,0xffff                                        | +006b2
        .dc.l   Data_020208                   | +006b8  -> $020208
        .dc.w   0x0025,0xc5ae,0xffff                                        | +006bc
        .dc.l   Data_020208                   | +006c2  -> $020208
        .dc.w   0x0025,0xc5e4,0xffff                                        | +006c6
        .dc.l   Data_020208                   | +006cc  -> $020208
        .dc.w   0x0025,0xc610,0xffff                                        | +006d0
        .dc.l   Data_020208                   | +006d6  -> $020208
        .dc.w   0x0025,0xc636,0xffff                                        | +006da
        .dc.l   Data_020208                   | +006e0  -> $020208
        .dc.w   0x0025,0xc658,0xffff,0x1600,0x0a00,0x002f,0x8820            | +006e4
        .dc.l   Data_010208                   | +006f2  -> $010208
        .dc.w   0x0025,0xc66e,0xffff,0x0a00,0xffff,0xffff                   | +006f6
        .dc.l   Data_010208                   | +00702  -> $010208
        .dc.w   0x0025,0xc6a6,0xffff                                        | +00706
        .dc.l   Data_010208                   | +0070c  -> $010208
        .dc.w   0x0025,0xc6da,0xffff                                        | +00710
        .dc.l   Data_010208                   | +00716  -> $010208
        .dc.w   0x0025,0xc726,0xffff                                        | +0071a
        .dc.l   Data_010208                   | +00720  -> $010208
        .dc.w   0x0025,0xc776,0xffff                                        | +00724
        .dc.l   Data_010208                   | +0072a  -> $010208
        .dc.w   0x0025,0xc7c2,0xffff                                        | +0072e
        .dc.l   Data_010208                   | +00734  -> $010208
        .dc.w   0x0025,0xc80c,0xffff                                        | +00738
        .dc.l   Data_010208                   | +0073e  -> $010208
        .dc.w   0x0025,0xc852,0xffff                                        | +00742
        .dc.l   Data_020208                   | +00748  -> $020208
        .dc.w   0x0025,0xc896,0xffff                                        | +0074c
        .dc.l   Data_020208                   | +00752  -> $020208
        .dc.w   0x0025,0xc8d6,0xffff                                        | +00756
        .dc.l   Data_020208                   | +0075c  -> $020208
        .dc.w   0x0025,0xc914,0xffff                                        | +00760
        .dc.l   Data_020208                   | +00766  -> $020208
        .dc.w   0x0025,0xc94c,0xffff                                        | +0076a
        .dc.l   Data_020208                   | +00770  -> $020208
        .dc.w   0x0025,0xc978,0xffff,0x1600,0x0312,0x00c8,0x0404,0x0000     | +00774
        .dc.w   0x0000,0x0000,0x0058,0xffe8                                 | +00784
        .dc.l   Data_180200                   | +0078c  -> $180200
        .dc.w   0x0024,0x363a,0xffff,0x0000,0x0000,0x0200,0x0024,0x3644     | +00790
        .dc.w   0xffff,0x0000                                               | +007a0
        .dc.l   Data_180200                   | +007a4  -> $180200
        .dc.w   0x0024,0x364e,0xffff                                        | +007a8
        .dc.l   Data_00ffe8                   | +007ae  -> $00FFE8
        .dc.w   0x0200,0x0024,0x3658,0xffff,0x0058                          | +007b2
        .dc.l   Data_180200                   | +007bc  -> $180200
        .dc.w   0x0024,0x3662,0xffff,0x0058,0xffe8,0x1d00,0x0412,0x00c8     | +007c0
        .dc.w   0x0404,0x0000,0x0000,0x0000,0x0058,0xffe8                   | +007d0
        .dc.l   Data_180200                   | +007dc  -> $180200
        .dc.w   0x0024,0x363a,0xffff,0x0000,0x0000,0x0200,0x0024,0x3644     | +007e0
        .dc.w   0xffff,0x0000                                               | +007f0
        .dc.l   Data_180200                   | +007f4  -> $180200
        .dc.w   0x0024,0x364e,0xffff                                        | +007f8
        .dc.l   Data_00ffe8                   | +007fe  -> $00FFE8
        .dc.w   0x0200,0x0024,0x3658,0xffff,0x0058                          | +00802
        .dc.l   Data_180200                   | +0080c  -> $180200
        .dc.w   0x0024,0x3662,0xffff,0x0058,0xffe8,0x1d00,0xffff,0xffff     | +00810
        .dc.w   0x0313,0x00c8,0x0404,0x0000                                 | +00820
        .dc.l   Data_00ffe8                   | +00828  -> $00FFE8
        .dc.w   0x0018,0xffd8,0x0058,0x0200,0x0024,0x363a,0xffff,0x0000     | +0082c
        .dc.w   0x0000,0x0200,0x0024,0x3644,0xffff,0xffe8,0x0058,0x0200     | +0083c
        .dc.w   0x0024,0x364e,0xffff,0xffe8,0xffd8,0x0200,0x0024,0x3658     | +0084c
        .dc.w   0xffff                                                      | +0085c
        .dc.l   Data_180058                   | +0085e  -> $180058
        .dc.w   0x0200,0x0024,0x3662,0xffff,0x0018,0xffd8,0x1d00,0x0413     | +00862
        .dc.w   0x00c8,0x0404,0x0000                                        | +00872
        .dc.l   Data_00ffe8                   | +00878  -> $00FFE8
        .dc.w   0x0018,0xffd8,0x0058,0x0200,0x0024,0x363a,0xffff,0x0000     | +0087c
        .dc.w   0x0000,0x0200,0x0024,0x3644,0xffff,0xffe8,0x0058,0x0200     | +0088c
        .dc.w   0x0024,0x364e,0xffff,0xffe8,0xffd8,0x0200,0x0024,0x3658     | +0089c
        .dc.w   0xffff                                                      | +008ac
        .dc.l   Data_180058                   | +008ae  -> $180058
        .dc.w   0x0200,0x0024,0x3662,0xffff,0x0018,0xffd8,0x1d00,0xffff     | +008b2
        .dc.w   0xffff                                                      | +008c2

| ----------------------------------------------------------------------------
|  Zero_1f88c4  @ $1F88C4  (28,476 B)
| ----------------------------------------------------------------------------
        .section .text.Zero_1f88c4, "ax", @progbits
        .global Zero_1f88c4
Zero_1f88c4:
        .fill   28476,1,0x00                      | +00000

| ----------------------------------------------------------------------------
|  Data_1ff800  @ $1FF800  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Data_1ff800, "ax", @progbits
        .global Data_1ff800
Data_1ff800:
        .dc.w   0x1234,0x5678                                               | +00000

| ----------------------------------------------------------------------------
|  Zero_1ff804  @ $1FF804  (2,044 B)
| ----------------------------------------------------------------------------
        .section .text.Zero_1ff804, "ax", @progbits
        .global Zero_1ff804
Zero_1ff804:
        .fill   2044,1,0x00                      | +00000
