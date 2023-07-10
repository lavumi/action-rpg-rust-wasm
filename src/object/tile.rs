use wgpu::Device;
use wgpu::util::DeviceExt;

use crate::renderer::{Mesh, Vertex};

pub fn make_tile_single_isometric(device: &Device, tile_size: [f32; 2], uv_size: [f32; 2]) -> Mesh {
    //region [ Vertex Data ]

    let tile_size_half = [tile_size[0] * 0.5, tile_size[1] * 0.5];
    let vertex: [Vertex; 4] = [
        //Front
        Vertex {
            position: [-tile_size_half[0], -tile_size_half[1], 0.0],
            tex_coords: [0., uv_size[1]],
            // tex_coords: [offset[0] , offset[1] + uv_size[1]],
        },
        Vertex {
            position: [tile_size_half[0], -tile_size_half[1], 0.0],
            tex_coords: uv_size,
            // tex_coords: [offset[0] +uv_size[0], offset[1] +uv_size[1]],
        },
        Vertex {
            position: [tile_size_half[0], tile_size_half[1], 0.0],
            tex_coords: [uv_size[0], 0.0],
            // tex_coords: [offset[0] +uv_size[0], offset[1] +0.0],
        },
        Vertex {
            position: [-tile_size_half[0], tile_size_half[1], 0.0],
            tex_coords: [0., 0.],
            // tex_coords: offset ,
        }
    ];
    let indices: [u16; 6] = [
        //front
        0, 1, 2,
        2, 3, 0,
    ];

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

    let num_indices = indices.len() as u32;
    let num_instances = 0;//instance_data.len() as u32;

    Mesh {
        vertex_buffer,
        index_buffer,
        instance_buffer: None,
        num_indices,
        num_instances,
        // texture: texture.into()
    }
}