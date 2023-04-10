pub mod input_handler;
mod renderer;
mod cube;
mod vertex;

use instant::Instant;
use log::{info, warn};


use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::{LogicalSize, PhysicalSize};


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
// use crate::input_handler::{InputHandler, UserInput};


struct State {
    window : Window,
    renderer: renderer::RenderState,
    camera : renderer::Camera,
    // cube : Cube,
    size : PhysicalSize<u32>,
    // input : InputHandler
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();


        let renderer = renderer::RenderState::new(&window).await;
        let camera = renderer::Camera::new( size.width as f32 / size.height as f32);

        // let cube = cube::Cube::new(&renderer.device);


        // let cube_instance_data = cube.get_instance_data();
        // renderer.set_render_target(cube.vertex, cube.indices);

        Self {
            window,
            renderer,
            size,
            camera,
            // cube
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.renderer.resize(new_size);
    }

    pub fn set_clear_color(&mut self, new_color: wgpu::Color) {
        self.renderer.set_clear_color(new_color);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                // input_handler::InputHandler::cursor_moved(position)
                self.set_clear_color(wgpu::Color {
                    r: position.x / self.size.width as f64,
                    g: position.y / self.size.height as f64,
                    b: 0.0,
                    a: 1.0,
                });
                true
            }
            _ => false,
        }
    }

    fn update(&mut self , dt : f32) {
        let camera_uniform = self.camera.update_view_proj();
        self.renderer.update_camera_buffer(camera_uniform);
        self.renderer.update_cube(dt);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {


    let title = "vumi_engine";
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

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize{width,height})
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(1024, 768));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(window).await;


    let mut prev_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {

        match event {
            Event::WindowEvent { ref event, window_id, } if window_id == state.window().id() => {
                if !state.input(event) {
                    // UPDATED!
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                let elapsed_time = prev_time.elapsed().as_millis() as f32 / 1000.0;
                state.update(elapsed_time);
                prev_time =  Instant::now();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.renderer.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
