use specs::{Read, ReadStorage, System, Write};
use crate::components::tile::Tile;
use crate::renderer::{GPUResourceManager, RenderState};

pub struct UpdateMeshes;

impl<'a> System<'a> for UpdateMeshes {
    type SystemData = (
        ReadStorage<'a, Tile>,
        Write<'a, GPUResourceManager>,
        Write<'a, RenderState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let ( tiles, mut gpu_resource_manager,renderer) = data;

        //
        let mut render_target_world = Vec::new();
        let mut render_target_creature = Vec::new();

        use specs::Join;
        for tile in tiles.join(){
            match tile.texture.as_str() {
                "world" => {
                    render_target_world.push(tile.to_tile_raw());
                }
                "creature" => {
                    render_target_creature.push(tile.to_tile_raw());
                }
                _=>{}
            }
        }

        gpu_resource_manager.update_mesh_instance("world",&renderer, render_target_world);
        gpu_resource_manager.update_mesh_instance("creature",&renderer, render_target_creature);
    }
}