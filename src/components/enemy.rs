use specs::{Component, VecStorage};

pub struct Enemy {
    pub speed: f32,
    face_timer: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            speed: 4.0,
            face_timer: 99.,
        }
    }
}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}


impl Enemy {
    pub fn new(speed: f32) -> Self {
        Enemy {
            speed,
            face_timer: 99.,
        }
    }

    pub fn update_tick(&mut self, dt: f32) -> bool {
        self.face_timer += dt;
        if self.face_timer > 0.5 {
            self.face_timer = 0.;
            return true;
        }
        return false;
    }
}