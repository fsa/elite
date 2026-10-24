//! Модель полёта: либо своя симуляция, либо адаптер чтения из Z80.
//! Рендер получает только данные для отрисовки.

use crate::render3d::{Rotation, Screen};
use crate::shipdata::Ship;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlightMode {
    Native,
    Z80,
}

#[derive(Clone, Copy, Debug)]
pub struct HudData {
    pub speed: f32,
    pub fuel: f32,
    pub shield: f32,
    pub laser_heat: f32,
    pub kills: u32,
    pub system: [u8; 32],
    pub system_len: usize,
    pub laser_timer: f32,
    pub damage_flash: f32,
    pub message: [u8; 32],
    pub message_len: usize,
    pub message_timer: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DrawBody {
    pub kind: u8, // 0 ship, 1 station, 2 planet, 3 sun
    pub has_model: bool,
    pub model_idx: u32,
    pub pos: [f32; 3],
    pub rot: Rotation,
    pub spin: [f32; 3],
    pub spin_rate: f32,
    pub radius: f32,
    pub color: u32,
}

#[derive(Clone)]
pub struct FlightView {
    pub camera_pos: [f32; 3],
    pub camera_rot: Rotation,
    pub bodies: Vec<DrawBody>,
    pub stars: Vec<[f32; 3]>,
    pub star_mags: Vec<u8>,
    pub hud: HudData,
}

pub trait FlightSource {
    fn mode(&self) -> FlightMode;
    fn tick(&mut self, controls: &Controls, dt: f32, mem: Option<&[u8]>, kladov: bool) -> FlightView;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Controls {
    pub pitch_up: bool,
    pub pitch_down: bool,
    pub roll_left: bool,
    pub roll_right: bool,
    pub throttle_up: bool,
    pub throttle_down: bool,
    pub fire: bool,
}

impl Controls {
    pub fn from_matrix(matrix: &[[bool; 5]; 8], kempston: &[bool; 5]) -> Self {
        let key = |row: usize, bit: usize| matrix[row][bit];
        Controls {
            pitch_up: key(1, 1) || kempston[3],
            pitch_down: key(0, 2) || kempston[2],
            roll_left: key(7, 3) || kempston[1],
            roll_right: key(7, 2) || kempston[0],
            throttle_up: key(7, 0),
            throttle_down: key(7, 1),
            fire: key(1, 0) || kempston[4],
        }
    }
}

fn read_signed24(b: &[u8]) -> f32 {
    if b.len() < 3 {
        return 0.0;
    }
    let v = ((b[2] as u32) << 16) | ((b[1] as u32) << 8) | (b[0] as u32);
    if (v & 0x800000) != 0 {
        ((v as i32) - 0x1000000) as f32
    } else {
        v as f32
    }
}

pub fn model_index(name: &str) -> Option<u32> {
    crate::shipdata::SHIPS.iter().position(|s| s.name == name).map(|i| i as u32)
}

pub fn get_model(idx: u32) -> Option<&'static Ship> {
    crate::shipdata::SHIPS.get(idx as usize)
}
