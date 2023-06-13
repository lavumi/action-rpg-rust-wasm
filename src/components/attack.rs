use specs::{Component, VecStorage};

pub struct Attack {
    pub duration : f32,
    pub dt : f32,
    pub direction : [f32;2],
    pub speed : f32
}


impl Component for Attack {
    type Storage = VecStorage<Self>;
}