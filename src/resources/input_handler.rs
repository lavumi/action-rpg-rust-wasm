use winit::event::{ElementState, VirtualKeyCode};

pub struct InputHandler{
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}


impl Default for InputHandler {
    fn default() -> Self {
        InputHandler {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}


impl InputHandler {
    pub fn receive_keyboard_input(&mut self, state : ElementState, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        match virtual_keycode {
            Some(code) if code == VirtualKeyCode::W  || code == VirtualKeyCode::Up => {
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
            Some(code) if code == VirtualKeyCode::A || code == VirtualKeyCode::Left=> {
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
            Some(code) if code == VirtualKeyCode::S || code == VirtualKeyCode::Down => {
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
            Some(code) if code == VirtualKeyCode::D || code == VirtualKeyCode::Right => {
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