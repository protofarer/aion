use std::time::{self, Duration};

use crate::game::{RunState, WindowDims};
use crate::time::Dt;
use crate::{components::*, dev};
use hecs::{Query, QueryBorrow, With, World};
use nalgebra_glm::Vec2;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

// todo ai input -> rotationalinputcpt
// human input -> rotationalinputcpt

pub fn system_process_ship_controls(
    world: &mut World,
    runstate: RunState,
    input: &WinitInputHelper,
) {
    for (_id, (rotational_input, move_attributes, transform, rigidbody, rotatablebody)) in world
        .query_mut::<(
            &mut RotationalInputCpt,
            &MoveAttributesCpt,
            &mut TransformCpt,
            &mut RigidBodyCpt,
            &mut RotatableBodyCpt,
        )>()
    {
        set_rotational_input(input, runstate, rotational_input);

        // read rotational input, change turn rate
        match rotational_input.turn_sign {
            Some(Turn::Right) => {
                rotatablebody.rotation_rate = move_attributes.turn_rate;
            }
            Some(Turn::Left) => {
                rotatablebody.rotation_rate = -move_attributes.turn_rate;
            }
            None => {
                rotatablebody.rotation_rate = 0.;
            }
        }

        // todo update heading

        // read thrust input and current heading, change rigidbody velocity
        match rotational_input.is_thrusting {
            true => {
                rigidbody.velocity.x = (transform.heading).cos() * move_attributes.speed as f32;
                rigidbody.velocity.y = (transform.heading).sin() * move_attributes.speed as f32;
            }
            false => {
                // test when decel cross 0 in either direction
                // todo don't hard set here, simply make no change, for maintaining momentum in prototype stage
                // todo repeats when no key is pressed...
                rigidbody.velocity = Vec2::default();
            }
        }
    }
}

// todo Ideally: key input event -> key<->control mapping -> control event or set control component
fn set_rotational_input(
    input: &WinitInputHelper,
    runstate: RunState,
    rotational_input: &mut RotationalInputCpt,
) {
    if runstate == RunState::Running {
        // HANDLE SINGLE MOVE KEYS
        if input.key_pressed(VirtualKeyCode::D) || input.key_held(VirtualKeyCode::D) {
            rotational_input.turn_sign = Some(Turn::Right);
        }
        if input.key_pressed(VirtualKeyCode::A) || input.key_held(VirtualKeyCode::A) {
            rotational_input.turn_sign = Some(Turn::Left);
        }
        if input.key_pressed(VirtualKeyCode::W) || input.key_held(VirtualKeyCode::W) {
            rotational_input.is_thrusting = true;
        }
    }

    // HANDLE KEY UPS
    if input.key_released(VirtualKeyCode::D) || input.key_released(VirtualKeyCode::A) {
        rotational_input.turn_sign = None;
    }
    if input.key_released(VirtualKeyCode::W) {
        rotational_input.is_thrusting = false;
    }
}

pub fn system_integrate_rotation(world: &mut World, dt: &Dt) {
    for (id, (transform, rotatablebody)) in
        world.query_mut::<(&mut TransformCpt, &RotatableBodyCpt)>()
    {
        transform.heading = (transform.heading + rotatablebody.rotation_rate * dt.0.as_secs_f32())
            % (2.0 * nalgebra_glm::pi::<f32>());
    }
}

pub fn system_integrate_translation(world: &mut World, dt: &Dt) {
    for (id, (transform, rigidbody)) in world.query_mut::<(&mut TransformCpt, &RigidBodyCpt)>() {
        transform.position += rigidbody.velocity * dt.0.as_secs_f32();
    }
}

// #[system(for_each)]
// pub fn update_positions(
//     transform: &mut TransformCpt,
//     rigidbody: &mut RigidBodyCpt,
//     #[resource] dt: &Duration,
// ) {
//     // transform.position.x += rigidbody.velocity.x * dt.0;
//     // transform.position.y += rigidbody.velocity.y * dt.0;
//     // ! does this work?
//     transform.position += rigidbody.velocity * dt.as_secs_f32();
// }

