use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component, Clone)]
pub struct Attack {
    pub duration: f32,
    pub dt: f32,
    pub movement: [f32; 2],
}

#[derive(Default, Component, Clone)]
pub struct AttackMaker {
    pub fire: bool,
}
