use std::time::{self, Duration};

use crate::archetypes::{gen_projectile, ArchProjectile};
use crate::game::{RunState, WindowDims};
use crate::pixel::{RED, WHITE};
use crate::time::Dt;
use crate::{components::*, dev, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};
use hecs::{Entity, Query, QueryBorrow, With, Without, World};
use nalgebra_glm::Vec2;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

// todo ai input -> rotationalinputcpt
// human input -> rotationalinputcpt

////////////////////////////////////////////////////////////////////////////////
// PROCESS INPUTS
////////////////////////////////////////////////////////////////////////////////

pub fn system_process_human_input(world: &mut World, runstate: RunState, input: &WinitInputHelper) {
    for (
        _id,
        (
            rotational_input,
            move_attributes,
            transform,
            rigidbody,
            rotatablebody,
            projectile_emitter,
        ),
    ) in world.query_mut::<With<
        (
            &mut RotationalInputCpt,
            &MoveAttributesCpt,
            &mut TransformCpt,
            &mut RigidBodyCpt,
            &mut RotatableBodyCpt,
            &mut ProjectileEmitterCpt,
        ),
        &HumanInputCpt,
    >>() {
        set_rotational_input_component_by_human(input, runstate, rotational_input);

        // set rotation_rate sign
        set_rotatablebody_component(rotational_input, rotatablebody, move_attributes);
        set_rigidbody_component(transform, rotational_input, rigidbody, move_attributes);

        if runstate == RunState::Running {
            // use not (!) keydowns to unset move inputs instead of keyups (released) because they work with low (<50) update rates (UPS)
            if input.key_pressed(VirtualKeyCode::Space) || input.key_held(VirtualKeyCode::Space) {
                projectile_emitter.intends_to_fire = true;
            } else {
                projectile_emitter.intends_to_fire = false;
            }
        }
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

// todo Ideally: key input event -> key<->control mapping -> control event or set control component
fn set_rotational_input_component_by_human(
    input: &WinitInputHelper,
    runstate: RunState,
    rotational_input: &mut RotationalInputCpt,
) {
    if runstate == RunState::Running {
        // use not (!) keydowns to unset move inputs instead of keyups (released) because they work with low (<50) update rates (UPS)
        if input.key_pressed(VirtualKeyCode::D) || input.key_held(VirtualKeyCode::D) {
            rotational_input.turn_sign = Some(Turn::Right);
        } else if input.key_pressed(VirtualKeyCode::A) || input.key_held(VirtualKeyCode::A) {
            rotational_input.turn_sign = Some(Turn::Left);
        } else {
            // explicit resets, don't depend on keyup
            rotational_input.turn_sign = None;
        }

        if input.key_pressed(VirtualKeyCode::W) || input.key_held(VirtualKeyCode::W) {
            rotational_input.is_thrusting = true;
        } else {
            // explicit resets, don't depend on keyup
            rotational_input.is_thrusting = false;
        }
    }
}
////////////////////////////////////////////////////////////////////////////////
// Projectile Creation
////////////////////////////////////////////////////////////////////////////////

pub fn system_projectile_emission(world: &mut World) {
    let mut projectiles_to_spawn: Vec<ArchProjectile> = vec![];
    for (ent, (tx, pe, cc)) in
        world.query_mut::<(&TransformCpt, &mut ProjectileEmitterCpt, &CircleColliderCpt)>()
    {
        if pe.intends_to_fire {
            let last_emit = pe.last_emission_time;
            if last_emit.elapsed().as_millis() as i32 >= pe.cooldown {
                pe.last_emission_time = time::Instant::now();
                let dx_theta = (tx.heading).cos();
                let dy_theta = (tx.heading).sin();
                let x = tx.position.x + dx_theta * cc.r;
                let y = tx.position.y + dy_theta * cc.r;
                let vx = pe.projectile_speed * dx_theta;
                let vy = pe.projectile_speed * dy_theta;
                let projectile =
                    gen_projectile(x, y, vx, vy, time::Duration::new(3, 0), pe.hit_damage, RED);
                projectiles_to_spawn.push(projectile);
            }
        }
    }
    for projectile in projectiles_to_spawn {
        world.spawn(projectile);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Integrate for Motion
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
// React to Game World Boundary
////////////////////////////////////////////////////////////////////////////////

pub fn system_boundary_restrict_circloid(world: &mut World) {
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

pub fn system_boundary_restrict_particletypes(world: &mut World) {
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

// tmp for development, keep avatars in view
pub fn test_system_boundary_restrict_particle(world: &mut World) {
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

////////////////////////////////////////////////////////////////////////////////
// Collision Detection
////////////////////////////////////////////////////////////////////////////////

pub fn system_collision_detection(world: &mut World) {
    let mut colliding_entities: Vec<(Entity, Entity)> = vec![];
    let circloid_components: Vec<(Entity, TransformCpt, CircleColliderCpt)>;
    {
        // Circloid vs Circloid
        let mut query_circloids = world.query::<(&TransformCpt, &CircleColliderCpt)>();
        circloid_components = query_circloids
            .iter()
            .map(|(e, (tx, cc))| (e, tx.clone(), cc.clone()))
            .collect::<Vec<_>>();
    }

    // todo how to query_mut and enumerate?
    for (i, (ent_a, tx_a, cc_a)) in circloid_components.iter().enumerate() {
        for (ent_b, tx_b, cc_b) in circloid_components[i + 1..].iter() {
            let dx = tx_b.position.x - tx_a.position.x;
            let dy = tx_b.position.y - tx_a.position.y;
            let dr = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
            if dr < (cc_a.r + cc_b.r) {
                colliding_entities.push((*ent_a, *ent_b));
            }
        }
    }
    {
        let mut query_projectiles = world.query::<With<&TransformCpt, &ParticleColliderCpt>>();
        let projectile_components = query_projectiles
            .iter()
            .map(|(e, tx)| (e, tx))
            .collect::<Vec<_>>();

        for (i, (circloid, tx_c, cc_c)) in circloid_components.iter().enumerate() {
            for (projectile, tx_p) in projectile_components.iter() {
                let dx = tx_p.position.x - tx_c.position.x;
                let dy = tx_p.position.y - tx_c.position.y;
                let dr = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
                if dr <= (cc_c.r) {
                    colliding_entities.push((*circloid, *projectile));
                }
            }
        }
    }

    for collision_pair in colliding_entities {
        world.spawn((CollisionDetectionEvent {
            a: collision_pair.0,
            b: collision_pair.1,
        },));
    }

    // for &entity in colliding_entities {
    //     world.despawn(entity);
    // }
}

////////////////////////////////////////////////////////////////////////////////
// Collision Resolution Dispatcher
////////////////////////////////////////////////////////////////////////////////

pub fn system_collision_resolution(world: &mut World) {
    // Different resolutions depending on the kind of collision detection event
    // e.g. Dispatches more event components to be handled by downstream systems
    // 1. vary on archetypes
    // 2. vary on event data (tbd another field on CollisionDetectionEvent)
    let mut colliding_circloids_projectiles: Vec<(Entity, Entity)> = vec![];
    {
        let mut query_collision_events = world.query::<&CollisionDetectionEvent>();
        let collision_events = query_collision_events.iter().collect::<Vec<_>>();

        for (ent, collision_event) in collision_events {
            let ent_a = collision_event.a;
            let ent_b = collision_event.b;

            // Collect collision data of interest

            // resolve projectile vs circloid
            if (world.get::<&ParticleColliderCpt>(ent_a).is_ok()
                && world.get::<&CircleColliderCpt>(ent_b).is_ok())
                || (world.get::<&CircleColliderCpt>(ent_a).is_ok()
                    && world.get::<&ParticleColliderCpt>(ent_b).is_ok())
            {
                // ? Dispatch further event components from here
                // for now just despawn as a resolution
                colliding_circloids_projectiles.push((ent_a, ent_b));
            }
            // resolve circloid vs circloid
            if (world.get::<&CircleColliderCpt>(ent_a).is_ok()
                && world.get::<&CircleColliderCpt>(ent_b).is_ok())
            {
                // ? Dispatch further event components from here
                // for now just despawn as a resolution
                colliding_circloids_projectiles.push((ent_a, ent_b));
            }
        }
    }

    // This is a reaction to a collision events, should be in another system
    for pair in colliding_circloids_projectiles.into_iter() {
        world.despawn(pair.0);
        world.despawn(pair.1);
    }
}
