use std::borrow::BorrowMut;

use specs::{Entities, Read, System, Write, WriteStorage};

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
        Write<'a, EnemyManager>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut tile, mut enemies, mut physics, mut transform, mut animation, mut enemy_manager, dt): Self::SystemData) {
        use specs::Join;

        if enemy_manager.update_spawn_timer( dt.0) == false {
            return;
        }


        let enemy_data = enemy_manager.get_enemy_info("zombie");

        entities.build_entity()
            .with(
                enemy_data.tile.clone(),
                tile.borrow_mut())
            .with(
                Enemy::new(1.),
                enemies.borrow_mut())
            .with(
                Transform::new([20.0, 2.0, 0.2], enemy_data.size),
                transform.borrow_mut())
            .with(
                Physics::default(),
                physics.borrow_mut())
            .with(
                enemy_data.animations.clone(),
                animation.borrow_mut())
            .build();
    }
}