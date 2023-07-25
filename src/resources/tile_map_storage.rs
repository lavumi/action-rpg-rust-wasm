use crate::components::{Tile, Transform};
use crate::renderer::InstanceTileRaw;

const MAP_SIZE: usize = 10;

#[allow(dead_code)]
const MAP_TILES: [[u8; MAP_SIZE]; MAP_SIZE] = [
    [19, 19, 19, 19, 19, 19, 19, 19, 19, 19, ],
    [19, 1, 1, 20, 20, 20, 20, 20, 20, 19],
    [19, 2, 2, 20, 20, 19, 20, 19, 20, 19],
    [19, 3, 3, 20, 20, 20, 19, 19, 20, 19],
    [19, 20, 20, 20, 20, 19, 20, 19, 20, 19],
    [19, 20, 20, 20, 20, 19, 19, 20, 20, 19],
    [19, 20, 20, 20, 20, 19, 20, 19, 20, 19],
    [19, 20, 20, 20, 20, 20, 19, 19, 20, 19],
    [19, 20, 20, 20, 20, 20, 20, 20, 20, 19],
    [19, 19, 19, 19, 19, 19, 19, 19, 19, 19, ],
];


struct TileChunk {
    pub center_position: [f32; 2],
    pub meshes: Vec<InstanceTileRaw>,
}


impl Default for TileChunk {
    fn default() -> Self {
        let uv = (Tile {
            uv: [0.0833333, 0.1666666, 0.0625, 0.125],
            atlas: "world".to_string(),
        }).get_uv();
        let model = (Transform {
            position: [0.0, 0.0, 0.0, ],
            size: [2.0, 1.0],
        }).get_matrix();


        let meshes = vec![InstanceTileRaw {
            model,
            uv,
        }];


        TileChunk {
            center_position: [0., 0.],
            meshes,
        }
    }
}

impl TileChunk {
    pub fn new(center_position: [f32; 2], chunk_size: i32) -> Self {
        let meshes = (-chunk_size..chunk_size).flat_map(|x| {
            (-chunk_size..chunk_size).map(move |y| {
                let uv = (Tile {
                    uv: [0.0833333, 0.1666666, 0.0625, 0.125],
                    atlas: "world".to_string(),
                }).get_uv();
                // let y_offset = if x % 2 == 0 { 0. } else { -0.5 };
                let model = (Transform::new(
                    [
                        x as f32 + center_position[0],
                        y as f32 + center_position[1],
                        0.0
                    ],
                    [1.0, 1.0],
                )).get_matrix();


                InstanceTileRaw {
                    model,
                    uv,
                }
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
    chunk_size: f32,
    camera_pos: [f32; 2],
}


impl Default for TileMapStorage {
    fn default() -> Self {
        let full_map_size = 20;
        let chunk_size: f32 = MAP_SIZE as f32;
        let tiles = (-full_map_size..full_map_size).flat_map(|x| {
            (-full_map_size..full_map_size).map(move |y| {
                TileChunk::new([x as f32 * chunk_size * 2.0, y as f32 * chunk_size * 2.0], chunk_size as i32)
            })
        }).collect::<Vec<_>>();

        let camera_pos = [0.0,0.0];
        let mut meshes = vec![];
        for tile in tiles.iter() {
            if tile.center_position[0] < camera_pos[0] + chunk_size * 3. &&
                tile.center_position[0] > camera_pos[0] - chunk_size * 3. &&
                tile.center_position[1] < camera_pos[1] + chunk_size * 3. &&
                tile.center_position[1] > camera_pos[1] - chunk_size * 3.
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
        if self.camera_pos[0] < camera_pos[0] + self.chunk_size &&
            self.camera_pos[0] > camera_pos[0] - self.chunk_size &&
            self.camera_pos[1] < camera_pos[1] + self.chunk_size &&
            self.camera_pos[1] > camera_pos[1] - self.chunk_size
        {
            return;
        }


        self.camera_pos = [
            ((camera_pos[0] / self.chunk_size / 2.0).round() * 20.0),
            ((camera_pos[1] / self.chunk_size / 2.0).round() * 20.0),
        ];

        // log::info!("camera updated to {} , {}" , camera_pos[0], camera_pos[1]);
        // log::info!("camera updated to {} , {}" , self.camera_pos[0], self.camera_pos[1]);
        // log::info!("chunk size {} " , self.chunk_size);


        self.meshes.clear();
        for tile in self.tiles.iter() {
            if tile.center_position[0] < self.camera_pos[0] + self.chunk_size * 3. &&
                tile.center_position[0] > self.camera_pos[0] - self.chunk_size * 3. &&
                tile.center_position[1] < self.camera_pos[1] + self.chunk_size * 3. &&
                tile.center_position[1] > self.camera_pos[1] - self.chunk_size * 3.
            {
                self.meshes.extend(tile.meshes.iter());
            }
        }
    }
}