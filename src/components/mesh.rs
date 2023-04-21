use std::sync::Arc;
use specs::{Component, VecStorage};
use crate::renderer::vertex::{Instance, Vertex};

// #[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub(crate) vertex_buffer: Option<Arc<wgpu::Buffer>>,
    pub(crate) index_buffer: Arc<wgpu::Buffer>,
    pub(crate) index_count: usize,

    // pub(crate) instances: Vec<Instance>,
}

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}

impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubMesh")
            .field("vertices", &self.vertices)
            .field("indices", &self.indices)
            .field("index_count", &self.index_count)
            .finish()
    }
}