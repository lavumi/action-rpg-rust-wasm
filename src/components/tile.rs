use specs::{Component, DenseVecStorage};
use specs_derive::Component;


#[derive(Component, Clone)]
pub struct Tile {
    pub tile_index: [u8; 2],
    //todo 타일 데이터에 uv 사이즈를 넣을 필요는 없을거 같은데... texture에서 들고오는 방법으로 생각해보자
    pub uv_size: [f32; 2],
    pub atlas: String,
}

impl Tile {
    pub fn get_uv(&self) -> [f32; 2] {
        [
            self.uv_size[0] * (self.tile_index[0] as f32) ,
            self.uv_size[1] * (self.tile_index[1] as f32)
        ]
    }
}

