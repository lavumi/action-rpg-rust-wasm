use specs::{Read, ReadStorage, System, WriteStorage};
use crate::components::attack::Attack;
use crate::components::tile::Tile;
use crate::resources::delta_time::DeltaTime;

pub struct UpdateAttack;

impl<'a> System<'a> for UpdateAttack {
    type SystemData = (
        ReadStorage<'a, Attack>,
        WriteStorage<'a, Tile>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;


        let ( attack,mut  tiles,dt) = data;
        for( attack, tile) in (&attack,&mut tiles).join(){
            // attack.dt += delta_time;
            let movement:[f32;2] = [
                dt.0 * attack.movement[0],
                dt.0 * attack.movement[1]
            ];
            tile.move_tile(movement);
        }
    }
}