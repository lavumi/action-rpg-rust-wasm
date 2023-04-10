// use winit::dpi::PhysicalPosition;
// use winit::event::WindowEvent;
//
// pub enum UserInput{
//     CursorMoved(PhysicalPosition<f64>),
//     KeyboardInput(winit::event::KeyboardInput)
// }
//
// pub struct InputHandler{
//     cursor_moved : UserInput,
// }
//
//
//
// impl InputHandler {
//     pub fn new (cursor_move : UserInput) -> Self {
//         Self {
//             cursor_moved : cursor_move
//         }
//     }
//
//     // pub fn set_cursor_moved(mut self, input : UserInput) -> bool {
//     //     match input {
//     //         UserInput::CursorMoved( position ) =>{
//     //             self.cursor_moved = input;
//     //             true
//     //         }
//     //         _ => false
//     //     }
//     // }
//
//     pub fn cursor_moved(self, point :&PhysicalPosition<f64> ) -> bool {
//         self.cursor_moved(point);
//         true
//     }
// }