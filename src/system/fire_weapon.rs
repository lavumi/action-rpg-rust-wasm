use specs::{Entities, Read, System, WriteStorage};
use crate::components::tile:: Tile;

pub struct FireWeapon;
impl<'a> System<'a> for FireWeapon {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Tile>,
    );

    fn run(&mut self, (entities, mut tiles): Self::SystemData) {
        let tile = entities.create();

        // 1) Either we insert the component by writing to its storage
        tiles.insert(tile, Tile{
            tile_index: [0,0],
            uv_size: [0.1,0.05],
            position: [0.0,0.0,0.1],
            flip: false,
            atlas: "fx".to_string(),
        }).expect("MakeTileFail!!!");

    }
}