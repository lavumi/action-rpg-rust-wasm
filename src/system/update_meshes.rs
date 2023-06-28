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
        gpu_resource_manager.update_mesh_instance("world_atlas", &renderer, render_target_world);


        let mut rt_character = Vec::new();
        let mut render_target_zombie = Vec::new();
        let mut render_target_ant = Vec::new();
        let mut render_target_minotaur = Vec::new();
        // let mut render_target_head = Vec::new();

        use specs::Join;
        for (tile, transform) in (&tiles, &transforms).join() {
            match tile.atlas.as_str() {
                "character/clothes" => {
                    rt_character.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }
                "enemy/ant" => {
                    render_target_ant.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }
                "enemy/minotaur" => {
                    render_target_minotaur.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }
                "enemy/zombie" => {
                    render_target_zombie.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }

                _ => {}
            }
        }

        //todo

        gpu_resource_manager.update_mesh_instance("character", &renderer, rt_character);
        gpu_resource_manager.update_mesh_instance("enemy/zombie", &renderer, render_target_zombie);
        gpu_resource_manager.update_mesh_instance("enemy/ant", &renderer, render_target_ant);
        gpu_resource_manager.update_mesh_instance("enemy/minotaur", &renderer, render_target_minotaur);
    }
}