use specs::{Entities, Read, System, WriteStorage};

use crate::components::{Attack, Collider};
use crate::resources::DeltaTime;

pub struct UpdateAttack;

impl<'a> System<'a> for UpdateAttack {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Collider>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut attack, mut physics, dt): Self::SystemData) {
        use specs::Join;
        // let (entities, mut attack,mut transforms,dt) = data;
        for (e, attack, physic) in (&entities, &mut attack, &mut physics).join() {
            attack.dt += dt.0;
            if attack.duration <= attack.dt {
                entities.delete(e).expect("delete bullet fail!!!");
                continue;
            }

            physic.velocity = [
                attack.movement[0] * dt.0,
                attack.movement[1] * dt.0,
            ];
        }
    }
}