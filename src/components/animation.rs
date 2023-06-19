use specs::{Component, VecStorage};

pub struct Animation {
    frame_uv_x: Vec<Vec<u8>>,
    frame_uv_y: u8,
    frame_time: f32,
    current_anim: usize,
    current_frame: usize,
    current_frame_time: f32,

    loop_animation: bool,
    prev_animation: usize,
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            frame_uv_x: vec![
                vec![0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 1, 1],
                vec![4, 5, 6, 7, 8, 9, 10, 11],
                vec![12, 13, 14, 15],
                vec![16, 17],
                vec![18, 19, 20, 21, 22, 23],
                vec![24, 25, 26, 27],
                vec![28, 29, 30, 31],
            ],
            frame_uv_y: 0,
            frame_time: 0.1,
            current_anim: 1,
            current_frame: 0,
            current_frame_time: 0.,
            loop_animation: false,
            prev_animation: 0
        }
    }
}


impl Animation {
    pub fn new(frame_uv_x: Vec<u8>, frame_uv_y: u8, frame_time: f32) -> Self {
        if frame_uv_x.len() == 0 {
            panic!("animation must have more than 1 frame");
        }
        Animation {
            frame_uv_x: vec![frame_uv_x],
            frame_uv_y,
            frame_time,
            current_anim: 0,
            current_frame: 0,
            current_frame_time: 0.,
            loop_animation: false,
            prev_animation: 0,
        }
    }

    pub fn change_direction(&mut self, direction: u8) {
        if self.frame_uv_y == direction { return; }

        self.frame_uv_y = direction;
        self.current_frame = 0;
    }

    pub fn change_animation(&mut self, animation_index: usize, loop_animation: bool) {
        if self.current_anim == animation_index { return; }

        self.loop_animation = loop_animation;
        if self.loop_animation {
            self.prev_animation = self.current_anim;
        }

        self.current_anim = animation_index;
        self.current_frame = 0;
    }

    pub fn lock_movement(&self) -> bool {
        return self.loop_animation;
    }

    pub fn run_animation(&mut self, delta_time: f32) -> [u8; 2] {
        self.update_animation_frame(delta_time);

        [
            self.frame_uv_x[self.current_anim][self.current_frame] % 16,
            self.frame_uv_y + self.frame_uv_x[self.current_anim][self.current_frame] / 16 * 8
        ]
    }


    fn update_animation_frame(&mut self, delta_time: f32) {
        self.current_frame_time += delta_time;
        if self.current_frame_time >= self.frame_time {
            self.current_frame_time = 0.0;
            self.current_frame += 1;
            if self.current_frame >= self.frame_uv_x[self.current_anim].len() {
                self.current_frame = 0;
                self.finish_loop_animation();
            }
        }
    }

    fn finish_loop_animation(&mut self) {
        if self.loop_animation == false {
            return
        }
        self.current_anim = self.prev_animation;
        self.prev_animation = 0;
        self.loop_animation = false;
    }
}