use specs::{Component, VecStorage};

pub struct Player{
    pub speed: f32
}

impl Default for Player {
    fn default() -> Self {
        Player{
            speed: 5.0
        }
    }
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}