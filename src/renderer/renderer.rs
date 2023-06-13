
use std::iter;
use std::sync::Arc;
use wgpu::{Buffer};
use winit::window::Window;
use crate::renderer::texture;
use crate::renderer::gpu_resource_manager::GPUResourceManager;
use crate::renderer::pipeline_manager::PipelineManager;



pub struct RenderState {
    pub(crate) device: wgpu::Device,
    surface: wgpu::Surface,

    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    color: wgpu::Color,
    depth_texture: texture::Texture,
}

impl Default for RenderState{
    fn default() -> Self {
        todo!()
    }
}

impl RenderState {
    pub async fn new(
        window: &Window
    ) -> Self {
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
        let color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0, };
        Self {
            device,
            surface,
            queue,
            config,
            color,
            depth_texture,
        }
    }

    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.color = color;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.surface.configure(&self.device, &self.config);
        }
    }


    pub fn update_camera_buffer(&self, camera_buffer: Arc<Buffer>, camera_uniform: [[f32; 4]; 4]) {
        self.queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }


    pub fn render(
        &self,
        gpu_resource_manager: &GPUResourceManager,
        pipeline_manager: &PipelineManager
    ) -> Result<(), wgpu::SurfaceError> {
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


            let render_pipeline = pipeline_manager.get_pipeline("tile_pl");
            render_pass.set_pipeline(render_pipeline);

            gpu_resource_manager.set_bind_group(&mut render_pass, "camera");

            gpu_resource_manager.set_bind_group(&mut render_pass, "world");
            gpu_resource_manager.render_meshes(&mut render_pass, "world");

            gpu_resource_manager.set_bind_group(&mut render_pass, "creature");
            gpu_resource_manager.render_meshes(&mut render_pass, "creature");


            gpu_resource_manager.set_bind_group(&mut render_pass, "fx");
            gpu_resource_manager.render_meshes(&mut render_pass, "fx");

        }


        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}