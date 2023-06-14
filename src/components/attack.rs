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

pub struct AttackMaker {
    delay: f32,
    current_delay: f32,
}

impl Default for AttackMaker {
    fn default() -> Self {
        AttackMaker {
            delay: 1.0,
            current_delay: 0.,
        }
    }
}

impl Component for AttackMaker {
    type Storage = VecStorage<Self>;
}


impl AttackMaker {
    pub(crate) fn update(&mut self, dt: f32) -> bool {
        self.current_delay += dt;
        if self.current_delay >= self.delay {
            self.current_delay = 0.;
            return true;
        }
        return false;
    }
}