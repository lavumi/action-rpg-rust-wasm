use specs::{Entities, Read, System, WriteStorage};

use crate::components::{Animation, Attack, AttackMaker, Tile, Transform};
use crate::resources::DeltaTime;

pub struct FireWeapon;
impl<'a> System<'a> for FireWeapon {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AttackMaker>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Animation>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut attack_makers, mut transforms, mut tiles, mut attacks, mut animation, dt): Self::SystemData) {
        use specs::Join;
        let mut bullets_to_fire :
            Vec<(
            [f32;3],
            [i8;2],
            u8)> = vec![];
        for (attack_maker, transform) in (&mut attack_makers, &transforms).join() {

            if attack_maker.update(dt.0) == false {
                continue;
            }

            bullets_to_fire.push((transform.position,transform.direction, 1));
        }


        for data in bullets_to_fire {
            let bullet = entities.create();
            transforms.insert(bullet,
                              Transform::new(data.0, [1.0, 1.0]),
            ).expect("MakeTileFail!!!");

            tiles.insert(bullet, Tile {
                tile_index: [0, 0],
                uv_size: [0.1, 0.05],
                atlas: "fx".to_string(),
            }).expect("MakeTileFail!!!");

            attacks.insert(bullet,
                           Attack::new(
                               1.0,
                               [data.1[0] as f32 * 10.0, data.1[1] as f32 * 10.0]),
            ).expect("MakeTileFail!!!");

            animation.insert(bullet,
                             Animation::new(
                                 vec![0, 1, 2, 1],
                                 10,
                                 0.2),
            ).expect("MakeTileFail!!!");
        }
    }
}