//! Образы, встраиваемые в исполняемый файл на этапе сборки.
//!
//! Это позволяет запускать игру без внешних файлов: образ RAM версии Кладова
//! и ROM ZX Spectrum 48K становятся частью бинарника. Пути в `include_bytes!`
//! относительные к этому файлу, поэтому сборка не зависит от текущего каталога.

/// Полный 48K-образ памяти версии Кладова (адреса 0x4000..0xFFFF).
pub const KLADOV_IMAGE: &[u8] = include_bytes!("../zx_bin/kladov.bin");

/// Стандартная ROM ZX Spectrum 48K.
pub const ROM_48: &[u8] = include_bytes!("../zx_bin/48.rom");

const _: () = assert!(KLADOV_IMAGE.len() == 0xC000);
const _: () = assert!(ROM_48.len() == 0x4000);
