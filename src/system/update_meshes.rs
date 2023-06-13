use specs::{Read, ReadStorage, System, Write};
use crate::components::tile::{InstanceTileRaw, Tile};
use crate::components::transform::Transform;
use crate::renderer::{ GPUResourceManager, RenderState};
use crate::resources::tile_map_storage::TileMapStorage;

pub struct UpdateMeshes;

impl<'a> System<'a> for UpdateMeshes {
    type SystemData = (
        ReadStorage<'a, Tile>,
        ReadStorage<'a, Transform>,
        Read<'a, TileMapStorage>,
        Write<'a, GPUResourceManager>,

        Write<'a, RenderState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let ( tiles,transforms, map_storage, mut gpu_resource_manager,renderer) = data;

        let render_target_world = map_storage.get_meshes();
        let mut render_target_creature = Vec::new();
        let mut render_target_fx = Vec::new();

        use specs::Join;
        for (tile, transform) in (&tiles, &transforms).join(){
            match tile.atlas.as_str() {
                "world" => {}
                "creature" => {
                    render_target_creature.push(InstanceTileRaw{
                        uv: tile.get_uv(),
                        model: transform.get_matrix()
                    });
                }
                "fx" => {
                    render_target_fx.push(InstanceTileRaw{
                        uv: tile.get_uv(),
                        model: transform.get_matrix()
                    });
                }
                _=> {}
            }
        }
        gpu_resource_manager.update_mesh_instance("creature",&renderer, render_target_creature);
        gpu_resource_manager.update_mesh_instance("fx",&renderer, render_target_fx);
        gpu_resource_manager.update_mesh_instance("world",&renderer, render_target_world);

    }
}