// use cgmath::{InnerSpace, Rotation3, Zero};
// use wgpu::Device;
// use wgpu::util::DeviceExt;
// use crate::vertex::{Instance, InstanceRaw, Vertex};
//
//
// const NUM_INSTANCES_PER_ROW: u32 = 1;
// const ROTATION_SPEED:f32 = 0.2;
//
// #[allow(unused_variables)]
// pub struct Cube {
//     // pub(crate) vertex_buffer: wgpu::Buffer,
//     // pub(crate) index_buffer: wgpu::Buffer,
//     // pub(crate) instance_buffer: wgpu::Buffer,
//
//
//
//     pub(crate) vertex: [Vertex; 16] ,
//     pub(crate) indices: [u16; 24],
//
//     instances: Vec<Instance>,
//     pub(crate) instance_data : Vec<InstanceRaw>
// }
//
//
// impl Cube {
//     pub fn new(device : &Device) -> Self {
//
//
//
//
//
//         // let vertex_buffer = device.create_buffer_init(
//         //     &wgpu::util::BufferInitDescriptor {
//         //         label: Some("Vertex Buffer"),
//         //         contents: bytemuck::cast_slice(VERTICES),
//         //         usage: wgpu::BufferUsages::VERTEX,
//         //     }
//         // );
//         //
//         // let index_buffer = device.create_buffer_init(
//         //     &wgpu::util::BufferInitDescriptor {
//         //         label: Some("Index Buffer"),
//         //         contents: bytemuck::cast_slice(INDICES),
//         //         usage: wgpu::BufferUsages::INDEX,
//         //     }
//         // );
//         // let num_indices = INDICES.len() as u32;
//         // let instance_buffer = device.create_buffer_init(
//         //     &wgpu::util::BufferInitDescriptor {
//         //         label: Some("Instance Buffer"),
//         //         contents: bytemuck::cast_slice(&instance_data),
//         //         usage: wgpu::BufferUsages::VERTEX,
//         //     }
//         // );
//         //
//         // let num_instances = instances.len() as u32;
//
//         Self{
//             instances,
//             vertex,
//             indices,
//             instance_data
//         }
//     }
//
//     pub fn update(&mut self){
//         for instance in &mut self.instances{
//             let amount = cgmath::Quaternion::from_angle_y(cgmath::Rad(ROTATION_SPEED));
//             let current = instance.rotation;
//             instance.rotation = amount * current;
//         }
//     }
//
//     // pub fn get_instance_data(&self) -> Box<[InstanceRaw]> {
//     //     self.instances
//     //         .iter()
//     //         .map(Instance::to_raw)
//     //         .collect::<A<_>>()
//     // }
// }