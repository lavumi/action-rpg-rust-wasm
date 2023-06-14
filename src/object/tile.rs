use wgpu::util::DeviceExt;

use crate::components::mesh::Mesh;
use crate::components::tile::InstanceTileRaw;
use crate::renderer::RenderState;
use crate::renderer::vertex:: Vertex;

pub fn make_tile_single_isometric(renderer: &RenderState, tile_size: f32, uv_size: [f32; 2]) -> Mesh {
    //region [ Vertex Data ]
    let tile_size_half = tile_size / 2.0;
    let vertex: [Vertex; 4] = [
        //Front
        Vertex {
            position: [-tile_size_half, -tile_size_half * 0.5, 0.0],
            tex_coords: [0., uv_size[1]],
            // tex_coords: [offset[0] , offset[1] + uv_size[1]],
        },
        Vertex {
            position: [tile_size_half, -tile_size_half * 0.5, 0.0],
            tex_coords: uv_size,
            // tex_coords: [offset[0] +uv_size[0], offset[1] +uv_size[1]],
        },
        Vertex {
            position: [tile_size_half, tile_size_half * 0.5, 0.0],
            tex_coords: [uv_size[0], 0.0],
            // tex_coords: [offset[0] +uv_size[0], offset[1] +0.0],
        },
        Vertex {
            position: [-tile_size_half, tile_size_half * 0.5, 0.0],
            tex_coords: [0., 0.],
            // tex_coords: offset ,
        }
    ];
    let indices: [u16; 6] = [
        //front
        0, 1, 2,
        2, 3, 0,
    ];

    let instance_data : Vec<InstanceTileRaw> = vec![];
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
        // texture: texture.into()
    }
}