use cgmath::One;
use rand::Rng;
use wgpu::util::DeviceExt;
use crate::components::cube_instance::CubeInstance;
use crate::components::mesh::Mesh;
use crate::components::tile::TileInstance;
use crate::renderer::RenderState;
use crate::renderer::vertex::{Instance, Vertex};

pub fn make_cube(renderer: &RenderState, is_left: bool) -> (Mesh, CubeInstance) {
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
                    let world_position = cgmath::Vector3 { x: (if is_left { -6 } else { 6 }) as f32, y: 0 as f32, z: 0 as f32 };
                    let position = cgmath::Vector3 { x: (x - 1) as f32 * 2.05, y: (y - 1) as f32 * 2.05, z: (z - 1) as f32 * 2.05 };
                    // let rotation = Quaternion::from_angle_x(cgmath::Deg(0.0));
                    Instance {
                        world_matrix: cgmath::Matrix4::from_translation(world_position),
                        model_matrix: cgmath::Matrix4::from_translation(position),
                        rpy_matrix: cgmath::Matrix4::one(),
                    }
                })
            })
        }).collect::<Vec<_>>();
    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    //endregion

    let vertex_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let index_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let instance_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );
    let num_indices = indices.len() as u32;
    let num_instances = instance_data.len() as u32;

    (Mesh {
        vertex_buffer,
        index_buffer,
        instance_buffer : Some(instance_buffer),
        num_indices,
        num_instances,
        texture: "world".into()
    },
     CubeInstance {
         changed: false,
         time_spend: 0.0,
         rpy_rnd: 99,
         instances,
     })
}

pub fn make_tile_map(renderer: &RenderState, texture : &str, tile_size : f32, uv_size: [f32;2]) -> Mesh {
    //region [ Vertex Data ]
    let vertex: [Vertex; 4] = [
        //Front
        Vertex {
            position: [0.0, 0.0, 1.0],
            tex_coords: [0.0, uv_size[1]],
        },
        Vertex {
            position: [tile_size, 0.0, 1.0],
            tex_coords: uv_size,
        },
        Vertex {
            position: [tile_size,tile_size, 1.0],
            tex_coords: [uv_size[0], 0.0],
        },
        Vertex {
            position: [0.0, tile_size, 1.0],
            tex_coords: [0.0, 0.0],
        }
    ];
    let indices: [u16; 6] = [
        //front
        0, 1, 2,
        2, 3, 0,
    ];


    let instances =
        (0..40).flat_map( |x| {
            (0..40).map(move |y| {
                let position = cgmath::Vector3 { x: (x - 20) as f32  * tile_size, y: (y - 20) as f32  * tile_size, z:  0.0 };
                let mut rng = rand::thread_rng();
                let tile = rng.gen_range(0..4);
                let tile_x = tile  as f32 * uv_size[0];
                let tile_y = 0.0;//(tile%2) as f32 * 0.02439;
                TileInstance{
                    uv: cgmath::Vector2 { x: tile_x  , y:  tile_y},
                    model_matrix: cgmath::Matrix4::from_translation(position),
                }
            })
        }).collect::<Vec<_>>();
    let instance_data = instances.iter().map(TileInstance::to_tile_raw).collect::<Vec<_>>();
    //endregion

    let vertex_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let index_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let instance_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );
    let num_indices = indices.len() as u32;
    let num_instances = instance_data.len() as u32;

    Mesh {
        vertex_buffer,
        index_buffer,
        instance_buffer : Some(instance_buffer),
        num_indices,
        num_instances,
        texture: texture.into()
    }
}
pub fn make_tile_single(renderer: &RenderState, texture : &str, tile_size : f32, uv_size: [f32;2]) -> Mesh {
    //region [ Vertex Data ]
    let vertex: [Vertex; 4] = [
        //Front
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0. ,  uv_size[1]],
            // tex_coords: [offset[0] , offset[1] + uv_size[1]],
        },
        Vertex {
            position: [tile_size, 0.0, 0.0],
            tex_coords: uv_size,
            // tex_coords: [offset[0] +uv_size[0], offset[1] +uv_size[1]],
        },
        Vertex {
            position: [tile_size,tile_size, 2.0],
            tex_coords: [uv_size[0], 0.0],
            // tex_coords: [offset[0] +uv_size[0], offset[1] +0.0],
        },
        Vertex {
            position: [0.0, tile_size, 2.0],
            tex_coords: [0.,0.] ,
            // tex_coords: offset ,
        }
    ];
    let indices: [u16; 6] = [
        //front
        0, 1, 2,
        2, 3, 0,
    ];


    let instances = vec![];
    // let instances =vec![TileInstance{
    //     uv: cgmath::Vector2 { x: 0.0  , y:  0.0},
    //     model_matrix: cgmath::Matrix4::one(),
    // }];

    let instance_data = instances.iter().map(TileInstance::to_tile_raw).collect::<Vec<_>>();
    //endregion

    let vertex_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let index_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let instance_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );
    let num_indices = indices.len() as u32;
    let num_instances = instance_data.len() as u32;

    Mesh {
        vertex_buffer,
        index_buffer,
        instance_buffer : Some(instance_buffer),
        num_indices,
        num_instances,
        texture: texture.into()
    }
}
