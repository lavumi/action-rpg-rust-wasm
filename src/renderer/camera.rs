use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;
use crate::renderer::GPUResourceManager;

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,

    uniform: CameraUniform
}

impl Default for Camera {
    fn default() -> Self {
        todo!()
    }
}

impl Camera {
    pub fn new(aspect_ratio : f32)-> Self {
        Self {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 2.0, 15.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: aspect_ratio,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
            uniform : CameraUniform::new(),
        }
    }

    pub fn build(&self, gpu_resource_manager: &mut GPUResourceManager, device : &wgpu::Device){
        let camera_uniform : [[f32; 4]; 4] = cgmath::Matrix4::identity().into();
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );


        let resources = camera_buffer.as_entire_binding();
        let camera_bind_group_layout = gpu_resource_manager.get_bind_group_layout("camera").unwrap();
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
        gpu_resource_manager.add_buffer("camera_matrix", camera_buffer);
        gpu_resource_manager.add_bind_group("simple_texture" ,0 , camera_bind_group );
    }

    pub fn update_view_proj(&mut self) -> [[f32; 4]; 4]{
        let vp = self.build_view_projection_matrix();
        self.uniform.update_view_proj(vp);
        self.uniform.view_proj
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fov_y), self.aspect, self.z_near, self.z_far);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, vp: cgmath::Matrix4<f32>) {
        self.view_proj = vp.into();
    }
}