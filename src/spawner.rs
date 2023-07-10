use specs::{Builder, Entity, World, WorldExt};

use crate::components::{Animation, AttackMaker, Direction, Forward, Movable, Physics, Player, Tile, Transform};

pub fn player(world : &mut World, player_x : f32, player_y : f32) -> Entity {
    let player = world
            .create_entity()
            .with(Player { speed: 5.0 })
            .with(AttackMaker::default())
            .with(Physics::default())
            .with(Tile {
                tile_index: [0, 0],
                uv_size: [0.0625, 0.0625],
                atlas: "character/clothes".to_string(),
            })
            .with(Transform::new([player_x, player_y, 0.2], [4.0, 4.0]))
            .with(Animation {
                name: "player".to_string(),
                speed: 1.0,
                index: 0,
                frame: 0,
                dt: 99.0,
            })
            .with(Movable(true))
            .with(Forward { direction: Direction::Down })
            .build();

    player
}