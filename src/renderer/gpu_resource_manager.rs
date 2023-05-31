use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{BindGroup, Buffer, BindGroupLayout, RenderPass};
use std::default::Default;
use cgmath::SquareMatrix;
use rand::Rng;
use wgpu::util::DeviceExt;
use crate::components::mesh::Mesh;
use crate::object::make_tile_single;
use crate::renderer::{RenderState, Texture};
use crate::renderer::vertex::TileInstance;

pub struct GPUResourceManager {
    textures : HashMap<String, Texture>,
    bind_group_layouts: HashMap<String, Arc<BindGroupLayout>>,
    bind_groups: HashMap<String, HashMap<u32, Arc<BindGroup>>>,
    buffers: HashMap<String, Arc<Buffer>>,
    //todo 이거 Arc<Mesh>로 바꿔야하는데...
    meshes_by_atlas: HashMap<String, Mesh>,
}


impl Default for GPUResourceManager{
    fn default() -> Self {
        Self{
            textures: Default::default(),
            bind_group_layouts: Default::default(),
            bind_groups: Default::default(),
            buffers: Default::default(),
            meshes_by_atlas: Default::default()
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
            Texture::from_bytes(device, queue, include_bytes!("../../assets/world_atlas.png"), "").unwrap();
        self.textures.insert("world".to_string() ,diffuse_texture );
        self.add_mesh("world" , make_tile_single(&renderer, "world", 2.0, [0., 0.],[1.0 / 35., 1.0 / 41.]));


        self.update_mesh_instance("world", renderer);

        let device = &renderer.device;
        let queue = &renderer.queue;
        let diffuse_texture =
            Texture::from_bytes(device, queue, include_bytes!("../../assets/creature_atlas.png"), "").unwrap();
        self.textures.insert("creature".to_string() ,diffuse_texture );
        self.add_mesh("creature" , make_tile_single(&renderer, "creature", 2.0, [3.0/32.,0.],[1.0/32.,1.0/41.]));
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
        self.make_bind_group("world",renderer);
        self.make_bind_group("creature",renderer);
    }

    fn make_bind_group(&mut self, name: &str, renderer : &RenderState){
        let device = &renderer.device;
        let diffuse_texture = self.textures.get(name).unwrap().clone();
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

        self.add_bind_group(name , 1 , diffuse_bind_group);
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
        self.add_bind_group("camera" ,0 , camera_bind_group );
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

    pub fn update_mesh_instance<T: Into<String>>(&mut self, name: T, renderer : &RenderState) {
        let instances =
            (0..40).flat_map( |x| {
                (0..40).map(move |y| {
                    let position = cgmath::Vector3 { x: (x  as f32 - 20.5 )  * 2.0, y: (y - 20) as f32  * 2.0, z:  -1.0 };
                    // let mut rng = rand::thread_rng();
                    let tile = 0;//rng.gen_range(0..4);
                    let tile_x = tile  as f32 * 1.0 / 35.;
                    let tile_y = 0.0;//(tile%2) as f32 * 0.02439;
                    TileInstance{
                        uv: cgmath::Vector2 { x: tile_x  , y:  tile_y},
                        model_matrix: cgmath::Matrix4::from_translation(position),
                    }
                })
            }).collect::<Vec<_>>();
        let instance_data = instances.iter().map(TileInstance::to_tile_raw).collect::<Vec<_>>();
        let instance_buffer = renderer.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        let mut mesh = self.meshes_by_atlas.get_mut(&name.into()).unwrap();
        mesh.replace_instance(instance_buffer, instance_data.len() as u32);
        // renderer.queue.write_buffer(&mesh.instance_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&instance_data));
    }

    fn add_mesh<T: Into<String>>(&mut self, name: T, mesh: Mesh){
        let name = name.into();
        if self.meshes_by_atlas.contains_key(&name) {
            panic!("Buffer already exists use `get_buffer` or use a different key.");
        }
        self.meshes_by_atlas.insert(name, mesh);
    }


    pub fn render_meshes<'a, T: Into<String>>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        name: T
    ){
        let mesh = self.meshes_by_atlas.get(&name.into()).unwrap();
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, mesh.instance_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..mesh.num_indices, 0, 0..mesh.num_instances);
    }
}