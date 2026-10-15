// sndrender.cpp - render Metal Slug sound commands to WAV by running the real
// sound driver (201-m1.bin, Z80 @ 4 MHz) against an emulated YM2610 (ymfm,
// 8 MHz) fed with the V-ROM (201-v1.bin + 201-v2.bin).
//
// Nothing is re-interpreted: the Z80 executes the original driver code, the
// 68000 side is reduced to what it does on the hardware (write a byte to the
// sound latch, which raises an NMI, one byte per video frame), and the YM2610
// timer A drives the driver's IRQ exactly as on the board.  The output is the
// chip's native rate (8 MHz / 144 = 55555.6 Hz), stereo 16 bit, mixed like the
// Neo Geo board (FM+ADPCM 0.98, SSG 0.28, as MAME does).
//
//   sndrender M1 V1 V2 OUT.wav SECONDS [options] BYTE...
//     --auto-stop        end after 1 s of digital silence (once commands are sent)
//     --warmup S         driver boot time before the first command (0.25 s)
//     --fade-at T        start a linear fade-out at T seconds ...
//     --fade-len F       ... lasting F seconds (default 8), then stop
//     --vgm PATH         also log every YM2610 write as a VGM 1.51 file, with
//                        the V-ROM pages actually read as ADPCM-A/B data blocks
//     --title TEXT       track title for the VGM GD3 tag
//     --loop-tracks K    music: K of its tracks loop forever.  The driver's
//                        infinite-loop jumps (backward C2, D0 with count 0)
//                        are hooked; pass 1 ends when K tracks have looped,
//                        pass n at the n-th loop of the last one to loop
//     --loops N          ... play N passes, then fade out (--fade-len)
//     --max S            hard cap in seconds for --auto-stop / --loops
//
// BYTE... are the command bytes in hex (e.g. "00 21" = music 21h,
// "10 35" = SFX 35h of table A, "11 2A" = table B).  They are written to the
// latch one per frame (59.185 Hz) after the warm-up (default 0.25 s).  With
// --auto-stop the render ends after 1 s of digital silence (SFX), never
// before the commands have been sent.

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdint>
#include <vector>
#include <string>
#include <cmath>
#include <unistd.h>

#include "ymfm_opn.h"
extern "C" {
#include "z80.h"
}

static const double Z80_CLOCK = 4000000.0;
static const uint32_t YM_CLOCK = 8000000;
static const double FRAME_HZ = 59.185606;   // Neo Geo MVS vertical refresh

static std::vector<uint8_t> load(const char *p) {
    FILE *f = fopen(p, "rb");
    if (!f) { fprintf(stderr, "sndrender: cannot open %s\n", p); exit(1); }
    std::vector<uint8_t> v;
    uint8_t buf[65536]; size_t n;
    while ((n = fread(buf, 1, sizeof buf, f)) > 0) v.insert(v.end(), buf, buf + n);
    fclose(f);
    return v;
}

struct Board : public ymfm::ymfm_interface {
    std::vector<uint8_t> m1, vrom;
    uint8_t ram[0x800];
    uint32_t bank[4];          // file offsets of windows 8000/C000/E000/F000
    uint8_t latch = 0;
    bool nmi_enabled = false;
    bool irq = false;
    int64_t timer_left[2] = {-1, -1};   // in YM clocks
    uint64_t ym_clock = 0;              // current YM time
    uint64_t busy_end = 0;
    ymfm::ym2610 *ym = nullptr;
    z80 cpu;
    // VGM logging
    bool log_vgm = false;
    uint64_t now44 = 0;                 // current time in 44.1 kHz VGM samples
    uint64_t last44 = 0;
    uint8_t addr_latch[2] = {0, 0};
    std::vector<uint8_t> vgm;           // command stream
    std::vector<uint8_t> used_a, used_b;   // V-ROM 256-byte pages read

    void vgm_wait() {
        while (now44 > last44) {
            uint64_t d = now44 - last44; if (d > 65535) d = 65535;
            vgm.push_back(0x61); vgm.push_back(d & 0xFF); vgm.push_back(d >> 8);
            last44 += d;
        }
    }
    void ym_write(uint8_t off, uint8_t v) {
        ym->write(off, v);
        if (!log_vgm) return;
        int hi = off >> 1;
        if (!(off & 1)) { addr_latch[hi] = v; return; }
        vgm_wait();
        vgm.push_back(hi ? 0x59 : 0x58); vgm.push_back(addr_latch[hi]); vgm.push_back(v);
    }

