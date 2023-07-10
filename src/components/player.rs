use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}