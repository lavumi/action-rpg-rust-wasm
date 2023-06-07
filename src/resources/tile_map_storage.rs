use crate::components::tile::{InstanceTileRaw, Tile};

pub struct TileMapStorage {
    tiles: Vec<Tile>,
    map_design : Vec<usize>
}


impl Default for TileMapStorage{
    fn default() -> Self {
        let tiles =
            (-10..10).flat_map(|x| {
                (-10..10).map(move |y| {
                    Tile{
                        tile_index: [0,0],
                        uv_size: [0.02857, 0.024390],
                        position: [(x*2) as f32,(y*2) as f32,0.0],
                        texture: "world".to_string(),
                        flip: false,
                    }
                })
            }).collect::<Vec<_>>();


        TileMapStorage{
            tiles,
            map_design: vec![0],
        }
    }
}


impl TileMapStorage {
    pub fn get_meshes(&self) -> Vec<InstanceTileRaw>{
        let mut meshes = Vec::new();
        for tile in self.tiles.iter() {
            meshes.push(tile.to_tile_raw());
        }
        meshes
    }

    pub fn update_tile_grid(&mut self, player_pos : [f32;2]){

    }
}