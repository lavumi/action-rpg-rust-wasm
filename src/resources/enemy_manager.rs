use std::collections::HashMap;

use crate::components::{Animation, Tile};

pub struct EnemyManager {
    enemy_templates: HashMap<String, EnemyTemplate>,
    spawn_timer : f32,
    timer_current : f32,
}

pub struct EnemyTemplate {
    pub tile: Tile,
    pub animations: Animation,
    pub size: [f32; 2],
    pub speed: f32,
}

impl Default for EnemyManager {
    fn default() -> Self {
        let mut enemy_templates = HashMap::default();

        // enemy_templates.insert("ant".into(), EnemyTemplate {
        //     tile: Tile {
        //         tile_index: [0, 0],
        //         uv_size: [0.0625, 0.0625],
        //         atlas: "enemy/ant".to_string(),
        //     },
        //     animations: Animation::new(
        //         vec![
        //             vec![0, 1, 2, 3, 2, 1],
        //             vec![4, 5, 6, 7, 8, 9, 10, 11],
        //         ],
        //         6,
        //         vec![0.2, 0.2],
        //     ),
        //     size: [2.0, 2.0],
        // });
        enemy_templates.insert("zombie".into(), EnemyTemplate {
            tile: Tile {
                tile_index: [0, 0],
                uv_size: [0.0625, 0.0625],
                atlas: "enemy/zombie".to_string(),
            },
            animations: Animation{
                name: "enemy/zombie".to_string(),
                speed: 1.0,
                index: 0,
                frame: 0,
                dt: 99.0,
            },
            size: [4.0, 4.0],
            speed: 2.0
        });
        // enemy_templates.insert("minotaur".into(), EnemyTemplate {
        //     tile: Tile {
        //         tile_index: [0, 0],
        //         uv_size: [0.0625, 0.0625],
        //         atlas: "enemy/minotaur".to_string(),
        //     },
        //     animations: Animation::new(
        //         vec![
        //             vec![0, 1, 2, 3, 2, 1],
        //             vec![4, 5, 6, 7, 8, 9, 10, 11],
        //         ],
        //         6,
        //         vec![0.2, 0.2],
        //     ),
        //     size: [6.0, 6.0],
        // });

        EnemyManager {
            enemy_templates,
            spawn_timer : 2.0,
            timer_current: 99.0
        }
    }
}

impl EnemyManager {
    pub fn get_enemy_info<T: Into<String>>(&self, name: T) -> &EnemyTemplate {
        let key = name.into();

        match self.enemy_templates.get(&key) {
            None => panic!("no enemy info {}", key),
            Some(v) => v
        }
    }

    pub fn update_spawn_timer(&mut self , dt : f32) -> bool {
        self.timer_current += dt;
        if self.timer_current >= self.spawn_timer {
            self.timer_current = 0.0;
            return true;
        }

        return false;
    }
}