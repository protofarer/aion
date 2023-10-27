use std::time::{self, Duration};

use crate::game::{RunState, WindowDims};
use crate::time::Dt;
use crate::{components::*, dev, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};
use hecs::{Entity, Query, QueryBorrow, With, Without, World};
use nalgebra_glm::Vec2;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

// todo ai input -> rotationalinputcpt
// human input -> rotationalinputcpt

pub fn system_process_human_input(world: &mut World, runstate: RunState, input: &WinitInputHelper) {
    for (_id, (rotational_input, move_attributes, transform, rigidbody, rotatablebody)) in world
        .query_mut::<With<
            (
                &mut RotationalInputCpt,
                &MoveAttributesCpt,
                &mut TransformCpt,
                &mut RigidBodyCpt,
                &mut RotatableBodyCpt,
            ),
            &HumanInputCpt,
        >>()
    {
        set_rotational_input_component_by_human(input, runstate, rotational_input);

        // set rotation_rate sign
        set_rotatablebody_component(rotational_input, rotatablebody, move_attributes);
        set_rigidbody_component(transform, rotational_input, rigidbody, move_attributes);
    }
}
fn set_rigidbody_component(
    transform: &TransformCpt,
    rotational_input: &RotationalInputCpt,
    rigidbody: &mut RigidBodyCpt,
    move_attributes: &MoveAttributesCpt,
) {
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

fn set_rotatablebody_component(
    rotational_input: &RotationalInputCpt,
    rotatablebody: &mut RotatableBodyCpt,
    move_attributes: &MoveAttributesCpt,
) {
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
}

// fn process_player_control_keys(&mut self) {
// let mut query = <(&HumanInputCpt, &mut RotationalInputCpt)>::query();

// let input = &self.input;
// let runstate = self.get_runstate();

// for (_human, mut rotational_input) in query.iter_mut(&mut self.world) {
//     set_rotational_input(input, runstate, &mut rotational_input);
// }

// if self.input.key_pressed(VirtualKeyCode::Space)
//     || self.input.key_held(VirtualKeyCode::Space)
// {
//     let mut query = <&mut CraftActionStateCpt>::query();
//     for state in query.iter_mut(&mut self.world) {
//         state.is_firing_primary = true;
//     }
// }
// }

// todo Ideally: key input event -> key<->control mapping -> control event or set control component
fn set_rotational_input_component_by_human(
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

// only for CircleColliders
pub fn system_boundary_restrict_circle(world: &mut World) {
    for (id, (transform, rigidbody, collision_circle)) in
        world.query_mut::<(&mut TransformCpt, &mut RigidBodyCpt, &CircleColliderCpt)>()
    {
        if transform.position.x + collision_circle.r as f32 >= LOGICAL_WINDOW_WIDTH as f32
            || transform.position.x < 0f32
        {
            rigidbody.velocity.x = -rigidbody.velocity.x;
        }
        if transform.position.x - collision_circle.r < 0f32 {
            transform.position.x = 0f32 + collision_circle.r;
        } else if transform.position.x + collision_circle.r as f32 >= LOGICAL_WINDOW_WIDTH as f32 {
            transform.position.x = (LOGICAL_WINDOW_WIDTH - collision_circle.r) as f32;
        }

        if transform.position.y + collision_circle.r as f32 >= LOGICAL_WINDOW_HEIGHT as f32
            || transform.position.y - collision_circle.r < 0f32
        {
            rigidbody.velocity.y = -rigidbody.velocity.y;
        }
        if transform.position.y - collision_circle.r < 0f32 {
            transform.position.y = 0f32 + collision_circle.r;
        } else if transform.position.y + collision_circle.r as f32 >= LOGICAL_WINDOW_HEIGHT as f32 {
            transform.position.y = (LOGICAL_WINDOW_HEIGHT - collision_circle.r) as f32;
        }
    }
}

pub fn system_circle_collision(world: &mut World) {
    let mut entities: Vec<(Entity, TransformCpt, CircleColliderCpt)> = vec![];
    // todo improve collecting entity and components, use chain method and
    // collector https://docs.rs/hecs/latest/hecs/struct.QueryBorrow.html
    for (entity, (transform, rigidbody)) in world.query_mut::<(&TransformCpt, &CircleColliderCpt)>()
    {
        entities.push((entity, *transform, *rigidbody));
    }
    let mut colliding_entities: Vec<&Entity> = vec![];
    for (i, (entity1, tx1, cx1)) in entities.iter().enumerate() {
        for (entity2, tx2, cx2) in entities[i + 1..].iter() {
            let dx = tx2.position.x - tx1.position.x;
            let dy = tx2.position.y - tx1.position.y;
            let dr = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
            if dr < (cx1.r + cx2.r) {
                dev!("COLLISION DETECTED");
                colliding_entities.push(entity1);
                colliding_entities.push(entity2);
            }
        }
    }
    for &entity in colliding_entities {
        world.despawn(entity);
    }
}

pub fn system_boundary_restrict_projectile(world: &mut World) {
    for (id, (transform, rigidbody)) in
        world.query_mut::<With<(&mut TransformCpt, &mut RigidBodyCpt), &ParticleColliderCpt>>()
    {
        if transform.position.x >= LOGICAL_WINDOW_WIDTH || transform.position.x < 0f32 {
            rigidbody.velocity.x = -rigidbody.velocity.x;
        }
        if transform.position.x < 0f32 {
            transform.position.x = 0f32;
        } else if transform.position.x >= LOGICAL_WINDOW_WIDTH {
            transform.position.x = LOGICAL_WINDOW_WIDTH - 1.;
        }

        if transform.position.y >= LOGICAL_WINDOW_HEIGHT || transform.position.y < 0f32 {
            rigidbody.velocity.y = -rigidbody.velocity.y;
        }
        if transform.position.y < 0f32 {
            transform.position.y = 0f32;
        } else if transform.position.y >= LOGICAL_WINDOW_HEIGHT {
            transform.position.y = LOGICAL_WINDOW_HEIGHT - 1.;
        }
    }
}

pub fn system_boundary_restrict_particle(world: &mut World) {
    for (id, (transform, rigidbody)) in
        world.query_mut::<Without<
            (&mut TransformCpt, &mut RigidBodyCpt),
            (&ParticleColliderCpt, &CircleColliderCpt),
        >>()
    {
        if transform.position.x >= LOGICAL_WINDOW_WIDTH || transform.position.x < 0f32 {
            rigidbody.velocity.x = -rigidbody.velocity.x;
        }
        if transform.position.x < 0f32 {
            transform.position.x = 0f32;
        } else if transform.position.x >= LOGICAL_WINDOW_WIDTH {
            transform.position.x = LOGICAL_WINDOW_WIDTH - 1.;
        }

        if transform.position.y >= LOGICAL_WINDOW_HEIGHT || transform.position.y < 0f32 {
            rigidbody.velocity.y = -rigidbody.velocity.y;
        }
        if transform.position.y < 0f32 {
            transform.position.y = 0f32;
        } else if transform.position.y >= LOGICAL_WINDOW_HEIGHT {
            transform.position.y = LOGICAL_WINDOW_HEIGHT - 1.;
        }
    }
}

// particles do not collide with each other
pub fn system_particle_collision(world: &mut World) {
    let particle_data = world
        .query::<With<&TransformCpt, &ParticleColliderCpt>>()
        .iter()
        .map(|(e, &tx)| (e, tx))
        .collect::<Vec<_>>();

    let circloid_data = world
        .query::<(&TransformCpt, &CircleColliderCpt)>()
        .iter()
        .map(|(e, (&tx, &cx))| (e, tx, cx))
        .collect::<Vec<_>>();

    let mut colliding_entities: Vec<&Entity> = vec![];
    for (i, (circloid, tx_c, cx_c)) in circloid_data.iter().enumerate() {
        for (particle, tx_p) in particle_data.iter() {
            let dx = tx_p.position.x - tx_c.position.x;
            let dy = tx_p.position.y - tx_c.position.y;
            let dr = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
            if dr <= (cx_c.r) {
                dev!("COLLISION DETECTED");
                colliding_entities.push(circloid);
                colliding_entities.push(particle);
            }
        }
    }
    for &entity in colliding_entities {
        world.despawn(entity);
    }
}
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
