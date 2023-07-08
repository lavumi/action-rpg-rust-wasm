use specs::{World, WorldExt};
use crate::system;
use crate::system::UnifiedDispatcher;


pub struct GameState {
    pub world : World,
    dispatcher : Box<dyn UnifiedDispatcher + 'static>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState{
            world : World::new(),
            dispatcher: system::build()
        }
    }
}



impl GameState {
    pub fn run_systems(&mut self) {
        self.dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }
}