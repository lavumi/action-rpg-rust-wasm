use crate::components::tile::{InstanceTileRaw, Tile};

pub struct TileMapStorage {
    tiles: Vec<Tile>,
    meshes: Vec<InstanceTileRaw>,
}


impl Default for TileMapStorage{
    fn default() -> Self {
        let tiles =
            (-100..100).flat_map(|x| {
                (-100..100).map(move |y| {
                    let tile = (y / 10) % 4;
                    Tile{
                        tile_index: [0,tile as u8 ],
                        uv_size: [0.02857, 0.024390],
                        position: [(x*2) as f32,(y*2) as f32,0.0],
                        texture: "world".to_string(),
                        flip: false,
                    }
                })
            }).collect::<Vec<_>>();


        TileMapStorage{
            tiles,
            meshes: vec![],
        }
    }
}


impl TileMapStorage {
    pub fn get_meshes(& self) -> Vec<InstanceTileRaw>{
        self.meshes.clone()
    }

    pub fn update_tile_grid(&mut self, camera_pos: [f32;2]){
        self.meshes.clear();
        for tile in self.tiles.iter() {
            if
            tile.position[0] < camera_pos[0] + 10.0 &&
                tile.position[0] > camera_pos[0] - 10.0 &&
                tile.position[1] < camera_pos[1] + 10.0 &&
                tile.position[1] > camera_pos[1] - 10.0
            {
                self.meshes.push(tile.to_tile_raw());
            }
        }

    }
}