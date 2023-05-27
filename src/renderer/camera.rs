use cgmath::{Point3, SquareMatrix};

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,

    uniform: CameraUniform
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 2.0, 15.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: 1.44,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
            uniform : CameraUniform::new(),
        }
    }

}

impl Camera {
    pub fn new(aspect_ratio : f32)-> Self {
        Self {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 2.0, 30.0).into(),
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