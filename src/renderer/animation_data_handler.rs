use std::fs;
use std::sync::Arc;

use log::info;
use serde::Deserialize;
use wgpu::util::DeviceExt;
use crate::renderer::{Texture, Vertex};

#[derive(Debug, Deserialize)]
struct FrameSize {
    x: Option<u16>,
    y: Option<u16>,
    w: u16,
    h: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FrameData {
    // filename : String,
    // rotated : bool,
    // trimmed : bool,
    frame: FrameSize,
    sprite_source_size: FrameSize,
    source_size: FrameSize,
    duration: i16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FrameTag {
    name: String,
    from: usize,
    to: usize,
    direction: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetaData {
    size: FrameSize,
    frame_tags: Vec<FrameTag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnimationJsonData {
    frames: Vec<FrameData>,
    meta: MetaData,
}


pub struct AnimationData {
    pub uv: Vec<[f32; 4]>,
    pub dt: Vec<f32>,
}

pub struct AnimationDataHandler {
    // pub character_animations_hashmap: HashMap<String, AnimationData>,
    pub character_animations: Vec<Arc<AnimationData>>,
}

impl Default for AnimationDataHandler {
    fn default() -> Self {
        AnimationDataHandler {
            // character_animations_hashmap: Default::default(),
            character_animations: Default::default()
        }
    }
}


impl AnimationDataHandler {
    pub fn init(&mut self) {
        let str = fs::read_to_string("./assets/character/06.json").expect("Unable to read file");
        let data: AnimationJsonData = serde_json::from_str(&str).expect("JSON was not well-formatted");

        let atlas_size = [data.meta.size.w as f32, data.meta.size.h as f32];
        for frame_tag in data.meta.frame_tags {
            let mut animation_data = AnimationData {
                uv: vec![],
                dt: vec![],
            };

            for i in frame_tag.from..frame_tag.to {
                let start_x = data.frames[i].frame.x.unwrap() as f32 / atlas_size[0];
                let start_y = data.frames[i].frame.y.unwrap() as f32 / atlas_size[1];
                let end_x = (data.frames[i].frame.x.unwrap() + data.frames[i].frame.w) as f32 / atlas_size[0];
                let end_y = (data.frames[i].frame.y.unwrap() + data.frames[i].frame.h) as f32 / atlas_size[1];
                animation_data.uv.push([
                    start_x, end_x, start_y, end_y
                ]);
                animation_data.dt.push(data.frames[i].duration as f32 / 1000.0);
            }
            self.character_animations.push(Arc::from(animation_data));
        }
        info!("load animation data success");
    }


    pub async fn render_test(&self) -> Result<(), wgpu::SurfaceError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();


        let str = fs::read_to_string("./assets/character/06_1.json").expect("Unable to read file");
        let data: AnimationJsonData = serde_json::from_str(&str).expect("JSON was not well-formatted");

        let sprite_length = data.frames.len();
        let texture_size = [
            64u32 * sprite_length as u32, 64u32
        ];


        let mut vertex: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];

        for vertex_index in 0..sprite_length as u16 {
            let frame_data = &data.frames[vertex_index as usize];
            let original_uv = [
                frame_data.frame.x.unwrap() as f32 / data.meta.size.w as f32,
                (frame_data.frame.x.unwrap() + frame_data.frame.w) as f32 / data.meta.size.w as f32,
                frame_data.frame.y.unwrap() as f32 / data.meta.size.h as f32,
                (frame_data.frame.y.unwrap() + frame_data.frame.h) as f32 / data.meta.size.h as f32
            ];

            let offset = [
                2.0 / sprite_length as f32 * vertex_index as f32,
                0.0
            ];

            let dest_uv = [
                offset[0] + frame_data.sprite_source_size.x.unwrap() as f32 / texture_size[0] as f32  * 2.0 - 1.0 ,
                offset[0] + (frame_data.sprite_source_size.x.unwrap() + frame_data.sprite_source_size.w) as f32 / texture_size[0] as f32 * 2.0 - 1.0 ,
                frame_data.sprite_source_size.y.unwrap() as f32 / texture_size[1] as f32 * 2.0 - 1.0,
                (frame_data.sprite_source_size.y.unwrap() + frame_data.sprite_source_size.h) as f32 / texture_size[1] as f32 * 2.0 - 1.0
            ];

            vertex.push(Vertex {
                position: [dest_uv[0], dest_uv[2], 0.0],
                tex_coords: [original_uv[0], original_uv[3]],
            });

            vertex.push(Vertex {
                position: [dest_uv[1], dest_uv[2], 0.0],
                tex_coords: [original_uv[1], original_uv[3]],
            });
            vertex.push(Vertex {
                position: [dest_uv[1], dest_uv[3], 0.0],
                tex_coords: [original_uv[1], original_uv[2]],
            });
            vertex.push(Vertex {
                position: [dest_uv[0], dest_uv[3], 0.0],
                tex_coords: [original_uv[0], original_uv[2]],
            });

            indices.push(0 + vertex_index * 4);
            indices.push(1 + vertex_index * 4);
            indices.push(2 + vertex_index * 4);
            indices.push(2 + vertex_index * 4);
            indices.push(3 + vertex_index * 4);
            indices.push(0 + vertex_index * 4);
        }




        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size[0],
                height: texture_size[1],
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
        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        // we need to store this for later
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
        let output_buffer = device.create_buffer(&output_buffer_desc);

        let shader = device.create_shader_module(wgpu::include_wgsl!("../../assets/rtt.wgsl"));

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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


        let diffuse_texture = Texture::from_bytes(&device, &queue, include_bytes!("../../assets/character/06_1.png"), "06_1").unwrap();

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let num_indices = indices.len() as u32;


        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });


        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            render_pass.set_pipeline(&render_pipeline);

            render_pass.set_bind_group(0, &diffuse_bind_group, &[]);
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
                    bytes_per_row: Some(u32_size * texture_size[0]),
                    rows_per_image: Some(texture_size[1]),
                },
            },
            texture_desc.size,
        );

        queue.submit(Some(encoder.finish()));

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
            device.poll(wgpu::Maintain::Wait);
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
}