use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component, Clone)]
pub struct Transform {
    pub position: [f32; 3],
    pub size: [f32; 2]
}


impl Transform {
    pub fn new(position: [f32; 3], size: [f32; 2]) -> Self {
        Transform {
            position,
            size,
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let position = cgmath::Vector3 { x: self.position[0], y: self.position[1], z: self.position[2] };
        let translation_matrix = cgmath::Matrix4::from_translation(position);
        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.size[0], self.size[1], 1.0);
        let model = (translation_matrix * scale_matrix).into();
        model
    }
}