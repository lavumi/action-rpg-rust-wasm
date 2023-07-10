use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component, Copy, Clone)]
pub struct Enemy {
    pub speed: f32,
    pub tick: f32,
}