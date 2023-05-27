use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{BindGroup, Buffer, BindGroupLayout, RenderPass};
use std::default::Default;
use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;
use crate::renderer::{RenderState, Texture};

pub struct GPUResourceManager {
    textures : HashMap<String, Texture>,
    bind_group_layouts: HashMap<String, Arc<BindGroupLayout>>,
    bind_groups: HashMap<String, HashMap<u32, Arc<BindGroup>>>,
    buffers: HashMap<String, Arc<Buffer>>,
}


impl Default for GPUResourceManager{
    fn default() -> Self {
        Self{
            textures: Default::default(),
            bind_group_layouts: Default::default(),
            bind_groups: Default::default(),
            buffers: Default::default(),
        }
    }
}


impl GPUResourceManager {
    pub fn initialize(&mut self,renderer : &RenderState){
        self.load_textures(&renderer);
        self.make_base_bind_group(&renderer);
        self.init_camera_resources(&renderer);
        self.init_base_resources(&renderer);
    }


    fn load_textures(&mut self,renderer : &RenderState){
        let device = &renderer.device;
        let queue = &renderer.queue;
        let diffuse_texture =
            Texture::from_bytes(device, queue, include_bytes!("../../assets/atlas.png"), "").unwrap();
        self.textures.insert("atlas".to_string() ,diffuse_texture );
    }

    fn make_base_bind_group(&mut self,renderer : &RenderState){
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


    fn init_base_resources(&mut self,renderer : &RenderState){
        let device = &renderer.device;
        let diffuse_texture = self.textures.get("atlas".into()).unwrap().clone();
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

        self.add_bind_group("instance" , 1 , diffuse_bind_group);
    }

    fn init_camera_resources(&mut self,renderer : &RenderState){
        let device = &renderer.device;
        let camera_uniform : [[f32; 4]; 4] = cgmath::Matrix4::identity().into();
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
        self.add_bind_group("instance" ,0 , camera_bind_group );

    }



    pub fn add_bind_group_layout<T: Into<String>>(
        &mut self,
        name: T,
        bind_group_layout: BindGroupLayout,
    ){
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
        name: T
    ) -> Option<Arc<BindGroupLayout>> {
        let key = name.into();
        match self.bind_group_layouts.get(&key) {
            Some(layout) => Some(layout.clone()),
            None => None,
        }
    }

    pub fn add_bind_group<T: Into<String>>(
        &mut self,
        name: T,
        bind_group_index : u32,
        bind_group: BindGroup,
    ){
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

    pub fn set_bind_group<'a, T: Into<String>>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        name: T
    ) {
        let key = name.into();
        if !self.bind_groups.contains_key(&key) {
            panic!("Resource Manager: Couldn't find any bind groups!");
        }
        let bind_groups = self.bind_groups.get(&key).unwrap();

        for (key, val) in bind_groups.iter() {
            render_pass.set_bind_group(*key, val, &[]);
        }
    }

    pub fn add_buffer<T: Into<String>>(&mut self, name: T, buffer: Buffer) {
        let name = name.into();
        if self.buffers.contains_key(&name) {
            panic!("Buffer already exists use `get_buffer` or use a different key.");
        }
        self.buffers.insert(name, Arc::new(buffer));
    }

    pub fn get_buffer<T: Into<String>>(&self, name: T) -> Arc<Buffer> {
        self.buffers.get(&name.into()).unwrap().clone()
    }
}