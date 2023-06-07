use cgmath::{One, Quaternion, Rotation3};
use specs::{Component, VecStorage};
use crate::renderer::vertex::Instance;

pub struct CubeInstance {
    pub(crate) changed: bool,
    pub(crate) time_spend: f32,
    pub(crate) rpy_rnd: usize,
    pub(crate) instances: Vec<Instance>,
}

impl Component for CubeInstance {
    type Storage = VecStorage<Self>;
}

const ROLL_ARRAY: &[[usize; 9]; 9] = &[
    [0, 3, 6, 9, 12, 15, 18, 21, 24],
    [1, 4, 7, 10, 13, 16, 19, 22, 25],
    [2, 5, 8, 11, 14, 17, 20, 23, 26],

    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],

    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26]
];

impl CubeInstance {
    pub fn run_cube(&mut self){
        let angle = if self.time_spend * 1.428 > 1.0 {1.0} else {self.time_spend / 0.7};//cmp::max(self.time_spend / 0.7 , 1.0);
        match self.rpy_rnd {
            0|1|2 =>{
                let rotation = Quaternion::from_angle_z(cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                for index in ROLL_ARRAY[self.rpy_rnd] {
                    self.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                }
            }
            3|4|5 =>{
                let rotation = Quaternion::from_angle_y(-cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                for index in ROLL_ARRAY[self.rpy_rnd] {
                    self.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                }
            }
            6|7|8 =>{
                let rotation = Quaternion::from_angle_x(cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                for index in ROLL_ARRAY[self.rpy_rnd] {
                    self.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                }
            }
            _ => {}
        }

        self.changed = true;

        if angle == 1.0 {
            self.finish_run_cube_shuffle();
        }
    }

    fn finish_run_cube_shuffle(&mut self) {

        for instance in &mut self.instances {
            instance.model_matrix = instance.rpy_matrix * instance.model_matrix;
            instance.rpy_matrix = cgmath::Matrix4::one()
        }

        let changed_blocks = &ROLL_ARRAY[self.rpy_rnd];

        self.instances.swap(changed_blocks[0] , changed_blocks[2]);
        self.instances.swap(changed_blocks[2] , changed_blocks[8]);
        self.instances.swap(changed_blocks[8] , changed_blocks[6]);

        self.instances.swap(changed_blocks[1] , changed_blocks[5]);
        self.instances.swap(changed_blocks[5] , changed_blocks[7]);
        self.instances.swap(changed_blocks[7] , changed_blocks[3]);

        self.rpy_rnd = 99;
    }
}