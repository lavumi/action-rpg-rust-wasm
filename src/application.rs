use instant::Instant;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use wgpu::SurfaceError;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::components::*;
use crate::renderer::{Camera, GPUResourceManager, PipelineManager, RenderState};
use crate::resources::*;
use crate::system::*;

// use winit::monitor::MonitorHandle;
// use winit::window::Fullscreen;

pub struct Application {
    world: World,
    window: Window,
    // monitor: MonitorHandle,
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

        // let monitor = event_loop
        //     .available_monitors()
        //     .next()
        //     .expect("no monitor found!");
        // println!("Monitor: {:?}", monitor.name());

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
        world.register::<Tile>();
        world.register::<Animation>();
        world.register::<Physics>();
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Attack>();
        world.register::<AttackMaker>();
        world.register::<Transform>();

        let renderer = RenderState::new(&window).await;
        let mut gpu_resource_manager = GPUResourceManager::default();
        gpu_resource_manager.initialize(&renderer);

        let mut pipeline_manager = PipelineManager::default();
        pipeline_manager.add_default_pipeline(&renderer, &gpu_resource_manager);

        gpu_resource_manager.init_atlas(&renderer);

        let size = window.inner_size();
        let prev_mouse_position = PhysicalPosition::new(0.0, 0.0);
        let prev_time = Instant::now();

        world.insert(renderer);
        world.insert(gpu_resource_manager);
        world.insert(pipeline_manager);

        world.insert(TileMapStorage::default());
        world.insert(EnemyManager::default());
        world.insert(InputHandler::default());
        world.insert(Camera::init_orthophathic(16, 12));
        world.insert(DeltaTime(0.05));
        world.insert(rand::thread_rng());

        world.create_entity()
            .with(Player::default())
            // .with(AttackMaker::default())
            .with(Physics::default())
            .with(Tile {
                tile_index: [0, 0],
                uv_size: [0.0625, 0.0625],
                atlas: "character/clothes".to_string(),
            })
            .with(Transform::new([0.0, 0.0, 0.2], [4.0, 4.0]))
            .with(Animation::default())
            .build();


        let mut updater = DispatcherBuilder::new()
            .with(SpawnEnemy, "spawn_enemy", &[])
            .build();
        updater.dispatch(&mut world);


        Self {
            world,
            window,
            // monitor,
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
                self.prev_time = Instant::now();

                //todo fix 처음 시작할때 elapse time 이 한순간 튀는데 이거 원인 찾아보자. + 처음 켜져마자 게임이 시작되면 안되는데...
                if elapsed_time > 0.2 {
                    return;
                }
                self.update(elapsed_time);

                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    // Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.renderer.resize(self.size),
                    Err(SurfaceError::Outdated) => {}
                    Err(SurfaceError::Lost) => {}
                    Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(SurfaceError::Timeout) => log::warn!("Surface timeout"),
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

    // #[allow(dead_code)]
    // fn toggle_full_screen(&mut self) {
    //     if self.window.fullscreen().is_none() {
    //         let fullscreen = Some(Fullscreen::Borderless(Some(self.monitor.clone())));
    //         self.window.set_fullscreen(fullscreen);
    //     } else {
    //         self.window.set_fullscreen(None);
    //     }
    // }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                let mut input_handler = self.world.write_resource::<InputHandler>();
                input_handler.receive_keyboard_input(input.state, input.virtual_keycode)
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.prev_mouse_position = position.clone();
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match button {
                    MouseButton::Left => {
                        // self.toggle_full_screen();
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
                .with(SpawnEnemy, "spawn_enemy", &[])
                .with(UpdateAnimation, "update_animation", &[])
                .with(FireWeapon, "fire_weapon", &[])
                .with(UpdatePlayer, "update_player", &[])
                .with(UpdateEnemy, "update_enemy", &["update_player"])
                .with(UpdateAttack, "update_attack", &["fire_weapon"])
                .with(UpdateCamera, "update_camera", &["update_player"])
                .with(UpdatePhysics, "update_physics", &["update_player"])
                .with(UpdateMeshes, "update_meshes", &["update_player", "update_enemy", "fire_weapon", "update_animation", "update_physics"])
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