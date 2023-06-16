use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;

use cgmath::SquareMatrix;
use wgpu::{BindGroup, BindGroupLayout, Buffer, RenderPass};
use wgpu::util::DeviceExt;

use crate::components::InstanceTileRaw;
use crate::components::Mesh;
use crate::object::make_tile_single_isometric;
use crate::renderer::{RenderState, Texture};

pub struct GPUResourceManager {
    bind_group_layouts: HashMap<String, Arc<BindGroupLayout>>,
    bind_groups: HashMap<String, HashMap<u32, Arc<BindGroup>>>,
    buffers: HashMap<String, Arc<Buffer>>,
    meshes_by_atlas: HashMap<String, Mesh>,
    atlas_map: HashMap<String, [f32; 2]>
}

impl Default for GPUResourceManager{
    fn default() -> Self {
        Self {
            bind_group_layouts: Default::default(),
            bind_groups: Default::default(),
            buffers: Default::default(),
            meshes_by_atlas: Default::default(),
            atlas_map: HashMap::from([
                ("world_atlas".to_string(), [0.0625, 0.0238095]),
                ("fx_atlas".to_string(), [0.1, 0.05]),
                ("character/clothes".to_string(), [0.03125, 0.125]),
                ("character/head_long".to_string(), [0.03125, 0.125]),
            ]),
        }
    }
}

impl GPUResourceManager {
    pub fn get_atlas_base_uv<T: Into<String>>(&self, atlas_name: T) -> [f32; 2] {
        let key = atlas_name.into();
        if !self.atlas_map.contains_key(&key) {
            panic!("Resource Manager: Couldn't find any bind groups! {key}");
        }

        self.atlas_map.get(&key).unwrap().clone()
    }

    pub fn initialize(&mut self, renderer: &RenderState) {
        self.init_base_bind_group(&renderer);
        self.init_camera_bind_group(&renderer);
    }

    pub fn init_atlas(&mut self, renderer: &RenderState) {
        for atlas_info in self.atlas_map.clone() {
            let atlas_name = atlas_info.0;
            let atlas_base_uv = atlas_info.1;


            // self.load_textures(atlas_name.as_str(), renderer);
            self.make_bind_group(atlas_name.as_str(), renderer);

            //todo 캐릭터는 파츠마다 mesh 추가할 필요 없음
            self.add_mesh(atlas_name.as_str(), make_tile_single_isometric(&renderer, 1.0, atlas_base_uv));
        }
    }

    fn init_base_bind_group(&mut self, renderer: &RenderState) {
        self.add_bind_group_layout(
            "texture_bind_group_layout",
            renderer.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            }));
        self.add_bind_group_layout(
            "camera_bind_group_layout",
            renderer.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            }));
    }

    fn init_camera_bind_group(&mut self, renderer: &RenderState) {
        let device = &renderer.device;
        let camera_uniform: [[f32; 4]; 4] = cgmath::Matrix4::identity().into();
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let resources = camera_buffer.as_entire_binding();
        let camera_bind_group_layout = self.get_bind_group_layout("camera_bind_group_layout").unwrap();
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: resources,
                }
            ],
            label: Some("camera_bind_group"),
        });
        self.add_buffer("camera_matrix", camera_buffer);
        self.add_bind_group("camera", 0, camera_bind_group);
    }

    fn make_bind_group<T: Into<String> + Copy>(&mut self, name: T, renderer: &RenderState) {
        let device = &renderer.device;
        let queue = &renderer.queue;
        let diffuse_texture =
            Texture::from_bytes(device, queue, format!("assets/{}.png", name.into()).as_str(), "").unwrap();


        let texture_bind_group_layout = self.get_bind_group_layout("texture_bind_group_layout").unwrap();
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

        self.add_bind_group(name.into(), 1, diffuse_bind_group);
    }

    fn add_bind_group<T: Into<String>>(
        &mut self,
        name: T,
        bind_group_index: u32,
        bind_group: BindGroup,
    ) {
        let key = name.into();
        if self.bind_groups.contains_key(&key) {
            let bind_groups = self.bind_groups.get_mut(&key).unwrap();
            bind_groups.insert(bind_group_index, Arc::new(bind_group));
        } else {
            let mut hash_map = HashMap::new();
            hash_map.insert(bind_group_index, Arc::new(bind_group));
            self.bind_groups.insert(key.clone(), hash_map);
        }
    }

    fn set_bind_group<'a, T: Into<String>>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        name: T,
    ) {
        let key = name.into();
        if !self.bind_groups.contains_key(&key) {
            panic!("Resource Manager: Couldn't find any bind groups! {key}");
        }
        let bind_groups = self.bind_groups.get(&key).unwrap();

        for (key, val) in bind_groups.iter() {
            render_pass.set_bind_group(*key, val, &[]);
        }
    }


    fn add_bind_group_layout<T: Into<String>>(
        &mut self,
        name: T,
        bind_group_layout: BindGroupLayout,
    ) {
        let key = name.into();
        if self.bind_group_layouts.contains_key(&key) {
            panic!(
                "Bind group layout already exists use `get_bind_group_layout` or a different key."
            );
        }
        self.bind_group_layouts
            .insert(key, Arc::new(bind_group_layout));
    }

    pub fn get_bind_group_layout<T: Into<String>>(
        &self,
        name: T,
    ) -> Option<Arc<BindGroupLayout>> {
        let key = name.into();
        match self.bind_group_layouts.get(&key) {
            Some(layout) => Some(layout.clone()),
            None => None,
        }
    }


    fn add_mesh<T: Into<String>>(&mut self, name: T, mesh: Mesh) {
        let name = name.into();
        if self.meshes_by_atlas.contains_key(&name) {
            panic!("Buffer already exists use `get_buffer` or use a different key.");
        }
        self.meshes_by_atlas.insert(name, mesh);
    }

    fn render_meshes<'a, T: Into<String>>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        name: T,
    ) {
        let mesh = self.meshes_by_atlas.get(&name.into()).unwrap();
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, mesh.instance_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..mesh.num_indices, 0, 0..mesh.num_instances);
    }

    fn add_buffer<T: Into<String>>(&mut self, name: T, buffer: Buffer) {
        let name = name.into();
        if self.buffers.contains_key(&name) {
            panic!("Buffer already exists use `get_buffer` or use a different key.");
        }
        self.buffers.insert(name, Arc::new(buffer));
    }


    pub fn get_buffer<T: Into<String>>(&self, name: T) -> Arc<Buffer> {
        self.buffers.get(&name.into()).unwrap().clone()
    }

    pub fn update_mesh_instance<T: Into<String>>(&mut self,
                                                 name: T,
                                                 renderer : &RenderState,
                                                 tile_instance: Vec<InstanceTileRaw>
    ) {
        let instance_buffer = renderer.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&tile_instance),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        let mesh = self.meshes_by_atlas.get_mut(&name.into()).unwrap();
        mesh.replace_instance(instance_buffer, tile_instance.len() as u32);
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
    ) {
        self.set_bind_group(render_pass, "camera");


        self.set_bind_group(render_pass, "world_atlas");
        self.render_meshes(render_pass, "world_atlas");

        self.set_bind_group(render_pass, "fx_atlas");
        self.render_meshes(render_pass, "fx_atlas");


        self.set_bind_group(render_pass, "character/clothes");
        self.render_meshes(render_pass, "character/clothes");

        self.set_bind_group(render_pass, "character/head_long");
        self.render_meshes(render_pass, "character/clothes");
    }
}