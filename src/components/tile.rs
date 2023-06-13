use specs::{Component, VecStorage};

pub struct Tile {
    pub(crate) tile_index: [u8;2],
    //todo 타일 데이터에 uv 사이즈를 넣을 필요는 없을거 같은데... texture에서 들고오는 방법으로 생각해보자
    pub(crate) uv_size: [f32;2],
    pub(crate) atlas: String
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}

impl Tile {
    pub fn get_uv(&self) ->  [f32; 2] {
        let uv = [
            self.uv_size[0] * (self.tile_index[0] as f32) ,
            self.uv_size[1] * (self.tile_index[1] as f32)
        ];
        uv
    }
}


pub struct TileInstance {
    pub(crate) uv: cgmath::Vector2<f32>,
    pub(crate) model_matrix: cgmath::Matrix4<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceTileRaw {
    pub(crate) uv: [f32; 2],
    pub(crate) model: [[f32; 4]; 4],
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