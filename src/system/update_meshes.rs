use specs::{Read, ReadStorage, System, Write};
use crate::components::tile::Tile;
use crate::renderer::{GPUResourceManager, RenderState};
use crate::resources::tile_map_storage::TileMapStorage;

pub struct UpdateMeshes;

impl<'a> System<'a> for UpdateMeshes {
    type SystemData = (
        ReadStorage<'a, Tile>,
        Read<'a, TileMapStorage>,
        Write<'a, GPUResourceManager>,

        Write<'a, RenderState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let ( tiles,map_storage, mut gpu_resource_manager,renderer) = data;
        let mut render_target_creature = Vec::new();

        use specs::Join;
        for tile in tiles.join(){
            match tile.texture.as_str() {
                "creature" => {
                    render_target_creature.push(tile.to_tile_raw());
                }
                _=>{}
            }
        }

        let render_target_world = map_storage.get_meshes();


        gpu_resource_manager.update_mesh_instance("world",&renderer, render_target_world);
        gpu_resource_manager.update_mesh_instance("creature",&renderer, render_target_creature);
    }
}