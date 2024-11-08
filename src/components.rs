use specs::*;
use specs_derive::Component;

#[derive(Component, Clone)]
pub struct Animation {
    pub anime_name: String,
    pub speed: f32,
    pub index: usize,
    pub frame: usize,
    pub dt: f32,
}

#[derive(Component, Clone)]
pub struct Movable(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction { Left, UpLeft, Up, UpRight, Right, DownRight, Down, DownLeft, None }

#[derive(Component, Clone)]
pub struct Forward {
    pub direction: Direction,
    pub right: bool,
}

#[derive(Component, Clone)]
pub struct Attack {
    pub duration: f32,
    pub dt: f32,
    pub movement: [f32; 2],
}

#[derive(Default, Component, Clone)]
pub struct AttackMaker {
    pub fire: bool,
}

#[derive(Component, Copy, Clone)]
pub struct Enemy {
    pub speed: f32,
    pub tick: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum BodyType { Static, Kinematic, Dynamic }

#[derive(Component, Clone)]
pub struct Collider {
    pub aabb_offset: [f32; 4],
    pub velocity: [f32; 2],
    pub is_trigger: bool,
    pub body_type: BodyType,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            aabb_offset: [-1.0, 0.0, -0.25, 0.25],
            velocity: [0., 0.],
            is_trigger: false,
            body_type: BodyType::Kinematic,
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}


#[derive(Component, Clone)]
pub struct Tile {
    pub uv: [f32; 4],
    pub atlas: String,
}

#[derive(Component, Clone)]
pub struct Transform {
    pub position: [f32; 3],
    pub size: [f32; 2],
}


impl Transform {
    pub fn new(position: [f32; 3], size: [f32; 2]) -> Self {
        Transform {
            position,
            size,
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let position = cgmath::Vector3 { x: self.position[0], y: self.position[1], z: self.position[2] };
        let translation_matrix = cgmath::Matrix4::from_translation(position);
        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.size[0], self.size[1], 1.0);
        let model = (translation_matrix * scale_matrix).into();
        model
    }
}


/**
convert velocity from tile grid movement to isometric grid movement
 */
pub fn convert_velocity(velocity: [f32; 2]) -> [f32; 2] {
    if velocity[0] != 0. && velocity[1] != 0. {
        let normalize = 0.4472135955;
        [velocity[0] * 2. * normalize, velocity[1] * normalize]
    } else {
        velocity
    }
}


pub fn direction_to_f32_array(dir: Direction) -> [f32; 2] {
    match dir {
        Direction::Left => { [-1., 0.] }
        Direction::UpLeft => { [-1., 1.] }
        Direction::Up => { [0., 1.] }
        Direction::UpRight => { [1., 1.] }
        Direction::Right => { [1., 0.] }
        Direction::DownRight => { [1., -1.] }
        Direction::Down => { [0., -1.] }
        Direction::DownLeft => { [-1., -1.] }
        Direction::None => { [0., 0.] }
    }
}