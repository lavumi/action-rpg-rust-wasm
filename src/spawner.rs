use specs::{Builder, Entity, World, WorldExt};

use crate::components::{Animation, AttackMaker, Collider, Direction, Forward, Movable, Player, Tile, Transform};

pub fn player(world : &mut World, player_x : f32, player_y : f32) -> Entity {
    let player = world
            .create_entity()
            .with(Player { speed: 5.0 })
            .with(AttackMaker::default())
            .with(Collider::default())
            .with(Tile {
                uv: [0.0, 0.0, 0.0, 0.0],
                atlas: "character".to_string(),
            })
            .with(Transform::new([player_x, player_y, 0.2], [2.0, 2.0]))
            .with(Animation {
                anime_name: "player".to_string(),
                speed: 1.0,
                index: 0,
                frame: 0,
                dt: 99.0,
            })
            .with(Movable(true))
            .with(Forward { direction: Direction::Down , right: true})
            .build();

    player
}