// #[system(for_each)]
// pub fn process_translational_input(
//     input: &TranslationalInputCpt,
//     movestats: &MovementStatsCpt,
//     rigidbody: &mut RigidBodyCpt,
// ) {
//     match input.direction {
//         Some(Direction::E) => {
//             rigidbody.velocity.x = movestats.speed as f32;
//             rigidbody.velocity.y = 0f32;
//         }
//         Some(Direction::SE) => {
//             let component_speed_f32 = movestats.speed as f32;
//             let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
//             rigidbody.velocity.x = component_speed;
//             rigidbody.velocity.y = component_speed;
//         }
//         Some(Direction::S) => {
//             rigidbody.velocity.x = 0f32;
//             rigidbody.velocity.y = movestats.speed as f32;
//         }
//         Some(Direction::SW) => {
//             let component_speed_f32 = movestats.speed as f32;
//             let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
//             rigidbody.velocity.x = -component_speed;
//             rigidbody.velocity.y = component_speed;
//         }
//         Some(Direction::W) => {
//             rigidbody.velocity.x = -(movestats.speed as f32);
//             rigidbody.velocity.y = 0f32;
//         }
//         Some(Direction::NW) => {
//             let component_speed_f32 = movestats.speed as f32;
//             let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
//             rigidbody.velocity.x = -component_speed;
//             rigidbody.velocity.y = -component_speed;
//         }
//         Some(Direction::N) => {
//             rigidbody.velocity.x = 0f32;
//             rigidbody.velocity.y = -(movestats.speed as f32);
//         }
//         Some(Direction::NE) => {
//             let component_speed_f32 = movestats.speed as f32;
//             let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
//             rigidbody.velocity.x = component_speed;
//             rigidbody.velocity.y = -component_speed;
//         }
//         None => {
//             rigidbody.velocity = Vec2::new(0.0, 0.0);
//         }
//     }
// }

// #[system(for_each)]
// pub fn world_boundary_bounce_rect(
//     transform: &mut TransformCpt,
//     rigidbody: &mut RigidBodyCpt,
//     collision_area: &BoxColliderCpt,
//     #[resource] window_dims: &WindowDims,
// ) {
//     if transform.position.x + collision_area.w as f32 >= window_dims.w as f32
//         || transform.position.x < 0f32
//     {
//         rigidbody.velocity.x = -rigidbody.velocity.x;
//     }
//     if transform.position.x < 0f32 {
//         transform.position.x = 0f32;
//     } else if transform.position.x + collision_area.w as f32 >= window_dims.w as f32 {
//         transform.position.x = (window_dims.w - collision_area.w) as f32;
//     }

//     if transform.position.y + collision_area.h as f32 >= window_dims.h as f32
//         || transform.position.y < 0f32
//     {
//         rigidbody.velocity.y = -rigidbody.velocity.y;
//     }
//     if transform.position.y < 0f32 {
//         transform.position.y = 0f32;
//     } else if transform.position.y + collision_area.h as f32 >= window_dims.h as f32 {
//         transform.position.y = (window_dims.h - collision_area.h) as f32;
//     }
// }

// #[system(for_each)]
// pub fn world_boundary_bounce_circle(
//     transform: &mut TransformCpt,
//     rigidbody: &mut RigidBodyCpt,
//     collision_circle: &CircleColliderCpt,
//     #[resource] window_dims: &WindowDims,
// ) {
//     if transform.position.x + collision_circle.r as f32 >= window_dims.w as f32
//         || transform.position.x - collision_circle.r < 0f32
//     {
//         rigidbody.velocity.x = -rigidbody.velocity.x;
//     }
//     if transform.position.x - collision_circle.r < 0f32 {
//         transform.position.x = 0f32 + collision_circle.r;
//     } else if transform.position.x + collision_circle.r as f32 >= window_dims.w as f32 {
//         transform.position.x = (window_dims.w - collision_circle.r) as f32;
//     }

