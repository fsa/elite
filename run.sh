#!/bin/sh
# Build (if needed) and run the Elite emulator.
#
#   ./run.sh                 — версия Кладова (zx_bin/kladov.bin), базовая
#   ./run.sh elite           — оригинальная версия (zx_bin/elite.bin)
#
# Дополнительные аргументы передаются эмулятору, например:
#   ./run.sh --frames 600 --dump out.ppm
#   ./run.sh --headless --frames 300
#   ./run.sh --fullscreen        — полный экран (или F11 / Alt+Enter в игре)
#   ./run.sh --rom /path/to/48.rom
set -e
cd "$(dirname "$0")"

case "$1" in
  elite|--elite)
    shift
    [ -f zx_bin/elite.bin ] || ./build.sh
    exec cargo run --release -- --elite "$@"
    ;;
  *)
    [ -f zx_bin/kladov.bin ] || python3 tools/make_kladov_image.py
    exec cargo run --release -- "$@"
    ;;
esac
