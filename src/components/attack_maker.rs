use specs::{Component, VecStorage};

pub struct AttackMaker {
    fire: bool,
}

impl Default for AttackMaker {
    fn default() -> Self {
        AttackMaker {
            fire: false,
        }
    }
}

impl Component for AttackMaker {
    type Storage = VecStorage<Self>;
}

impl AttackMaker {
    pub fn get_fire_condition(&self) -> bool {
        self.fire
    }

    pub fn set_fire(&mut self) {
        self.fire = true;
    }

    pub fn fire_finished(&mut self) {
        self.fire = false;
    }

}