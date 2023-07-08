use specs::{Read, ReadStorage, System, Write, WriteExpect, WriteStorage};

use crate::components::{Animation, AttackMaker, Physics, Player};
use crate::resources::{DeltaTime, InputHandler, Center};

pub struct UpdatePlayer;


fn check_direction(delta: [f32; 2]) -> u8 {
    let direction = [(delta[0] / delta[0].abs()) as i8, (delta[1] / delta[1].abs()) as i8];
    if direction[0] == -1 {
        if direction[1] == -1 { 7 } else if direction[1] == 0 { 0 } else { 1 }
    } else if direction[0] == 0 {
        if direction[1] == -1 { 6 } else if direction[1] == 0 {
            // panic!("direction is both 0");
            9
        } else { 2 }
    } else {
        if direction[1] == -1 { 5 } else if direction[1] == 0 { 4 } else { 3 }
    }
}

impl<'a> System<'a> for UpdatePlayer {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, AttackMaker>,
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Animation>,
        Read<'a, InputHandler>,
        Read<'a, DeltaTime>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            mut attack_maker,
            mut transforms,
            mut animations,
            input_handler,
            dt
        ) = data;

        use specs::Join;

        for (p, atk, physics, animation) in (&player, &mut attack_maker, &mut transforms, &mut animations).join() {
            if animation.lock_movement() {
                continue;
            }
            let speed = p.speed;
            let mut movement: [f32; 2] = [0., 0.];
            let mut animation_index: usize = 0;

            if input_handler.up {
                movement[1] += dt.0 * speed;
                animation_index = 1;
            }
            if input_handler.down {
                movement[1] -= dt.0 * speed;
                animation_index = 1;
            }
            if input_handler.left {
                movement[0] -= dt.0 * speed;
                animation_index = 1;
            }
            if input_handler.right {
                movement[0] += dt.0 * speed;
                animation_index = 1;
            }

            let direction = check_direction(movement);
            animation.change_direction(direction);
            animation.set_speed(p.speed);


            if input_handler.attack1 {
                movement = [0., 0.];
                animation_index = 6;
                atk.set_fire();
            }
            physics.set_velocity(movement);

            animation.change_animation(animation_index, input_handler.attack1);
        }
    }
}