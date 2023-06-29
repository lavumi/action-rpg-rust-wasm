use log::info;
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

    // let direction =
    //
    // let movement = [dir_x, dir_y];
    //
    // return (movement, direction);
}

impl<'a> System<'a> for UpdateEnemy {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Animation>,
        Read<'a, InputHandler>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            tr,
            mut enemy,
            mut physics,
            mut animations,
            input_handler,
            dt
        ) = data;

        use specs::Join;

        let player_pos = (&player, &tr).join().map(|(_, t)| { t.position }).collect::<Vec<_>>()[0];

        for (e, transform, physics, animation) in (&mut enemy, &tr, &mut physics, &mut animations).join() {
            if animation.lock_movement() {
                continue;
            }

            if e.update_tick(dt.0) == false {
                continue;
            }

            let animation_index: usize = 1;

            let direction = get_direction(transform.position, player_pos);


            animation.change_direction(direction.1);
            animation.set_speed(e.speed);

            let velocity = [direction.0[0] * e.speed * dt.0, direction.0[1] * e.speed * dt.0];
            physics.set_velocity(velocity);
            animation.change_animation(animation_index, input_handler.attack1);
        }
    }
}