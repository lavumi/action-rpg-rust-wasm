use winit::event::{ElementState, VirtualKeyCode};

#[derive(Default)]
pub struct InputHandler{
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}


impl InputHandler {
    pub fn receive_input(&mut self, state : ElementState , virtual_keycode: Option<VirtualKeyCode>)-> bool {
        match virtual_keycode {
            Some(code) if code == VirtualKeyCode::W => {
                true
            }
            Some(code) if code == VirtualKeyCode::A => {
                // camera.move_camera([-1.0,0.0]);
                true
            }
            Some(code) if code == VirtualKeyCode::S => {
                // camera.move_camera([0.0,-1.0]);
                true
            }
            Some(code) if code == VirtualKeyCode::D => {
                // camera.move_camera([1.0,0.0]);
                true
            }
            Some(_)  => false,
            None => false
        }
    }
}