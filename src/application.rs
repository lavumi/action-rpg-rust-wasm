
use instant::Instant;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::{ PhysicalPosition, PhysicalSize};
use crate::components::mesh::Mesh;
use crate::cube::Cube;
use crate::renderer::{Camera, GPUResourceManager, PipelineManager, RenderState, Texture};


pub struct Application {

    window : Window,
    renderer: RenderState,
    camera : Camera,
    cube : Cube,
    size : PhysicalSize<u32>,
    // input : InputHandler
    gpu_resource_manager : GPUResourceManager,
    pipeline_manager : PipelineManager,

    prev_mouse_position : PhysicalPosition<f64>,
    prev_time : Instant
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


        let size = window.inner_size();
        let mut gpu_resource_manager = GPUResourceManager::default();

        let mut pipeline_manager = PipelineManager::default();
        let renderer = RenderState::new(&window, &mut gpu_resource_manager).await;



        pipeline_manager.add_default_pipeline(&renderer , &gpu_resource_manager);


        Texture::load_texture(include_bytes!("../assets/atlas.png"),&mut gpu_resource_manager, &renderer.device, &renderer.queue);


        let camera = Camera::new( size.width as f32 / size.height as f32);
        camera.build(&mut gpu_resource_manager, &renderer.device);

        let cube = Cube::new(&renderer.device);
        let prev_mouse_position = PhysicalPosition::new(0.0, 0.0);
        let prev_time = Instant::now();




        Self {
            window,
            renderer,
            size,
            camera,
            gpu_resource_manager,
            pipeline_manager,
            cube,
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
                self.prev_time =  Instant::now();
                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.renderer.resize(self.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                self.window.request_redraw();
            }
            _ => {}
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.renderer.resize(new_size);
    }

    #[allow(dead_code)]
    pub fn set_clear_color(&mut self, new_color: wgpu::Color) {
        self.renderer.set_clear_color(new_color);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cube.rotate((position.x - self.prev_mouse_position.x) as f32, (position.y - self.prev_mouse_position.y) as f32);
                self.prev_mouse_position =  position.clone();
                true
            }
            WindowEvent::MouseInput {  state, button, .. } =>{
                match button {
                    MouseButton::Left => {
                        self.cube.toggle_rotate( state == &ElementState::Pressed );
                    }
                    _ => {}
                }
                false
            }
            _ => false,
        }
    }

    fn update(&mut self , dt : f32) {
        let camera_uniform = self.camera.update_view_proj();
        let camera_buffer = self.gpu_resource_manager.get_buffer("camera_matrix");
        self.renderer.update_camera_buffer(camera_buffer,camera_uniform);


        self.cube.update(dt);
        self.cube.update_instance( &self.renderer.queue);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render( &self.gpu_resource_manager, &mut self.pipeline_manager,&self.cube.get_render_component())
    }
}