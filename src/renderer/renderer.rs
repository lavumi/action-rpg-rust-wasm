use std::iter;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::components::{Tile, Transform};
use crate::renderer::{texture, Vertex};
use crate::renderer::gpu_resource_manager::GPUResourceManager;
use crate::renderer::mesh::InstanceTileRaw;
use crate::renderer::pipeline_manager::PipelineManager;

pub struct RenderState {
    pub device: wgpu::Device,
    surface: wgpu::Surface,

    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    pub gpu_resource_manager : GPUResourceManager,
    pub pipeline_manager : PipelineManager,

    color: wgpu::Color,
    depth_texture: texture::Texture,

    aspect_ratio: f32,
    viewport_data: [f32; 6],
}

impl RenderState {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });


        // # Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu`s features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            // .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);


        let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");
        let color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

        let aspect_ratio = size.width as f32 / size.height as f32;
        let viewport_data = [0., 0., size.width as f32, size.height as f32, 0., 1.];

        let mut gpu_resource_manager = GPUResourceManager::default();
        gpu_resource_manager.initialize(&device);
        let mut pipeline_manager = PipelineManager::default();
        pipeline_manager.init_pipelines(&device, config.format, &gpu_resource_manager);

        Self {
            device,
            surface,
            queue,
            config,
            gpu_resource_manager,
            pipeline_manager,
            color,
            depth_texture,
            aspect_ratio,
            viewport_data,
        }
    }

    pub fn load_atlas(&mut self){
        self.gpu_resource_manager.init_atlas(&self.device, &self.queue);
    }

    #[allow(dead_code)]
    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.color = color;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.surface.configure(&self.device, &self.config);

            let aspect_ratio = new_size.width as f32 / new_size.height as f32;

            if (self.aspect_ratio - aspect_ratio).abs() > 0.02 {
                if self.aspect_ratio < aspect_ratio { //width is bigger
                    let adjust_width = new_size.height as f32 * self.aspect_ratio;
                    let x_offset = (new_size.width as f32 - adjust_width) * 0.5;

                    self.viewport_data = [x_offset, 0., adjust_width, new_size.height as f32, 0., 1.];
                } else {
                    let adjust_height = new_size.width as f32 / self.aspect_ratio;
                    self.viewport_data = [0., 0., new_size.width as f32, adjust_height, 0., 1.];
                }
            } else {
                self.viewport_data = [0., 0., new_size.width as f32, new_size.height as f32, 0., 1.];
            }
        }
    }

    pub fn update_camera_buffer(&self, camera_uniform: [[f32; 4]; 4]) {
        let camera_buffer = self.gpu_resource_manager.get_buffer("camera_matrix");
        self.queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }


    pub fn update_map_instance(&mut self, tile_instance: Vec<InstanceTileRaw>) {
        self.gpu_resource_manager.update_mesh_instance("world", &self.device, &self.queue, tile_instance);
    }

    // fn update_mesh_instance<T: Into<String>>(&mut self, name: T, tile_instance: Vec<InstanceTileRaw>) {
    //     self.gpu_resource_manager.update_mesh_instance(name, &self.device, &self.queue, tile_instance);
    // }

    pub fn update_mesh_instance_bulk(&mut self, instance_data: Vec<(&Tile, &Transform)>){
        let mut rt_character = Vec::new();
        let mut rt_proj = Vec::new();
        let mut render_target_zombie = Vec::new();
        // let mut render_target_ant = Vec::new();
        // let mut render_target_minotaur = Vec::new();

        for (tile, transform) in instance_data {
            match tile.atlas.as_str() {
                "projectiles" => {
                    // let base_uv = self.gpu_resource_manager.get_atlas_base_uv("projectiles");
                    rt_proj.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }

                "character" => {
                    rt_character.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }
                // "enemy/ant" => {
                //     render_target_ant.push(InstanceTileRaw {
                //         uv: tile.get_uv(),
                //         model: transform.get_matrix(),
                //     });
                // }
                // "enemy/minotaur" => {
                //     render_target_minotaur.push(InstanceTileRaw {
                //         uv: tile.get_uv(),
                //         model: transform.get_matrix(),
                //     });
                // }
                "enemy/zombie" => {
                    render_target_zombie.push(InstanceTileRaw {
                        uv: tile.get_uv(),
                        model: transform.get_matrix(),
                    });
                }

                _ => {}
            }
        }


        self.gpu_resource_manager.update_mesh_instance("character", &self.device, &self.queue, rt_character);
        self.gpu_resource_manager.update_mesh_instance("projectiles", &self.device, &self.queue, rt_proj);
        self.gpu_resource_manager.update_mesh_instance("enemy/zombie", &self.device, &self.queue, render_target_zombie);
        // self.gpu_resource_manager.update_mesh_instance("enemy/ant", &self.device, &self.queue, render_target_ant);
        // self.gpu_resource_manager.update_mesh_instance("enemy/minotaur", &self.device, &self.queue, render_target_minotaur);
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_viewport(self.viewport_data[0],
                                     self.viewport_data[1],
                                     self.viewport_data[2],
                                     self.viewport_data[3],
                                     self.viewport_data[4],
                                     self.viewport_data[5]);

            let render_pipeline = self.pipeline_manager.get_pipeline("tile_pl");
            render_pass.set_pipeline(render_pipeline);
            self.gpu_resource_manager.render(&mut render_pass);
        }


        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }


    pub async fn render_to_texture(&self, texture_size: [u32; 2]) -> Result<(), wgpu::SurfaceError> {
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size[0],
                height: texture_size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };
        let texture = self.device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());


        let tile_size_half = [1.0 * 0.5, 1.0 * 0.5];
        let vertex: [Vertex; 4] = [
            //Front
            Vertex {
                position: [-tile_size_half[0], -tile_size_half[1], 0.0],
                tex_coords: [0.0, 0.0],
                // tex_coords: [offset[0] , offset[1] + uv_size[1]],
            },
            Vertex {
                position: [tile_size_half[0], -tile_size_half[1], 0.0],
                tex_coords: [1.0, 0.],
                // tex_coords: [offset[0] +uv_size[0], offset[1] +uv_size[1]],
            },
            Vertex {
                position: [tile_size_half[0], tile_size_half[1], 0.0],
                tex_coords: [1.0, 1.0],
                // tex_coords: [offset[0] +uv_size[0], offset[1] +0.0],
            },
            Vertex {
                position: [-tile_size_half[0], tile_size_half[1], 0.0],
                tex_coords: [0.0, 1.0],
                // tex_coords: offset ,
            }
        ];
        let indices: [u16; 6] = [
            //front
            0, 1, 2,
            2, 3, 0,
        ];

        //endregion

        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let num_indices = indices.len() as u32;


        let u32_size = std::mem::size_of::<u32>() as u32;
        let output_buffer_size = (u32_size * texture_size[0] * texture_size[1]) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST
                    // this tells wpgu that we want to read this buffer from the cpu
                    | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let output_buffer = self.device.create_buffer(&output_buffer_desc);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("RTT encoder") });

        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            };

            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
            let render_pipeline = self.pipeline_manager.get_pipeline("rtt_pl");
            render_pass.set_pipeline(&render_pipeline);

            self.gpu_resource_manager.render_test(&mut render_pass);


            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * texture_size[0]),
                    rows_per_image: Some(texture_size[1]),
                },
            },
            texture_desc.size,
        );

        self.queue.submit(Some(encoder.finish()));

        // We need to scope the mapping variables so that we can
        // unmap the buffer
        {
            let buffer_slice = output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            use image::{ImageBuffer, Rgba};
            let buffer =
                    ImageBuffer::<Rgba<u8>, _>::from_raw(texture_size[0], texture_size[1], data).unwrap();
            buffer.save("image.png").unwrap();
        }
        output_buffer.unmap();
        Ok(())
    }


    pub async fn render_to_texture2(&self) -> Result<(), wgpu::SurfaceError> {
        let tile_size_half = [1.0 * 0.5, 1.0 * 0.5];
        let vertex: [Vertex; 4] = [
            //Front
            Vertex {
                position: [-tile_size_half[0], -tile_size_half[1], 0.0],
                tex_coords: [0.0, 0.0],
                // tex_coords: [offset[0] , offset[1] + uv_size[1]],
            },
            Vertex {
                position: [tile_size_half[0], -tile_size_half[1], 0.0],
                tex_coords: [1.0, 0.],
                // tex_coords: [offset[0] +uv_size[0], offset[1] +uv_size[1]],
            },
            Vertex {
                position: [tile_size_half[0], tile_size_half[1], 0.0],
                tex_coords: [1.0, 1.0],
                // tex_coords: [offset[0] +uv_size[0], offset[1] +0.0],
            },
            Vertex {
                position: [-tile_size_half[0], tile_size_half[1], 0.0],
                tex_coords: [0.0, 1.0],
                // tex_coords: offset ,
            }
        ];
        let indices: [u16; 6] = [
            //front
            0, 1, 2,
            2, 3, 0,
        ];

        //endregion

        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let num_indices = indices.len() as u32;


        let texture_size = 256u32;
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size,
                height: texture_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };
        let texture = self.device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        // we need to store this for later
        let u32_size = std::mem::size_of::<u32>() as u32;

        let output_buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST
                    // this tells wpgu that we want to read this buffer from the cpu
                    | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let output_buffer = self.device.create_buffer(&output_buffer_desc);

        let shader = self.device.create_shader_module(wgpu::include_wgsl!("../../assets/rtt.wgsl"));

        let camera_bind_group_layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let texture_bind_group_layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let render_pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_desc.format,
                    blend: Some(wgpu::BlendState {
                        alpha: wgpu::BlendComponent::REPLACE,
                        color: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let mut encoder =
                self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            render_pass.set_pipeline(&render_pipeline);

            self.gpu_resource_manager.set_bind_group(&mut render_pass, "camera");
            self.gpu_resource_manager.set_bind_group(&mut render_pass, "p_06_1");

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);

            // render_pass.draw(0..3, 0..1);
        }

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * texture_size),
                    rows_per_image: Some(texture_size),
                },
            },
            texture_desc.size,
        );

        self.queue.submit(Some(encoder.finish()));

        // We need to scope the mapping variables so that we can
        // unmap the buffer
        {
            let buffer_slice = output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            use image::{ImageBuffer, Rgba};
            let buffer =
                    ImageBuffer::<Rgba<u8>, _>::from_raw(texture_size, texture_size, data).unwrap();
            buffer.save("image.png").unwrap();
        }
        output_buffer.unmap();

        Ok(())
    }
}