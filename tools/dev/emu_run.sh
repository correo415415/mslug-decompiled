#!/bin/bash
# Lanza pydmg en headless (partida con moneda+START automáticos).
#   tools/dev/emu_run.sh OUTDIR FRAMES DUMPEVERY "POKES" "INPUT" [WATCH] [ZIP]
# Salida: build/OUTDIR/frame-*.png + build/OUTDIR/watch.txt (líneas WATCH).
cd "$(dirname "$0")/../.."; O=build/$1; rm -rf $O; mkdir -p $O
ZIP=${7:-internal/mslug.zip}
A=(--headless --cart $ZIP --max-frames $2 --auto-coin-frame 400 --auto-press-start-frame 500 --press-period-frames 300 --dump-frames-dir $O --dump-every-frames $3)
[ -n "$4" ] && A+=(--poke-ram "$4")
[ -n "$5" ] && A+=(--input-script "$5")
[ -n "$6" ] && A+=(--watch-ram "$6")
timeout 900 internal/pydmg/target/release/pydmg-neogeo "${A[@]}" > $O/log.txt 2>&1
grep WATCH $O/log.txt > $O/watch.txt
ls $O | wc -l
