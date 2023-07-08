use specs::{ ReadExpect, System, Write};
use crate::resources::{Camera, Center, TileMapStorage};

pub struct UpdateCamera;

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (
        ReadExpect<'a, Center>,
        Write<'a, Camera>,
        Write<'a, TileMapStorage>,
        // Read<'a, GPUResourceManager>,
        // Read<'a, RenderState>

    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, mut camera,mut tile_map_storage) = data;
        let player_pos = [pos.0, pos.1];
        camera.move_camera(player_pos);
        tile_map_storage.update_tile_grid(player_pos);
    }
}