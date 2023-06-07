use cgmath::One;
use specs::{Component, VecStorage};

pub struct Tile {
    pub(crate) tile_index: [u8;2],
    pub(crate) uv_size: [f32;2],
    pub(crate) position: [f32;3],
    pub(crate) flip: bool,
    pub(crate) texture: String
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}

impl Tile {
    pub fn move_tile(&mut self ,delta: [f32;2]){
        self.position[0] += delta[0];
        self.position[1] += delta[1];

        if delta[0] > 0.0 {
            self.flip = true;
        }
        else if delta[0] < 0.0 {
            self.flip = false;
        }
    }

    pub fn to_tile_raw(&self) -> InstanceTileRaw {
        let uv = [
            self.uv_size[0] * (self.tile_index[0] as f32) ,
            self.uv_size[1] * (self.tile_index[1] as f32)
        ];
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
        InstanceTileRaw {
            model,
            uv
        }
    }
}


pub struct TileInstance {
    pub(crate) uv: cgmath::Vector2<f32>,
    pub(crate) model_matrix: cgmath::Matrix4<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceTileRaw {
    uv: [f32; 2],
    model: [[f32; 4]; 4],
}

impl TileInstance {
    pub fn to_tile_raw(&self) -> InstanceTileRaw {
        InstanceTileRaw {
            model: self.model_matrix.into(),
            uv: self.uv.into()
        }
    }
}

impl InstanceTileRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceTileRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 14]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}