    // ---- ymfm_interface ----
    void ymfm_set_timer(uint32_t tnum, int32_t duration) override { timer_left[tnum] = duration; }
    void ymfm_set_busy_end(uint32_t clocks) override { busy_end = ym_clock + clocks; }
    bool ymfm_is_busy() override { return ym_clock < busy_end; }
    void ymfm_update_irq(bool asserted) override { irq = asserted; }
    uint8_t ymfm_external_read(ymfm::access_class type, uint32_t address) override {
        if (type == ymfm::ACCESS_ADPCM_A || type == ymfm::ACCESS_ADPCM_B) {
            if (address >= vrom.size()) return 0;
            if (log_vgm) (type == ymfm::ACCESS_ADPCM_A ? used_a : used_b)[address >> 8] = 1;
            return vrom[address];
        }
        return 0;
    }
    void timer_expired(uint32_t t) { m_engine->engine_timer_expired(t); }

    // ---- Z80 memory / I/O ----
    uint8_t rd(uint16_t a) {
        if (a < 0x8000) return m1[a];
        if (a >= 0xF800) return ram[a - 0xF800];
        uint32_t off;
        if (a < 0xC000) off = bank[0] + (a - 0x8000);
        else if (a < 0xE000) off = bank[1] + (a - 0xC000);
        else if (a < 0xF000) off = bank[2] + (a - 0xE000);
        else off = bank[3] + (a - 0xF000);
        return m1[off % m1.size()];
    }
    void wr(uint16_t a, uint8_t v) { if (a >= 0xF800) ram[a - 0xF800] = v; }
    uint8_t in(uint8_t port, uint8_t hi) {
        switch (port) {
        case 0x00: return latch;
        case 0x04: case 0x05: case 0x06: case 0x07: return ym->read(port & 3);
        case 0x08: bank[3] = uint32_t(hi) * 0x0800; return 0;   // F000-F7FF, 2 KiB
        case 0x09: bank[2] = uint32_t(hi) * 0x1000; return 0;   // E000-EFFF, 4 KiB
        case 0x0A: bank[1] = uint32_t(hi) * 0x2000; return 0;   // C000-DFFF, 8 KiB
        case 0x0B: bank[0] = uint32_t(hi) * 0x4000; return 0;   // 8000-BFFF, 16 KiB
        }
        return 0xFF;
    }
    void out(uint8_t port, uint8_t v) {
        switch (port) {
        case 0x04: case 0x05: case 0x06: case 0x07: ym_write(port & 3, v); break;
        case 0x08: nmi_enabled = true; break;
        case 0x18: nmi_enabled = false; break;
        default: break;   // 0x0C = reply to the 68000 (ignored)
        }
    }
};

static Board *B;
static uint8_t cb_rd(void *, uint16_t a) { return B->rd(a); }
static void cb_wr(void *, uint16_t a, uint8_t v) { B->wr(a, v); }
// z80.c passes only the low port byte; the high byte is B for "in r,(c)"
// (the only form the driver uses for the bank ports 08..0B).
static uint8_t cb_in(z80 *z, uint8_t p) { return B->in(p, z->b); }
static void cb_out(z80 *z, uint8_t p, uint8_t v) { (void)z; B->out(p, v); }

static void put16(FILE *f, uint16_t v) { fputc(v & 0xFF, f); fputc(v >> 8, f); }
static void put32(FILE *f, uint32_t v) { put16(f, v & 0xFFFF); put16(f, v >> 16); }

static void le32(std::vector<uint8_t> &v, size_t at, uint32_t x) {
    for (int i = 0; i < 4; i++) v[at + i] = uint8_t(x >> (8 * i));
}
static void app32(std::vector<uint8_t> &v, uint32_t x) { for (int i = 0; i < 4; i++) v.push_back(uint8_t(x >> (8 * i))); }
static void app_u16s(std::vector<uint8_t> &v, const std::string &s) {
    // ASCII/UTF-8 subset -> UTF-16LE, null-terminated
    for (unsigned char c : s) { v.push_back(c); v.push_back(0); }
    v.push_back(0); v.push_back(0);
}

