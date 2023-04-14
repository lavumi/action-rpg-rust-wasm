use wgpu::BindGroup;

pub struct RenderComponent {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) instance_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,
    pub(crate) num_instances: u32,
    pub(crate) diffuse_bind_group : BindGroup
}