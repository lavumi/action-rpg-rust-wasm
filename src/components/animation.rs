use specs::{Component, VecStorage};


pub struct Animation {
    pub(crate) uv: [u8;2],
    frame_uv: Vec<[u8;2]>,
    frame_time: f32,
    current_frame: usize,
    current_frame_time: f32
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}

impl Animation {
    pub fn new(frame_uv: Vec<[u8;2]> , frame_time: f32) -> Self {
        if frame_uv.len() == 0 {
            panic!("animation must have more than 1 frame");
        }
        Animation{
            uv : frame_uv[0].clone(),
            frame_uv,
            frame_time,

            current_frame : 0,
            current_frame_time:0.
        }
    }

    pub fn run_animation(&mut self, delta_time: f32) {
        self.current_frame_time += delta_time;
        if self.current_frame_time >= self.frame_time {
            self.current_frame_time = 0.0;
            self.current_frame += 1;
            if self.current_frame >= self.frame_uv.len() {
                self.current_frame = 0;
            }
            self.uv = self.frame_uv[self.current_frame].clone();
        }
    }
}