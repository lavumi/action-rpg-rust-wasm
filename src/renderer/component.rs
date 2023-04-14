use wgpu::BindGroup;
use crate::renderer::sprite::Sprite;

pub struct RenderComponent {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) instance_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,
    pub(crate) num_instances: u32,
    pub(crate) sprite : Sprite
}