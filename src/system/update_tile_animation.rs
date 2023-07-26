// use std::collections::HashMap;

// use lazy_static::lazy_static;
use specs::{Read, ReadStorage, System, WriteStorage};

use crate::components::{Animation, Direction, Forward, Movable, Tile};
use crate::renderer::AnimationDataHandler;
use crate::resources::{DeltaTime};
//
// struct AnimationDataTemp {
//     pub data: Vec<Vec<u8>>,
//     pub dt: Vec<f32>,
// }
// lazy_static! {
//     static ref ANIMATION_HASH_MAP: HashMap<String, AnimationDataTemp> = {
//         let mut m = HashMap::new();
//         m.insert("player".to_string(), AnimationDataTemp{
//             data : vec![
//                 vec![0,  1,  2, 3,  2,  1],
//                 vec![4, 5, 6, 7, 8, 9, 10, 11],
//                 vec![12, 13, 14, 15],
//                 vec![16, 17],
//                 vec![18, 19, 20, 21, 22, 23],
//                 vec![24, 25, 26, 27],
//                 vec![28, 29, 30, 31],
//             ],
//             dt : vec![0.132, 0.066, 0.066, 0.066, 0.066, 0.132, 0.132]
//         });
//
//         m.insert("enemy/zombie".to_string(), AnimationDataTemp{
//             data : vec![
//                     vec![0, 1, 2, 3, 2, 1],
//                     vec![4, 5, 6, 7, 8, 9, 10, 11],
//                     vec![12, 13, 14, 15, 4],
//                     vec![16, 17, 18, 19],
//                     vec![20, 21],
//                     vec![22, 23, 24, 25, 26, 27],
//             ],
//             dt : vec![0.066, 0.066, 0.033, 0.033, 0.066, 0.066],
//         });
//         m
//     };
// }

pub struct UpdateAnimation;

impl<'a> System<'a> for UpdateAnimation {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Animation>,
        ReadStorage<'a, Forward>,
        WriteStorage<'a, Movable>,
        Read<'a, AnimationDataHandler>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mut tiles, mut animations, forwards, mut movable, anim_data) = data;
        use specs::Join;
        for (tile, ani, forward, mv) in (&mut tiles, &mut animations, &forwards, &mut movable).join() {
            if forward.direction == Direction::None { continue; }

            let my_anim_data = anim_data.character_animations[ani.index].as_ref();
            ani.dt += dt.0;
            if ani.dt >= my_anim_data.dt[0] * ani.speed {
                ani.dt = 0.;
                ani.frame += 1;
                if ani.frame >= my_anim_data.uv.len() {
                    ani.frame = 0;
                    if mv.0 == false {
                        ani.index = 0;
                        mv.0 = true;
                    }
                }
            }
            tile.uv = my_anim_data.uv[ani.frame];
        }
    }
}