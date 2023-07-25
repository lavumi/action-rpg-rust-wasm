use specs::{Read, ReadStorage, System, WriteStorage};

use crate::components::{Animation, AttackMaker, Collider, convert_velocity, Direction, Forward, Movable, Player};
use crate::resources::{DeltaTime, InputHandler};

pub struct UpdatePlayer;


fn check_direction(delta: [f32; 2]) -> Direction {
    let direction = [(delta[0] / delta[0].abs()) as i8, (delta[1] / delta[1].abs()) as i8];
    if direction[0] == -1 {
        if direction[1] == -1 {
            Direction::DownLeft
        } else if direction[1] == 0 {
            Direction::Left
        } else {
            Direction::UpLeft
        }
    } else if direction[0] == 0 {
        if direction[1] == -1 {
            Direction::Down
        } else if direction[1] == 0 {
            Direction::None
        } else {
            Direction::Up
        }
    } else {
        if direction[1] == -1 {
            Direction::DownRight
        } else if direction[1] == 0 {
            Direction::Right
        } else {
            Direction::UpRight
        }
    }
}

impl<'a> System<'a> for UpdatePlayer {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, AttackMaker>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Forward>,
        Read<'a, InputHandler>,
        Read<'a, DeltaTime>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            mut attack_maker,
            mut transforms,
            mut animations,
            mut movable,
            mut forwards,
            input_handler,
            dt
        ) = data;

        use specs::Join;

        for (p, atk, physics, animation, mov, forward)
        in (&player, &mut attack_maker, &mut transforms, &mut animations, &mut movable, &mut forwards).join() {
            if mov.0 == false {
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
            if direction != Direction::None && direction != forward.direction {
                forward.direction = direction;
                animation.frame = 0;
            }
            animation.speed = 5.0 / p.speed;

            if input_handler.attack1 {
                movement = [0., 0.];
                animation_index = 3;
                // atk.fire = true;
                mov.0 = false;
            }
            physics.velocity = convert_velocity(movement);

            if animation_index != animation.index {
                animation.index = animation_index;
                animation.frame = 0;
            }
        }
    }
}