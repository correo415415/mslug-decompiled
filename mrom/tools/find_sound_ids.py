#!/usr/bin/env python3
"""
find_sound_ids.py - which sound ids does the 68k program (P-ROM) send, and from where?

The 68k queues sound commands through Sound_PushCommandFiltered_0021a6 /
ClampD0ToRange ($219C), InputGuardCall219c ($2352) and Sound_Push0C_ThenClamp
($2222).  The 16-bit id in d0 is `type << 8 | cmd` (cmd >= 20h): type 00 =
music (MusicTable[cmd-20h]), 10h = SfxTableA, 11h = SfxTableB (see
mrom/docs/FORMATS.md §2).  This tool scans build/mslug_prom.bin for

  * calls preceded by `moveq #n,d0` / `move.w #n,d0`, and
  * a list of id tables indexed at run time (mission number, weapon, ...),

maps every call site to its function through tools/registry.py and writes
mrom/map/sound_ids.txt:

    <id hex>  <Name>  ; <users>

Music names are assigned here from the evidence (MUSIC_NAMES, hypotheses in
mrom/docs/FORMATS.md §7); SFX keep a numeric name plus their users.
mrom/tools/gen_sound_data.py reads the file to label headers, so the sound
sources do not depend on the P-ROM at build time.

    python3 mrom/tools/find_sound_ids.py
"""
import bisect, os, re, struct

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "build", "mslug_prom.bin")
OUT = os.path.join(ROOT, "mrom", "map", "sound_ids.txt")

TARGETS = {0x21A6: "filtered", 0x219C: "push", 0x2352: "guarded", 0x2222: "push0C"}

# id tables read with `move.w (aN,dN.w),d0` before one of the calls
# (cpu address, number of words, meaning)
TABLES = [
    (0x28DF00, 8, "MissionMusicTable_28DF00 [$106ECF mission]"),
    (0x28DF10, 8, "MissionStartVoiceTable_28DF10 [$106ECF mission]"),
    (0x032D28, 5, "Player_WeaponMusicTable_032d28 [weapon]"),
    (0x02B8CE, 5, "Slug_AccelRightMusicTbl_02b8ce"),
    (0x02BA34, 5, "Slug_AccelLeftMusicTbl_02ba34"),
    (0x02BB9A, 5, "Slug_DestroyedMusicTbl_02bb9a"),
    (0x0509D6, 4, "Allen_TauntVoiceTbl_0509d6"),
    (0x05E766, 5, "HitSoundTable_05e766"),
    (0x2D4A22, 6, "Table_2D4A22 (JsrAbsThunk_06ef06)"),
]

# MusicTable index -> name (evidence: P-ROM users, see FORMATS.md §7)
MUSIC_NAMES = {
    0x20: "Music_MissionComplete",   # BannerComplete_Boot / MissionComplete_Task
    0x21: "Music_Mission1",          # MissionMusicTable_28DF00[0] (also missions 7, 8)
    0x22: "Music_Mission2",
    0x23: "Music_Mission3",          # also Airship_Depart, M2Boss_Turret_DeathFall
    0x24: "Music_Mission4",
    0x25: "Music_Mission5",
    0x26: "Music_Mission6",
    0x27: "Music_AttractDemo",       # Attract_InitShow27_TaskAdd
    0x28: "Music_Boss",              # M2Boss intro, M5Tank, FinalBoss, scripted props
    0x29: "Music_SceneB",            # SceneB_Init (mission $0B sequence)
    0x2A: "Music_SceneC",            # SceneC_Init (also M2Boss body per its notes)
    0x2B: "Music_Ending",            # Ending_SpawnOrbitFx, Anim_State_F1
    0x2C: "Music_Title",             # LogoScene_Tpl (scene $0D)
}

