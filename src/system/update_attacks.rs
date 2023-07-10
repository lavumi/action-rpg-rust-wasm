use specs::{Entities, Read, System, WriteStorage};

use crate::components::{Attack, Transform};
use crate::resources::DeltaTime;

pub struct UpdateAttack;

impl<'a> System<'a> for UpdateAttack {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Transform>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut attack, mut transforms, dt): Self::SystemData) {
        use specs::Join;
        // let (entities, mut attack,mut transforms,dt) = data;
        for (e, attack, transform) in (&entities, &mut attack, &mut transforms).join() {
            attack.dt += dt.0;
            if attack.duration <= attack.dt {
                entities.delete(e).expect("delete bullet fail!!!");
                continue;
            }

            let move_delta = [
                attack.movement[0] * dt.0,
                attack.movement[1] * dt.0,
            ];

            transform.move_position(move_delta);
        }
    }
}