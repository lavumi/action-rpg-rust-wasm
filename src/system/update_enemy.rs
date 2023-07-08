use specs::{Read, ReadStorage, System, WriteStorage};

use crate::components::{Animation, Enemy, Physics, Player, Transform};
use crate::resources::{DeltaTime, InputHandler};

pub struct UpdateEnemy;


fn get_direction(enemy_pos: [f32; 3], player_pos: [f32; 3]) -> ([f32; 2], u8) {
    let delta = [player_pos[0] - enemy_pos[0], player_pos[1] - enemy_pos[1]];
    let tan = delta[1] / delta[0];

    if tan >= 2.41421356 || tan <= -2.41421356 {
        if delta[1] > 0.0 {
            ([0., 1.], 2)
        } else {
            ([0., -1.], 6)
        }
    } else if tan <= 2.41421356 && tan >= 0.41421356 {
        if delta[1] > 0.0 {
            ([1., 1.], 3)
        } else {
            ([-1., -1.], 7)
        }
    } else if tan <= 0.41421356 && tan >= -0.41421356 {
        if delta[0] > 0.0 {
            ([1., 0.], 4)
        } else {
            ([-1., 0.], 0)
        }
    } else if tan <= -0.41421356 && tan >= -2.41421356 {
        if delta[1] < 0.0 {
            ([1., -1.], 5)
        } else {
            ([-1., 1.], 1)
        }
    } else {
        panic!("set_direction error !!!")
    }
}

impl<'a> System<'a> for UpdateEnemy {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Animation>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            tr,
            mut enemy,
            mut physics,
            mut animations,
            dt
        ) = data;

        use specs::Join;

        let player_pos = (&player, &tr).join().map(|(_, t)| { t.position }).collect::<Vec<_>>()[0];

        for (e, transform, physics, animation) in (&mut enemy, &tr, &mut physics, &mut animations).join() {
            if animation.lock_movement() {
                e.reset_tick();
                continue;
            }

            if e.update_tick(dt.0) == false {
                continue;
            }

            let player_distance = (player_pos[0] - transform.position[0]).powi(2) + (player_pos[1] - transform.position[1]).powi(2);


            let animation_index: usize =
                if player_distance < 2.0 {
                    2
                } else if player_distance < 40.0 {
                    1
                } else {
                    0
                };


            animation.set_speed(e.speed);
            animation.change_animation(animation_index, player_distance < 2.0);

            if animation_index == 0 {
                continue;
            }


            let direction = get_direction(transform.position, player_pos);
            animation.change_direction(direction.1);


            let velocity = if animation_index != 1 { [0., 0.] } else { [direction.0[0] * e.speed * dt.0, direction.0[1] * e.speed * dt.0] };
            physics.set_velocity(velocity);
        }
    }
}