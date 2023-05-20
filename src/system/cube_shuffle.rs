use cgmath::{One, Quaternion, Rotation3};
use rand::Rng;
use rand::rngs::ThreadRng;
use specs::{Read, System, Write, WriteStorage};
use crate::components::cube_instance::CubeInstance;
use crate::components::mesh::Mesh;
use crate::renderer::{Camera, GPUResourceManager, RenderState};
use crate::renderer::vertex::Instance;
use crate::resources::delta_time::DeltaTime;


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


pub struct CubeShuffle;

impl<'a> System<'a> for CubeShuffle {
    type SystemData = (
        Read<'a, DeltaTime>,

        WriteStorage<'a, Mesh>,
        WriteStorage<'a, CubeInstance>,
        Write<'a, ThreadRng>,
        Read<'a, RenderState>
    );

    fn run(&mut self, (dt, mut meshes, mut instances, mut rng, renderer): Self::SystemData) {
        use specs::Join;
        for ( mesh, instant ) in (&mut meshes, &mut instances).join() {
            instant.time_spend += dt.0;
            if instant.time_spend > 1.0 {
                instant.time_spend = 0.0;
                instant.rpy_rnd =  rng.gen_range(0..9) as usize;
                return;
            }

            if instant.rpy_rnd == 99 {
                return;
            }

            {
                let angle = instant.time_spend / 0.7;
                match instant.rpy_rnd {
                    0|1|2 =>{
                        let rotation = Quaternion::from_angle_z(cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                        for index in ROLL_ARRAY[instant.rpy_rnd] {
                            instant.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                        }
                    }
                    3|4|5 =>{
                        let rotation = Quaternion::from_angle_y(-cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                        for index in ROLL_ARRAY[instant.rpy_rnd] {
                            instant.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                        }
                    }
                    6|7|8 =>{
                        let rotation = Quaternion::from_angle_x(cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                        for index in ROLL_ARRAY[instant.rpy_rnd] {
                            instant.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                        }
                    }
                    _ => {}
                }
            }

            if instant.time_spend >= 0.7 {
                {
                    let angle = 1.0;
                    match instant.rpy_rnd {
                        0|1|2 =>{
                            let rotation = Quaternion::from_angle_z(cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                            for index in ROLL_ARRAY[instant.rpy_rnd] {
                                instant.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                            }
                        }
                        3|4|5 =>{
                            let rotation = Quaternion::from_angle_y(-cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                            for index in ROLL_ARRAY[instant.rpy_rnd] {
                                instant.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                            }
                        }
                        6|7|8 =>{
                            let rotation = Quaternion::from_angle_x(cgmath::Rad(std::f32::consts::PI * 0.5) * angle);
                            for index in ROLL_ARRAY[instant.rpy_rnd] {
                                instant.instances[index].rpy_matrix = cgmath::Matrix4::from(rotation);
                            }
                        }
                        _ => {}
                    }
                    instant.changed = true;
                }
                {
                    for instance in &mut instant.instances {
                        instance.model_matrix = instance.rpy_matrix * instance.model_matrix;
                        instance.rpy_matrix = cgmath::Matrix4::one()
                    }

                    let changed_blocks = &ROLL_ARRAY[instant.rpy_rnd];




                    instant.instances.swap(changed_blocks[0] , changed_blocks[2]);
                    instant.instances.swap(changed_blocks[2] , changed_blocks[8]);
                    instant.instances.swap(changed_blocks[8] , changed_blocks[6]);



                    instant.instances.swap(changed_blocks[1] , changed_blocks[5]);
                    instant.instances.swap(changed_blocks[5] , changed_blocks[7]);
                    instant.instances.swap(changed_blocks[7] , changed_blocks[3]);
                }
                instant.rpy_rnd = 99;
            }

            //todo think 여기서 instance buffer 까지 업데이트 해 주는게 옳은 방법일까?
            let instance_data = instant
                .instances
                .iter()
                .map(Instance::to_raw)
                .collect::<Vec<_>>();

            renderer.queue.write_buffer(&mesh.instance_buffer, 0, bytemuck::cast_slice(&instance_data));

        }
    }

}