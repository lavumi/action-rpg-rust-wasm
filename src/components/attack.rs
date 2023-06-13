use specs::{Component, VecStorage};

pub struct Attack {
    duration: f32,
    dt: f32,
    movement: [f32; 2],
}


impl Component for Attack {
    type Storage = VecStorage<Self>;
}

impl Attack {
    pub fn new(duration: f32, movement: [f32; 2]) -> Self {
        Attack {
            duration,
            dt: 0.0,
            movement,
        }
    }

    pub fn update(&mut self, dt: f32) -> [f32; 2] {
        self.dt += dt;
        let movement: [f32; 2] = [
            dt * self.movement[0],
            dt * self.movement[1]
        ];
        movement
    }

    pub fn is_expired(&self) -> bool {
        return self.duration <= self.dt
    }
}