use cgmath::{One, Quaternion, Rotation3};
use wgpu::{Device, Queue};
use wgpu::util::DeviceExt;
use crate::vertex::{Instance, Vertex};
use rand;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::renderer::{ RenderComponent};

pub struct Cube {

    instances: Vec<Instance>,
    render_component : RenderComponent,

    changed: bool,
    can_rotate: bool,
    time_spend : f32,
    rpy_rnd : usize,
    rng: ThreadRng,

    // test_counter : usize
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


// const TEST_RAND: &[usize] = &[3,1,6];

impl Cube {
    pub fn new(device: &Device) -> Self {

        //region [ Vertex Data ]
        let vertex: [Vertex; 24] = [
            //Front
            Vertex {
                position: [-1.0, -1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                tex_coords: [0.33333, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                tex_coords: [0.33333, 0.5],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                tex_coords: [0.0, 0.5],
            },
            //Upper
            Vertex {
                position: [-1.0, 1.0, -1.0],
                tex_coords: [0.66666, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                tex_coords: [0.33333, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                tex_coords: [0.33333, 0.5],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                tex_coords: [0.66666, 0.5],
            },
            //back
            Vertex {
                position: [-1.0, -1.0, -1.0],
                tex_coords: [0.66666, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                tex_coords: [1.0, 0.5],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                tex_coords: [0.66666, 0.5],
            },
            //Down
            Vertex {
                position: [-1.0, -1.0, -1.0],
                tex_coords: [0.33333, 0.5],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                tex_coords: [0.66666, 0.5],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                tex_coords: [0.66666, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 1.0],
                tex_coords: [0.33333, 0.0],
            },
            //Left
            Vertex {
                position: [-1.0, -1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                tex_coords: [0.33333, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                tex_coords: [0.33333, 0.5],
            },
            Vertex {
                position: [-1.0, -1.0, 1.0],
                tex_coords: [0.0, 0.5],
            },
            //Right
            Vertex {
                position: [1.0, -1.0, -1.0],
                tex_coords: [1.0, 0.5],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                tex_coords: [0.66666, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                tex_coords: [0.66666, 0.5],

            },
        ];
        let indices: [u16; 36] = [
            //front
            0, 1, 2,
            2, 3, 0,


            //top
            6, 5, 4,
            4, 7, 6,


            //back
            10, 9, 8,
            8, 11, 10,


            //down
            12, 13, 14,
            14, 15, 12,

            //left
            18, 17, 16,
            16, 19, 18,

            //right
            20, 21, 22,
            22, 23, 20
        ];
        let instances =
            (0..3).flat_map(|x| {
                (0..3).flat_map(move |y| {
                    (0..3).map(move |z| {
                        let position = cgmath::Vector3 { x: (x - 1) as f32 * 2.05, y: (y - 1) as f32 * 2.05, z: (z - 1) as f32 * 2.05 };
                        // let rotation = Quaternion::from_angle_x(cgmath::Deg(0.0));
                        Instance {
                            world_matrix: cgmath::Matrix4::one(),
                            model_matrix: cgmath::Matrix4::from_translation(position),
                            rpy_matrix: cgmath::Matrix4::one(),
                        }
                    })
                })
            }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        //endregion

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let num_indices = indices.len() as u32;
        let num_instances = instance_data.len() as u32;
        let rng = rand::thread_rng();


        let render_component = RenderComponent{
            vertex_buffer,
            index_buffer,
            instance_buffer,
            num_indices,
            num_instances,
        };
        Self {
            render_component,
            instances,
            changed: false,
            can_rotate: false,
            time_spend : 0.0,
            rpy_rnd : 99,
            rng,
        }
    }


    pub fn toggle_rotate(&mut self, can_rotate: bool) {
        self.can_rotate = can_rotate;
    }

    #[allow(unused_variables)]
    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        if self.can_rotate == false {
            return;
        }

        for instance in &mut self.instances {
            let amount_x: Quaternion<f32> = Quaternion::from_angle_y(cgmath::Rad(0.01) * delta_x);
            let amount_y = Quaternion::from_angle_x(cgmath::Rad(0.01) * delta_y);
            let current = instance.world_matrix;
            instance.world_matrix = cgmath::Matrix4::from(amount_x * amount_y )* current;
        }
        self.changed = true;
    }

    pub fn update(&mut self, dt: f32) {
        self.time_spend += dt;

        if self.time_spend > 1.0 {
            self.time_spend = 0.0;

            // self.rpy_rnd = TEST_RAND[self.test_counter];
            // self.test_counter+=1;
            // if self.test_counter == TEST_RAND.len() {
            //     self.test_counter = 0;
            //     self.time_spend = -100.0;
            // }

            self.rpy_rnd =  self.rng.gen_range(0..9) as usize;




            return;
        }

        if self.rpy_rnd == 99 {
            return;
        }

        self.run_cube(self.time_spend / 0.7);

        if self.time_spend >= 0.7 {
            self.run_cube(1.0);
            self.finish_run_cube();
            self.rpy_rnd = 99;
        }

    }

    fn run_cube(&mut self , angle : f32){
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
    }

    fn finish_run_cube(&mut self ){
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
    }


    pub fn update_instance(&mut self, queue: &Queue) {
        if self.changed {
            let instance_data = self
                .instances
                .iter()
                .map(Instance::to_raw)
                // .map(|instant| instant.rotation * instant.position )
                .collect::<Vec<_>>();

            queue.write_buffer(&self.render_component.instance_buffer, 0, bytemuck::cast_slice(&instance_data));
        }
        self.changed = false;
    }


    pub fn get_render_component(&self) -> &RenderComponent {
        return &self.render_component;
    }
}