// VGM 1.51: header 0x80, ADPCM data blocks (0x82 = YM2610 ADPCM-A ROM,
// 0x83 = YM2610 DELTA-T ROM) with only the pages the chip read, then the
// command stream, then a GD3 tag.
static void write_vgm(Board &b, const char *path, const std::string &title, uint64_t total44) {
    b.now44 = total44; b.vgm_wait();
    b.vgm.push_back(0x66);
    std::vector<uint8_t> out(0x80, 0);
    memcpy(&out[0], "Vgm ", 4);
    le32(out, 0x08, 0x151);
    le32(out, 0x18, uint32_t(total44));
    le32(out, 0x24, 60);
    le32(out, 0x34, 0x80 - 0x34);
    le32(out, 0x4C, YM_CLOCK);            // bit 31 clear = YM2610 (not B)
    for (int k = 0; k < 2; k++) {
        std::vector<uint8_t> &used = k ? b.used_b : b.used_a;
        size_t n = used.size(), i = 0;
        while (i < n) {
            if (!used[i]) { i++; continue; }
            size_t j = i;
            while (j < n && used[j]) j++;
            uint32_t start = uint32_t(i << 8), len = uint32_t((j - i) << 8);
            out.push_back(0x67); out.push_back(0x66); out.push_back(k ? 0x83 : 0x82);
            app32(out, len + 8); app32(out, uint32_t(b.vrom.size())); app32(out, start);
            out.insert(out.end(), b.vrom.begin() + start, b.vrom.begin() + start + len);
            i = j;
        }
    }
    out.insert(out.end(), b.vgm.begin(), b.vgm.end());
    // GD3
    size_t gd3 = out.size();
    std::vector<uint8_t> g;
    app_u16s(g, title); app_u16s(g, "");
    app_u16s(g, "Metal Slug"); app_u16s(g, "");
    app_u16s(g, "SNK Neo Geo"); app_u16s(g, "");
    app_u16s(g, "Nazca / SNK"); app_u16s(g, "");
    app_u16s(g, "1996"); app_u16s(g, "mslug-decompiled sndrender");
    app_u16s(g, "Logged from the original sound driver (201-m1) on ymfm");
    memcpy(&out[0] + 0, "Vgm ", 4);
    out.insert(out.end(), {'G', 'd', '3', ' '});
    app32(out, 0x100); app32(out, uint32_t(g.size()));
    out.insert(out.end(), g.begin(), g.end());
    le32(out, 0x14, uint32_t(gd3 - 0x14));
    le32(out, 0x04, uint32_t(out.size() - 4));
    FILE *f = fopen(path, "wb");
    if (!f) { fprintf(stderr, "sndrender: cannot write %s\n", path); exit(1); }
    fwrite(out.data(), 1, out.size(), f);
    fclose(f);
}

