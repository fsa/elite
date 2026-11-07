#!/bin/sh
# Build (if needed) and run the Elite emulator.
#
# Аудиобэкенд выбирается автоматически: PipeWire, если найден libpipewire-0.3,
# иначе cpal. Принудительно:
#   ELITE_AUDIO=pipewire ./run.sh ...
#   ELITE_AUDIO=cpal     ./run.sh ...
# или первым аргументом:
#   ./run.sh --pipewire ...
#   ./run.sh --cpal ...
#
# Остальные аргументы передаются эмулятору, например:
#   ./run.sh --frames 600 --dump out.ppm
#   ./run.sh --headless --frames 300
#   ./run.sh --fullscreen        — полный экран (или F11 / Alt+Enter в игре)
#   ./run.sh --rom /path/to/48.rom
set -e
cd "$(dirname "$0")"

audio=${ELITE_AUDIO:-auto}
case "$1" in
    --pipewire | --cpal)
        audio=${1#--}
        shift
        ;;
esac
features=$(tools/audio_features.sh "$audio")

[ -f zx_bin/kladov.bin ] || python3 tools/make_kladov_image.py
exec cargo run --release $features -- "$@"
