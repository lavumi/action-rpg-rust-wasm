use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;

use cgmath::SquareMatrix;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue, RenderPass};
use wgpu::util::DeviceExt;

use crate::object::make_tile_mesh;
use crate::renderer::mesh::{InstanceTileRaw, Mesh};
use crate::renderer::Texture;

pub struct GPUResourceManager {
    bind_group_layouts: HashMap<String, Arc<BindGroupLayout>>,
    bind_groups: HashMap<String, HashMap<u32, Arc<BindGroup>>>,
    buffers: HashMap<String, Arc<Buffer>>,
    meshes_by_atlas: HashMap<String, Mesh>
}

impl Default for GPUResourceManager {
    fn default() -> Self {
        Self {
            bind_group_layouts: Default::default(),
            bind_groups: Default::default(),
            buffers: Default::default(),
            meshes_by_atlas: Default::default()
        }
    }
}

impl GPUResourceManager {

    pub fn initialize(&mut self, device: &Device) {
        self.init_base_layouts(&device);
        self.init_camera_bind_group(&device);
    }

    pub fn init_atlas(&mut self, device: &Device, queue: &Queue) {
        let diffuse_texture = Texture::from_bytes(device, queue, include_bytes!("../../assets/map/forest-cliff.png"), "forest").unwrap();
        self.make_bind_group("world", diffuse_texture, device);

        let diffuse_texture = Texture::from_bytes(device, queue, include_bytes!("../../assets/enemy/zombie.png"), "zombie").unwrap();
        self.make_bind_group("enemy/zombie", diffuse_texture, device);

        let diffuse_texture = Texture::from_bytes(device, queue, include_bytes!("../../assets/effects/projectiles.png"), "projectiles").unwrap();
        self.make_bind_group("projectiles", diffuse_texture, device);


        let diffuse_texture = Texture::from_bytes(device, queue, include_bytes!("../../assets/character/character.png"), "character").unwrap();
        self.make_bind_group("character", diffuse_texture, device);
    }

    pub fn init_meshes(&mut self, device: &Device) {
        self.add_mesh("world", make_tile_mesh(device, "world".to_string()));
        self.add_mesh("projectiles", make_tile_mesh(device, "projectiles".to_string()));
        self.add_mesh("character", make_tile_mesh(device, "character".to_string()));
        self.add_mesh("enemy/zombie", make_tile_mesh(device, "enemy/zombie".to_string()));
        self.add_mesh("test", make_tile_mesh(device, "test".to_string()));
    }

    fn init_base_layouts(&mut self, device: &Device) {
        self.add_bind_group_layout(
            "texture_bind_group_layout",
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

    fn init_camera_bind_group(&mut self, device: &Device) {
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

    fn make_bind_group<T: Into<String> + Copy>(&mut self, name: T, diffuse_texture: Texture, device: &Device) {
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

    pub fn set_bind_group<'a, T: Into<String>>(
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

        match mesh.instance_buffer {
            None => {
                // render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                // render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                // render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            }
            Some(_) => {
                //코드가 좀 안예쁘군...
                self.set_bind_group(render_pass, mesh.atlas_name.clone());

                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, mesh.instance_buffer.as_ref().unwrap().slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.num_indices, 0, 0..mesh.num_instances);
            }
        }
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
                                                 device: &Device,
                                                 queue: &Queue,
                                                 tile_instance: Vec<InstanceTileRaw>,
    ) {
        let name_str = name.into();
        let mesh = self.meshes_by_atlas.get_mut(&name_str).unwrap();
        if tile_instance.len() == 0 {
            //todo 0일 경우 기존의 버퍼를 삭제하는것이 아님. 이거때문에 오버해드 있을지도?
            //이게 왜 오버해드지? 오히려 없는거 아닌가? 과거의 나! 무슨 생각 이었나?
            mesh.num_instances = 0;
            return;
        }
        if mesh.num_instances == tile_instance.len() as u32 {
            queue.write_buffer(mesh.instance_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&tile_instance));
        } else {
            log::info!("update_mesh_instance {} before : {} , after : {}", name_str ,mesh.num_instances , tile_instance.len() );
            let instance_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(format!("Instance Buffer {}", name_str).as_str()),
                    contents: bytemuck::cast_slice(&tile_instance),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }
            );
            mesh.replace_instance(instance_buffer, tile_instance.len() as u32);
        }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
    ) {
        self.set_bind_group(render_pass, "camera");
        self.render_meshes(render_pass, "world");
        self.render_meshes(render_pass, "character");
        self.render_meshes(render_pass, "enemy/zombie");
        self.render_meshes(render_pass, "projectiles");
    }
}