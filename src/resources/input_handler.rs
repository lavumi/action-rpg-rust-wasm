use winit::event::{ElementState, VirtualKeyCode};

#[derive(Default)]
pub struct InputHandler{
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}


impl InputHandler {
    pub fn receive_keyboard_input(&mut self, state : ElementState, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        match virtual_keycode {
            Some(code) if code == VirtualKeyCode::W => {
                match state {
                    ElementState::Pressed => {
                        self.up = true;
                    }
                    ElementState::Released => {
                        self.up = false;
                    }
                }
                true
            }
            Some(code) if code == VirtualKeyCode::A => {
                match state {
                    ElementState::Pressed => {
                        self.left = true;
                    }
                    ElementState::Released => {
                        self.left = false;
                    }
                }
                true
            }
            Some(code) if code == VirtualKeyCode::S => {
                match state {
                    ElementState::Pressed => {
                        self.down = true;
                    }
                    ElementState::Released => {
                        self.down = false;
                    }
                }
                true
            }
            Some(code) if code == VirtualKeyCode::D => {
                match state {
                    ElementState::Pressed => {
                        self.right = true;
                    }
                    ElementState::Released => {
                        self.right = false;
                    }
                }
                true
            }
            Some(_)  => false,
            None => false
        }
    }
}