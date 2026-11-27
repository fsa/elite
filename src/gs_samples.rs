//! Сэмплы General Sound, извлечённые из `zx_src/Elite3GS.trd`.
//!
//! Файл генерируется `tools/extract_gs_samples.py`; вручную не править.
//! Порядок соответствует номеру звука из защёлки порта `0xB3`.

/// Период сэмплирования из каталога (`0x60`), 1 МГц / период.
pub const GS_SAMPLE_RATE: u32 = 1_000_000 / 96;

/// Все сэмплы подряд: `GS_SAMPLE_OFFSETS[id]..GS_SAMPLE_OFFSETS[id + 1]`.
pub const GS_SAMPLE_DATA: &[u8] = include_bytes!("../zx_bin/gs_samples.bin");

/// Смещения сэмплов в `GS_SAMPLE_DATA`; индекс 0 — пустой сэмпл.
pub const GS_SAMPLE_OFFSETS: [usize; 24] = [
    0,
    1024,
    18688,
    36352,
    39168,
    51200,
    60672,
    64512,
    76288,
    87552,
    106496,
    114432,
    128000,
    128768,
    135424,
    141824,
    144640,
    167424,
    186880,
    195328,
    199168,
    204800,
    210432,
    220160,
];

/// Вернуть 8-битный сэмпл по номеру из защёлки порта `0xB3`.
pub fn sample(id: u8) -> Option<&'static [u8]> {
    let i = id as usize;
    if i == 0 || i >= GS_SAMPLE_OFFSETS.len() {
        return None;
    }
    Some(&GS_SAMPLE_DATA[GS_SAMPLE_OFFSETS[i - 1]..GS_SAMPLE_OFFSETS[i]])
}
