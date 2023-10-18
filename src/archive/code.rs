// fn set_translational_input(&mut self, key_downs: Vec<Keycode>) {
//     let mut set_input_dir = |dir: Direction| {
//         let mut query = <&mut TranslationalInput>::query();
//         for input in query.iter_mut(&mut self.world) {
//             input.direction = Some(dir);
//         }
//     };

//     if let RunState::Running = self.run_state {
//         // ONLY ACTIVATE FOR TRANSLATIONAL HUMAN INPUTS... query the "player" input type
//         if key_downs.contains(&Keycode::W) && key_downs.contains(&Keycode::D) {
//             set_input_dir(Direction::NE)
//         } else if key_downs.contains(&Keycode::S) && key_downs.contains(&Keycode::D) {
//             set_input_dir(Direction::SE);
//         } else if key_downs.contains(&Keycode::S) && key_downs.contains(&Keycode::A) {
//             set_input_dir(Direction::SW);
//         } else if key_downs.contains(&Keycode::W) && key_downs.contains(&Keycode::A) {
//             set_input_dir(Direction::NW);
//         } else {
//             // HANDLE SINGLE MOVE KEYS
//             for keycode in key_downs.iter() {
//                 match keycode {
//                     Keycode::D => {
//                         set_input_dir(Direction::E);
//                     }
//                     Keycode::W => {
//                         set_input_dir(Direction::N);
//                     }
//                     Keycode::S => {
//                         set_input_dir(Direction::S);
//                     }
//                     Keycode::A => {
//                         set_input_dir(Direction::W);
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     }
// }
