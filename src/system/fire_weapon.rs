use specs::{Entities, Read, ReadStorage, System, WriteStorage};

use crate::components::{Animation, Attack, AttackMaker, BodyType, Collider, Direction, direction_to_f32_array, Forward, Tile, Transform};
use crate::resources::DeltaTime;

pub struct FireWeapon;

struct BulletData {
    start_position: [f32; 3],
    direction: Direction,
    bullet_type: u8,
}

impl<'a> System<'a> for FireWeapon {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AttackMaker>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Animation>,
        ReadStorage<'a, Forward>,
        Read<'a, DeltaTime>,
    );

    #[allow(unused_variables)]
    fn run(&mut self, (entities, mut attack_makers, mut transforms, mut tiles, mut attacks, mut physics, mut animation, forwards, dt): Self::SystemData) {
        use specs::Join;
        let mut bullets_to_fire:
                Vec<BulletData> = vec![];
        for (attack_maker, transform, anim, forward) in (&mut attack_makers, &transforms, &animation, &forwards).join() {
            if attack_maker.fire == false {
                continue;
            }
            attack_maker.fire = false;
            bullets_to_fire.push(BulletData {
                start_position: transform.position,
                direction: forward.direction,
                bullet_type: 1,
            });
        }

        for bullet_data in bullets_to_fire {
            let speed: f32 = 5.0;
            let mut movement = direction_to_f32_array(bullet_data.direction);

            movement[0] *= 2.0 * speed;
            movement[1] *= speed;
            entities.build_entity()
                    .with(
                        Transform::new(bullet_data.start_position, [1.0, 1.0]),
                        &mut transforms)
                    .with(
                        Tile {
                            tile_index: [bullet_data.direction as u8, 0],
                            uv_size: [0.125, 0.333333],
                            atlas: "projectiles".to_string(),
                        },
                        &mut tiles)
                    .with(
                        Attack {
                            duration: 1.0,
                            dt: 0.0,
                            movement,
                        },
                        &mut attacks)
                    .with(Collider {
                        aabb_offset: [-0.25, 0.25, -0.25, 0.25],
                        velocity: [0., 0.],
                        is_trigger: true,
                        body_type: BodyType::Dynamic,
                    }, &mut physics)
                .build();
        }
    }
}