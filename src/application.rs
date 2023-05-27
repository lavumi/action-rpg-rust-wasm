use cgmath::One;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use instant::Instant;
use wgpu::SurfaceError;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use crate::components::cube_instance::CubeInstance;

use crate::components::mesh::Mesh;
use crate::renderer::{Camera, GPUResourceManager, PipelineManager, RenderState};
use crate::renderer::vertex::{Instance, Vertex};
use crate::resources::delta_time::DeltaTime;
use crate::system::cube_shuffle::CubeShuffle;
use crate::system::update_camera::UpdateCamera;
use crate::system::render::Render;


pub struct Application {
    world: World,
    window: Window,
    size: PhysicalSize<u32>,
    prev_mouse_position: PhysicalPosition<f64>,
    prev_time: Instant,
}


fn make_cube(renderer: &RenderState, is_left: bool) -> (Mesh, CubeInstance) {
    //region [ Vertex Data ]
    let vertex: [Vertex; 24] = [
        //Front
        Vertex {
            position: [-1.0, -1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            tex_coords: [0.33333, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            tex_coords: [0.33333, 0.5],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            tex_coords: [0.0, 0.5],
        },
        //Upper
        Vertex {
            position: [-1.0, 1.0, -1.0],
            tex_coords: [0.66666, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            tex_coords: [0.33333, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            tex_coords: [0.33333, 0.5],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            tex_coords: [0.66666, 0.5],
        },
        //back
        Vertex {
            position: [-1.0, -1.0, -1.0],
            tex_coords: [0.66666, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            tex_coords: [1.0, 0.5],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            tex_coords: [0.66666, 0.5],
        },
        //Down
        Vertex {
            position: [-1.0, -1.0, -1.0],
            tex_coords: [0.33333, 0.5],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            tex_coords: [0.66666, 0.5],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            tex_coords: [0.66666, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0, 1.0],
            tex_coords: [0.33333, 0.0],
        },
        //Left
        Vertex {
            position: [-1.0, -1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            tex_coords: [0.33333, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            tex_coords: [0.33333, 0.5],
        },
        Vertex {
            position: [-1.0, -1.0, 1.0],
            tex_coords: [0.0, 0.5],
        },
        //Right
        Vertex {
            position: [1.0, -1.0, -1.0],
            tex_coords: [1.0, 0.5],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            tex_coords: [0.66666, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            tex_coords: [0.66666, 0.5],

        },
    ];
    let indices: [u16; 36] = [
        //front
        0, 1, 2,
        2, 3, 0,


        //top
        6, 5, 4,
        4, 7, 6,


        //back
        10, 9, 8,
        8, 11, 10,


        //down
        12, 13, 14,
        14, 15, 12,

        //left
        18, 17, 16,
        16, 19, 18,

        //right
        20, 21, 22,
        22, 23, 20
    ];
    let instances =
        (0..3).flat_map(|x| {
            (0..3).flat_map(move |y| {
                (0..3).map(move |z| {
                    let world_position = cgmath::Vector3 { x: (if is_left { -6 } else { 6 }) as f32, y: 0 as f32, z: 0 as f32 };
                    let position = cgmath::Vector3 { x: (x - 1) as f32 * 2.05, y: (y - 1) as f32 * 2.05, z: (z - 1) as f32 * 2.05 };
                    // let rotation = Quaternion::from_angle_x(cgmath::Deg(0.0));
                    Instance {
                        world_matrix: cgmath::Matrix4::from_translation(world_position),
                        model_matrix: cgmath::Matrix4::from_translation(position),
                        rpy_matrix: cgmath::Matrix4::one(),
                    }
                })
            })
        }).collect::<Vec<_>>();
    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    //endregion

    let vertex_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let index_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    let instance_buffer = renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    );
    let num_indices = indices.len() as u32;
    let num_instances = instance_data.len() as u32;

    (Mesh {
        vertex_buffer,
        index_buffer,
        instance_buffer,
        num_indices,
        num_instances,
    },
     CubeInstance {
         changed: false,
         time_spend: 0.0,
         rpy_rnd: 99,
         instances,
     })
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

        let renderer = RenderState::new(&window).await;
        let mut gpu_resource_manager = GPUResourceManager::default();
        gpu_resource_manager.initialize(&renderer);

        let mut pipeline_manager = PipelineManager::default();



        pipeline_manager.add_default_pipeline(&renderer, &gpu_resource_manager);


        let camera = Camera::new(size.width as f32 / size.height as f32);

        let prev_mouse_position = PhysicalPosition::new(0.0, 0.0);
        let prev_time = Instant::now();

        let mut world = World::new();
        world.register::<Mesh>();
        world.register::<CubeInstance>();

        let (mesh, instance) = make_cube(&renderer, false);
        world.create_entity()
            .with(mesh)
            .with(instance)
            .build();

        let (mesh2, instance2) = make_cube(&renderer, true);
        world.create_entity()
            .with(mesh2)
            .with(instance2)
            .build();


        world.insert(gpu_resource_manager);
        world.insert(pipeline_manager);
        world.insert(renderer);
        world.insert(camera);
        world.insert(DeltaTime(0.05));
        world.insert(rand::thread_rng());
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

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        // self.renderer.resize(new_size);
    }

    #[allow(dead_code)]
    pub fn set_clear_color(&mut self, new_color: wgpu::Color) {
        // self.renderer.set_clear_color(new_color);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                // self.cube.rotate((position.x - self.prev_mouse_position.x) as f32, (position.y - self.prev_mouse_position.y) as f32);
                self.prev_mouse_position = position.clone();
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match button {
                    MouseButton::Left => {
                        // self.cube.toggle_rotate( state == &ElementState::Pressed );
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
                .with(CubeShuffle, "cube_shuffle", &[])
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