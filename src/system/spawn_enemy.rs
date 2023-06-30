use std::borrow::BorrowMut;

use specs::{Entities, Read, System, WriteStorage};

use crate::components::{Animation, Attack, Enemy, Physics, Tile, Transform};
use crate::resources::{DeltaTime, EnemyManager};

pub struct SpawnEnemy;

impl<'a> System<'a> for SpawnEnemy {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Animation>,
        Read<'a, EnemyManager>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut tile, mut enemies, mut physics, mut transform, mut animation, enemy_manager, dt): Self::SystemData) {
        use specs::Join;

        let enemy_data = enemy_manager.get_enemy_info("zombie");

        entities.build_entity()
            .with(
                Tile {
                    tile_index: enemy_data.tile.tile_index,
                    uv_size: enemy_data.tile.uv_size,
                    atlas: enemy_data.tile.atlas.clone(),
                },
                tile.borrow_mut())
            .with(
                Enemy::new(1.),
                enemies.borrow_mut())
            .with(
                Transform::new([20.0, 2.0, 0.2], [4.0, 4.0]),
                transform.borrow_mut())
            .with(
                Physics::default(),
                physics.borrow_mut())
            .with(
                Animation::new(
                    vec![vec![0, 1, 2, 3, 2, 1], vec![4, 5, 6, 7, 8, 9, 10, 11]],
                    6,
                    0.2),
                animation.borrow_mut())
            .build();
    }
}