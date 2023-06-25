use rand::Rng;

use crate::components::{InstanceTileRaw, Tile, Transform};

struct TileChunk {
    pub center_position: [f32; 2],
    pub meshes: Vec<InstanceTileRaw>,
}


impl TileChunk {
    pub fn new(center_position: [f32; 2], chunk_size: i32) -> Self {
        let meshes = (-chunk_size..chunk_size).flat_map(|x| {
            (-chunk_size..chunk_size).map(move |y| {
                let _ = rand::thread_rng().gen_range(0..16) as u8;
                let uv = (Tile {
                    tile_index: [0, 20],
                    uv_size: [0.03125, 0.015625],
                    atlas: "world".to_string(),
                }).get_uv();
                let y_offset = if x % 2 == 0 { 0. } else { -0.5 };
                let model = (Transform::new(
                    [
                        x as f32 + center_position[0],
                        y as f32 + y_offset + center_position[1],
                        0.0
                    ],
                    [2.0, 1.0],
                )).get_matrix();


                InstanceTileRaw {
                    model,
                    uv,
                }
            })
        }).collect::<Vec<_>>();


        let chunk = TileChunk {
            center_position,
            meshes,
        };

        // chunk.add_decal(center_position, chunk_size);
        // chunk.add_decal(center_position, chunk_size);
        // chunk.add_decal(center_position, chunk_size);

        chunk
    }


    #[allow(unused)]
    fn add_decal(&mut self, center_position: [f32; 2], chunk_size: i32) {
        let mut rnd = rand::thread_rng();
        let x = rnd.gen_range(-chunk_size..chunk_size);
        let y = rnd.gen_range(-chunk_size..chunk_size);
        let decal = rnd.gen_range(0..10);

        self.meshes.push({
            let uv = (Tile {
                tile_index: [decal, 10],
                uv_size: [0.0625, 0.0238095],
                atlas: "world".to_string(),
            }).get_uv();

            let y_offset = if x % 2 == 0 { 0. } else { -0.5 };
            let model = (Transform::new(
                [
                    x as f32 + center_position[0],
                    y as f32 + y_offset + center_position[1] + 1.0,
                    0.1
                ],
                [2.0, 1.0],
            )).get_matrix();

            InstanceTileRaw {
                uv,
                model,
            }
        });

        self.meshes.push({
            let uv = (Tile {
                tile_index: [decal, 11],
                uv_size: [0.0625, 0.0238095],
                atlas: "world".to_string(),
            }).get_uv();

            let y_offset = if x % 2 == 0 { 0. } else { -0.5 };
            let model = (Transform::new(
                [
                    x as f32 + center_position[0],
                    y as f32 + y_offset + center_position[1],
                    0.1
                ],
                [2.0, 1.0],
            )).get_matrix();

            InstanceTileRaw {
                uv,
                model,
            }
        });
        // self.meshes.push( {
        //     let uv = (Tile {
        //         tile_index: [0, 10],
        //         uv_size: [0.0625, 0.0238095],
        //         atlas: "world".to_string(),
        //     }).get_uv();
        //
        //     let y_offset = if x % 2 == 0 { 0. } else { -0.5 };
        //     let model = (Transform::new(
        //         [
        //             x as f32 + center_position[0],
        //             y as f32 + y_offset + center_position[1] - 1.0,
        //             0.1
        //         ],
        //         [2.0, 1.0],
        //     )).get_matrix();
        //
        //     InstanceTileRaw{
        //         uv,
        //         model,
        //     }
        // });
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
        let chunk_size: f32 = 8.;
        let tiles = (-full_map_size..full_map_size).flat_map(|x| {
            (-full_map_size..full_map_size).map(move |y| {
                TileChunk::new([x as f32 * chunk_size * 2., y as f32 * chunk_size * 2.], chunk_size as i32)
            })
        }).collect::<Vec<_>>();

        let camera_pos = [0.0,0.0];
        let mut meshes = vec![];
        for tile in tiles.iter() {
            if tile.center_position[0] < camera_pos[0] + chunk_size * 3.0 &&
                tile.center_position[0] > camera_pos[0] - chunk_size * 3.0 &&
                tile.center_position[1] < camera_pos[1] + chunk_size * 3.0 &&
                tile.center_position[1] > camera_pos[1] - chunk_size * 3.0
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
        if self.camera_pos[0] < camera_pos[0] + self.chunk_size * 0.5 &&
            self.camera_pos[0] > camera_pos[0] - self.chunk_size * 0.5 &&
            self.camera_pos[1] < camera_pos[1] + self.chunk_size * 0.5 &&
            self.camera_pos[1] > camera_pos[1] - self.chunk_size * 0.5
        {
            return;
        }

        self.camera_pos = camera_pos;
        self.meshes.clear();
        for tile in self.tiles.iter() {
            if tile.center_position[0] < camera_pos[0] + self.chunk_size * 3.5 &&
                tile.center_position[0] > camera_pos[0] - self.chunk_size * 3.5 &&
                tile.center_position[1] < camera_pos[1] + self.chunk_size * 3.5 &&
                tile.center_position[1] > camera_pos[1] - self.chunk_size * 3.5
            {
                self.meshes.extend(tile.meshes.iter());
            }
        }
    }
}