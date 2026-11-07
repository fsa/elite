#!/bin/sh
# Выбирает фичи cargo для аудиобэкенда.
#
# Использование:
#   tools/audio_features.sh [auto|pipewire|cpal]
#
#   auto      — PipeWire, если pkg-config находит libpipewire-0.3;
#               иначе cpal. На не-Linux системах всегда cpal.
#   pipewire  — принудительно только PipeWire (Linux).
#   cpal      — принудительно только cpal (Linux/macOS/Windows).
#
# По умолчанию режим берётся из $ELITE_AUDIO, иначе auto.
# Результат — строка аргументов для cargo (может быть пустой).
set -e

mode=${1:-${ELITE_AUDIO:-auto}}

has_pipewire() {
    command -v pkg-config >/dev/null 2>&1 &&
        pkg-config --exists libpipewire-0.3 2>/dev/null
}

has_alsa() {
    command -v pkg-config >/dev/null 2>&1 &&
        pkg-config --exists alsa 2>/dev/null
}

case "$mode" in
    auto)
        case "$(uname -s 2>/dev/null)" in
            Linux*)
                if has_pipewire && has_alsa; then
                    # Фичи по умолчанию: PipeWire первым, cpal запасным.
                    :
                elif has_pipewire; then
                    printf '%s' '--no-default-features --features audio-pipewire'
                else
                    printf '%s' '--no-default-features --features audio-cpal'
                fi
                ;;
            *)
                # macOS и другие системы: cpal (CoreAudio/WASAPI/...).
                printf '%s' '--no-default-features --features audio-cpal'
                ;;
        esac
        ;;
    pipewire)
        printf '%s' '--no-default-features --features audio-pipewire'
        ;;
    cpal)
        printf '%s' '--no-default-features --features audio-cpal'
        ;;
    *)
        echo "audio_features: неизвестный режим '$mode' (auto, pipewire, cpal)" >&2
        exit 2
        ;;
esac
