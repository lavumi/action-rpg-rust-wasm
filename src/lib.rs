
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::application::Application;
use crate::winit_state::WinitState;

mod renderer;
pub mod winit_state;
pub mod application;
mod components;
mod system;
mod resources;
mod object;
mod game_state;
mod spawner;
mod utils;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn start(){
    let title = "wgpu_wasm";
    let width = 1024;
    let height = 768;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let (wb, event_loop) = WinitState::create(title, width, height );
    // let asset_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/").to_string();
    let mut application = Application::new(wb, &event_loop).await;
    event_loop.run(move |event, _,control_flow| {
        application.run(&event, control_flow);
    });
}