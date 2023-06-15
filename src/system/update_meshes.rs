use std::collections::HashMap;

use specs::{Read, ReadStorage, System, Write};

use crate::components::{InstanceTileRaw, Tile};
use crate::components::Transform;
use crate::renderer::{GPUResourceManager, RenderState};
use crate::resources::TileMapStorage;

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
        let (tiles, transforms, map_storage, mut gpu_resource_manager, renderer) = data;

        let render_target_world = map_storage.get_meshes();
        let mut render_target_creature = Vec::new();
        let mut render_target_fx = Vec::new();
        let mut render_target_head = Vec::new();


        // let mut render_target_dic: HashMap<String, Vec<InstanceTileRaw>> = HashMap::new();

        use specs::Join;
        for (tile, transform) in (&tiles, &transforms).join() {
            match tile.atlas.as_str() {
                "world" => {}
                "character/clothes" => {
                    render_target_creature.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }
                "fx" => {
                    render_target_fx.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }
                "head" => {
                    render_target_head.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }
                _ => {}
            }
        }

        //todo
        gpu_resource_manager.update_mesh_instance("fx_atlas", &renderer, render_target_fx);
        gpu_resource_manager.update_mesh_instance("world_atlas", &renderer, render_target_world);
        gpu_resource_manager.update_mesh_instance("character/clothes", &renderer, render_target_creature);
        // gpu_resource_manager.update_mesh_instance("head",&renderer, render_target_head);
        // gpu_resource_manager.update_mesh_instance("creature",&renderer, render_target_creature);
    }
}