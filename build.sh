#!/bin/sh
# Собрать Elite с автоматически выбранным аудиобэкендом.
#
#   ./build.sh            — PipeWire, если найден libpipewire-0.3, иначе cpal
#   ./build.sh --pipewire — принудительно только PipeWire
#   ./build.sh --cpal     — принудительно только cpal
#   ELITE_AUDIO=cpal ./build.sh
#
# Остальные аргументы передаются cargo, например:
#   ./build.sh --pipewire --tests
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

exec cargo build --release $features "$@"
