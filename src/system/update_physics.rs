use specs::{Read, ReadStorage, System, Write, WriteStorage};

use crate::components::{Physics, Transform};

pub struct UpdatePhysics;


impl<'a> System<'a> for UpdatePhysics {
    type SystemData = (
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut physics,
            mut transforms
        ) = data;

        use specs::Join;
        for (p, t) in (&mut physics, &mut transforms).join() {
            //todo check collision

            t.move_position(p.get_velocity());
        }
    }
}