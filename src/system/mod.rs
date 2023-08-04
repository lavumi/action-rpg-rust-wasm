pub use dispatcher::UnifiedDispatcher;
pub use fire_weapon::FireWeapon;
pub use spawn_enemy::SpawnEnemy;
pub use update_animation::UpdateAnimation;
pub use update_attacks::UpdateAttack;
pub use update_camera::UpdateCamera;
pub use update_enemy::UpdateEnemy;
pub use update_physics::UpdatePhysics;
pub use update_player::UpdatePlayer;

mod update_camera;
mod update_animation;
mod update_player;
mod fire_weapon;
mod update_attacks;
mod update_physics;
mod update_enemy;
mod spawn_enemy;
mod dispatcher;


pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}