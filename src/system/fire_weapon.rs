use specs::{Entities, Read, ReadStorage, System, WriteStorage};
use crate::components::animation::Animation;
use crate::components::attack::Attack;
use crate::components::attack_maker::AttackMaker;
use crate::components::tile:: Tile;
use crate::resources::delta_time::DeltaTime;

pub struct FireWeapon;
impl<'a> System<'a> for FireWeapon {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AttackMaker>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Animation>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut attack_makers, mut tiles, mut attacks, mut animation, dt): Self::SystemData) {
        use specs::Join;
        let mut bullets_to_fire : Vec<([f32;3], u8)> = vec![];
        for (attack_maker, tile) in (&mut attack_makers, &tiles).join() {

            if attack_maker.update(dt.0) == false {
                continue;
            }

            bullets_to_fire.push((tile.position, 1));
        }


        for data in bullets_to_fire {
            let bullet = entities.create();
            // 1) Either we insert the component by writing to its storage
            tiles.insert(bullet, Tile{
                tile_index: [0,0],
                uv_size: [0.1,0.05],
                position: [data.0[0],data.0[1],0.1],
                flip: false,
                atlas: "fx".to_string(),
            }).expect("MakeTileFail!!!");

            attacks.insert(bullet, Attack{
                duration: 0.0,
                dt: 0.0,
                movement: [10., 0.],
            }).expect("MakeTileFail!!!");

            animation.insert( bullet,
                              Animation::new(
                                  vec![[0, 10], [1, 10], [2, 10], [1, 10]],
                                  0.2))
                .expect("MakeTileFail!!!");
        }

        // for attack_maker in (&mut attack_makers).join() {
        //     if attack_maker.update(dt.0) == false {
        //         continue;
        //     }
        //
        //     let bullet = entities.create();
        //     // 1) Either we insert the component by writing to its storage
        //     tiles.insert(bullet, Tile{
        //         tile_index: [0,0],
        //         uv_size: [0.1,0.05],
        //         position: [0.0,0.0,0.1],
        //         flip: false,
        //         atlas: "fx".to_string(),
        //     }).expect("MakeTileFail!!!");
        //
        //     attacks.insert(bullet, Attack{
        //         duration: 0.0,
        //         dt: 0.0,
        //         movement: [10., 0.],
        //     }).expect("MakeTileFail!!!");
        // }



    }
}