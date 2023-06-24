use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use crate::components::{Physics, Transform};

pub struct UpdatePhysics;

fn check_collision( a :&[f32;4], b:&[f32;4]) -> bool{
    if a[1] < b[0] { return false; }
    if a[3] < b[2] { return false; }
    if a[0] > b[1] { return false; }
    if a[2] > b[3] { return false; }

    return true;
}


impl<'a> System<'a> for UpdatePhysics {
    type SystemData = (
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut physics, mut transforms) = data;
        let aabb_array = (&physics, &transforms)
                .join()
                .map( |(p, t)|{ p.get_aabb( t.position) })
                .collect::<Vec<_>>();

        for (p, t) in (&mut physics, &mut transforms).join() {
            let d_aabb = p.get_delta_aabb(t.position);
            let aabb = p.get_aabb(t.position);
            let mut collision = false;
            for t_aabb in &aabb_array {
                if t_aabb[0] == aabb[0] && t_aabb[2] == aabb[2] {
                    continue;
                }
                if check_collision( &d_aabb, t_aabb) {
                    collision = true;
                    break;
                }
            }

            if collision == false {
                t.move_position(p.get_velocity());
            }
        }
    }
}