//     if transform.position.y + collision_circle.r as f32 >= window_dims.h as f32
//         || transform.position.y < 0f32
//     {
//         rigidbody.velocity.y = -rigidbody.velocity.y;
//     }
//     if transform.position.y - collision_circle.r < 0f32 {
//         transform.position.y = 0f32 + collision_circle.r;
//     } else if transform.position.y + collision_circle.r as f32 >= window_dims.h as f32 {
//         transform.position.y = (window_dims.h - collision_circle.r) as f32;
//     }
// }

// // ? how to have this run only when input changes?
// #[system(for_each)]
// pub fn process_rotational_input(
//     rotational_input: &RotationalInputCpt,
//     move_stats: &MovementStatsCpt,
//     transform: &mut TransformCpt,
//     rigidbody: &mut RigidBodyCpt,
// ) {
//     match rotational_input.turn_sign {
//         Some(Turn::Right) => {
//             transform.rotation =
//                 (transform.rotation + move_stats.turn_rate) % (2.0 * nalgebra_glm::pi::<f32>());
//         }
//         Some(Turn::Left) => {
//             transform.rotation =
//                 (transform.rotation - move_stats.turn_rate) % (2.0 * nalgebra_glm::pi::<f32>());
//         }
//         _ => {}
//     }
//     match rotational_input.is_thrusting {
//         true => {
//             rigidbody.velocity.x = (transform.rotation).cos() * move_stats.speed as f32;
//             rigidbody.velocity.y = (transform.rotation).sin() * move_stats.speed as f32;
//         }
//         false => {
//             // test when decel cross 0 in either direction
//             // todo don't hard set here, simply make no change, for maintaining momentum in prototype stage
//             // todo repeats when no key is pressed...
//             rigidbody.velocity = Vec2::default();
//         }
//     }
// }

// #[system(for_each)]
// pub fn projectile_emission(
//     human_input: &HumanInputCpt,
//     craft_action: &CraftActionStateCpt,
//     transform: &TransformCpt,
//     proj_em: &mut ProjectileEmitterCpt,
//     commands: &mut CommandBuffer,
// ) {
//     if craft_action.is_firing_primary {
//         if proj_em.last_emission_time.is_none() {
//             proj_em.last_emission_time = Some(time::Instant::now());
//         } else if proj_em.last_emission_time.unwrap().elapsed().as_millis()
//             >= proj_em.cooldown as u128
//         {
//             proj_em.last_emission_time = Some(time::Instant::now());
//             commands.push((TransformCpt::new()));
//             // commands.push(
//             // (ProjectileCpt {
//             //     is_friendly: true,
//             //     hit_damage: 10,
//             //     duration: Duration::new(0, 10_000_000_000),
//             //     start_time: time::Instant::now(),
//             // }),
//             // );
//         }
//     }
// }

// #[system]
// pub fn circle_collision(
//     query: &mut Query<(Entity, &TransformCpt, &CircleColliderCpt)>,
//     commands: &mut CommandBuffer,
//     world: &mut SubWorld,
// ) {
//     let mut entities: Vec<(&Entity, &TransformCpt, &CircleColliderCpt)> = vec![];
//     let chunks = query;
//     for (entity_a, tx, cx) in chunks.iter(world) {
//         entities.push((entity_a, tx, cx));
//     }
//     let mut colliding_entities: Vec<&Entity> = vec![];
//     for (i, (entity1, tx1, cx1)) in entities.iter().enumerate() {
//         for (entity2, tx2, cx2) in entities[i + 1..].iter() {
//             let dx = tx2.position.x - tx1.position.x;
//             let dy = tx2.position.y - tx1.position.y;
//             let dr = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
//             if dr < (cx1.r + cx2.r) {
//                 dev!("COLLISION DETECTED");
//                 colliding_entities.push(entity1);
//                 colliding_entities.push(entity2);
//             }
//         }
//     }
//     for &entity in colliding_entities {
//         commands.remove(entity);
//     }

// for (i, entity1, transform, collision_circle) in query.iter(world).enumerate() {
//     for (entity2, transform, collision_circle) in )

// }
// }
