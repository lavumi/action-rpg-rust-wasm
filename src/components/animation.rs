use specs::*;
use specs_derive::Component;

#[derive(Component, Clone)]
pub struct Animation {
    pub name: String,
    pub speed: f32,
    pub index: usize,
    pub frame: usize,
    pub dt: f32,
}

#[derive(Component, Clone)]
pub struct Movable(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction { Left, UpLeft, Up, UpRight, Right, DownRight, Down, DownLeft, None }

#[derive(Component, Clone)]
pub struct Forward {
    pub direction: Direction,
}