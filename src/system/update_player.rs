use specs::{Read, ReadStorage, System, WriteStorage};
use crate::components::player::Player;
use crate::components::tile::Tile;
use crate::resources::delta_time::DeltaTime;
use crate::resources::input_handler::InputHandler;

pub struct UpdatePlayer;

impl<'a> System<'a> for UpdatePlayer {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Tile>,
        Read<'a, InputHandler>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player,mut tiles,input_handler, dt) = data;
        let speed = 2.0;
        use specs::Join;
        for (_, tile) in (&player, &mut tiles).join() {
            let mut current_position = tile.position.clone();
            if input_handler.up {
                current_position[1] += dt.0 * speed;
            }
            if input_handler.down {
                current_position[1] -= dt.0 * speed;
            }
            if input_handler.left {
                current_position[0] -= dt.0 * speed;
                tile.flip = false;
            }
            if input_handler.right {
                current_position[0] += dt.0 * speed;
                tile.flip = true;
            }
            tile.position = current_position;
        }
    }
}