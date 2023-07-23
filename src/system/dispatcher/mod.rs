use specs::prelude::World;

#[cfg(not(target_arch = "wasm32"))]
pub use multi_thread::*;
#[cfg(target_arch = "wasm32")]
pub use single_thread::*;

use super::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
mod single_thread;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod multi_thread;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs: *mut World);
}

construct_dispatcher!(
    (SpawnEnemy, "spawn_enemy", &[]),
    (UpdateAnimation, "update_animation", &[]),
    (FireWeapon, "fire_weapon", &[]),
    (UpdatePlayer, "update_player", &[]),
    (UpdateEnemy, "update_enemy", &["update_player"]),
    (UpdateAttack, "update_attack", &["fire_weapon"]),
    (UpdateCamera, "update_camera", &["update_player"]),
    (UpdatePhysics, "update_physics", &["update_player", "update_enemy"])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}