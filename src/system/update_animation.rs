// use std::collections::HashMap;

// use lazy_static::lazy_static;
use specs::{Read, ReadStorage, System, WriteStorage};

use crate::components::{Animation, Direction, Forward, Movable, Tile};
use crate::resources::AnimationDataHandler;
use crate::resources::DeltaTime;

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

            let my_anim_data = anim_data.get_anim_data(ani.anime_name.as_str(), ani.index);
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

            if forward.right {
                tile.uv = [
                    my_anim_data.uv[ani.frame][0],
                    my_anim_data.uv[ani.frame][1],
                    my_anim_data.uv[ani.frame][2],
                    my_anim_data.uv[ani.frame][3],
                ];
            } else {
                tile.uv = [
                    my_anim_data.uv[ani.frame][1],
                    my_anim_data.uv[ani.frame][0],
                    my_anim_data.uv[ani.frame][2],
                    my_anim_data.uv[ani.frame][3],
                ];
            };


        }
    }
}