use specs::{Read, ReadStorage, System, Write, WriteStorage};

use crate::components::{Animation, Player, Transform};
use crate::renderer::Camera;
use crate::resources::{DeltaTime, InputHandler, TileMapStorage};

pub struct UpdatePlayer;

impl<'a> System<'a> for UpdatePlayer {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Animation>,
        Read<'a, InputHandler>,
        Write<'a, Camera>,
        Write<'a, TileMapStorage>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            mut transforms,
            mut animations,
            input_handler,
            mut camera,
            mut tile_map_storage,
            dt
        ) = data;

        use specs::Join;
        for (p, transform, animation) in (&player, &mut transforms, &mut animations).join() {
            if animation.lock_movement() {
                continue;
            }

            let speed = p.speed;
            let mut movement: [f32; 2] = [0., 0.];
            let mut animation_index: usize = 0;

            if input_handler.up {
                movement[1] += dt.0 * speed;
                animation_index = 1;
            }
            if input_handler.down {
                movement[1] -= dt.0 * speed;
                animation_index = 1;
            }
            if input_handler.left {
                movement[0] -= dt.0 * speed;
                animation_index = 1;
            }
            if input_handler.right {
                movement[0] += dt.0 * speed;
                animation_index = 1;
            }

            if input_handler.attack1 {
                movement = [0., 0.];
                animation_index = 2;
            }

            let direction = transform.move_position(movement);

            animation.change_direction(direction);
            animation.change_animation(animation_index, input_handler.attack1);

            let camera_pos = camera.move_camera(movement);
            tile_map_storage.update_tile_grid(camera_pos);
        }
    }
}