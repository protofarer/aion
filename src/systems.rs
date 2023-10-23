use std::time::Duration;

use crate::game::WindowDims;
use crate::time::Dt;
use crate::{components::*, dev};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use nalgebra_glm::Vec2;

#[system(for_each)]
pub fn update_positions(
    transform: &mut TransformCpt,
    rigidbody: &mut RigidBodyCpt,
    #[resource] dt: &Duration,
) {
    // transform.position.x += rigidbody.velocity.x * dt.0;
    // transform.position.y += rigidbody.velocity.y * dt.0;
    // ! does this work?
    transform.position += rigidbody.velocity * dt.as_secs_f32();
}

#[system(for_each)]
pub fn process_translational_input(
    input: &TranslationalInputCpt,
    movestats: &MovementStatsCpt,
    rigidbody: &mut RigidBodyCpt,
) {
    match input.direction {
        Some(Direction::E) => {
            rigidbody.velocity.x = movestats.speed as f32;
            rigidbody.velocity.y = 0f32;
        }
        Some(Direction::SE) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = component_speed;
            rigidbody.velocity.y = component_speed;
        }
        Some(Direction::S) => {
            rigidbody.velocity.x = 0f32;
            rigidbody.velocity.y = movestats.speed as f32;
        }
        Some(Direction::SW) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = -component_speed;
            rigidbody.velocity.y = component_speed;
        }
        Some(Direction::W) => {
            rigidbody.velocity.x = -(movestats.speed as f32);
            rigidbody.velocity.y = 0f32;
        }
        Some(Direction::NW) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = -component_speed;
            rigidbody.velocity.y = -component_speed;
        }
        Some(Direction::N) => {
            rigidbody.velocity.x = 0f32;
            rigidbody.velocity.y = -(movestats.speed as f32);
        }
        Some(Direction::NE) => {
            let component_speed_f32 = movestats.speed as f32;
            let component_speed = (component_speed_f32.powi(2) / 2.0).sqrt();
            rigidbody.velocity.x = component_speed;
            rigidbody.velocity.y = -component_speed;
        }
        None => {
            rigidbody.velocity = Vec2::new(0.0, 0.0);
        }
    }
}

#[system(for_each)]
pub fn world_boundary_bounce_rect(
    transform: &mut TransformCpt,
    rigidbody: &mut RigidBodyCpt,
    collision_area: &BoxColliderCpt,
    #[resource] window_dims: &WindowDims,
) {
    if transform.position.x + collision_area.w as f32 >= window_dims.w as f32
        || transform.position.x < 0f32
    {
        rigidbody.velocity.x = -rigidbody.velocity.x;
    }
    if transform.position.x < 0f32 {
        transform.position.x = 0f32;
    } else if transform.position.x + collision_area.w as f32 >= window_dims.w as f32 {
        transform.position.x = (window_dims.w - collision_area.w) as f32;
    }

    if transform.position.y + collision_area.h as f32 >= window_dims.h as f32
        || transform.position.y < 0f32
    {
        rigidbody.velocity.y = -rigidbody.velocity.y;
    }
    if transform.position.y < 0f32 {
        transform.position.y = 0f32;
    } else if transform.position.y + collision_area.h as f32 >= window_dims.h as f32 {
        transform.position.y = (window_dims.h - collision_area.h) as f32;
    }
}

#[system(for_each)]
pub fn world_boundary_bounce_circle(
    transform: &mut TransformCpt,
    rigidbody: &mut RigidBodyCpt,
    collision_circle: &CircleColliderCpt,
    #[resource] window_dims: &WindowDims,
) {
    if transform.position.x + collision_circle.r as f32 >= window_dims.w as f32
        || transform.position.x - collision_circle.r < 0f32
    {
        rigidbody.velocity.x = -rigidbody.velocity.x;
    }
    if transform.position.x - collision_circle.r < 0f32 {
        transform.position.x = 0f32 + collision_circle.r;
    } else if transform.position.x + collision_circle.r as f32 >= window_dims.w as f32 {
        transform.position.x = (window_dims.w - collision_circle.r) as f32;
    }

    if transform.position.y + collision_circle.r as f32 >= window_dims.h as f32
        || transform.position.y < 0f32
    {
        rigidbody.velocity.y = -rigidbody.velocity.y;
    }
    if transform.position.y - collision_circle.r < 0f32 {
        transform.position.y = 0f32 + collision_circle.r;
    } else if transform.position.y + collision_circle.r as f32 >= window_dims.h as f32 {
        transform.position.y = (window_dims.h - collision_circle.r) as f32;
    }
}

// ? how to have this run only when input changes?
#[system(for_each)]
pub fn process_rotational_input(
    rotational_input: &RotationalInputCpt,
    move_stats: &MovementStatsCpt,
    transform: &mut TransformCpt,
    rigidbody: &mut RigidBodyCpt,
) {
    match rotational_input.turn_sign {
        Some(Turn::Right) => {
            transform.rotation =
                (transform.rotation + move_stats.turn_rate) % (2.0 * nalgebra_glm::pi::<f32>());
        }
        Some(Turn::Left) => {
            transform.rotation =
                (transform.rotation - move_stats.turn_rate) % (2.0 * nalgebra_glm::pi::<f32>());
        }
        _ => {}
    }
    match rotational_input.is_thrusting {
        true => {
            rigidbody.velocity.x = (transform.rotation).cos() * move_stats.speed as f32;
            rigidbody.velocity.y = (transform.rotation).sin() * move_stats.speed as f32;
        }
        false => {
            // test when decel cross 0 in either direction
            // todo don't hard set here, simply make no change, for maintaining momentum in prototype stage
            // todo repeats when no key is pressed...
            rigidbody.velocity = Vec2::default();
        }
    }
}

#[system(for_each)]
pub fn projectile_emission(
    pem: &ProjectileEmitter,
    proj: &Projectile,
    transform: &TransformCpt,
    rigidbody: &RigidBodyCpt,
) {
    // todo how to emit projectile on key press (player.is_firing_primary)
    // only fire if cooldown is up
}

#[system]
pub fn circle_collision(
    query: &mut Query<(Entity, &TransformCpt, &CircleColliderCpt)>,
    commands: &mut CommandBuffer,
    world: &mut SubWorld,
) {
    let mut entities: Vec<(&Entity, &TransformCpt, &CircleColliderCpt)> = vec![];
    let chunks = query;
    for (entity_a, tx, cx) in chunks.iter(world) {
        entities.push((entity_a, tx, cx));
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
        commands.remove(entity);
    }

    // for (i, entity1, transform, collision_circle) in query.iter(world).enumerate() {
    //     for (entity2, transform, collision_circle) in )

    // }
}
