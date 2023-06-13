use specs::{Entities, Read, ReadStorage, System, WriteStorage};
use crate::components::attack::Attack;
use crate::components::transform::Transform;
use crate::resources::delta_time::DeltaTime;

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
            if attack.is_expired() {
                entities.delete(e);
                continue;
            }

            // attack.dt += delta_time;
            let movement:[f32;2] = [
                dt.0 * attack.movement[0],
                dt.0 * attack.movement[1]
            ];
            attack.dt += dt.0;
            transform.move_position(movement);
        }


    }
}