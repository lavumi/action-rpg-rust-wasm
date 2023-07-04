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

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;


        let (entities, mut attack,mut transforms,dt) = data;
        for(e, attack, transform) in (&entities,&mut attack, &mut transforms).join(){
            let move_delta = attack.update(dt.0);
            _ = transform.move_position(move_delta);

            if attack.is_expired() {
                entities.delete(e).expect("delete bullet fail!!!");
                // continue;
            }
        }


    }
}