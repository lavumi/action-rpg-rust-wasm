use specs::{Component, VecStorage};

pub struct Enemy {
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            speed: 1.0
        }
    }
}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}