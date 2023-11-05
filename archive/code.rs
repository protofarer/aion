// KEY STATE FOR SDL
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

// RAW WINIT EVENT LOOP
// fn run(
//     event_loop: EventLoop<()>,
//     window: Window,
//     mut pixels: Pixels,
//     mut framework: Framework,
//     mut input: WinitInputHelper,
//     mut ctx: Context,
//     mut game: Game,
// ) -> Result<(), Error> {
//     game.setup();
//     game.loop_controller.run();
//     let mut timer = FrameTimer::new(16);

//     event_loop.run(move |event, _, control_flow| {
//         control_flow.set_poll();

//         // Handle input events
//         if input.update(&event) {
//             if input.close_requested()
//                 || (*game.get_loopstate() == LoopState::Stopped
//                     && input.key_pressed(VirtualKeyCode::Escape))
//             {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }

//             if let Some(scale_factor) = input.scale_factor() {
//                 framework.scale_factor(scale_factor);
//             }

//             if let Some(size) = input.window_resized() {
//                 if let Err(err) = pixels.resize_surface(size.width, size.height) {
//                     log_error("pixels.resize_surface", err);
//                     *control_flow = ControlFlow::Exit;
//                     return;
//                 }
//                 framework.resize(size.width, size.height);
//             }

//             game.process_input();

//             if *game.get_loopstate() == LoopState::Exiting {
//                 *control_flow = ControlFlow::Exit;
//             }
//         }

//         match event {
//             Event::MainEventsCleared => {
//                 ////////////////////////////////////////////////////////////////////
//                 // UPDATE
//                 ////////////////////////////////////////////////////////////////////
//                 // if *game.get_loopstate() == LoopState::Running {
//                 //     game.update();
//                 // }
//                 ////////////////////////////////////////////////////////////////////
//                 ////////////////////////////////////////////////////////////////////
//                 window.request_redraw();
//             }
//             Event::WindowEvent { event, .. } => {
//                 framework.handle_event(&event);
//             }
//             Event::RedrawRequested(_) => {
//                 let _dt = timer.tick();
//                 println!("dt: {}", _dt.as_millis());
//                 ////////////////////////////////////////////////////////////////////
//                 // RENDER
//                 ////////////////////////////////////////////////////////////////////
//                 // Clear current rendering target with drawing color
//                 // a faster clear?
//                 for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
//                     pixel.copy_from_slice(BLACK.as_bytes());
//                 }

//                 // Mutate frame buffer
//                 // game.draw(pixels.frame_mut());

//                 // Prepare egui
//                 // framework.prepare(&window);

//                 // Render everything together
//                 let render_result = pixels.render_with(|encoder, render_target, context| {
//                     // Render the world texture
//                     context.scaling_renderer.render(encoder, render_target);

//                     // Render egui
//                     // framework.render(encoder, render_target, context);

//                     Ok(())
//                 });

//                 if let Err(err) = render_result {
//                     log_error("pixels.render", err);
//                     *control_flow = ControlFlow::Exit;
//                 }
//                 ////////////////////////////////////////////////////////////////////
//                 ////////////////////////////////////////////////////////////////////
//             }
//             _ => (),
//         }
//     });
// }
