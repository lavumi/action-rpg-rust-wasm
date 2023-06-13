use specs::{Component, VecStorage};

pub struct Collision {
    rect_size: [f32; 2],
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}

impl Collision {
    pub fn new(rect_size: [f32; 2]) -> Self {
        Collision {
            rect_size
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
}