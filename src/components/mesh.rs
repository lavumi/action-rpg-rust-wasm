use specs::{Component, VecStorage};

// #[derive(Debug)]
pub struct Mesh {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) instance_buffer: Option<wgpu::Buffer>,
    pub(crate) num_indices: u32,
    pub(crate) num_instances: u32,
    // pub(crate) instances: Vec<Instance>,
    pub(crate) texture: String
}

impl Mesh {
    pub fn replace_instance(&mut self, buffer: wgpu::Buffer , num_instance : u32){
        self.instance_buffer = Some(buffer);
        self.num_instances = num_instance;
    }
}


impl Component for Mesh {
    type Storage = VecStorage<Self>;
}

impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubMesh")
            // .field("vertices", &self.vertices)
            // .field("indices", &self.indices)
            .field("index_count", &self.num_indices)
            .field("instance_count", &self.num_instances)
            .finish()
    }
}