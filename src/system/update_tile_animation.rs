use specs::{Read, ReadStorage, System, WriteStorage};

use crate::components::{Animation, Tile, Transform};
use crate::resources::DeltaTime;

pub struct UpdateAnimation;

impl<'a> System<'a> for UpdateAnimation {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Animation>,
        ReadStorage<'a, Transform>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mut tiles, mut animations, transforms) = data;
        use specs::Join;
        for (tile, animation, transform) in (&mut tiles, &mut animations, &transforms).join() {
            tile.tile_index = animation.run_animation(dt.0).clone();
        }
    }
}