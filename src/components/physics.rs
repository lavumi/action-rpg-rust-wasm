use specs::{Component, VecStorage};

pub struct Physics {
    rect_size: [f32; 2],
    velocity: [f32; 2],
}

impl Component for Physics {
    type Storage = VecStorage<Self>;
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            rect_size: [2.0, 2.0],
            velocity: [0., 0.],
        }
    }
}

impl Physics {
    pub fn new(rect_size: [f32; 2]) -> Self {
        Physics {
            rect_size,
            velocity: [0., 0.],
        }
    }

    pub fn get_aabb(&self, position: [f32; 2]) -> [f32; 4] {
        [
            position[0] - self.rect_size[0] / 2.,
            position[0] + self.rect_size[0] / 2.,
            position[1] - self.rect_size[1] / 2.,
            position[1] + self.rect_size[1] / 2.,
        ]
    }

    pub fn set_velocity(&mut self, velocity: [f32; 2]) {

        if velocity[0] != 0. && velocity[1] != 0. {
            let normalize = 0.4472135955;
            self.velocity = [ velocity[0] * 2. * normalize , velocity[1] * normalize];
        }
        else {
            self.velocity = velocity;
        }


    }

    pub fn get_velocity(&self) -> [f32; 2] {
        self.velocity
    }
}