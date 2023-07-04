use std::cmp::max;

use specs::{Entities, Entity, Join, System, WriteStorage};

use crate::components::{Physics, Transform};

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

struct CollisionData {
    entity: Entity,
    aabb: [f32; 4],
}


impl<'a> System<'a> for UpdatePhysics {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut physics, mut transforms) = data;
        let collisions = (&entities, &physics, &transforms)
            .join()
            .map(|(e, p, t)| {
                CollisionData {
                    entity: e,
                    aabb: p.get_aabb(t.position),
                }
            })
            .collect::<Vec<_>>();


        for (e, p, t) in (&entities, &mut physics, &mut transforms).join() {
            let aabb = p.get_aabb(t.position);
            let mut velocity = p.get_velocity();
            for col in &collisions {


                //자기 자신과 똑같은 것 체크 안함
                if e == col.entity { continue; }

                let t_aabb = &col.aabb;


                let collapse_velocity: [f32; 2] = //=[0.,0.];
                    if check_collision(&aabb, &[0., 0.], t_aabb) {
                        let my_center = [
                            (aabb[0] + aabb[1]) * 0.5,
                            (aabb[2] + aabb[3]) * 0.5
                        ];
                        let t_center = [
                            (t_aabb[0] + t_aabb[1]) * 0.5,
                            (t_aabb[2] + t_aabb[3]) * 0.5
                        ];

                        [
                            if my_center[0] < t_center[0] { t_aabb[0] - aabb[1] } else { t_aabb[1] - aabb[0] },
                            if my_center[1] < t_center[1] { t_aabb[2] - aabb[3] } else { t_aabb[3] - aabb[2] },
                        ]
                    } else {
                        [0., 0.]
                    };

                let collision_left_right = check_collision(&aabb, &[velocity[0], 0.], t_aabb);
                let collision_up_down = check_collision(&aabb, &[0., velocity[1]], t_aabb);


                if collision_left_right {
                    velocity[0] = collapse_velocity[0];
                }

                if collision_up_down {
                    velocity[1] = collapse_velocity[1];
                }

                if collision_up_down || collision_left_right {
                    break;
                }
            }
            t.move_position(velocity);
        }
    }
}