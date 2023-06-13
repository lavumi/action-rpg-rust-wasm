use cgmath::One;
use specs::{Component, VecStorage};

pub struct Transform{
    pub(crate) position: [f32;3],
    pub(crate) flip: bool,
    pub(crate) direction: [i8;2]
}



impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {
    pub fn new(position: [f32;3])->Self {
        Transform{
            position,
            flip: false,
            direction : [0, 0]
        }
    }

    pub fn move_position(&mut self ,delta: [f32;2]){
        self.position[0] += delta[0];
        self.position[1] += delta[1];

        if delta[0] != 0. || delta[1] != 0. {
            self.direction = [ (delta[0] / delta[0].abs()) as i8, (delta[1] / delta[1].abs()) as i8];
        }


        if delta[0] > 0.0 {
            self.flip = true;
        }
        else if delta[0] < 0.0 {
            self.flip = false;
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let position = cgmath::Vector3 { x: self.position[0] , y: self.position[1], z:  self.position[2]};
        let translation_matrix = cgmath::Matrix4::from_translation(position);
        let flip_matrix =
            if self.flip {
                cgmath::Matrix4::from_angle_y( cgmath::Rad( std::f32::consts::PI))
            }
            else {
                cgmath::Matrix4::one()
            };

        let model = (translation_matrix * flip_matrix  ).into();
        model
    }
}