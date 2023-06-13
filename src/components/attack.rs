use specs::{Component, VecStorage};

pub struct Attack {
    pub duration : f32,
    pub dt : f32,
    pub movement : [f32;2],
}


impl Component for Attack {
    type Storage = VecStorage<Self>;
}

impl Attack {
    pub fn is_expired(&self) -> bool {
        return self.duration <= self.dt
    }
}