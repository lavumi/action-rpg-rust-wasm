use specs::World;
use winit::dpi::PhysicalPosition;

pub struct GameState {
    pub world : World,
    prev_mouse_position: PhysicalPosition<f64>,
}