int main(int argc, char **argv) {
    if (argc < 6) {
        fprintf(stderr, "usage: sndrender M1 V1 V2 OUT.wav SECONDS [--auto-stop] [--warmup S] BYTE...\n");
        return 2;
    }
    Board board; B = &board;
    board.m1 = load(argv[1]);
    board.vrom = load(argv[2]);
    std::vector<uint8_t> v2 = load(argv[3]);
    board.vrom.insert(board.vrom.end(), v2.begin(), v2.end());
    const char *outp = argv[4];
    double seconds = atof(argv[5]);
    bool auto_stop = false;
    double warmup = 0.25, fade_at = -1, fade_len = 8, max_s = 0;
    int loop_tracks = 0, loops = 1;
    bool trace_tempo = false;        // debug: log FDE7/FDEB (tempo base/offset) changes
    const char *vgm_path = nullptr;
    std::string title = "";
    std::vector<uint8_t> cmds;
    for (int i = 6; i < argc; i++) {
        if (!strcmp(argv[i], "--auto-stop")) auto_stop = true;
        else if (!strcmp(argv[i], "--warmup") && i + 1 < argc) warmup = atof(argv[++i]);
        else if (!strcmp(argv[i], "--fade-at") && i + 1 < argc) fade_at = atof(argv[++i]);
        else if (!strcmp(argv[i], "--fade-len") && i + 1 < argc) fade_len = atof(argv[++i]);
        else if (!strcmp(argv[i], "--vgm") && i + 1 < argc) vgm_path = argv[++i];
        else if (!strcmp(argv[i], "--title") && i + 1 < argc) title = argv[++i];
        else if (!strcmp(argv[i], "--loop-tracks") && i + 1 < argc) loop_tracks = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--loops") && i + 1 < argc) loops = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--max") && i + 1 < argc) max_s = atof(argv[++i]);
        else if (!strcmp(argv[i], "--trace-tempo")) trace_tempo = true;
        else cmds.push_back(uint8_t(strtoul(argv[i], nullptr, 16)));
    }
    memset(board.ram, 0, sizeof board.ram);
    if (vgm_path) {
        board.log_vgm = true;
        board.used_a.assign(board.vrom.size() >> 8, 0);
        board.used_b.assign(board.vrom.size() >> 8, 0);
    }
    for (int i = 0; i < 4; i++) board.bank[i] = 0;

    ymfm::ym2610 ym(board);
    ym.set_fidelity(ymfm::OPN_FIDELITY_MIN);     // one output per FM sample: clock / 144
    ym.reset();
    board.ym = &ym;
    const uint32_t rate_num = YM_CLOCK;          // exact rate = 8e6/144
    const uint32_t clocks_per_sample = 144;
    const double z80_per_sample = Z80_CLOCK * clocks_per_sample / YM_CLOCK;   // 72

    z80_init(&board.cpu);
    board.cpu.read_byte = cb_rd;
    board.cpu.write_byte = cb_wr;
    board.cpu.port_in = cb_in;
    board.cpu.port_out = cb_out;

    FILE *f = fopen(outp, "wb");
    if (!f) { fprintf(stderr, "sndrender: cannot write %s\n", outp); return 1; }
    const uint32_t sr = (rate_num + clocks_per_sample / 2) / clocks_per_sample;   // 55556
    fwrite("RIFF\0\0\0\0WAVEfmt ", 1, 16, f);
    put32(f, 16); put16(f, 1); put16(f, 2); put32(f, sr); put32(f, sr * 4); put16(f, 4); put16(f, 16);
    fwrite("data\0\0\0\0", 1, 8, f);

    const uint64_t warm = uint64_t(warmup * YM_CLOCK / clocks_per_sample);
    uint64_t total = warm + uint64_t(seconds * YM_CLOCK / clocks_per_sample);
    const double sr_exact = double(YM_CLOCK) / clocks_per_sample;
    uint64_t fade_s = UINT64_MAX, fade_n = 1;
    if (fade_at >= 0) {
        fade_s = warm + uint64_t(fade_at * sr_exact);
        fade_n = uint64_t(fade_len * sr_exact); if (!fade_n) fade_n = 1;
        if (fade_s + fade_n < total) total = fade_s + fade_n;
    }
    if (loop_tracks > 0) fade_n = uint64_t(fade_len * sr_exact);
    if (max_s > 0) total = warm + uint64_t(max_s * sr_exact);
    // music loop tracking: 14 music track structs at FBFC, 0x23 bytes each
    int loop_count[14] = {0};
    int looped = 0, last_track = -1;
    const double samples_per_frame = YM_CLOCK / (clocks_per_sample * FRAME_HZ);
    const uint64_t cmds_done = warm + uint64_t((cmds.size() + 1) * samples_per_frame);
    size_t next_cmd = 0;
    double next_cmd_at = double(warm);
    uint64_t silent = 0, written = 0, irq_count = 0, nmi_count = 0;
    double z80_target = 0;
    ymfm::ym2610::output_data o;

    uint64_t ran = 0;
    for (uint64_t s = 0; s < total; s++) {
        ran = s + 1;
        board.now44 = s < warm ? 0 : uint64_t(double(s - warm) * 44100.0 / sr_exact);
        // 68000 side: one latch write per frame, each raising an NMI
        if (next_cmd < cmds.size() && double(s) >= next_cmd_at) {
            board.latch = cmds[next_cmd++];
            if (board.nmi_enabled) z80_gen_nmi(&board.cpu);
            next_cmd_at += samples_per_frame;
        }
        // run the Z80 for this sample's worth of cycles
        z80_target += z80_per_sample;
        while (double(board.cpu.cyc) < z80_target) {
            board.ym_clock = uint64_t(board.cpu.cyc) * 2;
            if (board.irq) z80_gen_int(&board.cpu, 0xFF);
            else board.cpu.int_pending = 0;
            uint16_t pc = board.cpu.pc;
            bool inf = (pc == 0x56D3 && (board.cpu.d & 0x80)) ||        // C2 jump, negative offset
                       (pc == 0x5626 && board.cpu.a == 0);                // D0 with loop count 0
            if (inf && loop_tracks > 0) {
                int t = (int(board.cpu.iy) - 0xFBFC) / 0x23;
                if (board.cpu.iy >= 0xFBFC && t < 14 && (board.cpu.iy - 0xFBFC) % 0x23 == 0) {
                    if (loop_count[t]++ == 0 && ++looped == loop_tracks) last_track = t;
                    if (t == last_track && loop_count[t] == loops && fade_s == UINT64_MAX) {
                        fade_s = s;                       // pass N complete: fade out from here
                        total = s + fade_n;
                        fprintf(stderr, "sndrender: loop point (pass %d) at %.2f s\n", loops,
                                double(s - warm) / sr_exact);
                    }
                }
            }
            z80_step(&board.cpu);
            if (trace_tempo) {
                static int lt = -1000;
                int cur = (board.ram[0xFDE7 - 0xF800] << 8) | board.ram[0xFDEB - 0xF800];
                if (cur != lt) {
                    fprintf(stderr, "tempo %.3f s base=%d off=%d\n", double(s > warm ? s - warm : 0) / sr_exact,
                            int8_t(board.ram[0xFDE7 - 0xF800]), int8_t(board.ram[0xFDEB - 0xF800]));
                    lt = cur;
                }
            }
            if (board.cpu.pc == 0x0038) irq_count++;
            if (board.cpu.pc == 0x0066) nmi_count++;
        }
        // YM timers (in YM clocks)
        board.ym_clock = s * clocks_per_sample;
        for (int t = 0; t < 2; t++) {
            if (board.timer_left[t] >= 0) {
                board.timer_left[t] -= clocks_per_sample;
                if (board.timer_left[t] <= 0) {
                    board.timer_left[t] = -1;
                    board.timer_expired(t);
                }
            }
        }
        ym.generate(&o, 1);
        if (s < warm) continue;             // driver boot: not part of the sound
        // Neo Geo mix (MAME neogeo.cpp): FM/ADPCM L,R at 0.98, SSG to both at 0.28
        double g = 1.0;
        if (s >= fade_s) g = 1.0 - double(s - fade_s) / double(fade_n);
        double l = g * (0.98 * o.data[0] + 0.28 * o.data[2]);
        double r = g * (0.98 * o.data[1] + 0.28 * o.data[2]);
        int il = int(lrint(l)), ir = int(lrint(r));
        if (il > 32767) il = 32767; if (il < -32768) il = -32768;
        if (ir > 32767) ir = 32767; if (ir < -32768) ir = -32768;
        put16(f, uint16_t(int16_t(il))); put16(f, uint16_t(int16_t(ir)));
        written++;
        if (auto_stop && s > cmds_done) {
            if (il == 0 && ir == 0) { if (++silent > sr) break; }
            else silent = 0;
        }
    }
    if (auto_stop && silent > 0) {
        // trim the trailing silence to 50 ms
        uint64_t keep = written - silent + sr / 20;
        if (keep < written) written = keep;
        fflush(f);
        if (ftruncate(fileno(f), 44 + written * 4) != 0) { /* ignore */ }
    }
    fseek(f, 4, SEEK_SET); put32(f, uint32_t(36 + written * 4));
    fseek(f, 40, SEEK_SET); put32(f, uint32_t(written * 4));
    fclose(f);
    if (vgm_path) write_vgm(board, vgm_path, title, uint64_t(double(written) * 44100.0 / sr_exact));
    fprintf(stderr, "sndrender: %s  %.2f s @ %u Hz  (%.1f driver ticks/s, %llu NMI)\n", outp, double(written) / sr, sr,
            irq_count * double(YM_CLOCK) / (double(ran) * clocks_per_sample), (unsigned long long)nmi_count);
    return 0;
}
