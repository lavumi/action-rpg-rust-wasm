use specs::{Builder, DispatcherBuilder, World, WorldExt};
use instant::Instant;
use wgpu::SurfaceError;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use crate::components::animation::Animation;


use crate::components::tile::Tile;
use crate::renderer::{Camera, GPUResourceManager, PipelineManager, RenderState};
use crate::resources::delta_time::DeltaTime;
#[allow(unused)]
use crate::system::cube_shuffle::CubeShuffle;
use crate::system::update_camera::UpdateCamera;
use crate::system::render::Render;
use crate::system::update_meshes::UpdateMeshes;
use crate::system::update_tile_animation::UpdateTileAnimation;


pub struct Application {
    world: World,
    window: Window,
    size: PhysicalSize<u32>,
    prev_mouse_position: PhysicalPosition<f64>,
    prev_time: Instant,
}




impl Application {
    pub async fn new(
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>) -> Self {
        let window = window_builder
            .build(&event_loop)
            .unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            // use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(1024, 768));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wgpu-wasm")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let mut world = World::new();
        // world.register::<Mesh>();
        world.register::<Tile>();
        world.register::<Animation>();

        let renderer = RenderState::new(&window).await;
        let mut gpu_resource_manager = GPUResourceManager::default();
        gpu_resource_manager.initialize(&renderer);

        let mut pipeline_manager = PipelineManager::default();
        pipeline_manager.add_default_pipeline(&renderer, &gpu_resource_manager);


        let size = window.inner_size();
        // let aspect_ratio = size.width as f32 / size.height as f32;
        let camera = Camera::init_ortho(16,12);

        let prev_mouse_position = PhysicalPosition::new(0.0, 0.0);
        let prev_time = Instant::now();


        world.insert(renderer);
        world.insert(gpu_resource_manager);
        world.insert(pipeline_manager);

        world.insert(camera);
        world.insert(DeltaTime(0.05));
        world.insert(rand::thread_rng());


        world.create_entity()
            .with( Tile{
                tile_index: [0,0],
                uv_size: [1.0/35.,1.0/41.],
                position: [0.0,0.0,0.0],
                texture: "world".to_string(),
            })
            .build();

        world.create_entity()
            .with( Tile{
                tile_index: [0,0],
                uv_size: [1.0/32.,1.0/41.],
                position: [0.0,0.0,0.1],
                texture: "creature".to_string(),
            })
            .with( Animation::new(vec![[0,0], [1,0]], 0.2))
            .build();

        Self {
            world,
            window,
            size,
            prev_mouse_position,
            prev_time,
        }
    }

    pub fn run(
        &mut self,
        event: &Event<'_, ()>,
        control_flow: &mut ControlFlow, // TODO: Figure out if we actually will use this...
    )
    {
        match event {
            Event::WindowEvent { ref event, window_id, } if window_id == &self.window.id() => {
                if !self.input(event) {
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
                            self.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            self.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == &self.window.id() => {
                let elapsed_time = self.prev_time.elapsed().as_millis() as f32 / 1000.0;
                self.update(elapsed_time);
                self.prev_time = Instant::now();
                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    // Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.renderer.resize(self.size),
                    Err(SurfaceError::Outdated) => {}
                    Err(SurfaceError::Lost) => {}
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
                self.world.maintain();
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                self.window.request_redraw();
            }
            _ => {}
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        let mut renderer = self.world.write_resource::<RenderState>();
        renderer.resize(new_size);
    }

    #[allow(dead_code)]
    fn set_clear_color(&mut self, new_color: wgpu::Color) {
        let mut renderer = self.world.write_resource::<RenderState>();
        renderer.set_clear_color(new_color);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. }=>{
                let mut camera = self.world.write_resource::<Camera>();
                match input.virtual_keycode {
                    Some(code) if code == VirtualKeyCode::W => {
                        camera.move_camera([0.0,1.0]);
                        true
                    }
                    Some(code) if code == VirtualKeyCode::A => {
                        camera.move_camera([-1.0,0.0]);
                        true
                    }
                    Some(code) if code == VirtualKeyCode::S => {
                        camera.move_camera([0.0,-1.0]);
                        true
                    }
                    Some(code) if code == VirtualKeyCode::D => {
                        camera.move_camera([1.0,0.0]);
                        true
                    }
                    Some(_)  => false,
                    None => false
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // self.cube.rotate((position.x - self.prev_mouse_position.x) as f32, (position.y - self.prev_mouse_position.y) as f32);
                self.prev_mouse_position = position.clone();
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match button {
                    MouseButton::Left => {
                        // self.cube.toggle_rotate( state == &ElementState::Pressed );
                      // self.set_clear_color( wgpu::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0, });
                    }
                    _ => {}
                }
                false
            }
            _ => false,
        }
    }

    fn update(&mut self, dt: f32) {
        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt);
        }
        {
            let mut updater = DispatcherBuilder::new()
                .with(UpdateCamera, "update_camera", &[])
                .with(UpdateMeshes, "update_meshes", &[])
                .with(UpdateTileAnimation, "update_tile_animation", &[])
                // .with(CubeShuffle, "cube_shuffle", &[])
                .build();
            updater.dispatch(&mut self.world);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut renderer = DispatcherBuilder::new()
            .with(Render, "render", &[])
            .build();
        renderer.dispatch(&mut self.world);
        Ok(())
    }
}