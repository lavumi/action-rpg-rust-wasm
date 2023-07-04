use specs::{Component, VecStorage};

pub struct Physics {
    aabb_offset: [f32; 4],
    velocity: [f32; 2],
}

impl Component for Physics {
    type Storage = VecStorage<Self>;
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            // aabb_offset: [-0.5, 0.5,-0.25,0.25],
            aabb_offset: [-1.0, 0.0,-0.25,0.25],
            velocity: [0., 0.],
        }
    }
}

impl Physics {
    #[allow(dead_code)]
    pub fn new(aabb_offset: [f32; 4]) -> Self {
        Physics {
            aabb_offset,
            velocity: [0., 0.],
        }
    }

    pub fn get_aabb(&self, position: [f32; 3]) -> [f32; 4] {
        [
            position[0] + self.aabb_offset[0] ,
            position[0] + self.aabb_offset[1] ,
            position[1] + self.aabb_offset[2] ,
            position[1] + self.aabb_offset[3] ,
        ]
    }

    #[allow(dead_code)]
    pub fn get_delta_aabb(&self, position: [f32; 3]) -> [f32; 4] {
        let curr_aabb = self.get_aabb(position);
        [
            curr_aabb[0] + self.velocity[0],
            curr_aabb[1] + self.velocity[0],
            curr_aabb[2] + self.velocity[1],
            curr_aabb[3] + self.velocity[1],
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