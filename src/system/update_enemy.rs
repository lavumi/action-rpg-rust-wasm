use specs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::{Animation, Collider, convert_velocity, Direction, direction_to_f32_array, Enemy, Forward, Movable, Transform};
use crate::resources::{Center, DeltaTime};

pub struct UpdateEnemy;


fn get_direction(enemy_pos: [f32; 3], player_pos: [f32; 2]) -> Direction {
    let delta = [player_pos[0] - enemy_pos[0], player_pos[1] - enemy_pos[1]];
    let tan = delta[1] / delta[0];

    if tan >= 2.41421356 || tan <= -2.41421356 {
        if delta[1] > 0.0 {
            return Direction::Up;
        } else {
            return Direction::Down;
        }
    } else if tan <= 2.41421356 && tan >= 0.41421356 {
        if delta[1] > 0.0 {
            return Direction::UpRight;
        } else {
            return Direction::DownLeft;
        }
    } else if tan <= 0.41421356 && tan >= -0.41421356 {
        if delta[0] > 0.0 {
            return Direction::Right;
        } else {
            return Direction::Left;
        }
    } else if tan <= -0.41421356 && tan >= -2.41421356 {
        if delta[1] < 0.0 {
            return Direction::DownRight;
        } else {
            return Direction::UpLeft;
        }
    } else {
        panic!("set_direction error !!!")
    }
}

impl<'a> System<'a> for UpdateEnemy {
    type SystemData = (
        ReadExpect<'a, Center>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Forward>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            pos,
            tr,
            mut enemy,
            mut physics,
            mut animations,
            mut movable,
            mut forwards,
            dt
        ) = data;
        let player_pos = [pos.0, pos.1];
        for (e, transform, p, animation, mov, forward)
        in (&mut enemy, &tr, &mut physics, &mut animations, &mut movable, &mut forwards).join() {
            if mov.0 == false {
                e.tick = 99.0;
                continue;
            }
            e.tick += dt.0;
            if e.tick > 1.0 / e.speed {
                e.tick = 0.;
                let player_distance = (player_pos[0] - transform.position[0]).powi(2) + (player_pos[1] - transform.position[1]).powi(2);
                let animation_index: usize =
                        if player_distance < 2.0 {
                            mov.0 = false;
                            2
                        } else if player_distance < 90.0 {
                            1
                        } else {
                            p.velocity = [0., 0.];
                            0
                        };

                if animation_index != animation.index {
                    animation.index = animation_index;
                    animation.frame = 0;
                    animation.speed = 5.0 / e.speed;
                }


                if animation_index != 0 {
                    let direction = get_direction(transform.position, player_pos);
                    if direction != Direction::None && direction != forward.direction {
                        forward.direction = direction;
                        animation.frame = 0;
                    }
                }
            }

            let f32_dir = direction_to_f32_array(forward.direction);
            let velocity = if animation.index != 1 { [0., 0.] } else { [f32_dir[0] * e.speed * dt.0, f32_dir[1] * e.speed * dt.0] };
            p.velocity = convert_velocity(velocity);
        }
    }
}