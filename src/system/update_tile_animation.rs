use std::collections::HashMap;
use lazy_static::lazy_static;
use specs::{Read, System, WriteStorage};

use crate::components::{Animation, Movable, Tile};
use crate::resources::DeltaTime;


struct AnimationData {
    pub data : Vec<Vec<u8>>,
    pub frame_time : Vec<f32>

}
lazy_static! {
    static ref ANIMATION_HASH_MAP: HashMap<String, AnimationData> = {
        let mut m = HashMap::new();
        m.insert("player".to_string(), AnimationData{
            data : vec![
                vec![0,  1,  2, 3,  2,  1],
                vec![4, 5, 6, 7, 8, 9, 10, 11],
                vec![12, 13, 14, 15],
                vec![16, 17],
                vec![18, 19, 20, 21, 22, 23],
                vec![24, 25, 26, 27],
                vec![28, 29, 30, 31],
            ],
            frame_time : vec![0.132, 0.066, 0.066, 0.066, 0.066, 0.132, 0.132]
        });

        m.insert("enemy/zombie".to_string(), AnimationData{
            data : vec![
                    vec![0, 1, 2, 3, 2, 1],
                    vec![4, 5, 6, 7, 8, 9, 10, 11],
                    vec![12, 13, 14, 15, 4],
                    vec![16, 17, 18, 19],
                    vec![20, 21],
                    vec![22, 23, 24, 25, 26, 27],
            ],
            frame_time : vec![0.066, 0.066, 0.033, 0.033, 0.066, 0.066],
        });


        m
    };
}

pub struct UpdateAnimation;

impl<'a> System<'a> for UpdateAnimation {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Movable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mut tiles, mut animations, mut movable) = data;
        use specs::Join;
        for (tile, ani, mv) in (&mut tiles, &mut animations, &mut movable).join() {
            let my_anim_data = &ANIMATION_HASH_MAP[&ani.name];

            ani.dt += dt.0;
            if ani.dt >= my_anim_data.frame_time[ani.index] * ani.speed {
                ani.dt = 0.;
                ani.frame += 1;
                if ani.frame >= my_anim_data.data[ani.index].len() {
                    ani.frame = 0;
                    if mv.0 == false {
                        ani.index = 0;
                        mv.0 = true;
                    }
                }
            }

            let dir_num = ani.direction.clone() as u8;
            tile.tile_index = [
                my_anim_data.data[ani.index][ani.frame] % 16,
                dir_num + my_anim_data.data[ani.index][ani.frame] / 16 * 8
            ];
        }
    }
}