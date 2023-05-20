


use specs::{Component, VecStorage};
use crate::renderer::vertex::Instance;

// #[derive(Debug)]
pub struct CubeInstance {
    pub(crate) changed: bool,
    pub(crate) can_rotate: bool,
    pub(crate) time_spend: f32,
    pub(crate) rpy_rnd: usize,
    pub(crate) instances: Vec<Instance>,
}

impl Component for CubeInstance {
    type Storage = VecStorage<Self>;
}