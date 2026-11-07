#!/bin/sh
# Quick checks: rebuild the Kladov game image and run the unit tests.
#   ./check.sh          -- build + unit tests
#   ./check.sh --full   -- above + full ZEXDOC instruction audit (needs
#                          tmp/zexdoc.com in the project, ~5 minutes)
#
# Аудиобэкенд выбирается автоматически (PipeWire, если он есть, иначе cpal).
# Принудительно: ELITE_AUDIO=pipewire ./check.sh
#                ELITE_AUDIO=cpal     ./check.sh
#                ./check.sh --cpal
set -e
cd "$(dirname "$0")"

full=0
audio=${ELITE_AUDIO:-auto}
for arg in "$@"; do
    case "$arg" in
        --full) full=1 ;;
        --pipewire | --cpal) audio=${arg#--} ;;
        *)
            echo "check: неизвестный аргумент '$arg' (--full, --pipewire, --cpal)" >&2
            exit 2
            ;;
    esac
done
features=$(tools/audio_features.sh "$audio")

python3 tools/make_kladov_image.py
cargo test --release $features

if [ "$full" = 1 ]; then
    cargo test --release $features zexdoc -- --ignored --nocapture
fi

echo "check: OK"