# SFX with a clear meaning from their users (others keep SfxA_NNN / SfxB_NNN)
SFX_NAMES = {
    # Table A = effects.  Names from the 68k functions that send them
    # (hypotheses: not yet confirmed by listening; the number is kept).
    0x1020: "SfxA_000_CamperDie",
    0x1021: "SfxA_001_DeathExplosion",
    0x1022: "SfxA_002_EnemyDeathCry",
    0x1023: "SfxA_003_StructureDestroyed",
    0x1024: "SfxA_004_LargePropBreak",
    0x1025: "SfxA_005_RocketHit",
    0x1027: "SfxA_007_GrenadeExplosion",
    0x1028: "SfxA_008_StructureCollapse",
    0x1029: "SfxA_009_SignBreak",
    0x102B: "SfxA_011_ArmorDamaged",
    0x102D: "SfxA_013_TurretDestroyed",
    0x102E: "SfxA_014_SmallPropBreak",
    0x102F: "SfxA_015_PropDamaged",
    0x1030: "SfxA_016_PropWreck",
    0x1032: "SfxA_018_BigExplosion",
    0x1034: "SfxA_020_SlugExplode",
    0x1039: "SfxA_025_CrateBurst",
    0x103E: "SfxA_030_BarrelPop",
    0x103F: "SfxA_031_Touchdown",
    0x1041: "SfxA_033_FinalBossHeadDie",
    0x1042: "SfxA_034_BurningScream",
    0x1043: "SfxA_035_ParatrooperDie",
    0x1052: "SfxA_050_WaterSplash",
    0x1053: "SfxA_051_PlayerDeath",
    0x105C: "SfxA_060_HostageThrown",
    0x105D: "SfxA_061_HostageAppear",
    0x105E: "SfxA_062_HostageRun",
    0x1060: "SfxA_064_HostagePose",
    0x1061: "SfxA_065_CrabLegDetach",
    0x1064: "SfxA_068_ShellToss",
    0x106A: "SfxA_074_BossExplode",
    0x1075: "SfxA_085_M5BossAttack",
    0x1076: "SfxA_086_ExplosiveProp",
    0x1083: "SfxA_099_SlugHPUp",
    0x1084: "SfxA_100_VehicleLaunch",
    0x1086: "SfxA_102_FireBurst",
    0x1089: "SfxA_105_SlugDamage",
    0x108C: "SfxA_108_EnemyShot",
    0x108D: "SfxA_109_TurretShell",
    0x108E: "SfxA_110_DoorBreak",
    0x1098: "SfxA_120_TrapFlame",
    0x109E: "SfxA_126_TankEngine",
    0x10A5: "SfxA_133_Mortar",
    0x10A8: "SfxA_136_HatchOpen",
    0x10AE: "SfxA_142_HutDoor",
    0x10AF: "SfxA_143_SlugAccelRight",
    0x10B0: "SfxA_144_SlugAccelLeft",
    0x10B2: "SfxA_146_SlugDestroyedSlide",
    0x10B8: "SfxA_152_GunshipAttack",
    0x10BB: "SfxA_155_WallBreak",
    0x10D4: "SfxA_180_ContinueCountdown",
    0x10D6: "SfxA_182_ResultsPrize",
    0x10D8: "SfxA_184_ResultsTally",
    0x10DE: "SfxA_190_GameOverFinal",
    0x10DF: "SfxA_191_GameOverBanner",
    0x10E0: "SfxA_192_PauseSlide",
    0x10E1: "SfxA_193_Pause",
    0x10E8: "SfxA_200_AimTurret",
    0x10E9: "SfxA_201_SlugDestroyed",
    0x10EA: "SfxA_202_FragLaunch",
    0x10F2: "SfxA_210_VehicleCrash",
    0x10F3: "SfxA_211_ShotgunFire",
    0x10F4: "SfxA_212_FlameFire",
    0x10F5: "SfxA_213_RocketFire",
    0x10F6: "SfxA_214_HMGFire",
    0x10F7: "SfxA_215_PistolFire",
    0x10FD: "SfxA_221_AmmoPickup",
    0x10FF: "SfxA_223_ShieldBreak",
    # Table A, remaining 82: named from an automated listening pass over the
    # export_wav.py renders (audio model description, cross-checked with the
    # 68k users where there are any). Hypotheses like the ones above.
    0x1026: "SfxA_006_HugeExplosion",
    0x102A: "SfxA_010_MachineGunBurst",
    0x102C: "SfxA_012_RapidBurst",
    0x1033: "SfxA_019_ExplosionDebris",
    0x1035: "SfxA_021_SmallExplosion",
    0x1036: "SfxA_022_MediumExplosionA",
    0x1037: "SfxA_023_MediumExplosionB",
    0x1038: "SfxA_024_MetalImpact",
    0x1040: "SfxA_032_SoldierScreamRise",
    0x1044: "SfxA_036_SoldierScreamFall",
    0x1048: "SfxA_040_MetalClang",
    0x104A: "SfxA_042_SoldierScream",
    0x104B: "SfxA_043_SquishA",
    0x104C: "SfxA_044_SquishB",
    0x104D: "SfxA_045_FireCrackleA",
    0x104E: "SfxA_046_FireCrackleB",
    0x104F: "SfxA_047_SoldierScreamLong",
    0x1051: "SfxA_049_SmallSplash",
    0x1054: "SfxA_052_MetalCrumble",
    0x1055: "SfxA_053_SoldierGruntA",
    0x1056: "SfxA_054_SoldierGruntB",
    0x1062: "SfxA_066_ExplosionPop",
    0x1065: "SfxA_069_MetalExplosion",
    0x1066: "SfxA_070_CannonBoom",
    0x1067: "SfxA_071_DebrisExplosion",
    0x1069: "SfxA_073_MutedExplosion",
    0x1071: "SfxA_081_EngineRumble",
    0x1073: "SfxA_083_TankTreads",
    0x1074: "SfxA_084_ServoWhine",
    0x1077: "SfxA_087_MetalClangB",
    0x107F: "SfxA_095_AirshipAlarm",
    0x1080: "SfxA_096_WaterChurn",
    0x1081: "SfxA_097_BigSplash",
    0x1085: "SfxA_101_QuickSplash",
    0x1087: "SfxA_103_MuffledPop",
    0x108A: "SfxA_106_GunshotEcho",
    0x108B: "SfxA_107_MetalLatch",
    0x108F: "SfxA_111_MetalStrike",
    0x1090: "SfxA_112_SlugShotA",
    0x1091: "SfxA_113_SlugShotB",
    0x1092: "SfxA_114_SlugThud",
    0x1099: "SfxA_121_Bubbling",
    0x109B: "SfxA_123_RocketWhoosh",
    0x109C: "SfxA_124_EngineAccel",
    0x109D: "SfxA_125_HeavyRoll",
    0x109F: "SfxA_127_GlassExplosion",
    0x10A0: "SfxA_128_Click",
    0x10A1: "SfxA_129_MotorRev",
    0x10A2: "SfxA_130_VehicleEngine",
    0x10A3: "SfxA_131_RumbleCrumble",
    0x10A4: "SfxA_132_LaunchThud",
    0x10A6: "SfxA_134_GearCrank",
    0x10A7: "SfxA_135_EngineRevUp",
    0x10A9: "SfxA_137_Chime",
    0x10AA: "SfxA_138_LaserZap",
    0x10AB: "SfxA_139_BoomA",
    0x10AC: "SfxA_140_BoomB",
    0x10AD: "SfxA_141_BlastC",
    0x10B1: "SfxA_145_TurboEngine",
    0x10B3: "SfxA_147_FlameRoar",
    0x10B4: "SfxA_148_EngineIdle",
    0x10B5: "SfxA_149_SweepDownA",
    0x10B6: "SfxA_150_SweepUpA",
    0x10B7: "SfxA_151_SweepUpB",
    0x10B9: "SfxA_153_Bloop",
    0x10BA: "SfxA_154_PulseSlow",
    0x10BF: "SfxA_159_PulseAccel",
    0x10C0: "SfxA_160_BubblePulse",
    0x10C1: "SfxA_161_SweepUpMid",
    0x10C2: "SfxA_162_SweepUpAccel",
    0x10C3: "SfxA_163_SweepUpFast",
    0x10C4: "SfxA_164_SweepUpHigh",
    0x10C5: "SfxA_165_SweepDownHigh",
    0x10C6: "SfxA_166_Crash",
    0x10D5: "SfxA_181_ToneUp",
    0x10D7: "SfxA_183_ToneDown",
    0x10E2: "SfxA_194_MetalClangC",
    0x10E3: "SfxA_195_MuffledImpact",
    0x10E4: "SfxA_196_SmallSplashB",
    0x10FA: "SfxA_218_MetalClick",
    0x10FB: "SfxA_219_GunshotBang",
    0x10FC: "SfxA_220_SlideRack",
    # Table B = digitised voices.  Names from their 68k users and, for the ids
    # without one, from listening to the export (mrom/tools/export_wav.py).
    0x1121: "Voice_FoodPickup",        # Item_Food_Pickup
    0x1123: "Voice_PlayerDeath",       # Player_Death_PlayMusic / Player_DeathPit (scream)
    0x1124: "Voice_Okay",              # weapon / weapon swap / bombs items: "Okay!"
    0x1126: "Voice_Bingo",             # "Bingo!" (no constant user)
    0x1127: "Voice_LetsGo",            # "Let's go!"
    0x1128: "Voice_HereWeGo",          # "Here we go!"
    0x1129: "Voice_Yeah",              # "Yeah!"
    0x112A: "Voice_YeeHaw",            # "Yee-haw!"
    0x112B: "Voice_PowerUp",           # "Power up!"
    0x112F: "Voice_Shotgun",           # Player_WeaponMusicTable_032d28[1]  "Shotgun!"
    0x112C: "Voice_FlameShot",         #   [2]  "Flame shot!"
    0x112E: "Voice_RocketLauncher",    #   [3]  "Rocket launcher!"
    0x112D: "Voice_HeavyMachineGun",   #   [4]  "Heavy machine gun!"
    0x1131: "Voice_Mission1Start",     # MissionStartVoiceTable_28DF10[mission]
    0x1132: "Voice_Mission2Start",
    0x1133: "Voice_Mission3Start",
    0x1134: "Voice_Mission4Start",
    0x1135: "Voice_Mission5Start",
    0x1136: "Voice_Mission6Start",
    0x1137: "Voice_MissionComplete",   # BannerComplete_Boot
    0x1138: "Voice_GameOver",          # "Game over"
    0x113A: "Voice_PowThankYou",       # Pow_Rescued: "Thank you!"
    0x113C: "Voice_AllenComeOnBoy",    # Allen_TauntVoiceTbl_0509d6: "Come on, boy!"
    0x113D: "Voice_AllenGoHomeToMommy",  # "Go home to mommy!"
    0x113E: "Voice_AllenMincemeat",    # "You're mincemeat!"
    0x113F: "Voice_AllenLaugh",        # "Ha ha ha"
    0x1140: "Voice_AllenSeeYouInHell", # Allen_Death: "See you in hell!"
}


