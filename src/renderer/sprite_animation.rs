use cgmath::Vector2;
use crate::renderer::Texture;

pub struct Animation{
    texture: Texture,
    atlas_setting: (usize, usize),

    uv_start: (f32, f32, f32, f32),
    uv_size: Vector2<f32>
}
