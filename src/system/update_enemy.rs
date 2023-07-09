use specs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::{Animation, Direction, Enemy, Movable, Physics, Transform};
use crate::resources::{DeltaTime, Center};

pub struct UpdateEnemy;


fn get_direction(enemy_pos: [f32; 3], player_pos: [f32; 2]) -> ([f32; 2], Direction) {
    let delta = [player_pos[0] - enemy_pos[0], player_pos[1] - enemy_pos[1]];
    let tan = delta[1] / delta[0];

    if tan >= 2.41421356 || tan <= -2.41421356 {
        if delta[1] > 0.0 {
            ([0., 1.], Direction::Up)
        } else {
            ([0., -1.], Direction::Down)
        }
    } else if tan <= 2.41421356 && tan >= 0.41421356 {
        if delta[1] > 0.0 {
            ([1., 1.], Direction::UpRight)
        } else {
            ([-1., -1.], Direction::DownLeft)
        }
    } else if tan <= 0.41421356 && tan >= -0.41421356 {
        if delta[0] > 0.0 {
            ([1., 0.], Direction::Right)
        } else {
            ([-1., 0.], Direction::Left)
        }
    } else if tan <= -0.41421356 && tan >= -2.41421356 {
        if delta[1] < 0.0 {
            ([1., -1.], Direction::DownRight)
        } else {
            ([-1., 1.], Direction::UpLeft)
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
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Movable>,
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
            dt
        ) = data;
        let player_pos = [pos.0, pos.1];
        for (e, transform, physics, animation, mov)
        in (&mut enemy, &tr, &mut physics, &mut animations, &mut movable).join() {
            if mov.0 == false {
                e.reset_tick();
                continue;
            }

            if e.update_tick(dt.0) == false {
                continue;
            }

            let player_distance = (player_pos[0] - transform.position[0]).powi(2) + (player_pos[1] - transform.position[1]).powi(2);


            let animation_index: usize =
                if player_distance < 2.0 {
                    mov.0 = false;
                    2
                } else if player_distance < 40.0 {
                    1
                } else {
                    0
                };


            animation.speed = 5.0 / e.speed;

            if animation_index != animation.index {
                animation.index = animation_index;
                animation.frame = 0;
            }

            if animation_index == 0 {
                physics.set_velocity([0.,0.]);
                continue;
            }


            let direction = get_direction(transform.position, player_pos);
            if direction.1 != Direction::None && direction.1 != animation.direction {
                animation.direction = direction.1.clone();//todo ??? clone 이 대체 왜 필요한거야
                animation.frame = 0;
            }


            let velocity = if animation_index != 1 { [0., 0.] } else { [direction.0[0] * e.speed * dt.0, direction.0[1] * e.speed * dt.0] };
            physics.set_velocity(velocity);
        }
    }
}