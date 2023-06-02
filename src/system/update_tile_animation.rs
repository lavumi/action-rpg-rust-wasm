use specs::{Read, System, WriteStorage};
use crate::components::animation::Animation;
use crate::components::tile::Tile;
use crate::resources::delta_time::DeltaTime;

pub struct UpdateTileAnimation;

impl<'a> System<'a> for UpdateTileAnimation {
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