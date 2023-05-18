use specs::{Read, ReadStorage, System, Write, WriteStorage};
use crate::components::mesh::Mesh;
use crate::renderer::{Camera, GPUResourceManager, RenderState};

// gpu_resource_manager: &GPUResourceManager,
// pipeline_manager : &PipelineManager,
pub struct UpdateCamera;

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (
        Write<'a, Camera>,
        Read<'a, GPUResourceManager>,
        Read<'a, RenderState>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (mut camera, gpu_resource_manager,renderer) =data;

        let camera_uniform = camera.update_view_proj();
        let camera_buffer = gpu_resource_manager.get_buffer("camera_matrix");

        renderer.update_camera_buffer(camera_buffer,camera_uniform);
    }
}