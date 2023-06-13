use specs::{Read, ReadStorage, System, Write, WriteStorage};
use crate::components::player::Player;
use crate::components::transform::Transform;
use crate::renderer::Camera;
use crate::resources::delta_time::DeltaTime;
use crate::resources::input_handler::InputHandler;
use crate::resources::tile_map_storage::TileMapStorage;

pub struct UpdatePlayer;

impl<'a> System<'a> for UpdatePlayer {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler>,
        Write<'a, Camera>,
        Write<'a, TileMapStorage>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player,mut transforms,input_handler,mut camera, mut tile_map_storage, dt) = data;
        use specs::Join;

        for (p, transform) in (&player, &mut transforms).join() {
            let speed = p.speed;
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
            transform.move_position(movement);
            //todo 이건 플레이어가 하나뿐 일때만 작동하는 멍청한 코드인데...
            //좀 스마트한 방법 없을까요
            let camera_pos = camera.move_camera(movement);
            tile_map_storage.update_tile_grid(camera_pos);
        }
    }
}