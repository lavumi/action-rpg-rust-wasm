use rand::Rng;
use rand::rngs::ThreadRng;
use specs::{Read, System, Write, WriteStorage};
use wgpu::Buffer;

use crate::components::cube_instance::CubeInstance;
use crate::components::mesh::Mesh;
use crate::renderer::RenderState;
use crate::renderer::vertex::Instance;
use crate::resources::delta_time::DeltaTime;

pub struct CubeShuffle;

impl<'a> System<'a> for CubeShuffle {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Mesh>,
        WriteStorage<'a, CubeInstance>,
        Write<'a, ThreadRng>,
        Read<'a, RenderState>
    );

    fn run(&mut self, (dt, mut meshes, mut instances, mut rng, renderer): Self::SystemData) {
        use specs::Join;
        for ( mesh, instant ) in (&mut meshes, &mut instances).join() {
            instant.time_spend += dt.0;
            if instant.time_spend > 1.0 {
                instant.time_spend = 0.0;
                instant.rpy_rnd =  rng.gen_range(0..9) as usize;
                continue;
            }

            if instant.rpy_rnd == 99 {
                continue;
            }
            instant.run_cube();

            //todo think 여기서 instance buffer 까지 업데이트 해 주는게 옳은 방법일까?
            let instance_data = instant
                .instances
                .iter()
                .map(Instance::to_raw)
                .collect::<Vec<_>>();

            renderer.queue.write_buffer(&mesh.instance_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&instance_data));

        }
    }
}