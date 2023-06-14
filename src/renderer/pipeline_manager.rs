use std::collections::HashMap;

use wgpu::{Face, ShaderModule};

use crate::components::tile::InstanceTileRaw;
use crate::renderer::{GPUResourceManager, RenderState, Texture};
use crate::renderer::vertex:: Vertex;

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
            color_states:vec![],
            depth_state: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            sample_count: 1,
            sampler_mask: 0,
            alpha_to_coverage_enabled: true,
            layouts: vec!["camera_bind_group_layout".to_string(), "texture_bind_group_layout".to_string()],
            front_face: wgpu::FrontFace::Ccw,
            // cull_mode: Some(Face::Back),
            cull_mode: None,
            depth_bias: 0,
        }
    }
}

impl PipelineDesc {
    pub fn build (
        &self ,
        shader: ShaderModule,
        render_state: &RenderState,
        gpu_resource_manager : &GPUResourceManager
    ) -> wgpu::RenderPipeline {

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
            render_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &bind_group_layout_ref,
                push_constant_ranges: &[],
            });

        let render_pipeline = render_state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), InstanceTileRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: render_state.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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

        render_pipeline
    }
}

pub struct PipelineManager{
    pipelines : HashMap<String ,  wgpu::RenderPipeline>
}

impl Default for PipelineManager {
    fn default() -> Self {
        let pipeline_manager = Self { pipelines: Default::default() };
        pipeline_manager
    }
}

impl PipelineManager {
    pub fn add_default_pipeline(
        &mut self,
        render_state: &RenderState,
        gpu_resource_manager : &GPUResourceManager
    ){
        // let shader = render_state.device.create_shader_module(wgpu::include_wgsl!("../../assets/shader_instance.wgsl"));
        // let render_pipeline = PipelineDesc::default().build( shader, &render_state,  &gpu_resource_manager);
        // self.add_pipeline("instance_pl".to_string() , render_pipeline);


        let shader = render_state.device.create_shader_module(wgpu::include_wgsl!("../../assets/shader_tile.wgsl"));
        let render_pipeline = PipelineDesc::default().build(shader, &render_state, &gpu_resource_manager);
        self.add_pipeline("tile_pl".to_string(), render_pipeline);
    }

    fn add_pipeline(&mut self,name: String , pipeline: wgpu::RenderPipeline){
        self.pipelines.insert(name, pipeline);
    }

    pub fn get_pipeline(&self , name: &str) -> &wgpu::RenderPipeline{
        self.pipelines.get(name).unwrap()
    }
}