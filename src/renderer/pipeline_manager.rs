use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{BindGroupLayout, Face, ShaderModule, TextureFormat};
use crate::renderer::{GPUResourceManager, Texture};
use crate::vertex::{InstanceRaw, Vertex};

#[derive(Debug, Hash, Clone)]
pub struct PipelineDesc {
    pub shader: String,
    pub primitive_topology: wgpu::PrimitiveTopology,
    pub color_states: Vec<Option<wgpu::ColorTargetState>>,
    pub depth_state: Option<wgpu::DepthStencilState>,

    pub sample_count: u32,
    pub sampler_mask: u64,
    pub alpha_to_coverage_enabled: bool,

    pub layouts: Vec<String>,
    pub front_face: wgpu::FrontFace,
    pub cull_mode: Option<Face>,
    pub depth_bias: i32,
}

impl Default for PipelineDesc {
    fn default() -> Self {
        Self {
            shader: "".to_string(),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: vec![],
            depth_state: Some(wgpu::DepthStencilState{
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            sample_count: 1,
            sampler_mask: 0,
            alpha_to_coverage_enabled: false,
            layouts: vec!["camera".to_string(), "texture".to_string()],
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            depth_bias: 0,
        }
    }
}


impl PipelineDesc {
    pub fn build (
        &self ,
        shader: ShaderModule,
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        gpu_resource_manager : &GPUResourceManager
    ) -> Pipeline {

        //이거 이렇게 2번 거쳐야 하나???
        //다른 좋은 방법 없나요
        let bind_group_layouts = self
            .layouts
            .iter()
            .map(|group_name| {
                gpu_resource_manager
                    .get_bind_group_layout(group_name)
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let bind_group_layout_ref = bind_group_layouts
            .iter()
            .map(|l| {
                l.as_ref()
            })
            .collect::<Vec<_>>();

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &bind_group_layout_ref,
                push_constant_ranges: &[],
            });





        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.primitive_topology,
                strip_index_format: None,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil : self.depth_state.clone(),
            multisample: wgpu::MultisampleState {
                count: self.sample_count, // 2.
                mask: !self.sampler_mask, // 3.
                alpha_to_coverage_enabled: self.alpha_to_coverage_enabled, // 4.
            },

            multiview: None,
        });


        Pipeline{
            desc : self.clone(),
            render_pipeline,
        }
    }
}

pub struct Pipeline {
    pub desc: PipelineDesc,
    pub render_pipeline: wgpu::RenderPipeline,
}

pub struct PipelineManager{
    pipeline : HashMap<String , Pipeline>
}