use std::borrow::BorrowMut;

use rand::Rng;
use rand::rngs::ThreadRng;
use specs::{Entities, Read, System, Write, WriteStorage};

use crate::components::{Animation, Direction, Enemy, Forward, Movable, Physics, Tile, Transform};
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
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Forward>,
        Write<'a, EnemyManager>,
        Read<'a, DeltaTime>,
        Write<'a, ThreadRng>,
    );

    fn run(&mut self, (entities, mut tile, mut enemies, mut physics, mut transform, mut animation, mut movable, mut forwards, mut enemy_manager, dt, mut rng): Self::SystemData) {
        if enemy_manager.update_spawn_timer(dt.0) == false {
            return;
        }


        let enemy_data = enemy_manager.get_enemy_info("zombie");

        let pos_x: f32 = rng.gen_range(-10.0..10.0);
        let pos_y: f32 = rng.gen_range(-10.0..10.0);
        entities.build_entity()
            .with(
                enemy_data.tile.clone(),
                tile.borrow_mut())
            .with(
                Enemy {
                    speed: enemy_data.speed,
                    tick: 99.0,
                },
                enemies.borrow_mut())
            .with(
                Transform::new([20.0 + pos_x, 2.0 + pos_y, 0.2], enemy_data.size),
                transform.borrow_mut())
                .with(
                    Physics::default(),
                    physics.borrow_mut())
                .with(
                    enemy_data.animations.clone(),
                    animation.borrow_mut())
                .with(
                    Movable(true),
                    movable.borrow_mut())
                .with(
                    Forward { direction: Direction::Down },
                    forwards.borrow_mut())
            .build();
    }
}