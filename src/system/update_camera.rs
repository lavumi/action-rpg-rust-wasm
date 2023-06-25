use specs::{Read, ReadStorage, System, Write};
use crate::components::{Player, Transform};
use crate::renderer::{Camera, GPUResourceManager, RenderState};
use crate::resources::TileMapStorage;

pub struct UpdateCamera;

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        Write<'a, Camera>,
        Write<'a, TileMapStorage>,
        Read<'a, GPUResourceManager>,
        Read<'a, RenderState>

    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, transforms, mut camera,mut tile_map_storage, gpu_resource_manager,renderer) = data;
        use specs::Join;
        let player_pos = (&player, &transforms).join().map( |(_, t)|{ t.position}).collect::<Vec<_>>()[0];

        camera.move_camera([player_pos[0], player_pos[1]]);
        tile_map_storage.update_tile_grid([player_pos[0], player_pos[1]]);

        let camera_uniform = camera.update_view_proj();
        let camera_buffer = gpu_resource_manager.get_buffer("camera_matrix");
        renderer.update_camera_buffer(camera_buffer,camera_uniform);
    }
}