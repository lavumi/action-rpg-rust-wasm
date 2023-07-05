use std::borrow::BorrowMut;

use specs::{Entities, Read, System, WriteStorage};

use crate::components::{Animation, Attack, AttackMaker, Physics, Tile, Transform};
use crate::resources::DeltaTime;

pub struct FireWeapon;

struct BulletData {
    start_position: [f32; 3],
    movement: [i8; 2],
    direction: u8,
    bullet_type: u8,
}

impl<'a> System<'a> for FireWeapon {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AttackMaker>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Animation>,
        Read<'a, DeltaTime>,
    );

    #[allow(unused_variables)]
    fn run(&mut self, (entities, mut attack_makers, mut transforms, mut tiles, mut attacks, mut physics, mut animation, dt): Self::SystemData) {
        use specs::Join;
        let mut bullets_to_fire:
            Vec<BulletData> = vec![];
        for (attack_maker, transform) in (&mut attack_makers, &transforms).join() {
            if attack_maker.get_fire_condition() == false {
                continue;
            }
            attack_maker.fire_finished();
            bullets_to_fire.push(BulletData {
                start_position: transform.position,
                movement: transform.direction,
                direction: transform.get_direction(),
                bullet_type: 1,
            });
        }

        for bullet_data in bullets_to_fire {
            entities.build_entity()
                .with(
                    Transform::new(bullet_data.start_position, [1.0, 1.0]),
                    transforms.borrow_mut())
                .with(
                    Tile {
                        tile_index: [bullet_data.direction, 0],
                        uv_size: [0.125, 0.333333],
                        atlas: "projectiles".to_string(),
                    },
                    tiles.borrow_mut())
                .with(
                    Attack::new(1.0, [bullet_data.movement[0] as f32 * 10.0, bullet_data.movement[1] as f32 * 5.0]),
                    attacks.borrow_mut())
                .build();
        }
    }
}