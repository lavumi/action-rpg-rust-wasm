use specs::{Read, ReadStorage, System};
use crate::components::mesh::Mesh;
use crate::renderer::{GPUResourceManager, PipelineManager, RenderState};

// gpu_resource_manager: &GPUResourceManager,
// pipeline_manager : &PipelineManager,
pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        Read<'a, GPUResourceManager>,
        Read<'a, PipelineManager>,
        Read<'a, RenderState>,
        ReadStorage<'a, Mesh>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (gpu_resource_manager, pipeline_manager, renderer, meshes) =data;
        for mesh in meshes.join() {
            let _ = renderer.render(&*gpu_resource_manager, &*pipeline_manager, mesh);
            // println!("Hello, {:?}", &position);
        }
    }
}