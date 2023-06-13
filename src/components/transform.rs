// use specs::{Component, VecStorage};
//
// pub struct Transform{
//     pub(crate) position: [f32;3],
//     pub(crate) flip: bool,
// }
//
// impl Component for Transform {
//     type Storage = VecStorage<Self>;
// }
//
// impl Transform {
//     pub fn move_tile(&mut self ,delta: [f32;2]){
//         self.position[0] += delta[0];
//         self.position[1] += delta[1];
//
//         if delta[0] > 0.0 {
//             self.flip = true;
//         }
//         else if delta[0] < 0.0 {
//             self.flip = false;
//         }
//     }
// }