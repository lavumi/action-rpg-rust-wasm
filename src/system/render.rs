use specs::{ Read, System};
use crate::renderer::{GPUResourceManager, PipelineManager, RenderState};

// gpu_resource_manager: &GPUResourceManager,
// pipeline_manager : &PipelineManager,
pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        Read<'a, GPUResourceManager>,
        Read<'a, PipelineManager>,
        Read<'a, RenderState>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gpu_resource_manager, pipeline_manager, renderer) =data;
        let _ = renderer.render(&*gpu_resource_manager, &*pipeline_manager);
    }
}