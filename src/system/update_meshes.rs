use specs::{Read, ReadStorage, System, Write};
use crate::components::tile:: Tile;
use crate::renderer::{ GPUResourceManager, RenderState};
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

        let render_target_world = map_storage.get_meshes();
        let mut render_target_creature = Vec::new();
        let mut render_target_fx = Vec::new();

        use specs::Join;
        for tile in tiles.join(){
            match tile.atlas.as_str() {
                "world" => {}
                "creature" => {
                    render_target_creature.push(tile.to_tile_raw());
                }
                "fx" => {
                    render_target_fx.push(tile.to_tile_raw());
                }
                _=> {}
            }
        }
        gpu_resource_manager.update_mesh_instance("creature",&renderer, render_target_creature);
        gpu_resource_manager.update_mesh_instance("fx",&renderer, render_target_fx);
        gpu_resource_manager.update_mesh_instance("world",&renderer, render_target_world);

    }
}