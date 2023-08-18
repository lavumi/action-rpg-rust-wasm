use std::collections::HashMap;

use rand::rngs::ThreadRng;
use specs::{Join, World, WorldExt};

use crate::components::*;
use crate::renderer::InstanceTileRaw;
use crate::resources::*;
use crate::spawner;
use crate::system;
use crate::system::UnifiedDispatcher;

pub struct GameState {
    pub world: World,
    dispatcher: Box<dyn UnifiedDispatcher + 'static>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            world: World::new(),
            dispatcher: system::build()
        }
    }
}



impl GameState {
    pub fn init(&mut self) {
        self.world.register::<Tile>();
        self.world.register::<Animation>();
        self.world.register::<Collider>();
        self.world.register::<Player>();
        self.world.register::<Enemy>();
        self.world.register::<Attack>();
        self.world.register::<AttackMaker>();
        self.world.register::<Transform>();
        self.world.register::<Movable>();
        self.world.register::<Forward>();


        let mut anim = AnimationDataHandler::default();
        anim.init_character_anim();
        anim.init_monster_anim();

        self.world.insert(anim);
        self.world.insert(Center::default());
        self.world.insert(TileMapStorage::default());
        self.world.insert(EnemyManager::default());
        self.world.insert(InputHandler::default());
        self.world.insert(Camera::init_orthographic(16, 12));
        self.world.insert(DeltaTime(0.05));
        self.world.insert(ThreadRng::default());

        let player_entity = spawner::player(&mut self.world, 0., 0.);
        self.world.insert(player_entity);
    }


    pub fn update(&mut self, dt: f32) {
        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt);
        }
        self.dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }

    pub fn handle_keyboard_input(&mut self, input: &winit::event::KeyboardInput) -> bool {
        let mut input_handler = self.world.write_resource::<InputHandler>();
        input_handler.receive_keyboard_input(input.state, input.virtual_keycode)
    }


    pub fn get_camera_uniform(&self) -> [[f32; 4]; 4] {
        let camera = self.world.read_resource::<Camera>();
        let camera_uniform = camera.get_view_proj();
        return camera_uniform;
    }

    pub fn get_map_instance(&self) -> Vec<InstanceTileRaw> {
        let map_storage = self.world.read_resource::<TileMapStorage>();
        let rt_map_tiles = map_storage.get_meshes();
        return rt_map_tiles;
    }

    pub fn get_character_instance(&self) -> HashMap<String, Vec<InstanceTileRaw>> {
        let tiles = self.world.read_storage::<Tile>();
        let transforms = self.world.read_storage::<Transform>();
        let rt_data = (&tiles, &transforms).join().collect::<Vec<_>>();

        let mut tile_instance_data_hashmap = HashMap::new();


        for (tile, transform) in rt_data {
            let atlas = tile.atlas.clone();
            let instance = InstanceTileRaw {
                uv: tile.uv.clone(),
                model: transform.get_matrix(),
            };

            tile_instance_data_hashmap
                    .entry(atlas)
                    .or_insert_with(Vec::new)
                    .push(instance);
        }

        tile_instance_data_hashmap
    }
}