def cpu2file(a):
    return a if a < 0x100000 else a - 0x100000


def load_registry():
    reg = []
    for ln in open(os.path.join(ROOT, "tools", "registry.py")):
        m = re.match(r'\s*\("(\w+)",\s*0x([0-9A-Fa-f]+),\s*(\d+)', ln)
        if m:
            reg.append((int(m.group(2), 16), m.group(1), int(m.group(3))))
    reg.sort()
    return reg


def main():
    d = open(PROM, "rb").read()
    reg = load_registry()
    starts = [r[0] for r in reg]

    def fn(a):
        i = bisect.bisect_right(starts, a) - 1
        return reg[i][1] if i >= 0 and a < reg[i][0] + reg[i][2] else f"${a:06X}"

    users = {}

    def add(v, who):
        if v == 0xFFFF or (v & 0xFF) < 0x20:
            return
        users.setdefault(v, [])
        if who not in users[v]:
            users[v].append(who)

    n_calls = 0
    for i in range(0, 0x100000 - 6, 2):           # code lives in the first MiB
        op = d[i:i + 2]
        if op in (b"\x4e\xb9", b"\x4e\xf9") and d[i + 2:i + 4] == b"\0\0":
            t = struct.unpack(">H", d[i + 4:i + 6])[0]
        elif op in (b"\x4e\xba", b"\x61\x00", b"\x60\x00", b"\x4e\xfa"):
            t = i + 2 + struct.unpack(">h", d[i + 2:i + 4])[0]
        else:
            continue
        if t not in TARGETS:
            continue
        for k in range(2, 16, 2):
            j = i - k
            if k >= 4 and d[j:j + 2] == b"\x30\x3c":
                add(struct.unpack(">H", d[j + 2:j + 4])[0], fn(i)); n_calls += 1
                break
            if k == 2 and d[j] == 0x70:
                add(d[j + 1], fn(i)); n_calls += 1
                break
    for a, n, what in TABLES:
        f = cpu2file(a)
        for k, v in enumerate(struct.unpack(f">{n}H", d[f:f + 2 * n])):
            add(v, f"{what.split(' ')[0]}[{k}]")

    lines = ["; sound_ids.txt - sound ids sent by the 68k program (generated by",
             "; mrom/tools/find_sound_ids.py from build/mslug_prom.bin; do not edit by hand).",
             "; id = type<<8 | cmd: 00xx music (MusicTable[cmd-20h]), 10xx SfxTableA, 11xx SfxTableB.",
             "; Ids only reached through script streams (ScriptOp04_Sound, PathScript_OpMusic,",
             "; MiniScript_Step, SceneScriptVM_Frame) or computed ids are not listed.",
             ";", "; id    name                      users"]
    for v in sorted(users):
        t, c = v >> 8, v & 0xFF
        if t == 0:
            name = MUSIC_NAMES.get(v, f"Music_{c - 0x20:02d}")
        elif v in SFX_NAMES:
            name = SFX_NAMES[v]
        elif t == 0x10:
            name = f"SfxA_{c - 0x20:03d}"
        elif t == 0x11:
            name = f"SfxB_{c - 0x20:03d}"
        else:
            name = f"Snd_{v:04X}"
        u = users[v]
        txt = ", ".join(u[:4]) + (f" (+{len(u) - 4})" if len(u) > 4 else "")
        lines.append(f"{v:04x}  {name:24s}  ; {txt}")
    for v, name in sorted(list(MUSIC_NAMES.items()) + list(SFX_NAMES.items())):
        if v not in users:                # keep every known name even if unused
            lines.append(f"{v:04x}  {name:24s}  ; (no constant user found)")
    open(OUT, "w").write("\n".join(lines) + "\n")
    print(f"[find_sound_ids] {n_calls} constant call sites + {len(TABLES)} tables -> "
          f"{len(users)} ids -> {os.path.relpath(OUT, ROOT)}")


if __name__ == "__main__":
    main()
