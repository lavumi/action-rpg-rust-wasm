use specs::{Read, System, Write};
use crate::renderer::{Camera, GPUResourceManager, RenderState};
use crate::resources::delta_time::DeltaTime;
use crate::resources::input_handler::InputHandler;

pub struct UpdateCamera;

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (
        Write<'a, Camera>,
        Read<'a, InputHandler>,
        Read<'a, DeltaTime>,
        Read<'a, GPUResourceManager>,
        Read<'a, RenderState>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, input_handler,dt, gpu_resource_manager,renderer) = data;
        let speed = 2.0;
        let mut movement:[f32;2] = [0.,0.];

        if input_handler.up {
            movement[1] += dt.0 * speed;
        }
        if input_handler.down {
            movement[1] -= dt.0 * speed;
        }
        if input_handler.left {
            movement[0] -= dt.0 * speed;
        }
        if input_handler.right {
            movement[0] += dt.0 * speed;
        }

        camera.move_camera(movement);
        let camera_uniform = camera.update_view_proj();
        let camera_buffer = gpu_resource_manager.get_buffer("camera_matrix");
        renderer.update_camera_buffer(camera_buffer,camera_uniform);
    }
}