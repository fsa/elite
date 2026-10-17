# Порт Elite с ZX-Spectrum

Этот проект — попытка переделать игру Elite с ZX-Spectrum под современные ОС с помощью ИИ.

## Запуск

Базовая версия проекта — версия Кладова.

```sh
./run.sh            # версия Кладова (zx_bin/kladov.bin)
./run.sh elite      # оригинальная версия (zx_bin/elite.bin)
```

Управление: стрелки — курс/джойстик (Kempston), `RightCtrl` — огонь, `Esc` — выход.
`LeftShift` (Caps Shift) — меню настроек Кладова (в нём переключают флаги клавишами
`B/D/F/R/Y/S`), `RightShift` (Symbol Shift) — скорость корабля.

## Звук

Игра использует только 1-битный бипер ZX-Spectrum (порт `0xFE`). Звук
выводится через PipeWire; `--mute` отключает вывод, `--wav FILE` сохраняет
моно-PCM (44100 Гц) в WAV-файл.

Для сборки нужны заголовки PipeWire (`pipewire-devel`/`libpipewire-0.3`).
Крейт `pipewire` использует bindgen: если установлен только `clang-libs` без
пакета `clang`, каталоги заголовков clang приходится задавать вручную — см.
`.cargo/config.toml` (`BINDGEN_EXTRA_CLANG_ARGS`).

Образ версии Кладова (`zx_bin/kladov.bin`) собирается из снапшота
`zx_src/kladov.snapshot` скриптом `python3 tools/make_kladov_image.py` (запускается
автоматически, если файла нет).
