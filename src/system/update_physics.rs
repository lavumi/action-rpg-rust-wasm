use specs::{Entities, Entity, Join, ReadExpect, System, WriteExpect, WriteStorage};

use crate::components::{BodyType, Direction, RigidBody, Transform};
use crate::resources::Center;

pub struct UpdatePhysics;

fn check_collision(my_aabb: &[f32; 4], my_delta: &[f32; 2], target_aabb: &[f32; 4]) -> bool {
    let my_delta_aabb = [
        my_aabb[0] + my_delta[0],
        my_aabb[1] + my_delta[0],
        my_aabb[2] + my_delta[1],
        my_aabb[3] + my_delta[1],
    ];
    let side_collision = my_delta_aabb[1] > target_aabb[0] && my_delta_aabb[0] < target_aabb[1];
    let up_down_collision = my_delta_aabb[2] < target_aabb[3] && my_delta_aabb[3] > target_aabb[2];

    if side_collision && up_down_collision {
        return true;
    }
    return false;
}


fn check_collision_direction(my_aabb: &[f32; 4], target_aabb: &[f32; 4]) -> Direction {
    let lt_rt_collision = my_aabb[1] - target_aabb[0] > 0. && target_aabb[1] - my_aabb[0] > 0.;
    let up_dn_collision = my_aabb[3] - target_aabb[2] > 0. && target_aabb[3] - my_aabb[2] > 0.;

    if !lt_rt_collision || !up_dn_collision {
        return Direction::None;
    }


    let lt_check = my_aabb[0] - target_aabb[0] >= 0.;
    let rt_check = my_aabb[1] - target_aabb[1] >= 0.;
    let dn_check = my_aabb[2] - target_aabb[2] >= 0.;
    let up_check = my_aabb[3] - target_aabb[3] >= 0.;


    if lt_check && rt_check && !dn_check && !up_check {
        return Direction::UpLeft;
    } else if (lt_check & !rt_check) && !dn_check && !up_check {
        return Direction::Up;
    } else if !lt_check && !rt_check && !dn_check && !up_check {
        return Direction::UpRight;
    } else if !lt_check && !rt_check && (!dn_check & up_check) {
        return Direction::Right;
    } else if !lt_check && !rt_check && dn_check && up_check {
        return Direction::DownRight;
    } else if (lt_check & !rt_check) && dn_check && up_check {
        return Direction::Down;
    } else if lt_check && rt_check && dn_check && up_check {
        return Direction::DownLeft;
    } else if lt_check && rt_check && (!dn_check & up_check) {
        return Direction::Right;
    }

    panic!("check direction Error!!!! {} {} {} {}", lt_check, rt_check, dn_check, up_check);
}

fn get_aabb(physic: &RigidBody, transform: &Transform) -> [f32; 4] {
    [
        transform.position[0] + physic.aabb_offset[0],
        transform.position[0] + physic.aabb_offset[1],
        transform.position[1] + physic.aabb_offset[2],
        transform.position[1] + physic.aabb_offset[3],
    ]
}

struct ColliderData {
    entity: Entity,
    aabb: [f32; 4],
    velocity: [f32; 2],
    body_type: BodyType,
}

impl<'a> System<'a> for UpdatePhysics {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, RigidBody>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Center>,
    );

    fn run(&mut self, (entities, mut physics, mut transforms, player, mut player_pos): Self::SystemData) {
        let colliders = (&entities, &physics, &transforms)
                .join()
                .filter(|(_, p, _)|
                        p.is_trigger == false
                )
                .map(|(e, p, t)| {
                    ColliderData {
                        entity: e,
                        aabb: get_aabb(p, t),
                        velocity: p.velocity,
                        body_type: p.body_type,
                    }
                })
                .collect::<Vec<_>>();


        for (e, p, t) in (&entities, &mut physics, &mut transforms).join() {
            let aabb = get_aabb(p, t);
            let mut velocity = p.velocity;
            for col in &colliders {
                //자기 자신과 똑같은 것 체크 안함
                if e == col.entity { continue; }
                if p.is_trigger == true { continue; }
                //비교할 대상 충돌체
                let t_aabb = &col.aabb;


                let collision_direction = check_collision_direction(&aabb, t_aabb);

                let lt_rt_force = match collision_direction {
                    Direction::Left | Direction::DownLeft | Direction::UpLeft => {
                        col.velocity[0].max(0.)
                    }
                    Direction::UpRight | Direction::Right | Direction::DownRight => {
                        col.velocity[0].min(0.)
                    }
                    _ => { 0. }
                };


                let up_dn_force = match collision_direction {
                    Direction::Up | Direction::UpRight | Direction::UpLeft => {
                        col.velocity[1].min(0.)
                    }
                    Direction::DownLeft | Direction::Down | Direction::DownRight => {
                        col.velocity[1].max(0.)
                    }
                    _ => { 0. }
                };


                velocity[0] += lt_rt_force;
                velocity[1] += up_dn_force;
            }


            t.position[0] += velocity[0];
            t.position[1] += velocity[1];
            t.position[2] = 1.0 - t.position[1] / 10000.0;

            if e == *player {
                player_pos.0 = t.position[0];
                player_pos.1 = t.position[1];
            }
        }
    }
}