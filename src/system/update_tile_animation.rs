use specs::{Read, System, WriteStorage};

use crate::components::{Animation, Tile};
use crate::resources::DeltaTime;

pub struct UpdateAnimation;

impl<'a> System<'a> for UpdateAnimation {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Animation>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mut tiles, mut animations) = data;
        use specs::Join;
        for( tile, animation) in (&mut tiles, &mut animations).join(){
            tile.tile_index = animation.run_animation(dt.0).clone();
        }
    }
}