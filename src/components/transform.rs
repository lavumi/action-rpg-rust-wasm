use specs::{Component, VecStorage};

pub struct Transform{
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub direction: [i8; 2],
}



impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {
    pub fn new(position: [f32; 3], size: [f32; 2]) -> Self {
        Transform {
            position,
            size,
            direction: [-1, 0],
        }
    }

    pub fn move_position(&mut self, delta: [f32; 2]) -> u8 {
        self.position[0] += delta[0];
        self.position[1] += delta[1];
        self.position[2] = 1.0 - self.position[1] / 10000.0;

        let dir_x = if delta[0] != 0. { (delta[0] / delta[0].abs()) as i8 } else { self.direction[0] };
        let dir_y = if delta[1] != 0. { (delta[1] / delta[1].abs()) as i8 } else { self.direction[1] };
        self.direction = [dir_x, dir_y];

        self.get_direction()
    }

    pub fn get_direction(&self) -> u8 {
        if self.direction[0] == -1 {
            if self.direction[1] == -1 { 7 } else if self.direction[1] == 0 { 0 } else { 1 }
        } else if self.direction[0] == 0 {
            if self.direction[1] == -1 { 6 } else if self.direction[1] == 0 {
                // panic!("direction is both 0");
                6
            } else { 2 }
        } else {
            if self.direction[1] == -1 { 5 } else if self.direction[1] == 0 { 4 } else { 3 }
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let position = cgmath::Vector3 { x: self.position[0], y: self.position[1], z: self.position[2] };
        let translation_matrix = cgmath::Matrix4::from_translation(position);
        // let flip_matrix =
        // if self.flip {
        //     cgmath::Matrix4::from_angle_y(cgmath::Rad(std::f32::consts::PI))
        // } else {
        //     cgmath::Matrix4::one()
        // };

        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.size[0], self.size[1], 1.0);
        let model = (translation_matrix * scale_matrix).into();
        model
    }
}