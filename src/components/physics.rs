use specs::{Component, DenseVecStorage};
use specs_derive::Component;


#[derive(Component, Clone)]
pub struct Physics {
    pub aabb_offset: [f32; 4],
    pub velocity: [f32; 2],
    pub is_trigger: bool,
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            aabb_offset: [-1.0, 0.0, -0.25, 0.25],
            velocity: [0., 0.],
            is_trigger: false,
        }
    }
}

/**
convert velocity from tile grid movement to isometric grid movement
 */
pub fn convert_velocity(velocity: [f32; 2]) -> [f32; 2] {
    if velocity[0] != 0. && velocity[1] != 0. {
        let normalize = 0.4472135955;
        [velocity[0] * 2. * normalize, velocity[1] * normalize]
    } else {
        velocity
    }
}