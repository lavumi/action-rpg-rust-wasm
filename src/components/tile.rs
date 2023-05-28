use specs::{Component, VecStorage};

pub struct Tile {
    pub(crate) tile_index: u8,
    pub(crate) position: [u32;2],
    pub(crate) texture: String
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}