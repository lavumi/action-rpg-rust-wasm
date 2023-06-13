use crate::components::tile::{InstanceTileRaw, Tile};


struct TileChunk {
    pub center_position: [f32; 2],
    pub meshes: Vec<InstanceTileRaw>,
}


impl TileChunk {
    pub fn new(center_position: [f32; 2], chunk_size: i32) -> Self {
        let meshes = (-chunk_size..chunk_size).flat_map(|x| {
            (-chunk_size..chunk_size).map(move |y| {
                let tile = ((center_position[0] + center_position[1]) / 20.0) as u8 % 4;
                //여기서 타일맵 프리셋 만들어서 넣어 줄 수도 있음
                (Tile {
                    tile_index: [0, tile as u8],
                    uv_size: [0.02857, 0.024390],
                    position: [x  as f32 + center_position[0], y  as f32 + center_position[1], 0.0],
                    atlas: "world".to_string(),
                    flip: false,
                }).to_tile_raw()
            })
        }).collect::<Vec<_>>();

        TileChunk {
            center_position,
            meshes,
        }
    }
}


pub struct TileMapStorage {
    tiles: Vec<TileChunk>,
    meshes: Vec<InstanceTileRaw>,
    chunk_size: i32,
    camera_pos : [f32;2]
}


impl Default for TileMapStorage {
    fn default() -> Self {
        let full_map_size = 20;
        let chunk_size = 8;
        let tiles = (-full_map_size..full_map_size).flat_map(|x| {
            (-full_map_size..full_map_size).map(move |y| {
                TileChunk::new([(x  * chunk_size )as f32 * 2., (y  * chunk_size ) as f32 * 2.0], chunk_size)
            })
        }).collect::<Vec<_>>();


        let camera_pos = [0.0,0.0];
        let mut meshes = vec![];
        for tile in tiles.iter() {
            if tile.center_position[0] < camera_pos[0] + chunk_size as f32 * 3.0 &&
                tile.center_position[0] > camera_pos[0] - chunk_size as f32 * 3.0 &&
                tile.center_position[1] < camera_pos[1] + chunk_size as f32 * 3.0 &&
                tile.center_position[1] > camera_pos[1] - chunk_size as f32 * 3.0
            {
               meshes.extend(tile.meshes.iter());
            }
        }



        TileMapStorage {
            tiles,
            meshes,
            chunk_size,
            camera_pos
        }
    }
}


impl TileMapStorage {
    pub fn get_meshes(&self) -> Vec<InstanceTileRaw> {
        self.meshes.clone()
    }

    pub fn update_tile_grid(&mut self, camera_pos: [f32; 2]) {
        if self.camera_pos[0] < camera_pos[0] + self.chunk_size as f32  &&
            self.camera_pos[0] > camera_pos[0] - self.chunk_size as f32  &&
            self.camera_pos[1] < camera_pos[1] + self.chunk_size as f32  &&
            self.camera_pos[1] > camera_pos[1] - self.chunk_size as f32
        {
            return;
        }

        self.camera_pos = camera_pos;
        self.meshes.clear();
        for tile in self.tiles.iter() {
            if tile.center_position[0] < camera_pos[0] + self.chunk_size as f32 * 3.0 &&
                tile.center_position[0] > camera_pos[0] - self.chunk_size as f32 * 3.0 &&
                tile.center_position[1] < camera_pos[1] + self.chunk_size as f32 * 3.0 &&
                tile.center_position[1] > camera_pos[1] - self.chunk_size as f32 * 3.0
            {
                self.meshes.extend(tile.meshes.iter());
            }
        }
    }
}