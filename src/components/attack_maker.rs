use specs::{Component, VecStorage};

pub struct AttackMaker {
    pub(crate) delay : f32,
    current_delay : f32
}

impl Default for AttackMaker {
    fn default() -> Self {
        AttackMaker {
            delay : 1.0,
            current_delay : 0.
        }
    }
}

impl Component for AttackMaker {
    type Storage = VecStorage<Self>;
}


impl AttackMaker {
    pub(crate) fn update(&mut self, dt : f32) -> bool {
        self.current_delay += dt;
        if self.current_delay >= self.delay {
            self.current_delay = 0.;
            return true;
        }
        return false;
    }
}